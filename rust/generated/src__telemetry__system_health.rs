unsafe extern "C" {
    unsafe fn system_health_observation_set_available(
        observation: *mut system_health_observation_t,
        value: ::core::ffi::c_double,
        unit: system_health_unit_t,
    );
    unsafe fn system_health_observation_set_unavailable(
        observation: *mut system_health_observation_t,
        capability: system_health_capability_t,
    );
    unsafe fn system_health_snapshot_append(
        snapshot: *mut system_health_snapshot_t,
        observation: *const system_health_observation_t,
    ) -> bool;
    unsafe fn system_health_observation_sink_append(
        sink: *mut system_health_observation_sink_t,
        observation: *const system_health_observation_t,
    ) -> bool;
    unsafe fn system_health_policy_validate_settings(
        settings: *const system_health_policy_settings_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
    unsafe fn linux_cgroup_collector_init(
        collector: *mut system_health_collector_t,
        state: *mut linux_cgroup_state_t,
    ) -> bool;
    unsafe fn linux_clock_sample(
        source: *const linux_clock_source_t,
        sample: *mut linux_clock_sample_t,
    ) -> ::core::ffi::c_int;
    unsafe fn linux_clock_evaluate(
        state: *mut linux_clock_state_t,
        sample: *const linux_clock_sample_t,
        startup_grace_ms: uint64_t,
        jump_threshold_ms: int64_t,
    ) -> linux_clock_result_t;
    unsafe fn linux_clock_default_source() -> linux_clock_source_t;
    unsafe fn linux_filesystem_collector_init(
        collector: *mut system_health_collector_t,
        state: *mut linux_filesystem_collector_state_t,
    );
    unsafe fn linux_network_state_init(
        state: *mut linux_network_state_t,
        scope: system_health_scope_t,
    );
    unsafe fn linux_network_collector_init(
        collector: *mut system_health_collector_t,
        state: *mut linux_network_state_t,
        interval_seconds: uint32_t,
        stale_after_seconds: uint32_t,
    ) -> bool;
    unsafe fn linux_proc_collector_init(
        collector: *mut system_health_collector_t,
        state: *mut linux_proc_state_t,
    ) -> bool;
    unsafe fn linux_process_collector_init(
        collector: *mut system_health_collector_t,
        state: *mut linux_process_collector_state_t,
    );
    unsafe fn linux_restart_read_evidence(
        proc_root: *const ::core::ffi::c_char,
        run_id: *const ::core::ffi::c_char,
        process_start_monotonic_ms: uint64_t,
        evidence: *mut linux_restart_evidence_t,
    ) -> ::core::ffi::c_int;
    unsafe fn linux_thermal_state_init(state: *mut linux_thermal_state_t);
    unsafe fn linux_thermal_collector_init(
        collector: *mut system_health_collector_t,
        state: *mut linux_thermal_state_t,
        interval_seconds: uint32_t,
        stale_after_seconds: uint32_t,
    ) -> bool;
    unsafe fn health_helper_reap_abandoned();
    unsafe fn health_helper_abandoned_count() -> uint32_t;
    unsafe fn system_health_register_builtin_providers(
        selection: *const ::core::ffi::c_char,
        registration: system_health_provider_registration_fn,
        context: *mut ::core::ffi::c_void,
    ) -> size_t;
    unsafe fn system_health_builtin_provider_request_device_refresh() -> bool;
    unsafe fn recording_io_take_device_refresh_request() -> bool;
    unsafe fn system_health_provider_registry_reset();
    unsafe fn system_health_provider_registry_register(
        provider: *const system_health_provider_t,
    ) -> bool;
    unsafe fn system_health_provider_registry_collect(
        context: *const system_health_collect_context_t,
        sink: *mut system_health_observation_sink_t,
        resources_dropped: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_provider_registry_destroy();
    unsafe fn system_health_provider_registry_count() -> size_t;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __pid_t = ::core::ffi::c_int;
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct system_health_sampling_tier_t(pub ::core::ffi::c_uint);
impl system_health_sampling_tier_t {
    pub const SYSTEM_HEALTH_TIER_FAST: Self = Self(0);
    pub const SYSTEM_HEALTH_TIER_NORMAL: Self = Self(1);
    pub const SYSTEM_HEALTH_TIER_SLOW: Self = Self(2);
    pub const SYSTEM_HEALTH_TIER_DEVICE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_collect_context_t {
    pub monotonic_ms: uint64_t,
    pub wall_time_ms: int64_t,
    pub proc_root: *const ::core::ffi::c_char,
    pub sys_root: *const ::core::ffi::c_char,
    pub cgroup_root: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_sink_t {
    pub items: *mut system_health_observation_t,
    pub capacity: size_t,
    pub count: size_t,
    pub dropped: size_t,
}
pub type system_health_collect_fn = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const system_health_collect_context_t,
        *mut system_health_observation_sink_t,
    ) -> ::core::ffi::c_int,
>;
pub type system_health_collector_destroy_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_collector_t {
    pub name: [::core::ffi::c_char; 32],
    pub scope: system_health_scope_t,
    pub tier: system_health_sampling_tier_t,
    pub interval_seconds: uint32_t,
    pub stale_after_seconds: uint32_t,
    pub state: *mut ::core::ffi::c_void,
    pub collect: system_health_collect_fn,
    pub destroy: system_health_collector_destroy_fn,
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
pub struct system_health_provider_resource_t {
    pub id: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_provider_inventory_t {
    pub resources: [system_health_provider_resource_t; 16],
    pub count: size_t,
    pub dropped: size_t,
}
pub type system_health_provider_discover_fn = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const system_health_collect_context_t,
        *mut system_health_provider_inventory_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_provider_t {
    pub name: [::core::ffi::c_char; 32],
    pub capability: system_health_capability_t,
    pub state: *mut ::core::ffi::c_void,
    pub discover: system_health_provider_discover_fn,
    pub collect: system_health_collect_fn,
    pub destroy: system_health_collector_destroy_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_options_t {
    pub enabled: bool,
    pub register_builtin_collectors: bool,
    pub tier_interval_ms: [uint32_t; 4],
    pub collector_deadline_ms: [uint32_t; 4],
    pub proc_root: [::core::ffi::c_char; 1024],
    pub sys_root: [::core::ffi::c_char; 1024],
    pub cgroup_root: [::core::ffi::c_char; 1024],
    pub root_path: [::core::ffi::c_char; 1024],
    pub recording_path: [::core::ffi::c_char; 1024],
    pub hardware_provider: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_summary_t {
    pub sequence: uint64_t,
    pub completed_monotonic_ms: uint64_t,
    pub completed_wall_time_ms: int64_t,
    pub observation_count: uint32_t,
    pub observations_dropped: uint32_t,
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
pub type system_health_runtime_t = system_health_runtime;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_runtime {
    pub options: system_health_options_t,
    pub lock: pthread_mutex_t,
    pub running: atomic_bool,
    pub started: bool,
    pub collectors: [collector_slot_t; 32],
    pub collector_count: size_t,
    pub provider_slot: collector_slot_t,
    pub tier_cache: [system_health_snapshot_t; 4],
    pub tier_valid: [bool; 4],
    pub latest: [system_health_snapshot_t; 2],
    pub active_latest: ::core::ffi::c_uint,
    pub ring: *mut system_health_summary_t,
    pub ring_head: size_t,
    pub ring_count: size_t,
    pub workers: [tier_worker_t; 4],
    pub stats: system_health_stats_t,
    pub proc: linux_proc_state_t,
    pub cgroup: linux_cgroup_state_t,
    pub process: linux_process_collector_state_t,
    pub filesystem: linux_filesystem_collector_state_t,
    pub thermal: linux_thermal_state_t,
    pub network: linux_network_state_t,
    pub clock_source: linux_clock_source_t,
    pub clock: linux_clock_state_t,
    pub run_id: [::core::ffi::c_char; 37],
    pub process_start_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_clock_state_t {
    pub initialized: bool,
    pub startup_monotonic_ms: uint64_t,
    pub previous_realtime_ms: int64_t,
    pub previous_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_clock_source_t {
    pub context: *mut ::core::ffi::c_void,
    pub realtime: linux_clock_read_time_fn,
    pub monotonic: linux_clock_read_time_fn,
    pub synchronization: linux_clock_read_sync_fn,
}
pub type linux_clock_read_sync_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut bool) -> ::core::ffi::c_int>;
pub type linux_clock_read_time_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut int64_t) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_network_state_t {
    pub interfaces: [linux_network_interface_state_t; 32],
    pub interface_count: size_t,
    pub scope: system_health_scope_t,
    pub primary_override: [::core::ffi::c_char; 64],
    pub resources_dropped_total: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_network_interface_state_t {
    pub internal_name: [::core::ffi::c_char; 64],
    pub id: [::core::ffi::c_char; 64],
    pub previous: [uint64_t; 8],
    pub exported_total: [uint64_t; 8],
    pub previous_valid: [bool; 8],
    pub previous_monotonic_ms: uint64_t,
    pub carrier_valid: bool,
    pub carrier: bool,
    pub carrier_flaps: uint64_t,
    pub present: bool,
    pub seen: bool,
    pub missing_cycles: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_thermal_state_t {
    pub sensors: [linux_thermal_sensor_state_t; 64],
    pub sensor_count: size_t,
    pub resources_dropped_total: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_thermal_sensor_state_t {
    pub id: [::core::ffi::c_char; 64],
    pub limit_metric: [::core::ffi::c_char; 64],
    pub present: bool,
    pub seen: bool,
    pub missing_cycles: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_collector_state_t {
    pub resources: [linux_filesystem_resource_t; 18],
    pub resource_count: size_t,
    pub resources_dropped: size_t,
    pub mountinfo_path: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_resource_t {
    pub logical_id: [::core::ffi::c_char; 64],
    pub path: [::core::ffi::c_char; 1024],
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 1024],
    pub expected_device_key: [::core::ffi::c_char; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_process_collector_state_t {
    pub status_path: [::core::ffi::c_char; 512],
    pub fd_directory_path: [::core::ffi::c_char; 512],
    pub pids_max_path: [::core::ffi::c_char; 512],
    pub fd_scan_limit: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_state_t {
    pub memberships: linux_cgroup_memberships_t,
    pub cpu_valid: bool,
    pub cpu_quota_valid: bool,
    pub cpu_max: linux_cgroup_cpu_max_t,
    pub oom_valid: bool,
    pub version: linux_cgroup_version_t,
    pub cpu: linux_cgroup_cpu_stat_t,
    pub oom_kills: uint64_t,
    pub sampled_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_cpu_stat_t {
    pub usage_usec: uint64_t,
    pub throttle_present: bool,
    pub periods: uint64_t,
    pub throttled_periods: uint64_t,
    pub throttled_usec: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct linux_cgroup_version_t(pub ::core::ffi::c_uint);
impl linux_cgroup_version_t {
    pub const LINUX_CGROUP_UNKNOWN: Self = Self(0);
    pub const LINUX_CGROUP_V1: Self = Self(1);
    pub const LINUX_CGROUP_V2: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_cpu_max_t {
    pub quota_usec: linux_cgroup_limit_t,
    pub period_usec: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_limit_t {
    pub unlimited: bool,
    pub value: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_memberships_t {
    pub version: linux_cgroup_version_t,
    pub unified: [::core::ffi::c_char; 512],
    pub cpu: [::core::ffi::c_char; 512],
    pub cpuacct: [::core::ffi::c_char; 512],
    pub memory: [::core::ffi::c_char; 512],
    pub pids: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_state_t {
    pub cpu_valid: bool,
    pub cpu: linux_proc_cpu_times_t,
    pub cpu_monotonic_ms: uint64_t,
    pub vmstat_valid: bool,
    pub vmstat: linux_proc_vmstat_t,
    pub vmstat_monotonic_ms: uint64_t,
    pub pressure_valid: [bool; 3],
    pub pressure: [linux_proc_pressure_t; 3],
    pub pressure_monotonic_ms: [uint64_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_pressure_t {
    pub some_present: bool,
    pub full_present: bool,
    pub some_avg10_ratio: ::core::ffi::c_double,
    pub full_avg10_ratio: ::core::ffi::c_double,
    pub some_total_usec: uint64_t,
    pub full_total_usec: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_vmstat_t {
    pub major_faults: uint64_t,
    pub swap_in_pages: uint64_t,
    pub swap_out_pages: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_cpu_times_t {
    pub user: uint64_t,
    pub nice: uint64_t,
    pub system: uint64_t,
    pub idle: uint64_t,
    pub iowait: uint64_t,
    pub irq: uint64_t,
    pub softirq: uint64_t,
    pub steal: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tier_worker_t {
    pub runtime: *mut system_health_runtime,
    pub tier: system_health_sampling_tier_t,
    pub thread: pthread_t,
    pub lock: pthread_mutex_t,
    pub wake: pthread_cond_t,
    pub initialized: bool,
    pub started: bool,
    pub requested: bool,
    pub startup_ready: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collector_slot_t {
    pub collector: system_health_collector_t,
    pub busy: atomic_bool,
    pub stats_lock: pthread_mutex_t,
    pub stats: system_health_collector_stats_t,
}
pub type atomic_bool = bool;
pub type system_health_provider_registration_fn =
    Option<unsafe extern "C" fn(*const system_health_provider_t, *mut ::core::ffi::c_void) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_restart_evidence_t {
    pub capability: system_health_capability_t,
    pub boot_id: [::core::ffi::c_char; 37],
    pub run_id: [::core::ffi::c_char; 37],
    pub host_uptime_seconds: ::core::ffi::c_double,
    pub process_start_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_clock_result_t {
    pub capability: system_health_capability_t,
    pub synchronization_known: bool,
    pub synchronized: bool,
    pub startup_grace_active: bool,
    pub jump_detected: bool,
    pub jump_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_clock_sample_t {
    pub capability: system_health_capability_t,
    pub synchronization_known: bool,
    pub synchronized: bool,
    pub realtime_ms: int64_t,
    pub monotonic_ms: uint64_t,
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
pub const SYSTEM_HEALTH_MAX_FILESYSTEMS: ::core::ffi::c_uint = 18 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_MAX_DEVICES: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_RING_SAMPLES: ::core::ffi::c_uint = 120 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_MAX_COLLECTORS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_MAX_OBSERVATIONS: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_TIER_COUNT: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LINUX_CLOCK_DEFAULT_STARTUP_GRACE_MS: ::core::ffi::c_ulonglong = (10
    as ::core::ffi::c_ulonglong)
    .wrapping_mul(60 as ::core::ffi::c_ulonglong)
    .wrapping_mul(1000 as ::core::ffi::c_ulonglong);
pub const LINUX_CLOCK_DEFAULT_JUMP_THRESHOLD_MS: ::core::ffi::c_longlong =
    2000 as ::core::ffi::c_longlong;
static mut lifecycle_lock: pthread_mutex_t = pthread_mutex_t {
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
static mut runtime: *mut system_health_runtime_t =
    ::core::ptr::null_mut::<system_health_runtime_t>();
static mut disabled_initialized: bool = false;
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
unsafe extern "C" fn bounded_copy(
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
pub unsafe extern "C" fn system_health_options_defaults(mut options: *mut system_health_options_t) {
    if options.is_null() {
        return;
    }
    memset(
        options as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_options_t>(),
    );
    (*options).enabled = r#true != 0;
    (*options).register_builtin_collectors = r#true != 0;
    (*options).tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0 as ::core::ffi::c_int as usize] =
        10000 as ::core::ffi::c_uint as uint32_t;
    (*options).tier_interval_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0
        as ::core::ffi::c_int as usize] = 60000 as ::core::ffi::c_uint as uint32_t;
    (*options).tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_SLOW.0 as ::core::ffi::c_int as usize] =
        300000 as ::core::ffi::c_uint as uint32_t;
    (*options).tier_interval_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0
        as ::core::ffi::c_int as usize] = 900000 as ::core::ffi::c_uint as uint32_t;
    (*options).collector_deadline_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0 as ::core::ffi::c_int as usize] =
        2000 as ::core::ffi::c_uint as uint32_t;
    (*options).collector_deadline_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0
        as ::core::ffi::c_int as usize] = 5000 as ::core::ffi::c_uint as uint32_t;
    (*options).collector_deadline_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_SLOW.0 as ::core::ffi::c_int as usize] =
        15000 as ::core::ffi::c_uint as uint32_t;
    (*options).collector_deadline_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0
        as ::core::ffi::c_int as usize] = 30000 as ::core::ffi::c_uint as uint32_t;
    bounded_copy(
        &raw mut (*options).proc_root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"/proc\0".as_ptr() as *const ::core::ffi::c_char,
    );
    bounded_copy(
        &raw mut (*options).sys_root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"/sys\0".as_ptr() as *const ::core::ffi::c_char,
    );
    bounded_copy(
        &raw mut (*options).cgroup_root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"/sys/fs/cgroup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    bounded_copy(
        &raw mut (*options).root_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
    );
    bounded_copy(
        &raw mut (*options).recording_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
    );
    bounded_copy(
        &raw mut (*options).hardware_provider as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"auto\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_options_from_policy(
    mut settings: *const system_health_policy_settings_t,
    mut recording_path: *const ::core::ffi::c_char,
    mut options: *mut system_health_options_t,
) -> ::core::ffi::c_int {
    if settings.is_null() || options.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if system_health_policy_validate_settings(
        settings,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    system_health_options_defaults(options);
    (*options).enabled = (*settings).enabled as ::core::ffi::c_int != 0
        && strcmp(
            &raw const (*settings).profile as *const ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int;
    (*options).tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0 as ::core::ffi::c_int as usize] =
        (*settings)
            .fast_interval_seconds
            .wrapping_mul(1000 as uint32_t);
    (*options).tier_interval_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0
        as ::core::ffi::c_int as usize] = (*settings)
        .normal_interval_seconds
        .wrapping_mul(1000 as uint32_t);
    (*options).tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_SLOW.0 as ::core::ffi::c_int as usize] =
        (*settings)
            .slow_interval_seconds
            .wrapping_mul(1000 as uint32_t);
    (*options).tier_interval_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0
        as ::core::ffi::c_int as usize] = (*settings)
        .device_interval_seconds
        .wrapping_mul(1000 as uint32_t);
    if !recording_path.is_null() && *recording_path.offset(0isize) as ::core::ffi::c_int != 0 {
        bounded_copy(
            &raw mut (*options).recording_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            recording_path,
        );
    }
    bounded_copy(
        &raw mut (*options).hardware_provider as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        &raw const (*settings).hardware_provider as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn register_collector_locked(
    mut state: *mut system_health_runtime_t,
    mut source: *const system_health_collector_t,
) -> bool {
    if state.is_null()
        || source.is_null()
        || (*source).name[0usize] == 0
        || (*source).collect.is_none()
        || memchr(
            &raw const (*source).name as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        )
        .is_null()
        || (*source).tier.0 < system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0
        || (*source).tier.0 > system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0
        || (*state).collector_count >= SYSTEM_HEALTH_MAX_COLLECTORS as size_t
    {
        return r#false != 0;
    }
    let mut index: size_t = 0 as size_t;
    while index < (*state).collector_count {
        if strcmp(
            &raw mut (*(&raw mut (*state).collectors as *mut collector_slot_t)
                .offset(index as isize))
            .collector
            .name as *mut ::core::ffi::c_char,
            &raw const (*source).name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        index = index.wrapping_add(1);
    }
    let c2rust_fresh0 = (*state).collector_count;
    (*state).collector_count = (*state).collector_count.wrapping_add(1);
    let mut slot: *mut collector_slot_t =
        (&raw mut (*state).collectors as *mut collector_slot_t).offset(c2rust_fresh0 as isize);
    (*slot).collector = *source;
    *&raw mut (*slot).busy = r#false != 0;
    if pthread_mutex_init(
        &raw mut (*slot).stats_lock,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        memset(
            slot as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<collector_slot_t>(),
        );
        (*state).collector_count = (*state).collector_count.wrapping_sub(1);
        return r#false != 0;
    }
    bounded_copy(
        &raw mut (*slot).stats.name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        &raw const (*source).name as *const ::core::ffi::c_char,
    );
    (*slot).stats.scope = (*source).scope;
    (*slot).stats.tier = (*source).tier;
    (*slot).stats.interval_seconds = (*source).interval_seconds;
    (*slot).stats.stale_after_seconds = (*source).stale_after_seconds;
    return r#true != 0;
}
unsafe extern "C" fn collector_stats_sync_contract(mut slot: *mut collector_slot_t) {
    pthread_mutex_lock(&raw mut (*slot).stats_lock);
    bounded_copy(
        &raw mut (*slot).stats.name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        &raw mut (*slot).collector.name as *mut ::core::ffi::c_char,
    );
    (*slot).stats.scope = (*slot).collector.scope;
    (*slot).stats.tier = (*slot).collector.tier;
    (*slot).stats.interval_seconds = (*slot).collector.interval_seconds;
    (*slot).stats.stale_after_seconds = (*slot).collector.stale_after_seconds;
    pthread_mutex_unlock(&raw mut (*slot).stats_lock);
}
unsafe extern "C" fn add_filesystem_resource(
    mut state: *mut linux_filesystem_collector_state_t,
    mut id: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
) {
    if state.is_null()
        || id.is_null()
        || path.is_null()
        || (*state).resource_count >= SYSTEM_HEALTH_MAX_FILESYSTEMS as size_t
    {
        return;
    }
    let c2rust_fresh1 = (*state).resource_count;
    (*state).resource_count = (*state).resource_count.wrapping_add(1);
    let mut resource: *mut linux_filesystem_resource_t = (&raw mut (*state).resources
        as *mut linux_filesystem_resource_t)
        .offset(c2rust_fresh1 as isize);
    memset(
        resource as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_filesystem_resource_t>(),
    );
    bounded_copy(
        &raw mut (*resource).logical_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        id,
    );
    bounded_copy(
        &raw mut (*resource).path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        path,
    );
}
unsafe extern "C" fn append_builtin_observation(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut metric: *const ::core::ffi::c_char,
    mut scope: system_health_scope_t,
    mut capability: system_health_capability_t,
    mut value_valid: bool,
    mut value: ::core::ffi::c_double,
    mut unit: system_health_unit_t,
) {
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
    bounded_copy(
        &raw mut observation.metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        metric,
    );
    bounded_copy(
        &raw mut observation.resource_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        if scope.0 == system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS.0 {
            b"process\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"host\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    observation.scope = scope;
    observation.sampled_monotonic_ms = (*context).monotonic_ms;
    observation.observed_wall_time_ms = (*context).wall_time_ms;
    if value_valid {
        system_health_observation_set_available(&raw mut observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&raw mut observation, capability);
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn collect_clock(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut system_health_runtime_t = opaque as *mut system_health_runtime_t;
    let mut sample: linux_clock_sample_t = linux_clock_sample_t {
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        synchronization_known: false,
        synchronized: false,
        realtime_ms: 0,
        monotonic_ms: 0,
    };
    if linux_clock_sample(&raw mut (*state).clock_source, &raw mut sample)
        != 0 as ::core::ffi::c_int
    {
        append_builtin_observation(
            sink,
            context,
            b"clock.synchronized\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
            r#false != 0,
            0.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: linux_clock_result_t = linux_clock_evaluate(
        &raw mut (*state).clock,
        &raw mut sample,
        LINUX_CLOCK_DEFAULT_STARTUP_GRACE_MS as uint64_t,
        LINUX_CLOCK_DEFAULT_JUMP_THRESHOLD_MS as int64_t,
    );
    append_builtin_observation(
        sink,
        context,
        b"clock.synchronized\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST,
        result.capability,
        result.synchronization_known,
        if result.synchronized as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    append_builtin_observation(
        sink,
        context,
        b"clock.jump_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST,
        result.capability,
        result.capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        if result.jump_detected as ::core::ffi::c_int != 0 {
            result.jump_ms as ::core::ffi::c_double / 1000.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_SECONDS,
    );
    append_builtin_observation(
        sink,
        context,
        b"clock.startup_grace\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        r#true != 0,
        if result.startup_grace_active as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn collect_restart(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut system_health_runtime_t = opaque as *mut system_health_runtime_t;
    let mut evidence: linux_restart_evidence_t = linux_restart_evidence_t {
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        boot_id: [0; 37],
        run_id: [0; 37],
        host_uptime_seconds: 0.,
        process_start_monotonic_ms: 0,
    };
    if linux_restart_read_evidence(
        (*context).proc_root,
        &raw mut (*state).run_id as *mut ::core::ffi::c_char,
        (*state).process_start_monotonic_ms,
        &raw mut evidence,
    ) != 0 as ::core::ffi::c_int
    {
        append_builtin_observation(
            sink,
            context,
            b"system.uptime_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
            r#false != 0,
            0.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_SECONDS,
        );
        return -1 as ::core::ffi::c_int;
    }
    append_builtin_observation(
        sink,
        context,
        b"system.uptime_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST,
        evidence.capability,
        evidence.capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        evidence.host_uptime_seconds,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_SECONDS,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn initialize_run_id(mut state: *mut system_health_runtime_t) {
    let mut now: uint64_t = monotonic_ms();
    let mut pid: ::core::ffi::c_ulong = getpid() as ::core::ffi::c_ulong;
    snprintf(
        &raw mut (*state).run_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        b"%08x-%04x-4%03x-8%03x-%012llx\0".as_ptr() as *const ::core::ffi::c_char,
        (now & 0xffffffff as uint64_t) as ::core::ffi::c_uint,
        (pid & 0xffff as ::core::ffi::c_ulong) as ::core::ffi::c_uint,
        (now >> 16 as ::core::ffi::c_uint & 0xfff as uint64_t) as ::core::ffi::c_uint,
        ((pid as uint64_t ^ now) & 0xfff as uint64_t) as ::core::ffi::c_uint,
        (now << 12 as ::core::ffi::c_uint ^ pid as uint64_t) as ::core::ffi::c_ulonglong
            & 0xffffffffffff as ::core::ffi::c_ulonglong,
    );
    (*state).process_start_monotonic_ms = now;
}
unsafe extern "C" fn register_portable_collectors(
    mut state: *mut system_health_runtime_t,
) -> ::core::ffi::c_int {
    let mut collector: system_health_collector_t = system_health_collector_t {
        name: [0; 32],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        tier: system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST,
        interval_seconds: 0,
        stale_after_seconds: 0,
        state: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        collect: None,
        destroy: None,
    };
    if !linux_proc_collector_init(&raw mut collector, &raw mut (*state).proc)
        || !register_collector_locked(state, &raw mut collector)
    {
        return -1 as ::core::ffi::c_int;
    }
    (*state).collectors[(*state).collector_count.wrapping_sub(1 as size_t)]
        .collector
        .interval_seconds = (*state).options.tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0 as ::core::ffi::c_int as usize]
        .wrapping_div(1000 as uint32_t);
    (*state).collectors[(*state).collector_count.wrapping_sub(1 as size_t)]
        .collector
        .stale_after_seconds = (*state).collectors
        [(*state).collector_count.wrapping_sub(1 as size_t)]
    .collector
    .interval_seconds
    .wrapping_mul(3 as uint32_t);
    if !linux_cgroup_collector_init(&raw mut collector, &raw mut (*state).cgroup)
        || !register_collector_locked(state, &raw mut collector)
    {
        return -1 as ::core::ffi::c_int;
    }
    (*state).collectors[(*state).collector_count.wrapping_sub(1 as size_t)]
        .collector
        .interval_seconds = (*state).options.tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0 as ::core::ffi::c_int as usize]
        .wrapping_div(1000 as uint32_t);
    (*state).collectors[(*state).collector_count.wrapping_sub(1 as size_t)]
        .collector
        .stale_after_seconds = (*state).collectors
        [(*state).collector_count.wrapping_sub(1 as size_t)]
    .collector
    .interval_seconds
    .wrapping_mul(3 as uint32_t);
    memset(
        &raw mut (*state).process as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_process_collector_state_t>(),
    );
    linux_process_collector_init(&raw mut collector, &raw mut (*state).process);
    if !register_collector_locked(state, &raw mut collector) {
        return -1 as ::core::ffi::c_int;
    }
    (*state).collectors[(*state).collector_count.wrapping_sub(1 as size_t)]
        .collector
        .interval_seconds = (*state).options.tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0 as ::core::ffi::c_int as usize]
        .wrapping_div(1000 as uint32_t);
    (*state).collectors[(*state).collector_count.wrapping_sub(1 as size_t)]
        .collector
        .stale_after_seconds = (*state).collectors
        [(*state).collector_count.wrapping_sub(1 as size_t)]
    .collector
    .interval_seconds
    .wrapping_mul(3 as uint32_t);
    memset(
        &raw mut (*state).filesystem as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_filesystem_collector_state_t>(),
    );
    add_filesystem_resource(
        &raw mut (*state).filesystem,
        b"root\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*state).options.root_path as *mut ::core::ffi::c_char,
    );
    if strcmp(
        &raw mut (*state).options.recording_path as *mut ::core::ffi::c_char,
        &raw mut (*state).options.root_path as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        add_filesystem_resource(
            &raw mut (*state).filesystem,
            b"recording\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*state).options.recording_path as *mut ::core::ffi::c_char,
        );
    }
    linux_filesystem_collector_init(&raw mut collector, &raw mut (*state).filesystem);
    if !register_collector_locked(state, &raw mut collector) {
        return -1 as ::core::ffi::c_int;
    }
    (*state).collectors[(*state).collector_count.wrapping_sub(1 as size_t)]
        .collector
        .interval_seconds = (*state).options.tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0 as ::core::ffi::c_int as usize]
        .wrapping_div(1000 as uint32_t);
    (*state).collectors[(*state).collector_count.wrapping_sub(1 as size_t)]
        .collector
        .stale_after_seconds = (*state).collectors
        [(*state).collector_count.wrapping_sub(1 as size_t)]
    .collector
    .interval_seconds
    .wrapping_mul(3 as uint32_t);
    linux_thermal_state_init(&raw mut (*state).thermal);
    if !linux_thermal_collector_init(
        &raw mut collector,
        &raw mut (*state).thermal,
        (*state)
            .options
            .tier_interval_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0
                as ::core::ffi::c_int as usize]
            .wrapping_div(1000 as uint32_t),
        (*state)
            .options
            .tier_interval_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0
                as ::core::ffi::c_int as usize]
            .wrapping_div(1000 as uint32_t)
            .wrapping_mul(3 as uint32_t),
    ) || !register_collector_locked(state, &raw mut collector)
    {
        return -1 as ::core::ffi::c_int;
    }
    linux_network_state_init(
        &raw mut (*state).network,
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER,
    );
    if !linux_network_collector_init(
        &raw mut collector,
        &raw mut (*state).network,
        (*state)
            .options
            .tier_interval_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0
                as ::core::ffi::c_int as usize]
            .wrapping_div(1000 as uint32_t),
        (*state)
            .options
            .tier_interval_ms[system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0
                as ::core::ffi::c_int as usize]
            .wrapping_div(1000 as uint32_t)
            .wrapping_mul(3 as uint32_t),
    ) || !register_collector_locked(state, &raw mut collector)
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut (*state).clock as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_clock_state_t>(),
    );
    (*state).clock_source = linux_clock_default_source();
    memset(
        &raw mut collector as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_collector_t>(),
    );
    bounded_copy(
        &raw mut collector.name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"linux_clock\0".as_ptr() as *const ::core::ffi::c_char,
    );
    collector.scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
    collector.tier = system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL;
    collector.interval_seconds = (*state).options.tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL.0 as ::core::ffi::c_int as usize]
        .wrapping_div(1000 as uint32_t);
    collector.stale_after_seconds = collector.interval_seconds.wrapping_mul(3 as uint32_t);
    collector.state = state as *mut ::core::ffi::c_void;
    collector.collect = Some(
        collect_clock
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_observation_sink_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_collect_fn;
    if !register_collector_locked(state, &raw mut collector) {
        return -1 as ::core::ffi::c_int;
    }
    initialize_run_id(state);
    memset(
        &raw mut collector as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_collector_t>(),
    );
    bounded_copy(
        &raw mut collector.name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"linux_restart\0".as_ptr() as *const ::core::ffi::c_char,
    );
    collector.scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
    collector.tier = system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_SLOW;
    collector.interval_seconds = (*state).options.tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_SLOW.0 as ::core::ffi::c_int as usize]
        .wrapping_div(1000 as uint32_t);
    collector.stale_after_seconds = collector.interval_seconds.wrapping_mul(3 as uint32_t);
    collector.state = state as *mut ::core::ffi::c_void;
    collector.collect = Some(
        collect_restart
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_observation_sink_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_collect_fn;
    if !register_collector_locked(state, &raw mut collector) {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn builtin_provider_callback(
    mut provider: *const system_health_provider_t,
    mut context: *mut ::core::ffi::c_void,
) -> bool {
    let mut full: bool =
        system_health_provider_registry_count() >= SYSTEM_HEALTH_MAX_DEVICES as size_t;
    let mut registered: bool = system_health_provider_registry_register(provider);
    if !registered && full as ::core::ffi::c_int != 0 && !context.is_null() {
        let mut state: *mut system_health_runtime_t = context as *mut system_health_runtime_t;
        (*state).stats.coverage_overflows = (*state).stats.coverage_overflows.wrapping_add(1);
    }
    return registered;
}
unsafe extern "C" fn init_worker(
    mut worker: *mut tier_worker_t,
    mut state: *mut system_health_runtime_t,
    mut tier: system_health_sampling_tier_t,
) -> ::core::ffi::c_int {
    let mut attributes: pthread_condattr_t = pthread_condattr_t { __size: [0; 4] };
    if pthread_mutex_init(
        &raw mut (*worker).lock,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if pthread_condattr_init(&raw mut attributes) != 0 as ::core::ffi::c_int {
        pthread_mutex_destroy(&raw mut (*worker).lock);
        return -1 as ::core::ffi::c_int;
    }
    pthread_condattr_setclock(&raw mut attributes, CLOCK_MONOTONIC);
    let mut result: ::core::ffi::c_int =
        pthread_cond_init(&raw mut (*worker).wake, &raw mut attributes);
    pthread_condattr_destroy(&raw mut attributes);
    if result != 0 as ::core::ffi::c_int {
        pthread_mutex_destroy(&raw mut (*worker).lock);
        return -1 as ::core::ffi::c_int;
    }
    (*worker).runtime = state as *mut system_health_runtime;
    (*worker).tier = tier;
    (*worker).initialized = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_init(
    mut options: *const system_health_options_t,
) -> ::core::ffi::c_int {
    if options.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut lifecycle_lock);
    if !runtime.is_null() || disabled_initialized as ::core::ffi::c_int != 0 {
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return -1 as ::core::ffi::c_int;
    }
    if !(*options).enabled {
        disabled_initialized = r#true != 0;
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return 0 as ::core::ffi::c_int;
    }
    let mut tier: size_t = 0 as size_t;
    while tier < SYSTEM_HEALTH_TIER_COUNT as size_t {
        if (*options).tier_interval_ms[tier] == 0 as uint32_t
            || (*options).collector_deadline_ms[tier] == 0 as uint32_t
        {
            pthread_mutex_unlock(&raw mut lifecycle_lock);
            return -1 as ::core::ffi::c_int;
        }
        tier = tier.wrapping_add(1);
    }
    let mut state: *mut system_health_runtime_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<system_health_runtime_t>(),
    ) as *mut system_health_runtime_t;
    if state.is_null() {
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return -1 as ::core::ffi::c_int;
    }
    (*state).ring = calloc(
        SYSTEM_HEALTH_RING_SAMPLES as size_t,
        ::core::mem::size_of::<system_health_summary_t>(),
    ) as *mut system_health_summary_t;
    if (*state).ring.is_null()
        || pthread_mutex_init(
            &raw mut (*state).lock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        ) != 0 as ::core::ffi::c_int
    {
        free((*state).ring as *mut ::core::ffi::c_void);
        free(state as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return -1 as ::core::ffi::c_int;
    }
    (*state).options = *options;
    *&raw mut (*state).running = r#false != 0;
    (*state).stats.initialized = r#true != 0;
    (*state).stats.enabled = r#true != 0;
    (*state).stats.ring_allocated = r#true != 0;
    system_health_provider_registry_reset();
    let mut tier_0: size_t = 0 as size_t;
    while tier_0 < SYSTEM_HEALTH_TIER_COUNT as size_t {
        if init_worker(
            (&raw mut (*state).workers as *mut tier_worker_t).offset(tier_0 as isize),
            state,
            system_health_sampling_tier_t(tier_0 as ::core::ffi::c_uint),
        ) != 0 as ::core::ffi::c_int
        {
            let mut prior: size_t = 0 as size_t;
            while prior < tier_0 {
                pthread_cond_destroy(
                    &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                        .offset(prior as isize))
                    .wake,
                );
                pthread_mutex_destroy(
                    &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                        .offset(prior as isize))
                    .lock,
                );
                prior = prior.wrapping_add(1);
            }
            pthread_mutex_destroy(&raw mut (*state).lock);
            free((*state).ring as *mut ::core::ffi::c_void);
            free(state as *mut ::core::ffi::c_void);
            pthread_mutex_unlock(&raw mut lifecycle_lock);
            return -1 as ::core::ffi::c_int;
        }
        tier_0 = tier_0.wrapping_add(1);
    }
    if (*options).register_builtin_collectors as ::core::ffi::c_int != 0
        && register_portable_collectors(state) != 0 as ::core::ffi::c_int
    {
        let mut index: size_t = 0 as size_t;
        while index < (*state).collector_count {
            pthread_mutex_destroy(
                &raw mut (*(&raw mut (*state).collectors as *mut collector_slot_t)
                    .offset(index as isize))
                .stats_lock,
            );
            index = index.wrapping_add(1);
        }
        let mut tier_1: size_t = 0 as size_t;
        while tier_1 < SYSTEM_HEALTH_TIER_COUNT as size_t {
            pthread_cond_destroy(
                &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                    .offset(tier_1 as isize))
                .wake,
            );
            pthread_mutex_destroy(
                &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                    .offset(tier_1 as isize))
                .lock,
            );
            tier_1 = tier_1.wrapping_add(1);
        }
        pthread_mutex_destroy(&raw mut (*state).lock);
        free((*state).ring as *mut ::core::ffi::c_void);
        free(state as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return -1 as ::core::ffi::c_int;
    }
    system_health_register_builtin_providers(
        &raw const (*options).hardware_provider as *const ::core::ffi::c_char,
        Some(
            builtin_provider_callback
                as unsafe extern "C" fn(
                    *const system_health_provider_t,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        state as *mut ::core::ffi::c_void,
    );
    memset(
        &raw mut (*state).provider_slot.collector as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_collector_t>(),
    );
    bounded_copy(
        &raw mut (*state).provider_slot.collector.name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"hardware_providers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*state).provider_slot.collector.scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE;
    (*state).provider_slot.collector.tier =
        system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE;
    (*state).provider_slot.collector.interval_seconds = (*options).tier_interval_ms
        [system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0 as ::core::ffi::c_int as usize]
        .wrapping_div(1000 as uint32_t);
    (*state).provider_slot.collector.stale_after_seconds = (*state)
        .provider_slot
        .collector
        .interval_seconds
        .wrapping_mul(3 as uint32_t);
    *&raw mut (*state).provider_slot.busy = r#false != 0;
    if pthread_mutex_init(
        &raw mut (*state).provider_slot.stats_lock,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        let mut index_0: size_t = 0 as size_t;
        while index_0 < (*state).collector_count {
            pthread_mutex_destroy(
                &raw mut (*(&raw mut (*state).collectors as *mut collector_slot_t)
                    .offset(index_0 as isize))
                .stats_lock,
            );
            index_0 = index_0.wrapping_add(1);
        }
        let mut tier_2: size_t = 0 as size_t;
        while tier_2 < SYSTEM_HEALTH_TIER_COUNT as size_t {
            pthread_cond_destroy(
                &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                    .offset(tier_2 as isize))
                .wake,
            );
            pthread_mutex_destroy(
                &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                    .offset(tier_2 as isize))
                .lock,
            );
            tier_2 = tier_2.wrapping_add(1);
        }
        system_health_provider_registry_destroy();
        pthread_mutex_destroy(&raw mut (*state).lock);
        free((*state).ring as *mut ::core::ffi::c_void);
        free(state as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return -1 as ::core::ffi::c_int;
    }
    collector_stats_sync_contract(&raw mut (*state).provider_slot);
    runtime = state;
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_register_collector(
    mut collector: *const system_health_collector_t,
) -> bool {
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut overflow: bool = !runtime.is_null()
        && !(*runtime).started
        && !collector.is_null()
        && (*collector).name[0usize] as ::core::ffi::c_int != 0
        && (*collector).collect.is_some()
        && !memchr(
            &raw const (*collector).name as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        )
        .is_null()
        && (*collector).tier.0 >= system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0
        && (*collector).tier.0 <= system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0
        && (*runtime).collector_count >= SYSTEM_HEALTH_MAX_COLLECTORS as size_t;
    if overflow {
        let mut index: size_t = 0 as size_t;
        while index < (*runtime).collector_count {
            if strcmp(
                &raw mut (*(&raw mut (*runtime).collectors as *mut collector_slot_t)
                    .offset(index as isize))
                .collector
                .name as *mut ::core::ffi::c_char,
                &raw const (*collector).name as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                overflow = r#false != 0;
                break;
            } else {
                index = index.wrapping_add(1);
            }
        }
    }
    let mut result: bool = !runtime.is_null()
        && !(*runtime).started
        && register_collector_locked(runtime, collector) as ::core::ffi::c_int != 0;
    if !result && overflow as ::core::ffi::c_int != 0 {
        pthread_mutex_lock(&raw mut (*runtime).lock);
        (*runtime).stats.coverage_overflows = (*runtime).stats.coverage_overflows.wrapping_add(1);
        pthread_mutex_unlock(&raw mut (*runtime).lock);
    }
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_register_provider(
    mut provider: *const system_health_provider_t,
) -> bool {
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut full: bool =
        system_health_provider_registry_count() >= SYSTEM_HEALTH_MAX_DEVICES as size_t;
    let mut result: bool = !runtime.is_null()
        && !(*runtime).started
        && system_health_provider_registry_register(provider) as ::core::ffi::c_int != 0;
    if !result
        && full as ::core::ffi::c_int != 0
        && !runtime.is_null()
        && !(*runtime).started
        && !provider.is_null()
    {
        pthread_mutex_lock(&raw mut (*runtime).lock);
        (*runtime).stats.coverage_overflows = (*runtime).stats.coverage_overflows.wrapping_add(1);
        pthread_mutex_unlock(&raw mut (*runtime).lock);
    }
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    return result;
}
unsafe extern "C" fn mark_stale(mut observation: *mut system_health_observation_t) {
    (*observation).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE;
    (*observation).freshness = system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_STALE;
    (*observation).value_valid = r#false != 0;
    (*observation).value = 0.0f64;
}
unsafe extern "C" fn compose_snapshot_locked(
    mut state: *mut system_health_runtime_t,
    mut now: uint64_t,
    mut wall: int64_t,
) {
    let mut destination: ::core::ffi::c_uint = if (*state).active_latest == 0 as ::core::ffi::c_uint
    {
        1 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    };
    let mut snapshot: *mut system_health_snapshot_t =
        (&raw mut (*state).latest as *mut system_health_snapshot_t).offset(destination as isize);
    memset(
        snapshot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_snapshot_t>(),
    );
    let mut tier: size_t = 0 as size_t;
    while tier < SYSTEM_HEALTH_TIER_COUNT as size_t {
        if (*state).tier_valid[tier] {
            let mut cache: *mut system_health_snapshot_t = (&raw mut (*state).tier_cache
                as *mut system_health_snapshot_t)
                .offset(tier as isize);
            let mut stale_ms: uint64_t =
                ((*state).options.tier_interval_ms[tier] as uint64_t).wrapping_mul(3 as uint64_t);
            let mut index: size_t = 0 as size_t;
            while index < (*cache).observation_count {
                let mut observation: system_health_observation_t = (*cache).observations[index];
                if observation.sampled_monotonic_ms > now
                    || now.wrapping_sub(observation.sampled_monotonic_ms) > stale_ms
                {
                    mark_stale(&raw mut observation);
                }
                system_health_snapshot_append(snapshot, &raw mut observation);
                index = index.wrapping_add(1);
            }
            (*snapshot).observations_dropped = (*snapshot)
                .observations_dropped
                .wrapping_add((*cache).observations_dropped);
        }
        tier = tier.wrapping_add(1);
    }
    (*state).stats.generations_completed = (*state).stats.generations_completed.wrapping_add(1);
    (*snapshot).sequence = (*state).stats.generations_completed;
    (*snapshot).completed_monotonic_ms = now;
    (*snapshot).completed_wall_time_ms = wall;
    (*state).active_latest = destination;
    let mut summary: *mut system_health_summary_t =
        (*state).ring.offset((*state).ring_head as isize);
    (*summary).sequence = (*snapshot).sequence;
    (*summary).completed_monotonic_ms = now;
    (*summary).completed_wall_time_ms = wall;
    (*summary).observation_count = (*snapshot).observation_count as uint32_t;
    (*summary).observations_dropped = (*snapshot).observations_dropped as uint32_t;
    (*state).ring_head = (*state)
        .ring_head
        .wrapping_add(1 as size_t)
        .wrapping_rem(SYSTEM_HEALTH_RING_SAMPLES as size_t);
    if (*state).ring_count < SYSTEM_HEALTH_RING_SAMPLES as size_t {
        (*state).ring_count = (*state).ring_count.wrapping_add(1);
    }
}
unsafe extern "C" fn collect_one(
    mut slot: *mut collector_slot_t,
    mut context: *const system_health_collect_context_t,
    mut deadline_ms: uint32_t,
    mut collected: *mut system_health_snapshot_t,
    mut errors: *mut uint64_t,
    mut timeouts: *mut uint64_t,
    mut overlap_skips: *mut uint64_t,
) -> bool {
    let mut expected: bool = r#false != 0;
    let c2rust_result = match ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*slot).busy).compare_exchange(*&raw mut expected, true, ::core::sync::atomic::Ordering::SeqCst, ::core::sync::atomic::Ordering::SeqCst) { Ok(old) => (old, true), Err(old) => (old, false) };
    *&raw mut expected = c2rust_result.0;
    if !c2rust_result.1 {
        *overlap_skips = (*overlap_skips).wrapping_add(1);
        pthread_mutex_lock(&raw mut (*slot).stats_lock);
        (*slot).stats.overlap_skips = (*slot).stats.overlap_skips.wrapping_add(1);
        pthread_mutex_unlock(&raw mut (*slot).stats_lock);
        return r#false != 0;
    }
    let mut first: size_t = (*collected).observation_count;
    let mut sink: system_health_observation_sink_t = system_health_observation_sink_t {
        items: &raw mut (*collected).observations as *mut system_health_observation_t,
        capacity: SYSTEM_HEALTH_MAX_OBSERVATIONS as size_t,
        count: (*collected).observation_count,
        dropped: 0 as size_t,
    };
    let mut started: uint64_t = monotonic_ms();
    pthread_mutex_lock(&raw mut (*slot).stats_lock);
    (*slot).stats.attempts = (*slot).stats.attempts.wrapping_add(1);
    (*slot).stats.last_attempt_monotonic_ms = started;
    pthread_mutex_unlock(&raw mut (*slot).stats_lock);
    let mut result: ::core::ffi::c_int = (*slot)
        .collector
        .collect
        .expect("non-null function pointer")(
        (*slot).collector.state, context, &raw mut sink
    );
    let mut finished: uint64_t = monotonic_ms();
    let mut duration: uint64_t = if finished >= started {
        finished.wrapping_sub(started)
    } else {
        0 as uint64_t
    };
    (*collected).observation_count = sink.count;
    (*collected).observations_dropped =
        (*collected).observations_dropped.wrapping_add(sink.dropped);
    if result != 0 as ::core::ffi::c_int {
        *errors = (*errors).wrapping_add(1);
    }
    let mut timed_out: bool = duration > deadline_ms as uint64_t;
    if timed_out {
        *timeouts = (*timeouts).wrapping_add(1);
        let mut index: size_t = first;
        while index < (*collected).observation_count {
            mark_stale(
                (&raw mut (*collected).observations as *mut system_health_observation_t)
                    .offset(index as isize),
            );
            index = index.wrapping_add(1);
        }
    }
    pthread_mutex_lock(&raw mut (*slot).stats_lock);
    (*slot).stats.completions = (*slot).stats.completions.wrapping_add(1);
    (*slot).stats.last_duration_ms = duration;
    if duration > (*slot).stats.maximum_duration_ms {
        (*slot).stats.maximum_duration_ms = duration;
    }
    if result != 0 as ::core::ffi::c_int {
        (*slot).stats.failures = (*slot).stats.failures.wrapping_add(1);
    }
    if timed_out {
        (*slot).stats.timeouts = (*slot).stats.timeouts.wrapping_add(1);
    }
    if result == 0 as ::core::ffi::c_int && !timed_out {
        (*slot).stats.last_success_monotonic_ms = finished;
    }
    pthread_mutex_unlock(&raw mut (*slot).stats_lock);
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*slot).busy).store(false, ::core::sync::atomic::Ordering::SeqCst);
    return r#true != 0;
}
unsafe extern "C" fn collect_providers(
    mut state: *mut system_health_runtime_t,
    mut context: *const system_health_collect_context_t,
    mut deadline_ms: uint32_t,
    mut sink: *mut system_health_observation_sink_t,
    mut resources_dropped: *mut size_t,
    mut errors: *mut uint64_t,
    mut timeouts: *mut uint64_t,
    mut overlap_skips: *mut uint64_t,
) -> bool {
    let mut slot: *mut collector_slot_t = &raw mut (*state).provider_slot;
    let mut expected: bool = r#false != 0;
    let c2rust_result = match ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*slot).busy).compare_exchange(*&raw mut expected, true, ::core::sync::atomic::Ordering::SeqCst, ::core::sync::atomic::Ordering::SeqCst) { Ok(old) => (old, true), Err(old) => (old, false) };
    *&raw mut expected = c2rust_result.0;
    if !c2rust_result.1 {
        *overlap_skips = (*overlap_skips).wrapping_add(1);
        pthread_mutex_lock(&raw mut (*slot).stats_lock);
        (*slot).stats.overlap_skips = (*slot).stats.overlap_skips.wrapping_add(1);
        pthread_mutex_unlock(&raw mut (*slot).stats_lock);
        return r#false != 0;
    }
    let mut started: uint64_t = monotonic_ms();
    pthread_mutex_lock(&raw mut (*slot).stats_lock);
    (*slot).stats.attempts = (*slot).stats.attempts.wrapping_add(1);
    (*slot).stats.last_attempt_monotonic_ms = started;
    pthread_mutex_unlock(&raw mut (*slot).stats_lock);
    let mut result: ::core::ffi::c_int =
        system_health_provider_registry_collect(context, sink, resources_dropped);
    let mut finished: uint64_t = monotonic_ms();
    let mut duration: uint64_t = if finished >= started {
        finished.wrapping_sub(started)
    } else {
        0 as uint64_t
    };
    let mut timed_out: bool = duration > deadline_ms as uint64_t;
    if result != 0 as ::core::ffi::c_int {
        *errors = (*errors).wrapping_add(1);
    }
    if timed_out {
        *timeouts = (*timeouts).wrapping_add(1);
    }
    pthread_mutex_lock(&raw mut (*slot).stats_lock);
    (*slot).stats.completions = (*slot).stats.completions.wrapping_add(1);
    (*slot).stats.last_duration_ms = duration;
    if duration > (*slot).stats.maximum_duration_ms {
        (*slot).stats.maximum_duration_ms = duration;
    }
    if result != 0 as ::core::ffi::c_int {
        (*slot).stats.failures = (*slot).stats.failures.wrapping_add(1);
    }
    if timed_out {
        (*slot).stats.timeouts = (*slot).stats.timeouts.wrapping_add(1);
    }
    if result == 0 as ::core::ffi::c_int && !timed_out {
        (*slot).stats.last_success_monotonic_ms = finished;
    }
    pthread_mutex_unlock(&raw mut (*slot).stats_lock);
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*slot).busy).store(false, ::core::sync::atomic::Ordering::SeqCst);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_collect_tier(
    mut tier: system_health_sampling_tier_t,
) -> ::core::ffi::c_int {
    if tier.0 < system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0
        || tier.0 > system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut state: *mut system_health_runtime_t = runtime;
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    if state.is_null() {
        return if disabled_initialized as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    let mut now: uint64_t = monotonic_ms();
    let mut wall: int64_t = wall_time_ms();
    let mut context: system_health_collect_context_t = system_health_collect_context_t {
        monotonic_ms: now,
        wall_time_ms: wall,
        proc_root: &raw mut (*state).options.proc_root as *mut ::core::ffi::c_char,
        sys_root: &raw mut (*state).options.sys_root as *mut ::core::ffi::c_char,
        cgroup_root: &raw mut (*state).options.cgroup_root as *mut ::core::ffi::c_char,
    };
    let mut collected: system_health_snapshot_t = system_health_snapshot_t {
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
        &raw mut collected as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_snapshot_t>(),
    );
    let mut errors: uint64_t = 0 as uint64_t;
    let mut timeouts: uint64_t = 0 as uint64_t;
    let mut skips: uint64_t = 0 as uint64_t;
    let mut completed: uint64_t = 0 as uint64_t;
    let mut index: size_t = 0 as size_t;
    while index < (*state).collector_count {
        let mut slot: *mut collector_slot_t =
            (&raw mut (*state).collectors as *mut collector_slot_t).offset(index as isize);
        if (*slot).collector.tier.0 == tier.0 {
            let mut before: size_t = collected.observation_count;
            let mut ran: bool = collect_one(
                slot,
                &raw mut context,
                (*state).options.collector_deadline_ms[tier.0 as usize],
                &raw mut collected,
                &raw mut errors,
                &raw mut timeouts,
                &raw mut skips,
            );
            if ran {
                completed = completed.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
    if tier.0 == system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0
        && system_health_provider_registry_count() > 0 as size_t
    {
        let mut sink: system_health_observation_sink_t = system_health_observation_sink_t {
            items: &raw mut collected.observations as *mut system_health_observation_t,
            capacity: SYSTEM_HEALTH_MAX_OBSERVATIONS as size_t,
            count: collected.observation_count,
            dropped: 0,
        };
        let mut provider_dropped: size_t = 0 as size_t;
        let mut ran_0: bool = collect_providers(
            state,
            &raw mut context,
            (*state).options.collector_deadline_ms[tier.0 as usize],
            &raw mut sink,
            &raw mut provider_dropped,
            &raw mut errors,
            &raw mut timeouts,
            &raw mut skips,
        );
        collected.observation_count = sink.count;
        collected.observations_dropped = collected
            .observations_dropped
            .wrapping_add(sink.dropped.wrapping_add(provider_dropped));
        if ran_0 {
            completed = completed.wrapping_add(1);
        }
    }
    pthread_mutex_lock(&raw mut (*state).lock);
    collected.completed_monotonic_ms = monotonic_ms();
    collected.completed_wall_time_ms = wall_time_ms();
    (*state).tier_cache[tier.0 as usize] = collected;
    (*state).tier_valid[tier.0 as usize] = r#true != 0;
    (*state).stats.collections_completed =
        (*state).stats.collections_completed.wrapping_add(completed);
    (*state).stats.collection_errors = (*state).stats.collection_errors.wrapping_add(errors);
    (*state).stats.collection_timeouts = (*state).stats.collection_timeouts.wrapping_add(timeouts);
    (*state).stats.overlap_skips = (*state).stats.overlap_skips.wrapping_add(skips);
    (*state).stats.observations_dropped = ((*state).stats.observations_dropped
        as ::core::ffi::c_ulong)
        .wrapping_add(collected.observations_dropped as ::core::ffi::c_ulong)
        as uint64_t;
    (*state).stats.coverage_overflows = ((*state).stats.coverage_overflows as ::core::ffi::c_ulong)
        .wrapping_add(collected.observations_dropped as ::core::ffi::c_ulong)
        as uint64_t;
    compose_snapshot_locked(
        state,
        collected.completed_monotonic_ms,
        collected.completed_wall_time_ms,
    );
    pthread_mutex_unlock(&raw mut (*state).lock);
    if tier.0 == system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0
        && recording_io_take_device_refresh_request() as ::core::ffi::c_int != 0
    {
        system_health_builtin_provider_request_device_refresh();
    }
    return if errors == 0 as uint64_t {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn add_ms_to_timespec(mut value: *mut timespec, mut milliseconds: uint32_t) {
    (*value).tv_sec += milliseconds.wrapping_div(1000 as uint32_t) as __time_t;
    (*value).tv_nsec += milliseconds.wrapping_rem(1000 as uint32_t) as ::core::ffi::c_long
        * 1000000 as ::core::ffi::c_long;
    if (*value).tv_nsec >= 1000000000 as ::core::ffi::c_long {
        (*value).tv_sec += 1;
        (*value).tv_nsec -= 1000000000 as ::core::ffi::c_long;
    }
}
unsafe extern "C" fn tier_worker_main(
    mut argument: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut worker: *mut tier_worker_t = argument as *mut tier_worker_t;
    let mut state: *mut system_health_runtime_t = (*worker).runtime;
    pthread_mutex_lock(&raw mut (*worker).lock);
    while ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).running).load(::core::sync::atomic::Ordering::SeqCst) as ::core::ffi::c_int
        != 0
        && !(*worker).startup_ready
    {
        pthread_cond_wait(&raw mut (*worker).wake, &raw mut (*worker).lock);
    }
    pthread_mutex_unlock(&raw mut (*worker).lock);
    while ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).running).load(::core::sync::atomic::Ordering::SeqCst) {
        pthread_mutex_lock(&raw mut (*worker).lock);
        (*worker).requested = r#false != 0;
        pthread_mutex_unlock(&raw mut (*worker).lock);
        system_health_collect_tier((*worker).tier);
        let mut deadline: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_MONOTONIC, &raw mut deadline);
        add_ms_to_timespec(
            &raw mut deadline,
            (*state).options.tier_interval_ms[(*worker).tier.0 as usize],
        );
        pthread_mutex_lock(&raw mut (*worker).lock);
        while ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).running).load(::core::sync::atomic::Ordering::SeqCst) as ::core::ffi::c_int
            != 0
            && !(*worker).requested
        {
            let mut result: ::core::ffi::c_int = pthread_cond_timedwait(
                &raw mut (*worker).wake,
                &raw mut (*worker).lock,
                &raw mut deadline,
            );
            if result == ETIMEDOUT {
                break;
            }
        }
        pthread_mutex_unlock(&raw mut (*worker).lock);
    }
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_start() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut state: *mut system_health_runtime_t = runtime;
    if state.is_null() {
        let mut result: ::core::ffi::c_int = if disabled_initialized as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return result;
    }
    if (*state).started {
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return 0 as ::core::ffi::c_int;
    }
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).running).store(true, ::core::sync::atomic::Ordering::SeqCst);
    let mut started: size_t = 0 as size_t;
    let mut tier: size_t = 0 as size_t;
    while tier < SYSTEM_HEALTH_TIER_COUNT as size_t {
        let mut worker: *mut tier_worker_t =
            (&raw mut (*state).workers as *mut tier_worker_t).offset(tier as isize);
        pthread_mutex_lock(&raw mut (*worker).lock);
        (*worker).startup_ready = r#false != 0;
        pthread_mutex_unlock(&raw mut (*worker).lock);
        if pthread_create(
            &raw mut (*worker).thread,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                tier_worker_main
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            worker as *mut ::core::ffi::c_void,
        ) != 0 as ::core::ffi::c_int
        {
            break;
        }
        (*worker).started = r#true != 0;
        started = started.wrapping_add(1);
        tier = tier.wrapping_add(1);
    }
    if started != SYSTEM_HEALTH_TIER_COUNT as size_t {
        ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).running).store(false, ::core::sync::atomic::Ordering::SeqCst);
        let mut tier_0: size_t = 0 as size_t;
        while tier_0 < started {
            pthread_mutex_lock(
                &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                    .offset(tier_0 as isize))
                .lock,
            );
            pthread_cond_signal(
                &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                    .offset(tier_0 as isize))
                .wake,
            );
            pthread_mutex_unlock(
                &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t)
                    .offset(tier_0 as isize))
                .lock,
            );
            pthread_join(
                (*state).workers[tier_0].thread,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            );
            (*state).workers[tier_0].started = r#false != 0;
            tier_0 = tier_0.wrapping_add(1);
        }
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return -1 as ::core::ffi::c_int;
    }
    (*state).started = r#true != 0;
    pthread_mutex_lock(&raw mut (*state).lock);
    (*state).stats.worker_threads = SYSTEM_HEALTH_TIER_COUNT as uint32_t;
    pthread_mutex_unlock(&raw mut (*state).lock);
    let mut tier_1: size_t = 0 as size_t;
    while tier_1 < SYSTEM_HEALTH_TIER_COUNT as size_t {
        let mut worker_0: *mut tier_worker_t =
            (&raw mut (*state).workers as *mut tier_worker_t).offset(tier_1 as isize);
        pthread_mutex_lock(&raw mut (*worker_0).lock);
        (*worker_0).startup_ready = r#true != 0;
        pthread_cond_signal(&raw mut (*worker_0).wake);
        pthread_mutex_unlock(&raw mut (*worker_0).lock);
        tier_1 = tier_1.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_request_tier(
    mut tier: system_health_sampling_tier_t,
) -> bool {
    if tier.0 < system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST.0
        || tier.0 > system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE.0
    {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut state: *mut system_health_runtime_t = runtime;
    if state.is_null() || !(*state).started {
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return r#false != 0;
    }
    let mut worker: *mut tier_worker_t =
        (&raw mut (*state).workers as *mut tier_worker_t).offset(tier.0 as isize);
    pthread_mutex_lock(&raw mut (*worker).lock);
    (*worker).requested = r#true != 0;
    pthread_cond_signal(&raw mut (*worker).wake);
    pthread_mutex_unlock(&raw mut (*worker).lock);
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_snapshot_copy(
    mut snapshot: *mut system_health_snapshot_t,
) -> bool {
    if snapshot.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut state: *mut system_health_runtime_t = runtime;
    if state.is_null() {
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut (*state).lock);
    *snapshot = (*state).latest[(*state).active_latest as usize];
    let mut available: bool = (*snapshot).sequence != 0 as uint64_t;
    pthread_mutex_unlock(&raw mut (*state).lock);
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    return available;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_summary_copy(
    mut summaries: *mut system_health_summary_t,
    mut capacity: size_t,
) -> size_t {
    if summaries.is_null() || capacity == 0 as size_t {
        return 0 as size_t;
    }
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut state: *mut system_health_runtime_t = runtime;
    if state.is_null() {
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return 0 as size_t;
    }
    pthread_mutex_lock(&raw mut (*state).lock);
    let mut count: size_t = if (*state).ring_count < capacity {
        (*state).ring_count
    } else {
        capacity
    };
    let mut oldest: size_t = (*state)
        .ring_head
        .wrapping_add(SYSTEM_HEALTH_RING_SAMPLES as size_t)
        .wrapping_sub((*state).ring_count)
        .wrapping_rem(SYSTEM_HEALTH_RING_SAMPLES as size_t);
    let mut index: size_t = 0 as size_t;
    while index < count {
        *summaries.offset(index as isize) = *(*state).ring.offset(
            oldest
                .wrapping_add(index)
                .wrapping_rem(SYSTEM_HEALTH_RING_SAMPLES as size_t) as isize,
        );
        index = index.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut (*state).lock);
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_get_stats(mut stats: *mut system_health_stats_t) {
    if stats.is_null() {
        return;
    }
    memset(
        stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_stats_t>(),
    );
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut state: *mut system_health_runtime_t = runtime;
    if !state.is_null() {
        pthread_mutex_lock(&raw mut (*state).lock);
        *stats = (*state).stats;
        pthread_mutex_unlock(&raw mut (*state).lock);
    } else if disabled_initialized {
        (*stats).initialized = r#true != 0;
    }
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    (*stats).abandoned_helpers = health_helper_abandoned_count();
}
unsafe extern "C" fn copy_collector_stats(
    mut slot: *mut collector_slot_t,
    mut now: uint64_t,
    mut output: *mut system_health_collector_stats_t,
) {
    pthread_mutex_lock(&raw mut (*slot).stats_lock);
    *output = (*slot).stats;
    pthread_mutex_unlock(&raw mut (*slot).stats_lock);
    bounded_copy(
        &raw mut (*output).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        &raw mut (*slot).collector.name as *mut ::core::ffi::c_char,
    );
    (*output).scope = (*slot).collector.scope;
    (*output).tier = (*slot).collector.tier;
    (*output).interval_seconds = (*slot).collector.interval_seconds;
    (*output).stale_after_seconds = (*slot).collector.stale_after_seconds;
    (*output).busy = ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*slot).busy).load(::core::sync::atomic::Ordering::SeqCst);
    let mut stale_ms: uint64_t =
        ((*output).stale_after_seconds as uint64_t).wrapping_mul(1000 as uint64_t);
    (*output).stale = (*output).attempts > 0 as uint64_t
        && ((*output).last_success_monotonic_ms == 0 as uint64_t
            || (*output).last_success_monotonic_ms > now
            || stale_ms > 0 as uint64_t
                && now.wrapping_sub((*output).last_success_monotonic_ms) > stale_ms);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_collector_stats_copy(
    mut stats: *mut system_health_collector_stats_t,
    mut capacity: size_t,
) -> size_t {
    if stats.is_null() || capacity == 0 as size_t {
        return 0 as size_t;
    }
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut state: *mut system_health_runtime_t = runtime;
    if state.is_null() {
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return 0 as size_t;
    }
    let mut now: uint64_t = monotonic_ms();
    let mut available: size_t = (*state).collector_count.wrapping_add(
        (if system_health_provider_registry_count() > 0 as size_t {
            1 as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as size_t,
    );
    let mut count: size_t = if available < capacity {
        available
    } else {
        capacity
    };
    let mut copied: size_t = 0 as size_t;
    while copied < (*state).collector_count && copied < count {
        copy_collector_stats(
            (&raw mut (*state).collectors as *mut collector_slot_t).offset(copied as isize),
            now,
            stats.offset(copied as isize),
        );
        copied = copied.wrapping_add(1);
    }
    if copied < count {
        copy_collector_stats(
            &raw mut (*state).provider_slot,
            now,
            stats.offset(copied as isize),
        );
        copied = copied.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    return copied;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_shutdown() {
    pthread_mutex_lock(&raw mut lifecycle_lock);
    let mut state: *mut system_health_runtime_t = runtime;
    if state.is_null() {
        disabled_initialized = r#false != 0;
        pthread_mutex_unlock(&raw mut lifecycle_lock);
        return;
    }
    runtime = ::core::ptr::null_mut::<system_health_runtime_t>();
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).running).store(false, ::core::sync::atomic::Ordering::SeqCst);
    let mut tier: size_t = 0 as size_t;
    while tier < SYSTEM_HEALTH_TIER_COUNT as size_t {
        let mut worker: *mut tier_worker_t =
            (&raw mut (*state).workers as *mut tier_worker_t).offset(tier as isize);
        pthread_mutex_lock(&raw mut (*worker).lock);
        (*worker).requested = r#true != 0;
        pthread_cond_signal(&raw mut (*worker).wake);
        pthread_mutex_unlock(&raw mut (*worker).lock);
        tier = tier.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut lifecycle_lock);
    let mut tier_0: size_t = 0 as size_t;
    while tier_0 < SYSTEM_HEALTH_TIER_COUNT as size_t {
        let mut worker_0: *mut tier_worker_t =
            (&raw mut (*state).workers as *mut tier_worker_t).offset(tier_0 as isize);
        if (*worker_0).started {
            pthread_join(
                (*worker_0).thread,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            );
        }
        tier_0 = tier_0.wrapping_add(1);
    }
    let mut index: size_t = 0 as size_t;
    while index < (*state).collector_count {
        if (*state).collectors[index].collector.destroy.is_some() {
            (*state).collectors[index]
                .collector
                .destroy
                .expect("non-null function pointer")(
                (*state).collectors[index].collector.state
            );
        }
        pthread_mutex_destroy(
            &raw mut (*(&raw mut (*state).collectors as *mut collector_slot_t)
                .offset(index as isize))
            .stats_lock,
        );
        index = index.wrapping_add(1);
    }
    system_health_provider_registry_destroy();
    pthread_mutex_destroy(&raw mut (*state).provider_slot.stats_lock);
    let mut tier_1: size_t = 0 as size_t;
    while tier_1 < SYSTEM_HEALTH_TIER_COUNT as size_t {
        pthread_cond_destroy(
            &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t).offset(tier_1 as isize))
                .wake,
        );
        pthread_mutex_destroy(
            &raw mut (*(&raw mut (*state).workers as *mut tier_worker_t).offset(tier_1 as isize))
                .lock,
        );
        tier_1 = tier_1.wrapping_add(1);
    }
    pthread_mutex_destroy(&raw mut (*state).lock);
    free((*state).ring as *mut ::core::ffi::c_void);
    free(state as *mut ::core::ffi::c_void);
    health_helper_reap_abandoned();
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
