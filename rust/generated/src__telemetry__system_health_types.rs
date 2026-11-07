unsafe extern "C" {
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type int64_t = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_sink_t {
    pub items: *mut system_health_observation_t,
    pub capacity: size_t,
    pub count: size_t,
    pub dropped: size_t,
}
pub const SYSTEM_HEALTH_MAX_OBSERVATIONS: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
static mut condition_codes: [*const ::core::ffi::c_char; 27] = [
    b"memory.available_low\0".as_ptr() as *const ::core::ffi::c_char,
    b"memory.oom_kill\0".as_ptr() as *const ::core::ffi::c_char,
    b"memory.swap_thrash\0".as_ptr() as *const ::core::ffi::c_char,
    b"cpu.saturation\0".as_ptr() as *const ::core::ffi::c_char,
    b"cpu.throttled\0".as_ptr() as *const ::core::ffi::c_char,
    b"io.pressure\0".as_ptr() as *const ::core::ffi::c_char,
    b"filesystem.bytes_low\0".as_ptr() as *const ::core::ffi::c_char,
    b"filesystem.inodes_low\0".as_ptr() as *const ::core::ffi::c_char,
    b"filesystem.read_only\0".as_ptr() as *const ::core::ffi::c_char,
    b"filesystem.write_failed\0".as_ptr() as *const ::core::ffi::c_char,
    b"thermal.high\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.link_down\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.error_rate\0".as_ptr() as *const ::core::ffi::c_char,
    b"clock.unsynchronized\0".as_ptr() as *const ::core::ffi::c_char,
    b"clock.jump\0".as_ptr() as *const ::core::ffi::c_char,
    b"process.fd_exhaustion\0".as_ptr() as *const ::core::ffi::c_char,
    b"process.pid_exhaustion\0".as_ptr() as *const ::core::ffi::c_char,
    b"process.allocation_failed\0".as_ptr() as *const ::core::ffi::c_char,
    b"storage.device_prefail\0".as_ptr() as *const ::core::ffi::c_char,
    b"storage.device_critical\0".as_ptr() as *const ::core::ffi::c_char,
    b"hardware.ecc_corrected\0".as_ptr() as *const ::core::ffi::c_char,
    b"hardware.ecc_uncorrectable\0".as_ptr() as *const ::core::ffi::c_char,
    b"hardware.fan_failed\0".as_ptr() as *const ::core::ffi::c_char,
    b"hardware.power_unstable\0".as_ptr() as *const ::core::ffi::c_char,
    b"health.collector_stale\0".as_ptr() as *const ::core::ffi::c_char,
    b"system.unexpected_restart\0".as_ptr() as *const ::core::ffi::c_char,
    b"event.delivery_degraded\0".as_ptr() as *const ::core::ffi::c_char,
];
unsafe extern "C" fn bounded_name(
    mut value: ::core::ffi::c_int,
    mut names: *const *const ::core::ffi::c_char,
    mut name_count: size_t,
) -> *const ::core::ffi::c_char {
    if value < 0 as ::core::ffi::c_int || value as size_t >= name_count {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return *names.offset(value as isize);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_scope_name(
    mut scope: system_health_scope_t,
) -> *const ::core::ffi::c_char {
    static mut names: [*const ::core::ffi::c_char; 5] = [
        b"process\0".as_ptr() as *const ::core::ffi::c_char,
        b"container\0".as_ptr() as *const ::core::ffi::c_char,
        b"host\0".as_ptr() as *const ::core::ffi::c_char,
        b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        b"device\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    return bounded_name(
        scope.0 as ::core::ffi::c_int,
        &raw const names as *const *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_capability_name(
    mut capability: system_health_capability_t,
) -> *const ::core::ffi::c_char {
    static mut names: [*const ::core::ffi::c_char; 5] = [
        b"available\0".as_ptr() as *const ::core::ffi::c_char,
        b"unsupported\0".as_ptr() as *const ::core::ffi::c_char,
        b"permission_denied\0".as_ptr() as *const ::core::ffi::c_char,
        b"stale\0".as_ptr() as *const ::core::ffi::c_char,
        b"error\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    return bounded_name(
        capability.0 as ::core::ffi::c_int,
        &raw const names as *const *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_unit_name(
    mut unit: system_health_unit_t,
) -> *const ::core::ffi::c_char {
    static mut names: [*const ::core::ffi::c_char; 8] = [
        b"none\0".as_ptr() as *const ::core::ffi::c_char,
        b"ratio\0".as_ptr() as *const ::core::ffi::c_char,
        b"bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        b"seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"celsius\0".as_ptr() as *const ::core::ffi::c_char,
        b"hertz\0".as_ptr() as *const ::core::ffi::c_char,
        b"boolean\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    return bounded_name(
        unit.0 as ::core::ffi::c_int,
        &raw const names as *const *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 8]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_severity_name(
    mut severity: system_health_severity_t,
) -> *const ::core::ffi::c_char {
    static mut names: [*const ::core::ffi::c_char; 4] = [
        b"none\0".as_ptr() as *const ::core::ffi::c_char,
        b"warning\0".as_ptr() as *const ::core::ffi::c_char,
        b"error\0".as_ptr() as *const ::core::ffi::c_char,
        b"critical\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    return bounded_name(
        severity.0 as ::core::ffi::c_int,
        &raw const names as *const *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_condition_code(
    mut condition: system_health_condition_t,
) -> *const ::core::ffi::c_char {
    if condition.0 < 0 as ::core::ffi::c_uint
        || condition.0 >= system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return condition_codes[condition.0 as usize];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_condition_from_code(
    mut code: *const ::core::ffi::c_char,
    mut condition_out: *mut system_health_condition_t,
) -> bool {
    if code.is_null() || condition_out.is_null() {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int {
        if strcmp(code, condition_codes[i as usize]) == 0 as ::core::ffi::c_int {
            *condition_out = system_health_condition_t(i as ::core::ffi::c_uint);
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_observation_set_available(
    mut observation: *mut system_health_observation_t,
    mut value: ::core::ffi::c_double,
    mut unit: system_health_unit_t,
) {
    if observation.is_null() {
        return;
    }
    (*observation).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    (*observation).freshness = system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_FRESH;
    (*observation).unit = unit;
    (*observation).value_valid = r#true != 0;
    (*observation).value = value;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_observation_set_unavailable(
    mut observation: *mut system_health_observation_t,
    mut capability: system_health_capability_t,
) {
    if observation.is_null() {
        return;
    }
    if capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    (*observation).capability = capability;
    (*observation).freshness = system_health_freshness_t(
        (if capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0 {
            system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_STALE.0 as ::core::ffi::c_int
        } else {
            system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*observation).unit = system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE;
    (*observation).value_valid = r#false != 0;
    (*observation).value = 0.0f64;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_snapshot_append(
    mut snapshot: *mut system_health_snapshot_t,
    mut observation: *const system_health_observation_t,
) -> bool {
    if snapshot.is_null() || observation.is_null() {
        return r#false != 0;
    }
    if (*snapshot).observation_count >= SYSTEM_HEALTH_MAX_OBSERVATIONS as size_t {
        (*snapshot).observations_dropped = (*snapshot).observations_dropped.wrapping_add(1);
        return r#false != 0;
    }
    let c2rust_fresh0 = (*snapshot).observation_count;
    (*snapshot).observation_count = (*snapshot).observation_count.wrapping_add(1);
    (*snapshot).observations[c2rust_fresh0] = *observation;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_observation_sink_append(
    mut sink: *mut system_health_observation_sink_t,
    mut observation: *const system_health_observation_t,
) -> bool {
    if sink.is_null()
        || observation.is_null()
        || (*sink).items.is_null()
        || (*sink).count >= (*sink).capacity
    {
        if !sink.is_null() {
            (*sink).dropped = (*sink).dropped.wrapping_add(1);
        }
        return r#false != 0;
    }
    let c2rust_fresh1 = (*sink).count;
    (*sink).count = (*sink).count.wrapping_add(1);
    *(*sink).items.offset(c2rust_fresh1 as isize) = *observation;
    return r#true != 0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
