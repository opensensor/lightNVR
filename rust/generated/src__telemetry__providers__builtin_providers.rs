unsafe extern "C" {
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn kernel_log_state_init(
        state: *mut kernel_log_state_t,
        path: *const ::core::ffi::c_char,
    );
    unsafe fn kernel_log_provider_init(
        provider: *mut system_health_provider_t,
        state: *mut kernel_log_state_t,
    );
    unsafe fn linux_hardware_state_init(
        state: *mut linux_hardware_state_t,
        installation_scope: *const ::core::ffi::c_char,
    );
    unsafe fn linux_hardware_provider_init(
        provider: *mut system_health_provider_t,
        state: *mut linux_hardware_state_t,
    );
    unsafe fn nvme_provider_state_init(
        state: *mut nvme_provider_state_t,
        installation_scope: *const ::core::ffi::c_char,
    );
    unsafe fn nvme_provider_init(
        provider: *mut system_health_provider_t,
        state: *mut nvme_provider_state_t,
    );
    unsafe fn smartctl_provider_state_init(
        state: *mut smartctl_provider_state_t,
        installation_scope: *const ::core::ffi::c_char,
    );
    unsafe fn smartctl_provider_request_refresh(state: *mut smartctl_provider_state_t) -> bool;
    unsafe fn smartctl_provider_init(
        provider: *mut system_health_provider_t,
        state: *mut smartctl_provider_state_t,
    );
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type ssize_t = isize;
pub type int64_t = i64;
pub type uint8_t = u8;
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
pub type system_health_provider_registration_fn =
    Option<unsafe extern "C" fn(*const system_health_provider_t, *mut ::core::ffi::c_void) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_log_state_t {
    pub descriptor: ::core::ffi::c_int,
    pub path: [::core::ffi::c_char; 256],
    pub ops: kernel_log_ops_t,
    pub capability: system_health_capability_t,
    pub last_sequence: uint64_t,
    pub sequence_valid: bool,
    pub recent_hashes: [uint64_t; 64],
    pub recent_count: size_t,
    pub recent_next: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_log_ops_t {
    pub open_log: kernel_log_open_fn,
    pub read_log: kernel_log_read_fn,
    pub close_log: kernel_log_close_fn,
}
pub type kernel_log_close_fn =
    Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>;
pub type kernel_log_read_fn =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void, size_t) -> ssize_t>;
pub type kernel_log_open_fn = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub type atomic_bool = bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nvme_provider_state_t {
    pub devices: [nvme_provider_device_t; 16],
    pub device_count: size_t,
    pub devices_dropped: size_t,
    pub discovery_capability: system_health_capability_t,
    pub installation_scope: [::core::ffi::c_char; 96],
    pub dev_root: [::core::ffi::c_char; 256],
    pub discovered_sys_root: [::core::ffi::c_char; 1024],
    pub ops: nvme_provider_ops_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nvme_provider_ops_t {
    pub open_device: nvme_provider_open_fn,
    pub read_health_log: nvme_provider_health_log_fn,
    pub close_device: nvme_provider_close_fn,
}
pub type nvme_provider_close_fn =
    Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>;
pub type nvme_provider_health_log_fn =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *mut uint8_t, size_t) -> ::core::ffi::c_int>;
pub type nvme_provider_open_fn = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nvme_provider_device_t {
    pub internal_name: [::core::ffi::c_char; 64],
    pub public_id: [::core::ffi::c_char; 64],
    pub previous_media_errors: uint64_t,
    pub previous_unsafe_shutdowns: uint64_t,
    pub previous_monotonic_ms: uint64_t,
    pub media_previous_valid: bool,
    pub unsafe_previous_valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smartctl_provider_state_t {
    pub devices: [smartctl_provider_device_t; 16],
    pub device_count: size_t,
    pub devices_dropped: size_t,
    pub discover_target_devices: bool,
    pub installation_scope: [::core::ffi::c_char; 96],
    pub program: [::core::ffi::c_char; 256],
    pub timeout_ms: uint32_t,
    pub terminate_grace_ms: uint32_t,
    pub run_helper: smartctl_helper_run_fn,
    pub wake_device_tier: smartctl_wake_device_tier_fn,
    pub capability: system_health_capability_t,
    pub refresh_pending: atomic_bool,
    pub refresh_requests: atomic_uint_fast64_t,
    pub refresh_coalesced: atomic_uint_fast64_t,
    pub refresh_collections: atomic_uint_fast64_t,
}
pub type atomic_uint_fast64_t = uint_fast64_t;
pub type smartctl_wake_device_tier_fn = Option<unsafe extern "C" fn() -> bool>;
pub type smartctl_helper_run_fn = Option<
    unsafe extern "C" fn(
        *const health_helper_request_t,
        *mut health_helper_result_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_helper_result_t {
    pub outcome: health_helper_outcome_t,
    pub exit_code: ::core::ffi::c_int,
    pub term_signal: ::core::ffi::c_int,
    pub latency_ms: uint32_t,
    pub output_truncated: bool,
    pub abandoned: bool,
    pub output_length: size_t,
    pub output: [::core::ffi::c_char; 4097],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct health_helper_outcome_t(pub ::core::ffi::c_uint);
impl health_helper_outcome_t {
    pub const HEALTH_HELPER_OK: Self = Self(0);
    pub const HEALTH_HELPER_EXITED: Self = Self(1);
    pub const HEALTH_HELPER_TIMED_OUT: Self = Self(2);
    pub const HEALTH_HELPER_BUSY: Self = Self(3);
    pub const HEALTH_HELPER_EXEC_ERROR: Self = Self(4);
    pub const HEALTH_HELPER_SYSTEM_ERROR: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_helper_request_t {
    pub program: *const ::core::ffi::c_char,
    pub argv: *const *mut ::core::ffi::c_char,
    pub timeout_ms: uint32_t,
    pub terminate_grace_ms: uint32_t,
    pub output_limit: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smartctl_provider_device_t {
    pub device_path: [::core::ffi::c_char; 256],
    pub sysfs_name: [::core::ffi::c_char; 64],
    pub public_id: [::core::ffi::c_char; 64],
    pub previous_uncorrectable: uint64_t,
    pub previous_crc: uint64_t,
    pub previous_media: uint64_t,
    pub previous_unsafe_shutdowns: uint64_t,
    pub previous_monotonic_ms: uint64_t,
    pub previous_uncorrectable_valid: bool,
    pub previous_crc_valid: bool,
    pub previous_media_valid: bool,
    pub previous_unsafe_valid: bool,
    pub target_managed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_hardware_state_t {
    pub resources: [linux_hardware_resource_t; 16],
    pub resource_count: size_t,
    pub resources_dropped: size_t,
    pub discovery_capability: system_health_capability_t,
    pub installation_scope: [::core::ffi::c_char; 96],
    pub discovered_sys_root: [::core::ffi::c_char; 1024],
    pub fan_hot_celsius: ::core::ffi::c_double,
    pub counters: [linux_hardware_counter_state_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_hardware_counter_state_t {
    pub used: bool,
    pub public_id: [::core::ffi::c_char; 64],
    pub metric: [::core::ffi::c_char; 64],
    pub previous: uint64_t,
    pub previous_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_hardware_resource_t {
    pub kind: linux_hardware_resource_kind_t,
    pub internal_name: [::core::ffi::c_char; 64],
    pub public_id: [::core::ffi::c_char; 64],
    pub channel: ::core::ffi::c_uint,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct linux_hardware_resource_kind_t(pub ::core::ffi::c_uint);
impl linux_hardware_resource_kind_t {
    pub const LINUX_HARDWARE_RESOURCE_FLASH: Self = Self(0);
    pub const LINUX_HARDWARE_RESOURCE_EDAC: Self = Self(1);
    pub const LINUX_HARDWARE_RESOURCE_FAN: Self = Self(2);
    pub const LINUX_HARDWARE_RESOURCE_BOARD: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct memory_order(pub ::core::ffi::c_uint);
impl memory_order {
    pub const memory_order_relaxed: Self = Self(0);
    pub const memory_order_consume: Self = Self(1);
    pub const memory_order_acquire: Self = Self(2);
    pub const memory_order_release: Self = Self(3);
    pub const memory_order_acq_rel: Self = Self(4);
    pub const memory_order_seq_cst: Self = Self(5);
}
static mut linux_hardware_state: linux_hardware_state_t = linux_hardware_state_t {
    resources: [linux_hardware_resource_t {
        kind: linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_FLASH,
        internal_name: [0; 64],
        public_id: [0; 64],
        channel: 0,
    }; 16],
    resource_count: 0,
    resources_dropped: 0,
    discovery_capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
    installation_scope: [0; 96],
    discovered_sys_root: [0; 1024],
    fan_hot_celsius: 0.,
    counters: [linux_hardware_counter_state_t {
        used: false,
        public_id: [0; 64],
        metric: [0; 64],
        previous: 0,
        previous_monotonic_ms: 0,
    }; 32],
};
static mut nvme_state: nvme_provider_state_t = nvme_provider_state_t {
    devices: [nvme_provider_device_t {
        internal_name: [0; 64],
        public_id: [0; 64],
        previous_media_errors: 0,
        previous_unsafe_shutdowns: 0,
        previous_monotonic_ms: 0,
        media_previous_valid: false,
        unsafe_previous_valid: false,
    }; 16],
    device_count: 0,
    devices_dropped: 0,
    discovery_capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
    installation_scope: [0; 96],
    dev_root: [0; 256],
    discovered_sys_root: [0; 1024],
    ops: nvme_provider_ops_t {
        open_device: None,
        read_health_log: None,
        close_device: None,
    },
};
static mut kernel_state: kernel_log_state_t = kernel_log_state_t {
    descriptor: 0,
    path: [0; 256],
    ops: kernel_log_ops_t {
        open_log: None,
        read_log: None,
        close_log: None,
    },
    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
    last_sequence: 0,
    sequence_valid: false,
    recent_hashes: [0; 64],
    recent_count: 0,
    recent_next: 0,
};
static mut smartctl_state: smartctl_provider_state_t = smartctl_provider_state_t {
    devices: [smartctl_provider_device_t {
        device_path: [0; 256],
        sysfs_name: [0; 64],
        public_id: [0; 64],
        previous_uncorrectable: 0,
        previous_crc: 0,
        previous_media: 0,
        previous_unsafe_shutdowns: 0,
        previous_monotonic_ms: 0,
        previous_uncorrectable_valid: false,
        previous_crc_valid: false,
        previous_media_valid: false,
        previous_unsafe_valid: false,
        target_managed: false,
    }; 16],
    device_count: 0,
    devices_dropped: 0,
    discover_target_devices: false,
    installation_scope: [0; 96],
    program: [0; 256],
    timeout_ms: 0,
    terminate_grace_ms: 0,
    run_helper: None,
    wake_device_tier: None,
    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
    refresh_pending: false,
    refresh_requests: 0,
    refresh_coalesced: 0,
    refresh_collections: 0,
};
static mut smartctl_active: atomic_bool = false;
unsafe extern "C" fn register_provider(
    mut callback: system_health_provider_registration_fn,
    mut context: *mut ::core::ffi::c_void,
    mut provider: *mut system_health_provider_t,
) -> bool {
    return callback.expect("non-null function pointer")(provider, context);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_register_builtin_providers(
    mut selection: *const ::core::ffi::c_char,
    mut registration: system_health_provider_registration_fn,
    mut context: *mut ::core::ffi::c_void,
) -> size_t {
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut smartctl_active).store(r#false != 0, ::core::sync::atomic::Ordering::Release);
    if registration.is_none()
        || selection.is_null()
        || strcmp(selection, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            && strcmp(
                selection,
                b"smartctl\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
    {
        return 0 as size_t;
    }
    let mut smartctl_selected: bool = strcmp(
        selection,
        b"smartctl\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut installation_scope: [::core::ffi::c_char; 96] = [
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
    ];
    event_identity_get_source(
        &raw mut installation_scope as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
    );
    linux_hardware_state_init(
        &raw mut linux_hardware_state,
        &raw mut installation_scope as *mut ::core::ffi::c_char,
    );
    nvme_provider_state_init(
        &raw mut nvme_state,
        &raw mut installation_scope as *mut ::core::ffi::c_char,
    );
    kernel_log_state_init(
        &raw mut kernel_state,
        b"/dev/kmsg\0".as_ptr() as *const ::core::ffi::c_char,
    );
    smartctl_provider_state_init(
        &raw mut smartctl_state,
        &raw mut installation_scope as *mut ::core::ffi::c_char,
    );
    smartctl_state.discover_target_devices = r#true != 0;
    let mut registered: size_t = 0 as size_t;
    let mut provider: system_health_provider_t = system_health_provider_t {
        name: [0; 32],
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        state: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        discover: None,
        collect: None,
        destroy: None,
    };
    linux_hardware_provider_init(&raw mut provider, &raw mut linux_hardware_state);
    if register_provider(registration, context, &raw mut provider) {
        registered = registered.wrapping_add(1);
    }
    if smartctl_selected {
        smartctl_provider_init(&raw mut provider, &raw mut smartctl_state);
    } else {
        nvme_provider_init(&raw mut provider, &raw mut nvme_state);
    }
    let mut device_registered: bool = register_provider(registration, context, &raw mut provider);
    if device_registered {
        registered = registered.wrapping_add(1);
    }
    if smartctl_selected as ::core::ffi::c_int != 0 && device_registered as ::core::ffi::c_int != 0
    {
        ::core::sync::atomic::AtomicBool::from_ptr(&raw mut smartctl_active).store(r#true != 0, ::core::sync::atomic::Ordering::Release);
    }
    kernel_log_provider_init(&raw mut provider, &raw mut kernel_state);
    if register_provider(registration, context, &raw mut provider) {
        registered = registered.wrapping_add(1);
    }
    return registered;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_builtin_provider_request_device_refresh() -> bool {
    if !::core::sync::atomic::AtomicBool::from_ptr(&raw mut smartctl_active).load(::core::sync::atomic::Ordering::Acquire) {
        return r#false != 0;
    }
    return smartctl_provider_request_refresh(&raw mut smartctl_state);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
