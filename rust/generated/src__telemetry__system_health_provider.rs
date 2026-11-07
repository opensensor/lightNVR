unsafe extern "C" {
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
pub const SYSTEM_HEALTH_MAX_DEVICES: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const PROVIDER_CAPACITY: ::core::ffi::c_uint = SYSTEM_HEALTH_MAX_DEVICES;
static mut providers: [system_health_provider_t; 16] = [system_health_provider_t {
    name: [0; 32],
    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
    state: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    discover: None,
    collect: None,
    destroy: None,
}; 16];
static mut provider_count: size_t = 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_provider_registry_reset() {
    memset(
        &raw mut providers as *mut system_health_provider_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[system_health_provider_t; 16]>(),
    );
    provider_count = 0 as size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_provider_registry_register(
    mut provider: *const system_health_provider_t,
) -> bool {
    if provider.is_null()
        || (*provider).name[0usize] == 0
        || (*provider).collect.is_none()
        || memchr(
            &raw const (*provider).name as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        )
        .is_null()
        || provider_count >= PROVIDER_CAPACITY as size_t
    {
        return r#false != 0;
    }
    let mut index: size_t = 0 as size_t;
    while index < provider_count {
        if strcmp(
            &raw mut (*(&raw mut providers as *mut system_health_provider_t).offset(index as isize))
                .name as *mut ::core::ffi::c_char,
            &raw const (*provider).name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        index = index.wrapping_add(1);
    }
    let c2rust_fresh0 = provider_count;
    provider_count = provider_count.wrapping_add(1);
    providers[c2rust_fresh0] = *provider;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_provider_registry_collect(
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
    mut resources_dropped: *mut size_t,
) -> ::core::ffi::c_int {
    let mut failures: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dropped: size_t = 0 as size_t;
    let mut index: size_t = 0 as size_t;
    while index < provider_count {
        let mut provider: *mut system_health_provider_t =
            (&raw mut providers as *mut system_health_provider_t).offset(index as isize);
        if (*provider).discover.is_some() {
            let mut inventory: system_health_provider_inventory_t =
                system_health_provider_inventory_t {
                    resources: [system_health_provider_resource_t {
                        id: [0; 64],
                        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    }; 16],
                    count: 0,
                    dropped: 0,
                };
            memset(
                &raw mut inventory as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<system_health_provider_inventory_t>(),
            );
            if (*provider).discover.expect("non-null function pointer")(
                (*provider).state,
                context,
                &raw mut inventory,
            ) != 0 as ::core::ffi::c_int
            {
                failures += 1;
            }
            dropped = dropped.wrapping_add(inventory.dropped);
        }
        if (*provider).collect.expect("non-null function pointer")((*provider).state, context, sink)
            != 0 as ::core::ffi::c_int
        {
            failures += 1;
        }
        index = index.wrapping_add(1);
    }
    if !resources_dropped.is_null() {
        *resources_dropped = dropped;
    }
    return if failures == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_provider_registry_destroy() {
    let mut index: size_t = 0 as size_t;
    while index < provider_count {
        if providers[index].destroy.is_some() {
            providers[index].destroy.expect("non-null function pointer")(providers[index].state);
        }
        index = index.wrapping_add(1);
    }
    system_health_provider_registry_reset();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_provider_registry_count() -> size_t {
    return provider_count;
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
