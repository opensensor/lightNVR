unsafe extern "C" {
    unsafe fn system_health_capability_name(
        capability: system_health_capability_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn system_health_unit_name(unit: system_health_unit_t) -> *const ::core::ffi::c_char;
    unsafe fn system_health_condition_code(
        condition: system_health_condition_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn system_health_condition_from_code(
        code: *const ::core::ffi::c_char,
        condition_out: *mut system_health_condition_t,
    ) -> bool;
    unsafe fn system_health_observation_set_available(
        observation: *mut system_health_observation_t,
        value: ::core::ffi::c_double,
        unit: system_health_unit_t,
    );
    unsafe fn system_health_observation_set_unavailable(
        observation: *mut system_health_observation_t,
        capability: system_health_capability_t,
    );
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_system_health_incident_apply(
        action: system_health_incident_action_t,
        signal: *const system_health_incident_signal_t,
        incident_out: *mut system_health_incident_record_t,
    ) -> db_system_health_result_t;
    unsafe fn db_system_health_incident_get_uuid(
        incident_uuid: *const ::core::ffi::c_char,
        incident_out: *mut system_health_incident_record_t,
    ) -> db_system_health_result_t;
    unsafe fn db_system_health_incident_list(
        include_closed: bool,
        cursor: *const system_health_incident_cursor_t,
        incidents: *mut system_health_incident_record_t,
        max_count: ::core::ffi::c_int,
        next_cursor: *mut system_health_incident_cursor_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_system_health_transition_list_pending(
        after_id: int64_t,
        transitions: *mut system_health_incident_transition_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_system_health_transition_set_reconciliation(
        incident_uuid: *const ::core::ffi::c_char,
        event_id: *const ::core::ffi::c_char,
        reconciliation: system_health_reconciliation_t,
    ) -> db_system_health_result_t;
    unsafe fn db_system_health_run_open(
        run_id: *const ::core::ffi::c_char,
        boot_id: *const ::core::ffi::c_char,
        started_at_ms: int64_t,
        run_out: *mut system_health_process_run_t,
        previous_out: *mut system_health_process_run_t,
        had_previous: *mut bool,
    ) -> db_system_health_result_t;
    unsafe fn db_system_health_run_close(
        run_id: *const ::core::ffi::c_char,
        closed_at_ms: int64_t,
    ) -> db_system_health_result_t;
    unsafe fn system_health_policy_snapshot(
        snapshot: *mut system_health_policy_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_ParseWithOpts(
        value: *const ::core::ffi::c_char,
        return_parse_end: *mut *const ::core::ffi::c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn event_producer_publish_system_health_transition(
        transition: *const system_health_transition_t,
        event_id: *const ::core::ffi::c_char,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn linux_restart_read_evidence(
        proc_root: *const ::core::ffi::c_char,
        run_id: *const ::core::ffi::c_char,
        process_start_monotonic_ms: uint64_t,
        evidence: *mut linux_restart_evidence_t,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_snapshot_copy(snapshot: *mut system_health_snapshot_t) -> bool;
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
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct db_system_health_result_t(pub ::core::ffi::c_int);
impl db_system_health_result_t {
    pub const DB_SYSTEM_HEALTH_OK: Self = Self(0);
    pub const DB_SYSTEM_HEALTH_RESUMED: Self = Self(1);
    pub const DB_SYSTEM_HEALTH_NOT_FOUND: Self = Self(2);
    pub const DB_SYSTEM_HEALTH_INVALID: Self = Self(-2);
    pub const DB_SYSTEM_HEALTH_CONFLICT: Self = Self(-3);
    pub const DB_SYSTEM_HEALTH_ERROR: Self = Self(-1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_incident_action_t(pub ::core::ffi::c_uint);
impl system_health_incident_action_t {
    pub const SYSTEM_HEALTH_INCIDENT_OPEN: Self = Self(0);
    pub const SYSTEM_HEALTH_INCIDENT_ESCALATE: Self = Self(1);
    pub const SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE: Self = Self(2);
    pub const SYSTEM_HEALTH_INCIDENT_RECOVER: Self = Self(3);
    pub const SYSTEM_HEALTH_INCIDENT_ONE_SHOT: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_reconciliation_t(pub ::core::ffi::c_uint);
impl system_health_reconciliation_t {
    pub const SYSTEM_HEALTH_RECONCILIATION_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING: Self = Self(1);
    pub const SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING: Self = Self(2);
    pub const SYSTEM_HEALTH_RECONCILIATION_RECONCILED: Self = Self(3);
    pub const SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_signal_t {
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub observed_at_ms: int64_t,
    pub observation_json: [::core::ffi::c_char; 2049],
    pub event_id: [::core::ffi::c_char; 37],
    pub reconciliation: system_health_reconciliation_t,
    pub boot_id: [::core::ffi::c_char; 64],
    pub run_id: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_record_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub condition_code: [::core::ffi::c_char; 64],
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub first_seen_at_ms: int64_t,
    pub last_seen_at_ms: int64_t,
    pub closed_at_ms: int64_t,
    pub observation_json: [::core::ffi::c_char; 2049],
    pub alert_event_id: [::core::ffi::c_char; 37],
    pub recovery_event_id: [::core::ffi::c_char; 37],
    pub reconciliation: system_health_reconciliation_t,
    pub boot_id: [::core::ffi::c_char; 64],
    pub run_id: [::core::ffi::c_char; 37],
    pub revision: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_cursor_t {
    pub valid: bool,
    pub last_seen_at_ms: int64_t,
    pub uuid: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_transition_t {
    pub id: int64_t,
    pub transition_uuid: [::core::ffi::c_char; 37],
    pub incident_uuid: [::core::ffi::c_char; 37],
    pub action: system_health_incident_action_t,
    pub from_state: system_health_state_t,
    pub from_state_valid: bool,
    pub to_state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub observed_at_ms: int64_t,
    pub observation_json: [::core::ffi::c_char; 2049],
    pub event_id: [::core::ffi::c_char; 37],
    pub reconciliation: system_health_reconciliation_t,
    pub boot_id: [::core::ffi::c_char; 64],
    pub run_id: [::core::ffi::c_char; 37],
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
pub struct system_health_profile_t(pub ::core::ffi::c_uint);
impl system_health_profile_t {
    pub const SYSTEM_HEALTH_PROFILE_BALANCED: Self = Self(0);
    pub const SYSTEM_HEALTH_PROFILE_CONSERVATIVE: Self = Self(1);
    pub const SYSTEM_HEALTH_PROFILE_DISABLED: Self = Self(2);
    pub const SYSTEM_HEALTH_PROFILE_CUSTOM: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_threshold_direction_t(pub ::core::ffi::c_uint);
impl system_health_threshold_direction_t {
    pub const SYSTEM_HEALTH_THRESHOLD_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE: Self = Self(1);
    pub const SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE: Self = Self(2);
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
pub struct system_health_condition_policy_t {
    pub condition: system_health_condition_t,
    pub profile: system_health_profile_t,
    pub enabled: bool,
    pub overridden: bool,
    pub direction: system_health_threshold_direction_t,
    pub unit: system_health_unit_t,
    pub warning_threshold: ::core::ffi::c_double,
    pub critical_threshold: ::core::ffi::c_double,
    pub recovery_threshold: ::core::ffi::c_double,
    pub warning_for_seconds: uint32_t,
    pub critical_for_seconds: uint32_t,
    pub recovery_for_seconds: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_t {
    pub generation: uint64_t,
    pub settings: system_health_policy_settings_t,
    pub conditions: [system_health_condition_policy_t; 27],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_evaluator {
    pub config: system_health_evaluator_config_t,
    pub slots: [evaluator_slot_t; 64],
    pub availability: [availability_slot_t; 128],
    pub stats: system_health_evaluator_stats_t,
    pub last_snapshot_sequence: uint64_t,
    pub view_lock: pthread_mutex_t,
    pub views: [[system_health_incident_view_t; 64]; 2],
    pub view_counts: [size_t; 2],
    pub active_view: ::core::ffi::c_uint,
    pub published_stats: system_health_evaluator_stats_t,
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
pub struct availability_slot_t {
    pub used: bool,
    pub available_ever: bool,
    pub metric: [::core::ffi::c_char; 64],
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub unavailable_since_ms: uint64_t,
    pub seen_sequence: uint64_t,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evaluator_slot_t {
    pub used: bool,
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub pending_severity: system_health_severity_t,
    pub pending_since_ms: uint64_t,
    pub recovery_since_ms: uint64_t,
    pub last_material_ms: uint64_t,
    pub last_material_value: ::core::ffi::c_double,
    pub last_material_value_valid: bool,
    pub incident_id: [::core::ffi::c_char; 37],
    pub first_observed_wall_ms: int64_t,
    pub first_observed_monotonic_ms: uint64_t,
    pub last_observed_wall_ms: int64_t,
    pub last_observation: system_health_observation_t,
    pub last_episode_sample_ms: uint64_t,
    pub last_persisted_wall_ms: int64_t,
    pub persistence_pending: bool,
    pub pending_action: system_health_incident_action_t,
    pub pending_previous_state: system_health_state_t,
    pub pending_previous_severity: system_health_severity_t,
    pub pending_event_id: [::core::ffi::c_char; 37],
    pub pending_target_state: system_health_state_t,
    pub pending_target_severity: system_health_severity_t,
    pub pending_candidate: evaluator_candidate_t,
    pub pending_wall_ms: int64_t,
    pub next_retry_ms: uint64_t,
    pub retry_delay_ms: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evaluator_candidate_t {
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub observation: system_health_observation_t,
    pub desired: system_health_severity_t,
    pub recovery_safe: bool,
    pub immediate: bool,
    pub one_shot: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_evaluator_config_t {
    pub policy: system_health_policy_t,
    pub boot_id: [::core::ffi::c_char; 64],
    pub run_id: [::core::ffi::c_char; 37],
    pub stale_after_ms: uint32_t,
    pub retry_initial_ms: uint32_t,
    pub retry_max_ms: uint32_t,
    pub material_change_debounce_ms: uint32_t,
    pub material_change_ratio: ::core::ffi::c_double,
    pub persist: system_health_persist_transition_fn,
    pub persist_context: *mut ::core::ffi::c_void,
    pub transition_sink: system_health_transition_sink_fn,
    pub transition_sink_context: *mut ::core::ffi::c_void,
}
pub type system_health_transition_sink_fn =
    Option<unsafe extern "C" fn(*const system_health_transition_t, *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_transition_t {
    pub action: system_health_incident_action_t,
    pub incident_id: [::core::ffi::c_char; 37],
    pub event_id: [::core::ffi::c_char; 37],
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub previous_state: system_health_state_t,
    pub state: system_health_state_t,
    pub previous_severity: system_health_severity_t,
    pub severity: system_health_severity_t,
    pub observation: system_health_observation_t,
    pub threshold_direction: system_health_threshold_direction_t,
    pub threshold_value: ::core::ffi::c_double,
    pub threshold_for_ms: uint32_t,
    pub first_observed_at_ms: int64_t,
    pub incident_duration_ms: uint64_t,
    pub observed_at_ms: int64_t,
    pub persisted: bool,
}
pub type system_health_persist_transition_fn = Option<
    unsafe extern "C" fn(
        system_health_incident_action_t,
        *const system_health_incident_signal_t,
        *mut system_health_incident_record_t,
        *mut ::core::ffi::c_void,
    ) -> db_system_health_result_t,
>;
pub type system_health_evaluator_t = system_health_evaluator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_evaluation_context_t {
    pub recording_expected_known: bool,
    pub recording_expected: bool,
    pub service_degraded_known: bool,
    pub service_degraded: bool,
    pub viable_recording_targets_known: bool,
    pub viable_recording_targets: uint32_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_immediate_resource_t(pub ::core::ffi::c_uint);
impl system_health_immediate_resource_t {
    pub const SYSTEM_HEALTH_IMMEDIATE_PROCESS: Self = Self(0);
    pub const SYSTEM_HEALTH_IMMEDIATE_ROOT: Self = Self(1);
    pub const SYSTEM_HEALTH_IMMEDIATE_RECORDING: Self = Self(2);
    pub const SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_restart_classification_t(pub ::core::ffi::c_uint);
impl system_health_restart_classification_t {
    pub const SYSTEM_HEALTH_RESTART_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_RESTART_PROCESS: Self = Self(1);
    pub const SYSTEM_HEALTH_RESTART_HOST: Self = Self(2);
}
pub type atomic_uint_fast64_t = uint_fast64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evaluator_service_t {
    pub lock: pthread_mutex_t,
    pub wake: pthread_cond_t,
    pub thread: pthread_t,
    pub initialized: bool,
    pub running: bool,
    pub disabled: bool,
    pub evaluator: *mut system_health_evaluator_t,
    pub context: system_health_evaluation_context_t,
    pub run: system_health_process_run_t,
    pub transition_sink: system_health_transition_sink_fn,
    pub transition_sink_context: *mut ::core::ffi::c_void,
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
pub struct linux_restart_evidence_t {
    pub capability: system_health_capability_t,
    pub boot_id: [::core::ffi::c_char; 37],
    pub run_id: [::core::ffi::c_char; 37],
    pub host_uptime_seconds: ::core::ffi::c_double,
    pub process_start_monotonic_ms: uint64_t,
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
pub const SYSTEM_HEALTH_MAX_INCIDENTS: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g1_start: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g_size: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __g1_orig_size: 0 as ::core::ffi::c_uint,
        __wrefs: 0 as ::core::ffi::c_uint,
        __g_signals: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __unused_initialized_1: 0 as ::core::ffi::c_uint,
        __unused_initialized_2: 0 as ::core::ffi::c_uint,
    },
};
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EVALUATOR_CANDIDATE_MAX: ::core::ffi::c_uint = 96 as ::core::ffi::c_uint;
pub const EVALUATOR_AVAILABILITY_MAX: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
static mut immediate_counts: [[atomic_uint_fast64_t; 3]; 27] = [[0; 3]; 27];
unsafe extern "C" fn monotonic_ms() -> uint64_t {
    let mut value: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_MONOTONIC, &raw mut value) != 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    return (value.tv_sec as uint64_t)
        .wrapping_mul(1000 as uint64_t)
        .wrapping_add((value.tv_nsec as uint64_t).wrapping_div(1000000 as uint64_t));
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
unsafe extern "C" fn copy_string(
    mut destination: *mut ::core::ffi::c_char,
    mut capacity: size_t,
    mut source: *const ::core::ffi::c_char,
) {
    if destination.is_null() || capacity == 0 as size_t {
        return;
    }
    snprintf(
        destination,
        capacity,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !source.is_null() {
            source
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_config_defaults(
    mut config: *mut system_health_evaluator_config_t,
    mut policy: *const system_health_policy_t,
) {
    if config.is_null() {
        return;
    }
    memset(
        config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_evaluator_config_t>(),
    );
    if !policy.is_null() {
        (*config).policy = *policy;
    }
    (*config).stale_after_ms = 180000 as ::core::ffi::c_uint as uint32_t;
    (*config).retry_initial_ms = 1000 as ::core::ffi::c_uint as uint32_t;
    (*config).retry_max_ms = 60000 as ::core::ffi::c_uint as uint32_t;
    (*config).material_change_debounce_ms = 300000 as ::core::ffi::c_uint as uint32_t;
    (*config).material_change_ratio = 0.10f64;
    copy_string(
        &raw mut (*config).boot_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"unknown-boot\0".as_ptr() as *const ::core::ffi::c_char,
    );
    lightnvr_uuid_generate_v4(&raw mut (*config).run_id as *mut ::core::ffi::c_char);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_create(
    mut config: *const system_health_evaluator_config_t,
) -> *mut system_health_evaluator_t {
    if config.is_null()
        || (*config).boot_id[0usize] == 0
        || !lightnvr_uuid_is_valid(&raw const (*config).run_id as *const ::core::ffi::c_char)
        || (*config).retry_initial_ms == 0 as uint32_t
        || (*config).retry_max_ms < (*config).retry_initial_ms
        || (*config).material_change_ratio <= 0.0f64
        || (*config).material_change_ratio.is_finite() as ::core::ffi::c_int == 0
    {
        return ::core::ptr::null_mut::<system_health_evaluator_t>();
    }
    let mut evaluator: *mut system_health_evaluator_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<system_health_evaluator_t>(),
    ) as *mut system_health_evaluator_t;
    if evaluator.is_null() {
        return ::core::ptr::null_mut::<system_health_evaluator_t>();
    }
    if pthread_mutex_init(
        &raw mut (*evaluator).view_lock,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        free(evaluator as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<system_health_evaluator_t>();
    }
    (*evaluator).config = *config;
    return evaluator;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_destroy(
    mut evaluator: *mut system_health_evaluator_t,
) {
    if !evaluator.is_null() {
        pthread_mutex_destroy(&raw mut (*evaluator).view_lock);
    }
    free(evaluator as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn find_slot(
    mut evaluator: *mut system_health_evaluator_t,
    mut condition: system_health_condition_t,
    mut subject: *const ::core::ffi::c_char,
    mut create: bool,
) -> *mut evaluator_slot_t {
    let mut empty: *mut evaluator_slot_t = ::core::ptr::null_mut::<evaluator_slot_t>();
    let mut index: size_t = 0 as size_t;
    while index < SYSTEM_HEALTH_MAX_INCIDENTS as size_t {
        let mut slot: *mut evaluator_slot_t =
            (&raw mut (*evaluator).slots as *mut evaluator_slot_t).offset(index as isize);
        if (*slot).used as ::core::ffi::c_int != 0
            && (*slot).condition.0 == condition.0
            && strcmp(
                &raw mut (*slot).subject as *mut ::core::ffi::c_char,
                subject,
            ) == 0 as ::core::ffi::c_int
        {
            return slot;
        }
        if !(*slot).used && empty.is_null() {
            empty = slot;
        }
        index = index.wrapping_add(1);
    }
    if !create || empty.is_null() {
        return ::core::ptr::null_mut::<evaluator_slot_t>();
    }
    memset(
        empty as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<evaluator_slot_t>(),
    );
    (*empty).used = r#true != 0;
    (*empty).condition = condition;
    copy_string(
        &raw mut (*empty).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        subject,
    );
    (*empty).state = system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN;
    (*evaluator).stats.tracked_conditions = (*evaluator).stats.tracked_conditions.wrapping_add(1);
    return empty;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_resume(
    mut evaluator: *mut system_health_evaluator_t,
    mut incident: *const system_health_incident_record_t,
) -> ::core::ffi::c_int {
    let mut condition: system_health_condition_t =
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW;
    if evaluator.is_null()
        || incident.is_null()
        || (*incident).uuid[0usize] == 0
        || !system_health_condition_from_code(
            &raw const (*incident).condition_code as *const ::core::ffi::c_char,
            &raw mut condition,
        )
        || (*incident).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED.0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut slot: *mut evaluator_slot_t = find_slot(
        evaluator,
        condition,
        &raw const (*incident).subject as *const ::core::ffi::c_char,
        r#true != 0,
    );
    if slot.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (*slot).scope = (*incident).scope;
    (*slot).state = (*incident).state;
    (*slot).severity = (*incident).severity;
    (*slot).last_persisted_wall_ms = (*incident).last_seen_at_ms;
    (*slot).first_observed_wall_ms = (*incident).first_seen_at_ms;
    (*slot).last_observed_wall_ms = (*incident).last_seen_at_ms;
    copy_string(
        &raw mut (*slot).incident_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        &raw const (*incident).uuid as *const ::core::ffi::c_char,
    );
    (*evaluator).stats.active_incidents = (*evaluator).stats.active_incidents.wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn find_observation(
    mut snapshot: *const system_health_snapshot_t,
    mut metric: *const ::core::ffi::c_char,
    mut resource: *const ::core::ffi::c_char,
) -> *const system_health_observation_t {
    let mut index: size_t = 0 as size_t;
    while index < (*snapshot).observation_count {
        let mut observation: *const system_health_observation_t =
            (&raw const (*snapshot).observations as *const system_health_observation_t)
                .offset(index as isize);
        if strcmp(
            &raw const (*observation).metric as *const ::core::ffi::c_char,
            metric,
        ) == 0 as ::core::ffi::c_int
            && (resource.is_null()
                || strcmp(
                    &raw const (*observation).resource_id as *const ::core::ffi::c_char,
                    resource,
                ) == 0 as ::core::ffi::c_int)
        {
            return observation;
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<system_health_observation_t>();
}
unsafe extern "C" fn metric_suffix(
    mut metric: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> bool {
    let mut metric_length: size_t = strlen(metric);
    let mut suffix_length: size_t = strlen(suffix);
    return metric_length >= suffix_length
        && strcmp(
            metric
                .offset(metric_length as isize)
                .offset(-(suffix_length as isize)),
            suffix,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_candidate(
    mut candidates: *mut evaluator_candidate_t,
    mut count: *mut size_t,
    mut condition: system_health_condition_t,
    mut subject: *const ::core::ffi::c_char,
    mut observation: *const system_health_observation_t,
) -> *mut evaluator_candidate_t {
    let mut index: size_t = 0 as size_t;
    while index < *count {
        if (*candidates.offset(index as isize)).condition.0 == condition.0
            && strcmp(
                &raw mut (*candidates.offset(index as isize)).subject as *mut ::core::ffi::c_char,
                subject,
            ) == 0 as ::core::ffi::c_int
        {
            return candidates.offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    if *count >= EVALUATOR_CANDIDATE_MAX as size_t {
        return ::core::ptr::null_mut::<evaluator_candidate_t>();
    }
    let c2rust_fresh1 = *count;
    *count = (*count).wrapping_add(1);
    let mut candidate: *mut evaluator_candidate_t = candidates.offset(c2rust_fresh1 as isize);
    memset(
        candidate as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<evaluator_candidate_t>(),
    );
    (*candidate).condition = condition;
    copy_string(
        &raw mut (*candidate).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        subject,
    );
    if !observation.is_null() {
        (*candidate).scope = (*observation).scope;
        (*candidate).observation = *observation;
    }
    return candidate;
}
unsafe extern "C" fn apply_threshold(
    mut rule: *const system_health_condition_policy_t,
    mut candidate: *mut evaluator_candidate_t,
) {
    if candidate.is_null()
        || !(*candidate).observation.value_valid
        || !(*rule).enabled
        || (*rule).direction.0
            == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE.0
        || (*candidate).observation.unit.0 != (*rule).unit.0
    {
        return;
    }
    let mut value: ::core::ffi::c_double = (*candidate).observation.value;
    let mut critical: bool = if (*rule).direction.0
        == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE.0
    {
        (value < (*rule).critical_threshold) as ::core::ffi::c_int
    } else {
        (value > (*rule).critical_threshold) as ::core::ffi::c_int
    } != 0;
    let mut warning: bool = if (*rule).direction.0
        == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE.0
    {
        (value < (*rule).warning_threshold) as ::core::ffi::c_int
    } else {
        (value > (*rule).warning_threshold) as ::core::ffi::c_int
    } != 0;
    (*candidate).recovery_safe = if (*rule).direction.0
        == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE.0
    {
        (value > (*rule).recovery_threshold) as ::core::ffi::c_int
    } else {
        (value < (*rule).recovery_threshold) as ::core::ffi::c_int
    } != 0;
    (*candidate).desired = system_health_severity_t(
        (if critical as ::core::ffi::c_int != 0 {
            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0 as ::core::ffi::c_int
        } else if warning as ::core::ffi::c_int != 0 {
            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0 as ::core::ffi::c_int
        } else {
            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn ratio_observation(
    mut available: *const system_health_observation_t,
    mut capacity: *const system_health_observation_t,
    mut metric: *const ::core::ffi::c_char,
) -> system_health_observation_t {
    let mut result: system_health_observation_t = *available;
    copy_string(
        &raw mut result.metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        metric,
    );
    if !(*available).value_valid
        || capacity.is_null()
        || !(*capacity).value_valid
        || (*capacity).value <= 0.0f64
    {
        result.value_valid = r#false != 0;
        result.value = 0.0f64;
        result.capability = system_health_capability_t(
            if (*available).capability.0
                != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
            {
                (*available).capability.0
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            },
        );
    } else {
        result.value = (*available).value / (*capacity).value;
        result.unit = system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO;
    }
    return result;
}
unsafe extern "C" fn availability_slot(
    mut evaluator: *mut system_health_evaluator_t,
    mut observation: *const system_health_observation_t,
) -> *mut availability_slot_t {
    let mut empty: *mut availability_slot_t = ::core::ptr::null_mut::<availability_slot_t>();
    let mut index: size_t = 0 as size_t;
    while index < EVALUATOR_AVAILABILITY_MAX as size_t {
        let mut slot: *mut availability_slot_t =
            (&raw mut (*evaluator).availability as *mut availability_slot_t).offset(index as isize);
        if (*slot).used as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*slot).metric as *mut ::core::ffi::c_char,
                &raw const (*observation).metric as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut (*slot).subject as *mut ::core::ffi::c_char,
                &raw const (*observation).resource_id as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            return slot;
        }
        if !(*slot).used && empty.is_null() {
            empty = slot;
        }
        index = index.wrapping_add(1);
    }
    if empty.is_null() {
        return ::core::ptr::null_mut::<availability_slot_t>();
    }
    memset(
        empty as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<availability_slot_t>(),
    );
    (*empty).used = r#true != 0;
    copy_string(
        &raw mut (*empty).metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw const (*observation).metric as *const ::core::ffi::c_char,
    );
    copy_string(
        &raw mut (*empty).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw const (*observation).resource_id as *const ::core::ffi::c_char,
    );
    (*empty).scope = (*observation).scope;
    return empty;
}
unsafe extern "C" fn candidate_from_observation(
    mut evaluator: *mut system_health_evaluator_t,
    mut candidates: *mut evaluator_candidate_t,
    mut count: *mut size_t,
    mut condition: system_health_condition_t,
    mut subject: *const ::core::ffi::c_char,
    mut observation: *const system_health_observation_t,
) -> *mut evaluator_candidate_t {
    let mut candidate: *mut evaluator_candidate_t =
        add_candidate(candidates, count, condition, subject, observation);
    if !candidate.is_null() {
        apply_threshold(
            (&raw mut (*evaluator).config.policy.conditions
                as *mut system_health_condition_policy_t)
                .offset(condition.0 as isize),
            candidate,
        );
    }
    return candidate;
}
unsafe extern "C" fn merge_candidate_evidence(
    mut candidate: *mut evaluator_candidate_t,
    mut observation: *const system_health_observation_t,
    mut desired: system_health_severity_t,
    mut recovery_safe: bool,
    mut immediate: bool,
    mut one_shot: bool,
) {
    if candidate.is_null() || observation.is_null() {
        return;
    }
    let mut had_evidence: bool = (*candidate).desired.0
        != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
        || (*candidate).recovery_safe as ::core::ffi::c_int != 0
        || (*candidate).immediate as ::core::ffi::c_int != 0
        || (*candidate).one_shot as ::core::ffi::c_int != 0;
    let mut previous_desired: system_health_severity_t = (*candidate).desired;
    if desired.0 > (*candidate).desired.0 {
        (*candidate).desired = desired;
        (*candidate).scope = (*observation).scope;
        (*candidate).observation = *observation;
    }
    if desired.0 != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0 {
        (*candidate).recovery_safe = r#false != 0;
    } else if !had_evidence {
        (*candidate).recovery_safe = recovery_safe;
    } else if previous_desired.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0 {
        (*candidate).recovery_safe = (*candidate).recovery_safe as ::core::ffi::c_int != 0
            && recovery_safe as ::core::ffi::c_int != 0;
    }
    (*candidate).immediate =
        (*candidate).immediate as ::core::ffi::c_int != 0 || immediate as ::core::ffi::c_int != 0;
    (*candidate).one_shot = if had_evidence as ::core::ffi::c_int != 0 {
        ((*candidate).one_shot as ::core::ffi::c_int != 0 && one_shot as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int
    } else {
        one_shot as ::core::ffi::c_int
    } != 0;
}
unsafe extern "C" fn observation_available(mut value: *const system_health_observation_t) -> bool {
    return !value.is_null()
        && (*value).capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && (*value).freshness.0 != system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_STALE.0
        && (*value).value_valid as ::core::ffi::c_int != 0
        && (*value).value.is_finite() as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn collect_stale_candidates(
    mut evaluator: *mut system_health_evaluator_t,
    mut snapshot: *const system_health_snapshot_t,
    mut candidates: *mut evaluator_candidate_t,
    mut candidate_count: *mut size_t,
) {
    let mut now: uint64_t = (*snapshot).completed_monotonic_ms;
    let mut index: size_t = 0 as size_t;
    while index < (*snapshot).observation_count {
        let mut observation: *const system_health_observation_t =
            (&raw const (*snapshot).observations as *const system_health_observation_t)
                .offset(index as isize);
        let mut slot: *mut availability_slot_t = availability_slot(evaluator, observation);
        if !slot.is_null() {
            (*slot).seen_sequence = (*snapshot).sequence;
            (*slot).capability = (*observation).capability;
            if observation_available(observation) {
                (*slot).available_ever = r#true != 0;
                (*slot).unavailable_since_ms = 0 as uint64_t;
            } else if (*slot).available_ever as ::core::ffi::c_int != 0
                && (*slot).unavailable_since_ms == 0 as uint64_t
            {
                (*slot).unavailable_since_ms = now;
            }
        }
        index = index.wrapping_add(1);
    }
    let mut index_0: size_t = 0 as size_t;
    while index_0 < EVALUATOR_AVAILABILITY_MAX as size_t {
        let mut availability: *mut availability_slot_t = (&raw mut (*evaluator).availability
            as *mut availability_slot_t)
            .offset(index_0 as isize);
        if !(!(*availability).used || !(*availability).available_ever) {
            let mut candidate: *mut evaluator_candidate_t = add_candidate(
                candidates,
                candidate_count,
                system_health_condition_t::SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE,
                &raw mut (*availability).subject as *mut ::core::ffi::c_char,
                ::core::ptr::null::<system_health_observation_t>(),
            );
            if !candidate.is_null() {
                (*candidate).scope = (*availability).scope;
                copy_string(
                    &raw mut (*candidate).observation.metric as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    b"collector.freshness\0".as_ptr() as *const ::core::ffi::c_char,
                );
                copy_string(
                    &raw mut (*candidate).observation.resource_id as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    &raw mut (*availability).subject as *mut ::core::ffi::c_char,
                );
                (*candidate).observation.scope = (*availability).scope;
                (*candidate).observation.sampled_monotonic_ms = now;
                (*candidate).observation.observed_wall_time_ms = (*snapshot).completed_wall_time_ms;
                let mut overdue: bool = (*availability).unavailable_since_ms > 0 as uint64_t
                    && now >= (*availability).unavailable_since_ms
                    && now.wrapping_sub((*availability).unavailable_since_ms)
                        >= (*evaluator).config.stale_after_ms as uint64_t;
                if overdue {
                    (*candidate).desired = system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING;
                    (*candidate).observation.capability = (*availability).capability;
                    (*candidate).observation.freshness =
                        system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_STALE;
                    (*candidate).observation.value_valid = r#false != 0;
                    (*candidate).recovery_safe = r#false != 0;
                }
            }
        }
        index_0 = index_0.wrapping_add(1);
    }
    let mut candidate_index: size_t = 0 as size_t;
    while candidate_index < *candidate_count {
        let mut candidate_0: *mut evaluator_candidate_t =
            candidates.offset(candidate_index as isize);
        if (*candidate_0).condition.0
            == system_health_condition_t::SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE.0
        {
            let mut all_available: bool = r#true != 0;
            let mut index_1: size_t = 0 as size_t;
            while index_1 < EVALUATOR_AVAILABILITY_MAX as size_t {
                let mut availability_0: *mut availability_slot_t =
                    (&raw mut (*evaluator).availability as *mut availability_slot_t)
                        .offset(index_1 as isize);
                if (*availability_0).used as ::core::ffi::c_int != 0
                    && (*availability_0).available_ever as ::core::ffi::c_int != 0
                    && strcmp(
                        &raw mut (*availability_0).subject as *mut ::core::ffi::c_char,
                        &raw mut (*candidate_0).subject as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    && (*availability_0).unavailable_since_ms != 0 as uint64_t
                {
                    all_available = r#false != 0;
                    break;
                } else {
                    index_1 = index_1.wrapping_add(1);
                }
            }
            (*candidate_0).recovery_safe = all_available;
            if all_available {
                system_health_observation_set_available(
                    &raw mut (*candidate_0).observation,
                    1.0f64,
                    system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
                );
            }
        }
        candidate_index = candidate_index.wrapping_add(1);
    }
}
unsafe extern "C" fn build_candidates(
    mut evaluator: *mut system_health_evaluator_t,
    mut snapshot: *const system_health_snapshot_t,
    mut context: *const system_health_evaluation_context_t,
    mut candidates: *mut evaluator_candidate_t,
    mut candidate_count: *mut size_t,
) {
    *candidate_count = 0 as size_t;
    collect_stale_candidates(evaluator, snapshot, candidates, candidate_count);
    let mut memory: *const system_health_observation_t =
        ::core::ptr::null::<system_health_observation_t>();
    let mut cpu: *const system_health_observation_t =
        ::core::ptr::null::<system_health_observation_t>();
    let mut cpu_pressure: bool = r#false != 0;
    let mut cpu_full_pressure: bool = r#false != 0;
    let mut io_full_pressure: bool = r#false != 0;
    let mut startup_grace: bool = r#false != 0;
    let mut index: size_t = 0 as size_t;
    while index < (*snapshot).observation_count {
        let mut observation: *const system_health_observation_t =
            (&raw const (*snapshot).observations as *const system_health_observation_t)
                .offset(index as isize);
        if metric_suffix(
            &raw const (*observation).metric as *const ::core::ffi::c_char,
            b"memory.available_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            && observation_available(observation) as ::core::ffi::c_int != 0
            && (memory.is_null()
                || (*observation).scope.0 == system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER.0)
        {
            memory = observation;
        }
        if (strcmp(
            &raw const (*observation).metric as *const ::core::ffi::c_char,
            b"host.cpu.busy_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || metric_suffix(
                &raw const (*observation).metric as *const ::core::ffi::c_char,
                b"cpu.usage_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0)
            && observation_available(observation) as ::core::ffi::c_int != 0
            && (cpu.is_null()
                || (*observation).scope.0 == system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER.0)
        {
            cpu = observation;
        }
        if (strcmp(
            &raw const (*observation).metric as *const ::core::ffi::c_char,
            b"host.pressure.cpu.some_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw const (*observation).metric as *const ::core::ffi::c_char,
                b"host.pressure.cpu.some_seconds_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
            && observation_available(observation) as ::core::ffi::c_int != 0
            && (*observation).value > 0.0f64
        {
            cpu_pressure = r#true != 0;
        }
        if strcmp(
            &raw const (*observation).metric as *const ::core::ffi::c_char,
            b"host.pressure.cpu.full_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && observation_available(observation) as ::core::ffi::c_int != 0
            && (*observation).value > 0.0f64
        {
            cpu_full_pressure = r#true != 0;
        }
        if strcmp(
            &raw const (*observation).metric as *const ::core::ffi::c_char,
            b"host.pressure.io.full_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && observation_available(observation) as ::core::ffi::c_int != 0
            && (*observation).value > 0.0f64
        {
            io_full_pressure = r#true != 0;
        }
        if strcmp(
            &raw const (*observation).metric as *const ::core::ffi::c_char,
            b"clock.startup_grace\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && observation_available(observation) as ::core::ffi::c_int != 0
            && (*observation).value != 0.0f64
        {
            startup_grace = r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    if !memory.is_null() {
        candidate_from_observation(
            evaluator,
            candidates,
            candidate_count,
            system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
            b"memory\0".as_ptr() as *const ::core::ffi::c_char,
            memory,
        );
    }
    if !cpu.is_null() {
        let mut candidate: *mut evaluator_candidate_t = candidate_from_observation(
            evaluator,
            candidates,
            candidate_count,
            system_health_condition_t::SYSTEM_HEALTH_CONDITION_CPU_SATURATION,
            b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
            cpu,
        );
        let mut degraded: bool = !context.is_null()
            && (*context).service_degraded_known as ::core::ffi::c_int != 0
            && (*context).service_degraded as ::core::ffi::c_int != 0;
        if !candidate.is_null()
            && (*candidate).desired.0 != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
            && !cpu_pressure
            && !degraded
        {
            (*candidate).desired = system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
            (*candidate).recovery_safe = r#false != 0;
        } else if !candidate.is_null()
            && (*candidate).desired.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
            && !cpu_full_pressure
            && !degraded
        {
            (*candidate).desired = system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING;
        }
    }
    let mut index_0: size_t = 0 as size_t;
    while index_0 < (*snapshot).observation_count {
        let mut observation_0: *const system_health_observation_t =
            (&raw const (*snapshot).observations as *const system_health_observation_t)
                .offset(index_0 as isize);
        if observation_available(observation_0) {
            let mut metric: *const ::core::ffi::c_char =
                &raw const (*observation_0).metric as *const ::core::ffi::c_char;
            if metric_suffix(
                metric,
                b"cpu.throttled_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                let mut candidate_0: *mut evaluator_candidate_t = candidate_from_observation(
                    evaluator,
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_CPU_THROTTLED,
                    b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
                    observation_0,
                );
                let mut degraded_0: bool = !context.is_null()
                    && (*context).service_degraded_known as ::core::ffi::c_int != 0
                    && (*context).service_degraded as ::core::ffi::c_int != 0;
                if !candidate_0.is_null()
                    && (*candidate_0).desired.0
                        == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                    && !degraded_0
                {
                    (*candidate_0).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING;
                }
            } else if strcmp(
                metric,
                b"host.pressure.io.some_seconds_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_1: *mut evaluator_candidate_t = candidate_from_observation(
                    evaluator,
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_IO_PRESSURE,
                    b"io\0".as_ptr() as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_1.is_null()
                    && (*candidate_1).desired.0
                        == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                    && !io_full_pressure
                {
                    (*candidate_1).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING;
                }
            } else if strcmp(
                metric,
                b"process.fd_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                candidate_from_observation(
                    evaluator,
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION,
                    b"process\0".as_ptr() as *const ::core::ffi::c_char,
                    observation_0,
                );
            } else if strcmp(
                metric,
                b"process.pid_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                candidate_from_observation(
                    evaluator,
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION,
                    b"process\0".as_ptr() as *const ::core::ffi::c_char,
                    observation_0,
                );
            } else if metric_suffix(
                metric,
                b"pids.available_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                let mut used: system_health_observation_t = *observation_0;
                copy_string(
                    &raw mut used.metric as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    b"process.pid_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                );
                used.value = 1.0f64 - (*observation_0).value;
                candidate_from_observation(
                    evaluator,
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION,
                    b"process\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut used,
                );
            } else if strcmp(
                metric,
                b"network.error_drop_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                candidate_from_observation(
                    evaluator,
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
            } else if metric_suffix(
                metric,
                b"memory.oom_kills_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_2: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL,
                    b"memory\0".as_ptr() as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_2.is_null() {
                    (*candidate_2).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL;
                    (*candidate_2).immediate = r#true != 0;
                    (*candidate_2).one_shot = r#true != 0;
                }
            } else if strcmp(
                metric,
                b"filesystem.read_only\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_3: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_3.is_null() {
                    (*candidate_3).desired = system_health_severity_t(
                        (if (*observation_0).value != 0.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    (*candidate_3).recovery_safe = (*observation_0).value == 0.0f64;
                    (*candidate_3).immediate = (*observation_0).value != 0.0f64;
                }
            } else if strcmp(
                metric,
                b"filesystem.mount_present\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_4: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_4.is_null() && (*observation_0).value == 0.0f64 {
                    let mut no_fallback: bool = !context.is_null()
                        && (*context).viable_recording_targets_known as ::core::ffi::c_int != 0
                        && (*context).viable_recording_targets == 0 as uint32_t;
                    (*candidate_4).desired = system_health_severity_t(
                        (if no_fallback as ::core::ffi::c_int != 0 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_ERROR.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    (*candidate_4).immediate = r#true != 0;
                    (*candidate_4).recovery_safe = r#false != 0;
                } else if !candidate_4.is_null() {
                    (*candidate_4).recovery_safe = r#true != 0;
                }
            } else if strcmp(
                metric,
                b"filesystem.write_failed\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_5: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_5.is_null() {
                    (*candidate_5).desired = system_health_severity_t(
                        (if (*observation_0).value != 0.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    (*candidate_5).recovery_safe = (*observation_0).value == 0.0f64;
                    (*candidate_5).immediate = (*observation_0).value != 0.0f64;
                }
            } else if strcmp(
                metric,
                b"clock.synchronized\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && !startup_grace
            {
                let mut candidate_6: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED,
                    b"clock\0".as_ptr() as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_6.is_null() {
                    (*candidate_6).desired = system_health_severity_t(
                        (if (*observation_0).value == 0.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    (*candidate_6).recovery_safe = (*observation_0).value != 0.0f64;
                    (*candidate_6).immediate = (*observation_0).value == 0.0f64;
                }
            } else if strcmp(
                metric,
                b"clock.jump_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_7: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_CLOCK_JUMP,
                    b"clock\0".as_ptr() as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_7.is_null() {
                    (*candidate_7).desired = system_health_severity_t(
                        (if (*observation_0).value < -2.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    (*candidate_7).recovery_safe = (*observation_0).value >= -2.0f64;
                    (*candidate_7).immediate = (*candidate_7).desired.0
                        != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0;
                }
            } else if strcmp(
                metric,
                b"storage.device.prefail\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_8: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                merge_candidate_evidence(
                    candidate_8,
                    observation_0,
                    system_health_severity_t(
                        (if (*observation_0).value != 0.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    ),
                    (*observation_0).value == 0.0f64,
                    (*observation_0).value != 0.0f64,
                    r#false != 0,
                );
            } else if strcmp(
                metric,
                b"storage.device.critical\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_9: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                merge_candidate_evidence(
                    candidate_9,
                    observation_0,
                    system_health_severity_t(
                        (if (*observation_0).value != 0.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    ),
                    (*observation_0).value == 0.0f64,
                    (*observation_0).value != 0.0f64,
                    r#false != 0,
                );
            } else if strcmp(
                metric,
                b"storage.device.pre_eol\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut prefail: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                merge_candidate_evidence(
                    prefail,
                    observation_0,
                    system_health_severity_t(
                        (if (*observation_0).value >= 2.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    ),
                    (*observation_0).value < 2.0f64,
                    (*observation_0).value >= 2.0f64,
                    r#false != 0,
                );
                let mut critical: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                merge_candidate_evidence(
                    critical,
                    observation_0,
                    system_health_severity_t(
                        (if (*observation_0).value >= 3.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    ),
                    (*observation_0).value < 3.0f64,
                    (*observation_0).value >= 3.0f64,
                    r#false != 0,
                );
            } else if strcmp(
                metric,
                b"storage.device.life_used_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    metric,
                    b"storage.device.percentage_used_ratio\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_10: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                let mut desired: system_health_severity_t = system_health_severity_t(
                    (if (*observation_0).value >= 0.95f64 {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                            as ::core::ffi::c_int
                    } else if (*observation_0).value >= 0.80f64 {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                            as ::core::ffi::c_int
                    } else {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                            as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                );
                merge_candidate_evidence(
                    candidate_10,
                    observation_0,
                    desired,
                    (*observation_0).value < 0.75f64,
                    r#false != 0,
                    r#false != 0,
                );
            } else if strcmp(
                metric,
                b"storage.device.available_spare_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut candidate_11: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                let mut desired_0: system_health_severity_t = system_health_severity_t(
                    (if (*observation_0).value <= 0.05f64 {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                            as ::core::ffi::c_int
                    } else if (*observation_0).value <= 0.10f64 {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                            as ::core::ffi::c_int
                    } else {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                            as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                );
                merge_candidate_evidence(
                    candidate_11,
                    observation_0,
                    desired_0,
                    (*observation_0).value > 0.15f64,
                    r#false != 0,
                    r#false != 0,
                );
            } else if strcmp(
                metric,
                b"storage.device.media_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_12: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                merge_candidate_evidence(
                    candidate_12,
                    observation_0,
                    system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL,
                    r#false != 0,
                    r#true != 0,
                    r#true != 0,
                );
            } else if strcmp(
                metric,
                b"hardware.ecc.corrected_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_13: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                merge_candidate_evidence(
                    candidate_13,
                    observation_0,
                    system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING,
                    r#false != 0,
                    r#false != 0,
                    r#true != 0,
                );
            } else if strcmp(
                metric,
                b"hardware.ecc.uncorrectable_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_14: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                merge_candidate_evidence(
                    candidate_14,
                    observation_0,
                    system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL,
                    r#false != 0,
                    r#true != 0,
                    r#true != 0,
                );
            } else if strcmp(
                metric,
                b"hardware.fan.failed\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut hot: *const system_health_observation_t = find_observation(
                    snapshot,
                    b"hardware.fan.hot\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                );
                let mut hot_now: bool =
                    observation_available(hot) as ::core::ffi::c_int != 0 && (*hot).value != 0.0f64;
                let mut candidate_15: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                let mut desired_1: system_health_severity_t = system_health_severity_t(
                    (if (*observation_0).value == 0.0f64 {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                            as ::core::ffi::c_int
                    } else if hot_now as ::core::ffi::c_int != 0 {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                            as ::core::ffi::c_int
                    } else {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                            as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                );
                merge_candidate_evidence(
                    candidate_15,
                    observation_0,
                    desired_1,
                    (*observation_0).value == 0.0f64,
                    hot_now as ::core::ffi::c_int != 0 && (*observation_0).value != 0.0f64,
                    r#false != 0,
                );
            } else if strcmp(
                metric,
                b"hardware.power.unstable\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    metric,
                    b"hardware.throttled\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut unstable: bool = strcmp(
                    metric,
                    b"hardware.power.unstable\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int;
                let mut candidate_16: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                let mut desired_2: system_health_severity_t = system_health_severity_t(
                    (if (*observation_0).value == 0.0f64 {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                            as ::core::ffi::c_int
                    } else if unstable as ::core::ffi::c_int != 0 {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                            as ::core::ffi::c_int
                    } else {
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                            as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                );
                merge_candidate_evidence(
                    candidate_16,
                    observation_0,
                    desired_2,
                    (*observation_0).value == 0.0f64,
                    unstable as ::core::ffi::c_int != 0 && (*observation_0).value != 0.0f64,
                    r#false != 0,
                );
            } else if strcmp(
                metric,
                b"kernel.machine_check_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_17: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_17.is_null() {
                    (*candidate_17).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL;
                    (*candidate_17).immediate = r#true != 0;
                    (*candidate_17).one_shot = r#true != 0;
                }
            } else if strcmp(
                metric,
                b"kernel.block_io_error_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_18: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_18.is_null() {
                    (*candidate_18).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL;
                    (*candidate_18).immediate = r#true != 0;
                    (*candidate_18).one_shot = r#true != 0;
                }
            } else if strcmp(
                metric,
                b"kernel.filesystem_remount_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_19: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_19.is_null() {
                    (*candidate_19).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL;
                    (*candidate_19).immediate = r#true != 0;
                    (*candidate_19).one_shot = r#true != 0;
                }
            } else if strcmp(
                metric,
                b"kernel.thermal_shutdown_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_20: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_THERMAL_HIGH,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_20.is_null() {
                    (*candidate_20).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL;
                    (*candidate_20).immediate = r#true != 0;
                    (*candidate_20).one_shot = r#true != 0;
                }
            } else if strcmp(
                metric,
                b"kernel.oom_kill_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*observation_0).value > 0.0f64
            {
                let mut candidate_21: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL,
                    &raw const (*observation_0).resource_id as *const ::core::ffi::c_char,
                    observation_0,
                );
                if !candidate_21.is_null() {
                    (*candidate_21).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL;
                    (*candidate_21).immediate = r#true != 0;
                    (*candidate_21).one_shot = r#true != 0;
                }
            }
        }
        index_0 = index_0.wrapping_add(1);
    }
    let mut index_1: size_t = 0 as size_t;
    while index_1 < (*snapshot).observation_count {
        let mut available: *const system_health_observation_t =
            (&raw const (*snapshot).observations as *const system_health_observation_t)
                .offset(index_1 as isize);
        if strcmp(
            &raw const (*available).metric as *const ::core::ffi::c_char,
            b"filesystem.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut capacity: *const system_health_observation_t = find_observation(
                snapshot,
                b"filesystem.capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*available).resource_id as *const ::core::ffi::c_char,
            );
            let mut ratio: system_health_observation_t = ratio_observation(
                available,
                capacity,
                b"filesystem.available_bytes_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            );
            candidate_from_observation(
                evaluator,
                candidates,
                candidate_count,
                system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW,
                &raw const (*available).resource_id as *const ::core::ffi::c_char,
                &raw mut ratio,
            );
        } else if strcmp(
            &raw const (*available).metric as *const ::core::ffi::c_char,
            b"filesystem.available_inodes\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut capacity_0: *const system_health_observation_t = find_observation(
                snapshot,
                b"filesystem.capacity_inodes\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*available).resource_id as *const ::core::ffi::c_char,
            );
            let mut ratio_0: system_health_observation_t = ratio_observation(
                available,
                capacity_0,
                b"filesystem.available_inodes_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            );
            candidate_from_observation(
                evaluator,
                candidates,
                candidate_count,
                system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW,
                &raw const (*available).resource_id as *const ::core::ffi::c_char,
                &raw mut ratio_0,
            );
        } else if strcmp(
            &raw const (*available).metric as *const ::core::ffi::c_char,
            b"network.carrier\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut primary: *const system_health_observation_t = find_observation(
                snapshot,
                b"network.primary\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*available).resource_id as *const ::core::ffi::c_char,
            );
            if observation_available(primary) as ::core::ffi::c_int != 0
                && (*primary).value != 0.0f64
                && observation_available(available) as ::core::ffi::c_int != 0
            {
                let mut candidate_22: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN,
                    &raw const (*available).resource_id as *const ::core::ffi::c_char,
                    available,
                );
                if !candidate_22.is_null() {
                    (*candidate_22).desired = system_health_severity_t(
                        (if (*available).value == 0.0f64 {
                            if !context.is_null()
                                && (*context).recording_expected_known as ::core::ffi::c_int != 0
                                && (*context).recording_expected as ::core::ffi::c_int != 0
                            {
                                system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                                    as ::core::ffi::c_int
                            } else {
                                system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                                    as ::core::ffi::c_int
                            }
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    (*candidate_22).recovery_safe = (*available).value != 0.0f64;
                }
            }
        } else if strcmp(
            &raw const (*available).metric as *const ::core::ffi::c_char,
            b"thermal.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut critical_0: *const system_health_observation_t = find_observation(
                snapshot,
                b"thermal.critical_celsius\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*available).resource_id as *const ::core::ffi::c_char,
            );
            if observation_available(available) as ::core::ffi::c_int != 0
                && observation_available(critical_0) as ::core::ffi::c_int != 0
            {
                let mut candidate_23: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t::SYSTEM_HEALTH_CONDITION_THERMAL_HIGH,
                    &raw const (*available).resource_id as *const ::core::ffi::c_char,
                    available,
                );
                if !candidate_23.is_null() {
                    (*candidate_23).desired = system_health_severity_t(
                        (if (*available).value >= (*critical_0).value {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
                                as ::core::ffi::c_int
                        } else if (*available).value >= (*critical_0).value - 10.0f64 {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
                                as ::core::ffi::c_int
                        } else {
                            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                                as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    (*candidate_23).recovery_safe =
                        (*available).value < (*critical_0).value - 15.0f64;
                    (*candidate_23).immediate = (*candidate_23).desired.0
                        == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0;
                }
            }
        }
        index_1 = index_1.wrapping_add(1);
    }
}
unsafe extern "C" fn clamped_wall(mut slot: *mut evaluator_slot_t, mut wall: int64_t) -> int64_t {
    if wall <= 0 as int64_t {
        wall = 1 as int64_t;
    }
    if wall < (*slot).last_persisted_wall_ms {
        wall = (*slot).last_persisted_wall_ms;
    }
    return wall;
}
unsafe extern "C" fn signal_from_candidate(
    mut evaluator: *const system_health_evaluator_t,
    mut slot: *const evaluator_slot_t,
    mut candidate: *const evaluator_candidate_t,
    mut action: system_health_incident_action_t,
    mut target_state: system_health_state_t,
    mut target_severity: system_health_severity_t,
    mut wall: int64_t,
    mut signal: *mut system_health_incident_signal_t,
) {
    memset(
        signal as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_incident_signal_t>(),
    );
    (*signal).condition = (*slot).condition;
    copy_string(
        &raw mut (*signal).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw const (*slot).subject as *const ::core::ffi::c_char,
    );
    (*signal).scope = (*slot).scope;
    (*signal).state = target_state;
    (*signal).severity = system_health_severity_t(
        if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0 {
            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
        } else {
            target_severity.0
        },
    );
    (*signal).observed_at_ms = wall;
    (*signal).reconciliation = system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE;
    copy_string(
        &raw mut (*signal).boot_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw const (*evaluator).config.boot_id as *const ::core::ffi::c_char,
    );
    copy_string(
        &raw mut (*signal).run_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        &raw const (*evaluator).config.run_id as *const ::core::ffi::c_char,
    );
    if (*candidate).observation.value_valid {
        snprintf(
            &raw mut (*signal).observation_json as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2049]>(),
            b"{\"metric\":\"%s\",\"resource\":\"%s\",\"value\":%.12g,\"unit\":\"%s\"}\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*candidate).observation.metric as *const ::core::ffi::c_char,
            &raw const (*slot).subject as *const ::core::ffi::c_char,
            (*candidate).observation.value,
            system_health_unit_name((*candidate).observation.unit),
        );
    } else {
        snprintf(
            &raw mut (*signal).observation_json as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2049]>(),
            b"{\"metric\":\"%s\",\"resource\":\"%s\",\"capability\":\"%s\"}\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*candidate).observation.metric as *const ::core::ffi::c_char,
            &raw const (*slot).subject as *const ::core::ffi::c_char,
            system_health_capability_name((*candidate).observation.capability),
        );
    };
}
unsafe extern "C" fn fill_transition(
    mut evaluator: *const system_health_evaluator_t,
    mut slot: *const evaluator_slot_t,
    mut candidate: *const evaluator_candidate_t,
    mut action: system_health_incident_action_t,
    mut previous_state: system_health_state_t,
    mut previous_severity: system_health_severity_t,
    mut target_state: system_health_state_t,
    mut target_severity: system_health_severity_t,
    mut event_id: *const ::core::ffi::c_char,
    mut wall: int64_t,
    mut persisted: bool,
    mut transition: *mut system_health_transition_t,
) {
    memset(
        transition as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_transition_t>(),
    );
    (*transition).action = action;
    copy_string(
        &raw mut (*transition).incident_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        &raw const (*slot).incident_id as *const ::core::ffi::c_char,
    );
    copy_string(
        &raw mut (*transition).event_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        event_id,
    );
    (*transition).condition = (*slot).condition;
    copy_string(
        &raw mut (*transition).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw const (*slot).subject as *const ::core::ffi::c_char,
    );
    (*transition).scope = (*slot).scope;
    (*transition).previous_state = previous_state;
    (*transition).state = target_state;
    (*transition).previous_severity = previous_severity;
    (*transition).severity = target_severity;
    (*transition).observation = (*candidate).observation;
    let mut rule: *const system_health_condition_policy_t =
        (&raw const (*evaluator).config.policy.conditions
            as *const system_health_condition_policy_t)
            .offset((*slot).condition.0 as isize);
    (*transition).threshold_direction = (*rule).direction;
    (*transition).threshold_value =
        if target_severity.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0 {
            (*rule).critical_threshold
        } else {
            (*rule).warning_threshold
        };
    let mut dwell_seconds: uint32_t =
        if target_severity.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0 {
            (*rule).critical_for_seconds
        } else {
            (*rule).warning_for_seconds
        };
    if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0 {
        (*transition).threshold_value = (*rule).recovery_threshold;
        dwell_seconds = (*rule).recovery_for_seconds;
    }
    (*transition).threshold_for_ms = dwell_seconds.wrapping_mul(1000 as uint32_t);
    (*transition).first_observed_at_ms = (*slot).first_observed_wall_ms;
    (*transition).incident_duration_ms = if (*slot).first_observed_monotonic_ms > 0 as uint64_t
        && (*candidate).observation.sampled_monotonic_ms >= (*slot).first_observed_monotonic_ms
    {
        (*candidate)
            .observation
            .sampled_monotonic_ms
            .wrapping_sub((*slot).first_observed_monotonic_ms)
    } else {
        0 as uint64_t
    };
    (*transition).observed_at_ms = wall;
    (*transition).persisted = persisted;
}
unsafe extern "C" fn persistence_succeeded(mut result: db_system_health_result_t) -> bool {
    return result.0 == db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
        || result.0 == db_system_health_result_t::DB_SYSTEM_HEALTH_RESUMED.0;
}
unsafe extern "C" fn deliver_transition(
    mut evaluator: *mut system_health_evaluator_t,
    mut slot: *mut evaluator_slot_t,
    mut candidate: *const evaluator_candidate_t,
    mut action: system_health_incident_action_t,
    mut previous_state: system_health_state_t,
    mut previous_severity: system_health_severity_t,
    mut target_state: system_health_state_t,
    mut target_severity: system_health_severity_t,
    mut event_id: *const ::core::ffi::c_char,
    mut wall: int64_t,
    mut persisted: bool,
) {
    if (*evaluator).config.transition_sink.is_none() {
        return;
    }
    let mut transition: system_health_transition_t = system_health_transition_t {
        action: system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN,
        incident_id: [0; 37],
        event_id: [0; 37],
        condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        previous_state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        previous_severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
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
        threshold_direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
        threshold_value: 0.,
        threshold_for_ms: 0,
        first_observed_at_ms: 0,
        incident_duration_ms: 0,
        observed_at_ms: 0,
        persisted: false,
    };
    fill_transition(
        evaluator,
        slot,
        candidate,
        action,
        previous_state,
        previous_severity,
        target_state,
        target_severity,
        event_id,
        wall,
        persisted,
        &raw mut transition,
    );
    (*evaluator)
        .config
        .transition_sink
        .expect("non-null function pointer")(
        &raw mut transition,
        (*evaluator).config.transition_sink_context,
    );
}
unsafe extern "C" fn emit_action(
    mut evaluator: *mut system_health_evaluator_t,
    mut slot: *mut evaluator_slot_t,
    mut candidate: *const evaluator_candidate_t,
    mut action: system_health_incident_action_t,
    mut previous_state: system_health_state_t,
    mut previous_severity: system_health_severity_t,
    mut target_state: system_health_state_t,
    mut target_severity: system_health_severity_t,
    mut now: uint64_t,
    mut wall: int64_t,
) -> bool {
    wall = clamped_wall(slot, wall);
    let mut event_id: [::core::ffi::c_char; 37] = [
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
    ];
    if (*evaluator).config.transition_sink.is_some()
        && lightnvr_uuid_generate_v4(&raw mut event_id as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if (*evaluator).config.persist.is_none() {
        if (*slot).incident_id[0usize] == 0 {
            lightnvr_uuid_generate_v4(&raw mut (*slot).incident_id as *mut ::core::ffi::c_char);
        }
        (*slot).last_persisted_wall_ms = wall;
        (*evaluator).stats.transitions = (*evaluator).stats.transitions.wrapping_add(1);
        deliver_transition(
            evaluator,
            slot,
            candidate,
            action,
            previous_state,
            previous_severity,
            target_state,
            target_severity,
            &raw mut event_id as *mut ::core::ffi::c_char,
            wall,
            r#false != 0,
        );
        return r#true != 0;
    }
    let mut signal: system_health_incident_signal_t = system_health_incident_signal_t {
        condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        observed_at_ms: 0,
        observation_json: [0; 2049],
        event_id: [0; 37],
        reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
        boot_id: [0; 64],
        run_id: [0; 37],
    };
    signal_from_candidate(
        evaluator,
        slot,
        candidate,
        action,
        target_state,
        target_severity,
        wall,
        &raw mut signal,
    );
    if event_id[0usize] != 0 {
        copy_string(
            &raw mut signal.event_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            &raw mut event_id as *mut ::core::ffi::c_char,
        );
        signal.reconciliation = system_health_reconciliation_t(
            (if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0 {
                system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING.0
                    as ::core::ffi::c_int
            } else {
                system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING.0
                    as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
    }
    let mut incident: system_health_incident_record_t = system_health_incident_record_t {
        uuid: [0; 37],
        condition_code: [0; 64],
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        first_seen_at_ms: 0,
        last_seen_at_ms: 0,
        closed_at_ms: 0,
        observation_json: [0; 2049],
        alert_event_id: [0; 37],
        recovery_event_id: [0; 37],
        reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
        boot_id: [0; 64],
        run_id: [0; 37],
        revision: 0,
    };
    let mut result: db_system_health_result_t = (*evaluator)
        .config
        .persist
        .expect("non-null function pointer")(
        action,
        &raw mut signal,
        &raw mut incident,
        (*evaluator).config.persist_context,
    );
    if persistence_succeeded(result) {
        copy_string(
            &raw mut (*slot).incident_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            &raw mut incident.uuid as *mut ::core::ffi::c_char,
        );
        (*slot).last_persisted_wall_ms = incident.last_seen_at_ms;
        (*evaluator).stats.transitions = (*evaluator).stats.transitions.wrapping_add(1);
        deliver_transition(
            evaluator,
            slot,
            candidate,
            action,
            previous_state,
            previous_severity,
            target_state,
            target_severity,
            &raw mut event_id as *mut ::core::ffi::c_char,
            wall,
            r#true != 0,
        );
        return r#true != 0;
    }
    (*slot).persistence_pending = r#true != 0;
    (*slot).pending_action = action;
    (*slot).pending_previous_state = previous_state;
    (*slot).pending_previous_severity = previous_severity;
    copy_string(
        &raw mut (*slot).pending_event_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        &raw mut event_id as *mut ::core::ffi::c_char,
    );
    (*slot).pending_target_state = target_state;
    (*slot).pending_target_severity = target_severity;
    (*slot).pending_candidate = *candidate;
    (*slot).pending_wall_ms = wall;
    (*slot).retry_delay_ms = (*evaluator).config.retry_initial_ms;
    (*slot).next_retry_ms = now.wrapping_add((*slot).retry_delay_ms as uint64_t);
    (*evaluator).stats.persistence_failures =
        (*evaluator).stats.persistence_failures.wrapping_add(1);
    (*evaluator).stats.pending_persistence = (*evaluator).stats.pending_persistence.wrapping_add(1);
    return r#false != 0;
}
unsafe extern "C" fn retry_pending(
    mut evaluator: *mut system_health_evaluator_t,
    mut slot: *mut evaluator_slot_t,
    mut now: uint64_t,
) {
    if !(*slot).persistence_pending
        || now < (*slot).next_retry_ms
        || (*evaluator).config.persist.is_none()
    {
        return;
    }
    let mut signal: system_health_incident_signal_t = system_health_incident_signal_t {
        condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        observed_at_ms: 0,
        observation_json: [0; 2049],
        event_id: [0; 37],
        reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
        boot_id: [0; 64],
        run_id: [0; 37],
    };
    signal_from_candidate(
        evaluator,
        slot,
        &raw mut (*slot).pending_candidate,
        (*slot).pending_action,
        (*slot).pending_target_state,
        (*slot).pending_target_severity,
        clamped_wall(slot, (*slot).pending_wall_ms),
        &raw mut signal,
    );
    if (*slot).pending_event_id[0usize] != 0 {
        copy_string(
            &raw mut signal.event_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            &raw mut (*slot).pending_event_id as *mut ::core::ffi::c_char,
        );
        signal.reconciliation = system_health_reconciliation_t(
            (if (*slot).pending_action.0
                == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0
            {
                system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING.0
                    as ::core::ffi::c_int
            } else {
                system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING.0
                    as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
    }
    let mut incident: system_health_incident_record_t = system_health_incident_record_t {
        uuid: [0; 37],
        condition_code: [0; 64],
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        first_seen_at_ms: 0,
        last_seen_at_ms: 0,
        closed_at_ms: 0,
        observation_json: [0; 2049],
        alert_event_id: [0; 37],
        recovery_event_id: [0; 37],
        reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
        boot_id: [0; 64],
        run_id: [0; 37],
        revision: 0,
    };
    let mut result: db_system_health_result_t = (*evaluator)
        .config
        .persist
        .expect("non-null function pointer")(
        (*slot).pending_action,
        &raw mut signal,
        &raw mut incident,
        (*evaluator).config.persist_context,
    );
    (*evaluator).stats.persistence_retries = (*evaluator).stats.persistence_retries.wrapping_add(1);
    if persistence_succeeded(result) {
        copy_string(
            &raw mut (*slot).incident_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            &raw mut incident.uuid as *mut ::core::ffi::c_char,
        );
        (*slot).last_persisted_wall_ms = incident.last_seen_at_ms;
        deliver_transition(
            evaluator,
            slot,
            &raw mut (*slot).pending_candidate,
            (*slot).pending_action,
            (*slot).pending_previous_state,
            (*slot).pending_previous_severity,
            (*slot).pending_target_state,
            (*slot).pending_target_severity,
            &raw mut (*slot).pending_event_id as *mut ::core::ffi::c_char,
            signal.observed_at_ms,
            r#true != 0,
        );
        (*slot).persistence_pending = r#false != 0;
        (*slot).pending_event_id[0usize] = '\0' as ::core::ffi::c_char;
        (*evaluator).stats.pending_persistence =
            (*evaluator).stats.pending_persistence.wrapping_sub(1);
        (*evaluator).stats.transitions = (*evaluator).stats.transitions.wrapping_add(1);
        return;
    }
    (*evaluator).stats.persistence_failures =
        (*evaluator).stats.persistence_failures.wrapping_add(1);
    let mut doubled: uint64_t = ((*slot).retry_delay_ms as uint64_t).wrapping_mul(2 as uint64_t);
    (*slot).retry_delay_ms = if doubled > (*evaluator).config.retry_max_ms as uint64_t {
        (*evaluator).config.retry_max_ms
    } else {
        doubled as uint32_t
    };
    (*slot).next_retry_ms = now.wrapping_add((*slot).retry_delay_ms as uint64_t);
}
unsafe extern "C" fn material_change(
    mut evaluator: *const system_health_evaluator_t,
    mut slot: *const evaluator_slot_t,
    mut candidate: *const evaluator_candidate_t,
    mut now: uint64_t,
) -> bool {
    if !(*candidate).observation.value_valid
        || !(*slot).last_material_value_valid
        || now.wrapping_sub((*slot).last_material_ms)
            < (*evaluator).config.material_change_debounce_ms as uint64_t
    {
        return r#false != 0;
    }
    let mut scale: ::core::ffi::c_double = fabs((*slot).last_material_value);
    if scale < 1e-9f64 {
        scale = 1.0f64;
    }
    return fabs((*candidate).observation.value - (*slot).last_material_value) / scale
        >= (*evaluator).config.material_change_ratio;
}
unsafe extern "C" fn severity_dwell_ms(
    mut rule: *const system_health_condition_policy_t,
    mut severity: system_health_severity_t,
) -> uint64_t {
    let mut seconds: uint32_t =
        if severity.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0 {
            (*rule).critical_for_seconds
        } else {
            (*rule).warning_for_seconds
        };
    return (seconds as uint64_t).wrapping_mul(1000 as uint64_t);
}
unsafe extern "C" fn evaluate_candidate(
    mut evaluator: *mut system_health_evaluator_t,
    mut candidate: *const evaluator_candidate_t,
    mut now: uint64_t,
    mut wall: int64_t,
) {
    if (*candidate).one_shot {
        if (*candidate).desired.0 != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0 {
            system_health_evaluator_one_shot(
                evaluator,
                (*candidate).condition,
                &raw const (*candidate).subject as *const ::core::ffi::c_char,
                (*candidate).scope,
                (*candidate).desired,
                &raw const (*candidate).observation,
                wall,
            );
        }
        return;
    }
    let mut slot: *mut evaluator_slot_t = find_slot(
        evaluator,
        (*candidate).condition,
        &raw const (*candidate).subject as *const ::core::ffi::c_char,
        r#true != 0,
    );
    if slot.is_null() {
        return;
    }
    (*slot).scope = (*candidate).scope;
    (*slot).last_observation = (*candidate).observation;
    (*slot).last_observed_wall_ms = wall;
    retry_pending(evaluator, slot, now);
    if (*slot).persistence_pending {
        return;
    }
    let mut rule: *const system_health_condition_policy_t =
        (&raw mut (*evaluator).config.policy.conditions as *mut system_health_condition_policy_t)
            .offset((*candidate).condition.0 as isize);
    if !(*rule).enabled {
        return;
    }
    if (*slot).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN.0 {
        if (*candidate).desired.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0 {
            if (*candidate).recovery_safe {
                (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_HEALTHY;
            }
            return;
        }
        (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_PENDING;
        (*slot).pending_severity = (*candidate).desired;
        (*slot).pending_since_ms = now;
    }
    if (*slot).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_HEALTHY.0
        || (*slot).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_PENDING.0
    {
        if (*candidate).desired.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0 {
            (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_HEALTHY;
            (*slot).pending_since_ms = 0 as uint64_t;
            (*slot).pending_severity = system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
            return;
        }
        if (*slot).state.0 != system_health_state_t::SYSTEM_HEALTH_STATE_PENDING.0
            || (*slot).pending_severity.0 != (*candidate).desired.0
        {
            (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_PENDING;
            (*slot).pending_severity = (*candidate).desired;
            (*slot).pending_since_ms = now;
        }
        let mut dwell: uint64_t = severity_dwell_ms(rule, (*candidate).desired);
        if !(*candidate).immediate && now.wrapping_sub((*slot).pending_since_ms) < dwell {
            return;
        }
        let mut previous_state: system_health_state_t = (*slot).state;
        (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_OPEN;
        (*slot).severity = (*candidate).desired;
        (*slot).first_observed_monotonic_ms = (*slot).pending_since_ms;
        (*slot).first_observed_wall_ms =
            wall - now.wrapping_sub((*slot).pending_since_ms) as int64_t;
        (*slot).last_material_ms = now;
        (*slot).last_material_value = (*candidate).observation.value;
        (*slot).last_material_value_valid = (*candidate).observation.value_valid;
        (*evaluator).stats.active_incidents = (*evaluator).stats.active_incidents.wrapping_add(1);
        emit_action(
            evaluator,
            slot,
            candidate,
            system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN,
            previous_state,
            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
            system_health_state_t::SYSTEM_HEALTH_STATE_OPEN,
            (*slot).severity,
            now,
            wall,
        );
        return;
    }
    if (*slot).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING.0 {
        if (*candidate).desired.0 != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0 {
            let mut previous_state_0: system_health_state_t = (*slot).state;
            (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_OPEN;
            (*slot).recovery_since_ms = 0 as uint64_t;
            emit_action(
                evaluator,
                slot,
                candidate,
                system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
                previous_state_0,
                (*slot).severity,
                system_health_state_t::SYSTEM_HEALTH_STATE_OPEN,
                (*slot).severity,
                now,
                wall,
            );
            return;
        }
        if !(*candidate).recovery_safe {
            return;
        }
        let mut dwell_0: uint64_t =
            ((*rule).recovery_for_seconds as uint64_t).wrapping_mul(1000 as uint64_t);
        if now.wrapping_sub((*slot).recovery_since_ms) < dwell_0 {
            return;
        }
        let mut previous_severity: system_health_severity_t = (*slot).severity;
        (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED;
        (*slot).severity = system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
        if (*evaluator).stats.active_incidents > 0 as size_t {
            (*evaluator).stats.active_incidents =
                (*evaluator).stats.active_incidents.wrapping_sub(1);
        }
        emit_action(
            evaluator,
            slot,
            candidate,
            system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER,
            system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING,
            previous_severity,
            system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED,
            previous_severity,
            now,
            wall,
        );
        return;
    }
    if (*slot).state.0 != system_health_state_t::SYSTEM_HEALTH_STATE_OPEN.0 {
        return;
    }
    if (*candidate).desired.0 > (*slot).severity.0 {
        if (*slot).pending_severity.0 != (*candidate).desired.0 {
            (*slot).pending_severity = (*candidate).desired;
            (*slot).pending_since_ms = now;
        }
        let mut dwell_1: uint64_t = severity_dwell_ms(rule, (*candidate).desired);
        if !(*candidate).immediate && now.wrapping_sub((*slot).pending_since_ms) < dwell_1 {
            return;
        }
        let mut previous: system_health_severity_t = (*slot).severity;
        (*slot).severity = (*candidate).desired;
        (*slot).pending_severity = system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
        (*slot).last_material_ms = now;
        emit_action(
            evaluator,
            slot,
            candidate,
            system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ESCALATE,
            system_health_state_t::SYSTEM_HEALTH_STATE_OPEN,
            previous,
            system_health_state_t::SYSTEM_HEALTH_STATE_OPEN,
            (*slot).severity,
            now,
            wall,
        );
        return;
    }
    if (*candidate).recovery_safe {
        let mut previous_state_1: system_health_state_t = (*slot).state;
        (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING;
        (*slot).recovery_since_ms = now;
        emit_action(
            evaluator,
            slot,
            candidate,
            system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
            previous_state_1,
            (*slot).severity,
            system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING,
            (*slot).severity,
            now,
            wall,
        );
        return;
    }
    if (*candidate).desired.0 == (*slot).severity.0
        && material_change(evaluator, slot, candidate, now) as ::core::ffi::c_int != 0
    {
        (*slot).last_material_ms = now;
        (*slot).last_material_value = (*candidate).observation.value;
        (*slot).last_material_value_valid = r#true != 0;
        emit_action(
            evaluator,
            slot,
            candidate,
            system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
            system_health_state_t::SYSTEM_HEALTH_STATE_OPEN,
            (*slot).severity,
            system_health_state_t::SYSTEM_HEALTH_STATE_OPEN,
            (*slot).severity,
            now,
            wall,
        );
    }
}
unsafe extern "C" fn add_immediate_candidates(
    mut candidates: *mut evaluator_candidate_t,
    mut candidate_count: *mut size_t,
    mut monotonic: uint64_t,
    mut wall: int64_t,
) {
    let mut condition_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while condition_index
        < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int
    {
        let mut resource_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while resource_index
            < system_health_immediate_resource_t::SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT.0
                as ::core::ffi::c_int
        {
            let mut count: uint64_t = ::core::intrinsics::atomic_xchg::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                (&raw mut *(&raw mut immediate_counts as *mut [atomic_uint_fast64_t; 3])
                    .offset(condition_index as isize)
                    as *mut atomic_uint_fast64_t)
                    .offset(resource_index as isize),
                0 as uint_fast64_t,
            );
            if count != 0 as uint64_t {
                let mut subject: *const ::core::ffi::c_char = if resource_index
                    == system_health_immediate_resource_t::SYSTEM_HEALTH_IMMEDIATE_PROCESS.0
                        as ::core::ffi::c_int
                {
                    b"process\0".as_ptr() as *const ::core::ffi::c_char
                } else if resource_index
                    == system_health_immediate_resource_t::SYSTEM_HEALTH_IMMEDIATE_ROOT.0
                        as ::core::ffi::c_int
                {
                    b"root\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"recording\0".as_ptr() as *const ::core::ffi::c_char
                };
                let mut observation: system_health_observation_t = system_health_observation_t {
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
                };
                memset(
                    &raw mut observation as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<system_health_observation_t>(),
                );
                copy_string(
                    &raw mut observation.metric as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    system_health_condition_code(system_health_condition_t(
                        condition_index as ::core::ffi::c_uint,
                    )),
                );
                copy_string(
                    &raw mut observation.resource_id as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    subject,
                );
                observation.scope = system_health_scope_t(
                    (if resource_index
                        == system_health_immediate_resource_t::SYSTEM_HEALTH_IMMEDIATE_PROCESS.0
                            as ::core::ffi::c_int
                    {
                        system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS.0 as ::core::ffi::c_int
                    } else {
                        system_health_scope_t::SYSTEM_HEALTH_SCOPE_FILESYSTEM.0
                            as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                );
                observation.sampled_monotonic_ms = monotonic;
                observation.observed_wall_time_ms = wall;
                system_health_observation_set_available(
                    &raw mut observation,
                    count as ::core::ffi::c_double,
                    system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
                );
                let mut candidate: *mut evaluator_candidate_t = add_candidate(
                    candidates,
                    candidate_count,
                    system_health_condition_t(condition_index as ::core::ffi::c_uint),
                    subject,
                    &raw mut observation,
                );
                if !candidate.is_null() {
                    (*candidate).desired =
                        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL;
                    (*candidate).immediate = r#true != 0;
                    (*candidate).one_shot = condition_index
                        == system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL
                            .0 as ::core::ffi::c_int
                        || condition_index
                            == system_health_condition_t::SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED
                                .0 as ::core::ffi::c_int;
                }
            }
            resource_index += 1;
        }
        condition_index += 1;
    }
}
unsafe extern "C" fn publish_views(mut evaluator: *mut system_health_evaluator_t) {
    let mut destination: ::core::ffi::c_uint =
        if (*evaluator).active_view == 0 as ::core::ffi::c_uint {
            1 as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        };
    let mut count: size_t = 0 as size_t;
    let mut oldest_pending_wall_time_ms: int64_t = 0 as int64_t;
    let mut index: size_t = 0 as size_t;
    while index < SYSTEM_HEALTH_MAX_INCIDENTS as size_t {
        let mut slot: *const evaluator_slot_t =
            (&raw mut (*evaluator).slots as *mut evaluator_slot_t).offset(index as isize);
        if (*slot).used as ::core::ffi::c_int != 0
            && (*slot).persistence_pending as ::core::ffi::c_int != 0
            && (*slot).pending_wall_ms > 0 as int64_t
            && (oldest_pending_wall_time_ms == 0 as int64_t
                || (*slot).pending_wall_ms < oldest_pending_wall_time_ms)
        {
            oldest_pending_wall_time_ms = (*slot).pending_wall_ms;
        }
        if !(!(*slot).used
            || (*slot).state.0 != system_health_state_t::SYSTEM_HEALTH_STATE_OPEN.0
                && (*slot).state.0 != system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING.0
                && !(*slot).persistence_pending)
        {
            let c2rust_fresh0 = count;
            count = count.wrapping_add(1);
            let mut view: *mut system_health_incident_view_t = (&raw mut *(&raw mut (*evaluator)
                .views
                as *mut [system_health_incident_view_t; 64])
                .offset(destination as isize)
                as *mut system_health_incident_view_t)
                .offset(c2rust_fresh0 as isize);
            memset(
                view as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<system_health_incident_view_t>(),
            );
            copy_string(
                &raw mut (*view).incident_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                &raw const (*slot).incident_id as *const ::core::ffi::c_char,
            );
            (*view).condition = (*slot).condition;
            copy_string(
                &raw mut (*view).subject as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                &raw const (*slot).subject as *const ::core::ffi::c_char,
            );
            (*view).scope = (*slot).scope;
            (*view).state = (*slot).state;
            (*view).severity = (*slot).severity;
            (*view).first_observed_at_ms = (*slot).first_observed_wall_ms;
            (*view).last_observed_at_ms = (*slot).last_observed_wall_ms;
            (*view).observation = (*slot).last_observation;
            (*view).persistence_pending = (*slot).persistence_pending;
        }
        index = index.wrapping_add(1);
    }
    pthread_mutex_lock(&raw mut (*evaluator).view_lock);
    (*evaluator).view_counts[destination as usize] = count;
    (*evaluator).stats.oldest_pending_wall_time_ms = oldest_pending_wall_time_ms;
    (*evaluator).published_stats = (*evaluator).stats;
    (*evaluator).active_view = destination;
    pthread_mutex_unlock(&raw mut (*evaluator).view_lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_one_shot(
    mut evaluator: *mut system_health_evaluator_t,
    mut condition: system_health_condition_t,
    mut subject: *const ::core::ffi::c_char,
    mut scope: system_health_scope_t,
    mut severity: system_health_severity_t,
    mut observation: *const system_health_observation_t,
    mut observed_at_ms: int64_t,
) -> ::core::ffi::c_int {
    if evaluator.is_null()
        || subject.is_null()
        || *subject.offset(0isize) == 0
        || observation.is_null()
        || condition.0 < 0 as ::core::ffi::c_uint
        || condition.0 >= system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0
        || severity.0 < system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0
        || severity.0 > system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut slot: *mut evaluator_slot_t = find_slot(evaluator, condition, subject, r#true != 0);
    if slot.is_null() || (*slot).persistence_pending as ::core::ffi::c_int != 0 {
        return -1 as ::core::ffi::c_int;
    }
    if (*slot).last_episode_sample_ms == (*observation).sampled_monotonic_ms {
        return 0 as ::core::ffi::c_int;
    }
    (*slot).last_episode_sample_ms = (*observation).sampled_monotonic_ms;
    (*slot).scope = scope;
    (*slot).state = system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED;
    (*slot).severity = severity;
    (*slot).first_observed_wall_ms = observed_at_ms;
    (*slot).first_observed_monotonic_ms = (*observation).sampled_monotonic_ms;
    (*slot).last_observed_wall_ms = observed_at_ms;
    (*slot).last_observation = *observation;
    let mut candidate: evaluator_candidate_t = evaluator_candidate_t {
        condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
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
        desired: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        recovery_safe: false,
        immediate: false,
        one_shot: false,
    };
    memset(
        &raw mut candidate as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<evaluator_candidate_t>(),
    );
    candidate.condition = condition;
    candidate.scope = scope;
    candidate.observation = *observation;
    candidate.desired = severity;
    candidate.one_shot = r#true != 0;
    copy_string(
        &raw mut candidate.subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        subject,
    );
    let mut result: bool = emit_action(
        evaluator,
        slot,
        &raw mut candidate,
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ONE_SHOT,
        system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED,
        severity,
        (*observation).sampled_monotonic_ms,
        observed_at_ms,
    );
    publish_views(evaluator);
    return if result as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_evaluate(
    mut evaluator: *mut system_health_evaluator_t,
    mut snapshot: *const system_health_snapshot_t,
    mut context: *const system_health_evaluation_context_t,
) -> ::core::ffi::c_int {
    if evaluator.is_null()
        || snapshot.is_null()
        || (*snapshot).sequence == 0 as uint64_t
        || (*snapshot).completed_monotonic_ms == 0 as uint64_t
        || (*snapshot).completed_wall_time_ms <= 0 as int64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    if (*snapshot).sequence <= (*evaluator).last_snapshot_sequence {
        return 0 as ::core::ffi::c_int;
    }
    (*evaluator).last_snapshot_sequence = (*snapshot).sequence;
    let mut index: size_t = 0 as size_t;
    while index < SYSTEM_HEALTH_MAX_INCIDENTS as size_t {
        if (*evaluator).slots[index].used {
            retry_pending(
                evaluator,
                (&raw mut (*evaluator).slots as *mut evaluator_slot_t).offset(index as isize),
                (*snapshot).completed_monotonic_ms,
            );
        }
        index = index.wrapping_add(1);
    }
    let mut candidates: [evaluator_candidate_t; 96] = [evaluator_candidate_t {
        condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
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
        desired: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        recovery_safe: false,
        immediate: false,
        one_shot: false,
    }; 96];
    let mut count: size_t = 0;
    build_candidates(
        evaluator,
        snapshot,
        context,
        &raw mut candidates as *mut evaluator_candidate_t,
        &raw mut count,
    );
    add_immediate_candidates(
        &raw mut candidates as *mut evaluator_candidate_t,
        &raw mut count,
        (*snapshot).completed_monotonic_ms,
        (*snapshot).completed_wall_time_ms,
    );
    let mut index_0: size_t = 0 as size_t;
    while index_0 < count {
        evaluate_candidate(
            evaluator,
            (&raw mut candidates as *mut evaluator_candidate_t).offset(index_0 as isize),
            (*snapshot).completed_monotonic_ms,
            (*snapshot).completed_wall_time_ms,
        );
        index_0 = index_0.wrapping_add(1);
    }
    publish_views(evaluator);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_get_stats(
    mut evaluator: *const system_health_evaluator_t,
    mut stats: *mut system_health_evaluator_stats_t,
) {
    if stats.is_null() {
        return;
    }
    memset(
        stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_evaluator_stats_t>(),
    );
    if evaluator.is_null() {
        return;
    }
    pthread_mutex_lock(&raw const (*evaluator).view_lock as *mut pthread_mutex_t);
    *stats = (*evaluator).published_stats;
    pthread_mutex_unlock(&raw const (*evaluator).view_lock as *mut pthread_mutex_t);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_active_copy(
    mut evaluator: *const system_health_evaluator_t,
    mut incidents: *mut system_health_incident_view_t,
    mut capacity: size_t,
) -> size_t {
    if evaluator.is_null() || incidents.is_null() || capacity == 0 as size_t {
        return 0 as size_t;
    }
    pthread_mutex_lock(&raw const (*evaluator).view_lock as *mut pthread_mutex_t);
    let mut count: size_t = (*evaluator).view_counts[(*evaluator).active_view as usize];
    if count > capacity {
        count = capacity;
    }
    memcpy(
        incidents as *mut ::core::ffi::c_void,
        &raw const *(&raw const (*evaluator).views as *const [system_health_incident_view_t; 64])
            .offset((*evaluator).active_view as isize)
            as *const system_health_incident_view_t as *const ::core::ffi::c_void,
        count.wrapping_mul(::core::mem::size_of::<system_health_incident_view_t>()),
    );
    pthread_mutex_unlock(&raw const (*evaluator).view_lock as *mut pthread_mutex_t);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_note_immediate(
    mut condition: system_health_condition_t,
    mut resource: system_health_immediate_resource_t,
) {
    if condition.0 < 0 as ::core::ffi::c_uint
        || condition.0 >= system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0
        || resource.0 < 0 as ::core::ffi::c_uint
        || resource.0
            >= system_health_immediate_resource_t::SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT.0
    {
        return;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        (&raw mut *(&raw mut immediate_counts as *mut [atomic_uint_fast64_t; 3])
            .offset(condition.0 as isize) as *mut atomic_uint_fast64_t)
            .offset(resource.0 as isize),
        1 as uint_fast64_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_classify_previous_run(
    mut previous: *const system_health_process_run_t,
    mut had_previous: bool,
    mut current_boot_id: *const ::core::ffi::c_char,
) -> system_health_restart_classification_t {
    if !had_previous
        || previous.is_null()
        || (*previous).clean_close as ::core::ffi::c_int != 0
        || current_boot_id.is_null()
        || *current_boot_id.offset(0isize) == 0
    {
        return system_health_restart_classification_t::SYSTEM_HEALTH_RESTART_NONE;
    }
    return system_health_restart_classification_t(
        (if strcmp(
            &raw const (*previous).boot_id as *const ::core::ffi::c_char,
            current_boot_id,
        ) == 0 as ::core::ffi::c_int
        {
            system_health_restart_classification_t::SYSTEM_HEALTH_RESTART_PROCESS.0
                as ::core::ffi::c_int
        } else {
            system_health_restart_classification_t::SYSTEM_HEALTH_RESTART_HOST.0
                as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
static mut service: evaluator_service_t = evaluator_service_t {
    lock: pthread_mutex_t {
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
    },
    wake: PTHREAD_COND_INITIALIZER,
    thread: 0,
    initialized: false,
    running: false,
    disabled: false,
    evaluator: ::core::ptr::null_mut::<system_health_evaluator_t>(),
    context: system_health_evaluation_context_t {
        recording_expected_known: false,
        recording_expected: false,
        service_degraded_known: false,
        service_degraded: false,
        viable_recording_targets_known: false,
        viable_recording_targets: 0,
    },
    run: system_health_process_run_t {
        run_id: [0; 37],
        boot_id: [0; 64],
        started_at_ms: 0,
        closed_at_ms: 0,
        clean_close: false,
    },
    transition_sink: None,
    transition_sink_context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
};
unsafe extern "C" fn persist_to_database(
    mut action: system_health_incident_action_t,
    mut signal: *const system_health_incident_signal_t,
    mut incident_out: *mut system_health_incident_record_t,
    mut context: *mut ::core::ffi::c_void,
) -> db_system_health_result_t {
    return db_system_health_incident_apply(action, signal, incident_out);
}
unsafe extern "C" fn replay_unit(mut name: *const ::core::ffi::c_char) -> system_health_unit_t {
    if name.is_null() {
        return system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE;
    }
    let mut unit: ::core::ffi::c_int =
        system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE.0 as ::core::ffi::c_int;
    while unit <= system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN.0 as ::core::ffi::c_int {
        if strcmp(
            name,
            system_health_unit_name(system_health_unit_t(unit as ::core::ffi::c_uint)),
        ) == 0 as ::core::ffi::c_int
        {
            return system_health_unit_t(unit as ::core::ffi::c_uint);
        }
        unit += 1;
    }
    return system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE;
}
unsafe extern "C" fn replay_capability(
    mut name: *const ::core::ffi::c_char,
) -> system_health_capability_t {
    if name.is_null() {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    let mut capability: ::core::ffi::c_int =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 as ::core::ffi::c_int;
    while capability
        < system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_COUNT.0 as ::core::ffi::c_int
    {
        if strcmp(
            name,
            system_health_capability_name(system_health_capability_t(
                capability as ::core::ffi::c_uint,
            )),
        ) == 0 as ::core::ffi::c_int
        {
            return system_health_capability_t(capability as ::core::ffi::c_uint);
        }
        capability += 1;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn replay_transition(
    mut policy: *const system_health_policy_t,
    mut stored: *const system_health_incident_transition_t,
    mut incident: *const system_health_incident_record_t,
    mut transition: *mut system_health_transition_t,
) -> bool {
    if policy.is_null() || stored.is_null() || incident.is_null() || transition.is_null() {
        return r#false != 0;
    }
    memset(
        transition as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_transition_t>(),
    );
    if !system_health_condition_from_code(
        &raw const (*incident).condition_code as *const ::core::ffi::c_char,
        &raw mut (*transition).condition,
    ) {
        return r#false != 0;
    }
    (*transition).action = (*stored).action;
    copy_string(
        &raw mut (*transition).incident_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        &raw const (*stored).incident_uuid as *const ::core::ffi::c_char,
    );
    copy_string(
        &raw mut (*transition).event_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        &raw const (*stored).event_id as *const ::core::ffi::c_char,
    );
    copy_string(
        &raw mut (*transition).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw const (*incident).subject as *const ::core::ffi::c_char,
    );
    (*transition).scope = (*incident).scope;
    (*transition).previous_state =
        system_health_state_t(if (*stored).from_state_valid as ::core::ffi::c_int != 0 {
            (*stored).from_state.0
        } else {
            system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN.0
        });
    (*transition).state = (*stored).to_state;
    (*transition).severity = (*stored).severity;
    (*transition).previous_severity = system_health_severity_t(
        if (*stored).action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0 {
            (*stored).severity.0
        } else {
            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
        },
    );
    (*transition).observed_at_ms = (*stored).observed_at_ms;
    (*transition).first_observed_at_ms = (*incident).first_seen_at_ms;
    (*transition).incident_duration_ms = if (*stored).observed_at_ms >= (*incident).first_seen_at_ms
    {
        ((*stored).observed_at_ms - (*incident).first_seen_at_ms) as uint64_t
    } else {
        0 as uint64_t
    };
    (*transition).persisted = r#true != 0;
    let mut rule: *const system_health_condition_policy_t = (&raw const (*policy).conditions
        as *const system_health_condition_policy_t)
        .offset((*transition).condition.0 as isize);
    (*transition).threshold_direction = (*rule).direction;
    if (*stored).action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0 {
        (*transition).threshold_value = (*rule).recovery_threshold;
        (*transition).threshold_for_ms =
            (*rule).recovery_for_seconds.wrapping_mul(1000 as uint32_t);
    } else if (*stored).severity.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0 {
        (*transition).threshold_value = (*rule).critical_threshold;
        (*transition).threshold_for_ms =
            (*rule).critical_for_seconds.wrapping_mul(1000 as uint32_t);
    } else {
        (*transition).threshold_value = (*rule).warning_threshold;
        (*transition).threshold_for_ms = (*rule).warning_for_seconds.wrapping_mul(1000 as uint32_t);
    }
    let mut observation: *mut cJSON = cJSON_ParseWithOpts(
        &raw const (*stored).observation_json as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        r#true,
    );
    if cJSON_IsObject(observation) == 0 {
        cJSON_Delete(observation);
        return r#false != 0;
    }
    let mut metric: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        observation,
        b"metric\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut resource: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        observation,
        b"resource\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut value: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        observation,
        b"value\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut unit: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        observation,
        b"unit\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut capability: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        observation,
        b"capability\0".as_ptr() as *const ::core::ffi::c_char,
    );
    copy_string(
        &raw mut (*transition).observation.metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        if cJSON_IsString(metric) != 0 {
            (*metric).valuestring
        } else {
            &raw const (*incident).condition_code as *const ::core::ffi::c_char
        },
    );
    copy_string(
        &raw mut (*transition).observation.resource_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        if cJSON_IsString(resource) != 0 {
            (*resource).valuestring
        } else {
            &raw const (*incident).subject as *const ::core::ffi::c_char
        },
    );
    (*transition).observation.scope = (*incident).scope;
    (*transition).observation.observed_wall_time_ms = (*stored).observed_at_ms;
    if cJSON_IsNumber(value) != 0 && (*value).valuedouble.is_finite() as ::core::ffi::c_int != 0 {
        system_health_observation_set_available(
            &raw mut (*transition).observation,
            (*value).valuedouble,
            replay_unit(if cJSON_IsString(unit) != 0 {
                (*unit).valuestring
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            }),
        );
    } else {
        system_health_observation_set_unavailable(
            &raw mut (*transition).observation,
            replay_capability(if cJSON_IsString(capability) != 0 {
                (*capability).valuestring
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            }),
        );
    }
    cJSON_Delete(observation);
    return r#true != 0;
}
unsafe extern "C" fn production_transition_sink(
    mut transition: *const system_health_transition_t,
    mut context: *mut ::core::ffi::c_void,
) {
    let mut error: [::core::ffi::c_char; 256] = [0; 256];
    if event_producer_publish_system_health_transition(
        transition,
        &raw const (*transition).event_id as *const ::core::ffi::c_char,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Health transition %s remains pending: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*transition).event_id as *const ::core::ffi::c_char,
            &raw mut error as *mut ::core::ffi::c_char,
        );
        return;
    }
    if db_system_health_transition_set_reconciliation(
        &raw const (*transition).incident_id as *const ::core::ffi::c_char,
        &raw const (*transition).event_id as *const ::core::ffi::c_char,
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECONCILED,
    )
    .0 != db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Health transition %s was enqueued but not reconciled\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*transition).event_id as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn reconcile_pending_transitions(mut policy: *const system_health_policy_t) {
    let mut after_id: int64_t = 0 as int64_t;
    loop {
        let mut pending: [system_health_incident_transition_t; 16] =
            [system_health_incident_transition_t {
                id: 0,
                transition_uuid: [0; 37],
                incident_uuid: [0; 37],
                action: system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN,
                from_state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
                from_state_valid: false,
                to_state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
                severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
                observed_at_ms: 0,
                observation_json: [0; 2049],
                event_id: [0; 37],
                reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
                boot_id: [0; 64],
                run_id: [0; 37],
            }; 16];
        let mut count: ::core::ffi::c_int = db_system_health_transition_list_pending(
            after_id,
            &raw mut pending as *mut system_health_incident_transition_t,
            ::core::mem::size_of::<[system_health_incident_transition_t; 16]>()
                .wrapping_div(::core::mem::size_of::<system_health_incident_transition_t>())
                as ::core::ffi::c_int,
        );
        if count < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Could not enumerate pending health event transitions\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index < count {
            after_id = pending[index as usize].id;
            let mut incident: system_health_incident_record_t = system_health_incident_record_t {
                uuid: [0; 37],
                condition_code: [0; 64],
                subject: [0; 64],
                scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
                state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
                severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
                first_seen_at_ms: 0,
                last_seen_at_ms: 0,
                closed_at_ms: 0,
                observation_json: [0; 2049],
                alert_event_id: [0; 37],
                recovery_event_id: [0; 37],
                reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
                boot_id: [0; 64],
                run_id: [0; 37],
                revision: 0,
            };
            let mut transition: system_health_transition_t = system_health_transition_t {
                action: system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN,
                incident_id: [0; 37],
                event_id: [0; 37],
                condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
                subject: [0; 64],
                scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
                previous_state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
                state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
                previous_severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
                severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
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
                threshold_direction:
                    system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
                threshold_value: 0.,
                threshold_for_ms: 0,
                first_observed_at_ms: 0,
                incident_duration_ms: 0,
                observed_at_ms: 0,
                persisted: false,
            };
            if db_system_health_incident_get_uuid(
                &raw mut (*(&raw mut pending as *mut system_health_incident_transition_t)
                    .offset(index as isize))
                .incident_uuid as *mut ::core::ffi::c_char,
                &raw mut incident,
            )
            .0 != db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
                || !replay_transition(
                    policy,
                    (&raw mut pending as *mut system_health_incident_transition_t)
                        .offset(index as isize),
                    &raw mut incident,
                    &raw mut transition,
                )
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Could not rebuild pending health transition %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*(&raw mut pending as *mut system_health_incident_transition_t)
                        .offset(index as isize))
                    .transition_uuid as *mut ::core::ffi::c_char,
                );
            } else {
                production_transition_sink(&raw mut transition, NULL);
            }
            index += 1;
        }
        if count
            < ::core::mem::size_of::<[system_health_incident_transition_t; 16]>()
                .wrapping_div(::core::mem::size_of::<system_health_incident_transition_t>())
                as ::core::ffi::c_int
        {
            return;
        }
    }
}
unsafe extern "C" fn evaluator_service_main(
    mut argument: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut last_sequence: uint64_t = 0 as uint64_t;
    let mut last_reconcile_ms: uint64_t = 0 as uint64_t;
    loop {
        pthread_mutex_lock(&raw mut service.lock);
        let mut running: bool = service.running;
        let mut context: system_health_evaluation_context_t = service.context;
        pthread_mutex_unlock(&raw mut service.lock);
        if !running {
            break;
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
        if system_health_snapshot_copy(&raw mut snapshot) as ::core::ffi::c_int != 0
            && snapshot.sequence > last_sequence
        {
            system_health_evaluator_evaluate(
                service.evaluator,
                &raw mut snapshot,
                &raw mut context,
            );
            last_sequence = snapshot.sequence;
        }
        let mut now: uint64_t = monotonic_ms();
        if service.transition_sink
            == Some(
                production_transition_sink
                    as unsafe extern "C" fn(
                        *const system_health_transition_t,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )
            && now.wrapping_sub(last_reconcile_ms) >= 5000 as uint64_t
        {
            reconcile_pending_transitions(&raw mut (*service.evaluator).config.policy);
            last_reconcile_ms = now;
        }
        let mut deadline: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &raw mut deadline);
        deadline.tv_nsec += 250000000 as ::core::ffi::c_long;
        if deadline.tv_nsec >= 1000000000 as ::core::ffi::c_long {
            deadline.tv_sec += 1;
            deadline.tv_nsec -= 1000000000 as ::core::ffi::c_long;
        }
        pthread_mutex_lock(&raw mut service.lock);
        if service.running {
            pthread_cond_timedwait(
                &raw mut service.wake,
                &raw mut service.lock,
                &raw mut deadline,
            );
        }
        pthread_mutex_unlock(&raw mut service.lock);
    }
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_service_start() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut service.lock);
    if service.initialized {
        pthread_mutex_unlock(&raw mut service.lock);
        return 0 as ::core::ffi::c_int;
    }
    let mut policy: system_health_policy_t = system_health_policy_t {
        generation: 0,
        settings: system_health_policy_settings_t {
            enabled: false,
            profile: [0; 16],
            fast_interval_seconds: 0,
            normal_interval_seconds: 0,
            slow_interval_seconds: 0,
            device_interval_seconds: 0,
            write_probe_enabled: false,
            hardware_provider: [0; 16],
            presence_interval_seconds: 0,
            incident_retention_days: 0,
        },
        conditions: [system_health_condition_policy_t {
            condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
            profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
            enabled: false,
            overridden: false,
            direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            warning_threshold: 0.,
            critical_threshold: 0.,
            recovery_threshold: 0.,
            warning_for_seconds: 0,
            critical_for_seconds: 0,
            recovery_for_seconds: 0,
        }; 27],
    };
    if system_health_policy_snapshot(&raw mut policy) != 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut service.lock);
        return -1 as ::core::ffi::c_int;
    }
    if !policy.settings.enabled
        || strcmp(
            &raw mut policy.settings.profile as *mut ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        service.initialized = r#true != 0;
        service.disabled = r#true != 0;
        pthread_mutex_unlock(&raw mut service.lock);
        return 0 as ::core::ffi::c_int;
    }
    let mut run_id: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut run_id as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut service.lock);
        return -1 as ::core::ffi::c_int;
    }
    let mut evidence: linux_restart_evidence_t = linux_restart_evidence_t {
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        boot_id: [0; 37],
        run_id: [0; 37],
        host_uptime_seconds: 0.,
        process_start_monotonic_ms: 0,
    };
    if linux_restart_read_evidence(
        b"/proc\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut run_id as *mut ::core::ffi::c_char,
        monotonic_ms(),
        &raw mut evidence,
    ) != 0 as ::core::ffi::c_int
        || evidence.capability.0 != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        pthread_mutex_unlock(&raw mut service.lock);
        return -1 as ::core::ffi::c_int;
    }
    let mut config: system_health_evaluator_config_t = system_health_evaluator_config_t {
        policy: system_health_policy_t {
            generation: 0,
            settings: system_health_policy_settings_t {
                enabled: false,
                profile: [0; 16],
                fast_interval_seconds: 0,
                normal_interval_seconds: 0,
                slow_interval_seconds: 0,
                device_interval_seconds: 0,
                write_probe_enabled: false,
                hardware_provider: [0; 16],
                presence_interval_seconds: 0,
                incident_retention_days: 0,
            },
            conditions: [system_health_condition_policy_t {
                condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
                profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
                enabled: false,
                overridden: false,
                direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
                unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
                warning_threshold: 0.,
                critical_threshold: 0.,
                recovery_threshold: 0.,
                warning_for_seconds: 0,
                critical_for_seconds: 0,
                recovery_for_seconds: 0,
            }; 27],
        },
        boot_id: [0; 64],
        run_id: [0; 37],
        stale_after_ms: 0,
        retry_initial_ms: 0,
        retry_max_ms: 0,
        material_change_debounce_ms: 0,
        material_change_ratio: 0.,
        persist: None,
        persist_context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        transition_sink: None,
        transition_sink_context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    system_health_evaluator_config_defaults(&raw mut config, &raw mut policy);
    copy_string(
        &raw mut config.boot_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw mut evidence.boot_id as *mut ::core::ffi::c_char,
    );
    copy_string(
        &raw mut config.run_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        &raw mut run_id as *mut ::core::ffi::c_char,
    );
    config.persist = Some(
        persist_to_database
            as unsafe extern "C" fn(
                system_health_incident_action_t,
                *const system_health_incident_signal_t,
                *mut system_health_incident_record_t,
                *mut ::core::ffi::c_void,
            ) -> db_system_health_result_t,
    ) as system_health_persist_transition_fn;
    config.transition_sink = (if (&raw const service.transition_sink).read().is_some() {
        service.transition_sink
    } else {
        Some(
            production_transition_sink
                as unsafe extern "C" fn(
                    *const system_health_transition_t,
                    *mut ::core::ffi::c_void,
                ) -> (),
        )
    }) as system_health_transition_sink_fn;
    config.transition_sink_context = service.transition_sink_context;
    service.transition_sink = config.transition_sink;
    let mut evaluator: *mut system_health_evaluator_t =
        system_health_evaluator_create(&raw mut config);
    if evaluator.is_null() {
        pthread_mutex_unlock(&raw mut service.lock);
        return -1 as ::core::ffi::c_int;
    }
    let mut active: [system_health_incident_record_t; 64] = [system_health_incident_record_t {
        uuid: [0; 37],
        condition_code: [0; 64],
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        first_seen_at_ms: 0,
        last_seen_at_ms: 0,
        closed_at_ms: 0,
        observation_json: [0; 2049],
        alert_event_id: [0; 37],
        recovery_event_id: [0; 37],
        reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
        boot_id: [0; 64],
        run_id: [0; 37],
        revision: 0,
    }; 64];
    let mut next: system_health_incident_cursor_t = system_health_incident_cursor_t {
        valid: false,
        last_seen_at_ms: 0,
        uuid: [0; 37],
    };
    let mut active_count: ::core::ffi::c_int = db_system_health_incident_list(
        r#false != 0,
        ::core::ptr::null::<system_health_incident_cursor_t>(),
        &raw mut active as *mut system_health_incident_record_t,
        SYSTEM_HEALTH_MAX_INCIDENTS as ::core::ffi::c_int,
        &raw mut next,
    );
    if active_count < 0 as ::core::ffi::c_int {
        system_health_evaluator_destroy(evaluator);
        pthread_mutex_unlock(&raw mut service.lock);
        return -1 as ::core::ffi::c_int;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < active_count {
        system_health_evaluator_resume(
            evaluator,
            (&raw mut active as *mut system_health_incident_record_t).offset(index as isize),
        );
        index += 1;
    }
    publish_views(evaluator);
    let mut previous: system_health_process_run_t = system_health_process_run_t {
        run_id: [0; 37],
        boot_id: [0; 64],
        started_at_ms: 0,
        closed_at_ms: 0,
        clean_close: false,
    };
    let mut had_previous: bool = r#false != 0;
    let mut started_wall: int64_t = wall_time_ms();
    let mut run_result: db_system_health_result_t = db_system_health_run_open(
        &raw mut run_id as *mut ::core::ffi::c_char,
        &raw mut evidence.boot_id as *mut ::core::ffi::c_char,
        started_wall,
        &raw mut service.run,
        &raw mut previous,
        &raw mut had_previous,
    );
    if run_result.0 != db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0 {
        system_health_evaluator_destroy(evaluator);
        memset(
            &raw mut service.run as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<system_health_process_run_t>(),
        );
        pthread_mutex_unlock(&raw mut service.lock);
        return -1 as ::core::ffi::c_int;
    }
    service.evaluator = evaluator;
    service.context.viable_recording_targets_known = r#false != 0;
    let mut restart: system_health_restart_classification_t = system_health_classify_previous_run(
        &raw mut previous,
        had_previous,
        &raw mut evidence.boot_id as *mut ::core::ffi::c_char,
    );
    if restart.0 != system_health_restart_classification_t::SYSTEM_HEALTH_RESTART_NONE.0 {
        let mut observation: system_health_observation_t = system_health_observation_t {
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
        };
        memset(
            &raw mut observation as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<system_health_observation_t>(),
        );
        copy_string(
            &raw mut observation.metric as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            if restart.0 == system_health_restart_classification_t::SYSTEM_HEALTH_RESTART_PROCESS.0
            {
                b"system.process_restart\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"system.host_reboot\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        copy_string(
            &raw mut observation.resource_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"host\0".as_ptr() as *const ::core::ffi::c_char,
        );
        observation.scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
        observation.sampled_monotonic_ms = monotonic_ms();
        observation.observed_wall_time_ms = started_wall;
        system_health_observation_set_available(
            &raw mut observation,
            1.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
        system_health_evaluator_one_shot(
            evaluator,
            system_health_condition_t::SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART,
            b"host\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST,
            system_health_severity_t::SYSTEM_HEALTH_SEVERITY_ERROR,
            &raw mut observation,
            started_wall,
        );
    }
    if service.transition_sink
        == Some(
            production_transition_sink
                as unsafe extern "C" fn(
                    *const system_health_transition_t,
                    *mut ::core::ffi::c_void,
                ) -> (),
        )
    {
        reconcile_pending_transitions(&raw mut config.policy);
    }
    service.running = r#true != 0;
    if pthread_create(
        &raw mut service.thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            evaluator_service_main
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        service.running = r#false != 0;
        db_system_health_run_close(
            &raw mut service.run.run_id as *mut ::core::ffi::c_char,
            wall_time_ms(),
        );
        system_health_evaluator_destroy(evaluator);
        service.evaluator = ::core::ptr::null_mut::<system_health_evaluator_t>();
        memset(
            &raw mut service.run as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<system_health_process_run_t>(),
        );
        pthread_mutex_unlock(&raw mut service.lock);
        return -1 as ::core::ffi::c_int;
    }
    service.initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut service.lock);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_service_set_context(
    mut context: *const system_health_evaluation_context_t,
) {
    if context.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut service.lock);
    service.context = *context;
    pthread_cond_signal(&raw mut service.wake);
    pthread_mutex_unlock(&raw mut service.lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_service_set_transition_sink(
    mut sink: system_health_transition_sink_fn,
    mut context: *mut ::core::ffi::c_void,
) {
    pthread_mutex_lock(&raw mut service.lock);
    if !service.initialized {
        service.transition_sink = sink;
        service.transition_sink_context = context;
    }
    pthread_mutex_unlock(&raw mut service.lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_service_active_copy(
    mut incidents: *mut system_health_incident_view_t,
    mut capacity: size_t,
) -> size_t {
    pthread_mutex_lock(&raw mut service.lock);
    let mut evaluator: *mut system_health_evaluator_t = service.evaluator;
    let mut count: size_t = if !evaluator.is_null() {
        system_health_evaluator_active_copy(evaluator, incidents, capacity)
    } else {
        0 as size_t
    };
    pthread_mutex_unlock(&raw mut service.lock);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_service_get_stats(
    mut stats: *mut system_health_evaluator_stats_t,
) {
    if stats.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut service.lock);
    if !service.evaluator.is_null() {
        system_health_evaluator_get_stats(service.evaluator, stats);
    } else {
        memset(
            stats as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<system_health_evaluator_stats_t>(),
        );
    }
    pthread_mutex_unlock(&raw mut service.lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_service_copy_run(
    mut run: *mut system_health_process_run_t,
) -> bool {
    if run.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut service.lock);
    let mut available: bool = service.initialized as ::core::ffi::c_int != 0
        && !service.disabled
        && service.run.run_id[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    if available {
        *run = service.run;
    } else {
        memset(
            run as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<system_health_process_run_t>(),
        );
    }
    pthread_mutex_unlock(&raw mut service.lock);
    return available;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_evaluator_service_shutdown(mut clean_shutdown: bool) {
    pthread_mutex_lock(&raw mut service.lock);
    if !service.initialized {
        pthread_mutex_unlock(&raw mut service.lock);
        return;
    }
    if service.disabled {
        service.disabled = r#false != 0;
        service.initialized = r#false != 0;
        pthread_mutex_unlock(&raw mut service.lock);
        return;
    }
    service.running = r#false != 0;
    pthread_cond_signal(&raw mut service.wake);
    pthread_mutex_unlock(&raw mut service.lock);
    pthread_join(
        service.thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    if clean_shutdown as ::core::ffi::c_int != 0
        && service.run.run_id[0usize] as ::core::ffi::c_int != 0
    {
        db_system_health_run_close(
            &raw mut service.run.run_id as *mut ::core::ffi::c_char,
            wall_time_ms(),
        );
    }
    pthread_mutex_lock(&raw mut service.lock);
    system_health_evaluator_destroy(service.evaluator);
    service.evaluator = ::core::ptr::null_mut::<system_health_evaluator_t>();
    memset(
        &raw mut service.run as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_process_run_t>(),
    );
    memset(
        &raw mut service.context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_evaluation_context_t>(),
    );
    service.initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut service.lock);
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
