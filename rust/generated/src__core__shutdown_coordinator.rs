unsafe extern "C" {
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
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
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
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
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub type atomic_bool = bool;
pub type atomic_int = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct component_state_t(pub ::core::ffi::c_uint);
impl component_state_t {
    pub const COMPONENT_RUNNING: Self = Self(0);
    pub const COMPONENT_STOPPING: Self = Self(1);
    pub const COMPONENT_STOPPED: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct component_type_t(pub ::core::ffi::c_uint);
impl component_type_t {
    pub const COMPONENT_DETECTION_THREAD: Self = Self(0);
    pub const COMPONENT_SERVER_THREAD: Self = Self(1);
    pub const COMPONENT_HLS_WRITER: Self = Self(2);
    pub const COMPONENT_MP4_WRITER: Self = Self(3);
    pub const COMPONENT_OTHER: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct component_info_t {
    pub name: [::core::ffi::c_char; 64],
    pub r#type: component_type_t,
    pub state: atomic_int,
    pub context: *mut ::core::ffi::c_void,
    pub priority: ::core::ffi::c_int,
    pub generation: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shutdown_coordinator_t {
    pub shutdown_initiated: atomic_bool,
    pub coordinator_destroyed: atomic_bool,
    pub component_count: atomic_int,
    pub components: [component_info_t; 32],
    pub mutex: pthread_mutex_t,
    pub all_stopped_cond: pthread_cond_t,
    pub all_components_stopped: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct component_priority_t {
    pub id: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
}
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_COMPONENTS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut g_coordinator: shutdown_coordinator_t = shutdown_coordinator_t {
    shutdown_initiated: false,
    coordinator_destroyed: false,
    component_count: 0,
    components: [component_info_t {
        name: [0; 64],
        r#type: component_type_t::COMPONENT_DETECTION_THREAD,
        state: 0,
        context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        priority: 0,
        generation: 0,
    }; 32],
    mutex: pthread_mutex_t {
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
    all_stopped_cond: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter { __value64: 0 },
            __g1_start: __atomic_wide_counter { __value64: 0 },
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
            __unused_initialized_1: 0,
            __unused_initialized_2: 0,
        },
    },
    all_components_stopped: false,
};
pub const MAX_COMPONENT_GENERATION: ::core::ffi::c_int = INT_MAX / MAX_COMPONENTS;
unsafe extern "C" fn make_component_id(
    mut slot: ::core::ffi::c_int,
    mut generation: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return generation * MAX_COMPONENTS + slot;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn component_id_slot(
    mut component_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if component_id < 0 as ::core::ffi::c_int {
        -1 as ::core::ffi::c_int
    } else {
        component_id % MAX_COMPONENTS
    };
}
unsafe extern "C" fn resolve_component_locked(
    mut component_id: ::core::ffi::c_int,
) -> *mut component_info_t {
    if component_id < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<component_info_t>();
    }
    let mut slot: ::core::ffi::c_int = component_id % MAX_COMPONENTS;
    let mut generation: ::core::ffi::c_int = component_id / MAX_COMPONENTS;
    if slot
        >= ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g_coordinator.component_count,
        )
    {
        return ::core::ptr::null_mut::<component_info_t>();
    }
    let mut component: *mut component_info_t =
        (&raw mut g_coordinator.components as *mut component_info_t).offset(slot as isize);
    return if (*component).generation == generation {
        component
    } else {
        ::core::ptr::null_mut::<component_info_t>()
    };
}
static mut g_background_abort_requested: atomic_bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_background_abort() {
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_background_abort_requested).store(true, ::core::sync::atomic::Ordering::SeqCst);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_background_abort_requested() -> bool {
    return ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_background_abort_requested).load(::core::sync::atomic::Ordering::SeqCst);
}
static mut g_sync_initialized: bool = r#false != 0;
unsafe extern "C" fn reset_coordinator_state_locked() {
    memset(
        &raw mut g_coordinator.components as *mut component_info_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[component_info_t; 32]>(),
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_coordinator.component_count,
        0 as ::core::ffi::c_int,
    );
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.shutdown_initiated).store(false, ::core::sync::atomic::Ordering::SeqCst);
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.coordinator_destroyed).store(false, ::core::sync::atomic::Ordering::SeqCst);
    g_coordinator.all_components_stopped = r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_shutdown_coordinator() -> ::core::ffi::c_int {
    if !g_sync_initialized {
        memset(
            &raw mut g_coordinator as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<shutdown_coordinator_t>(),
        );
        if pthread_mutex_init(
            &raw mut g_coordinator.mutex,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize shutdown coordinator mutex\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if pthread_cond_init(
            &raw mut g_coordinator.all_stopped_cond,
            ::core::ptr::null::<pthread_condattr_t>(),
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize shutdown coordinator condition variable\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            pthread_mutex_destroy(&raw mut g_coordinator.mutex);
            return -1 as ::core::ffi::c_int;
        }
        g_sync_initialized = r#true != 0;
        reset_coordinator_state_locked();
    } else {
        pthread_mutex_lock(&raw mut g_coordinator.mutex);
        reset_coordinator_state_locked();
        pthread_mutex_unlock(&raw mut g_coordinator.mutex);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Shutdown coordinator initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_coordinator_cleanup() {
    if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.coordinator_destroyed).swap(true, ::core::sync::atomic::Ordering::SeqCst) {
        return;
    }
    if !g_sync_initialized {
        return;
    }
    pthread_mutex_lock(&raw mut g_coordinator.mutex);
    pthread_cond_broadcast(&raw mut g_coordinator.all_stopped_cond);
    pthread_mutex_unlock(&raw mut g_coordinator.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Shutdown coordinator cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_coordinator_destroyed() -> bool {
    return ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.coordinator_destroyed).load(::core::sync::atomic::Ordering::SeqCst);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_component(
    mut name: *const ::core::ffi::c_char,
    mut r#type: component_type_t,
    mut context: *mut ::core::ffi::c_void,
    mut priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot register component with NULL name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_coordinator.mutex);
    if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.coordinator_destroyed).load(::core::sync::atomic::Ordering::SeqCst) {
        pthread_mutex_unlock(&raw mut g_coordinator.mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot register component %s: shutdown coordinator already cleaned up\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g_coordinator.component_count);
    let mut reuse_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*(&raw mut g_coordinator.components as *mut component_info_t)
                .offset(i as isize))
            .state,
        ) == component_state_t::COMPONENT_STOPPED.0 as ::core::ffi::c_int
        {
            if reuse_idx < 0 as ::core::ffi::c_int {
                reuse_idx = i;
            }
            if strcmp(
                &raw mut (*(&raw mut g_coordinator.components as *mut component_info_t)
                    .offset(i as isize))
                .name as *mut ::core::ffi::c_char,
                name,
            ) == 0 as ::core::ffi::c_int
            {
                reuse_idx = i;
                break;
            }
        }
        i += 1;
    }
    let mut slot: ::core::ffi::c_int = 0;
    if reuse_idx >= 0 as ::core::ffi::c_int {
        slot = reuse_idx;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Reusing STOPPED component slot %d (was '%s') for new component %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            slot,
            &raw mut (*(&raw mut g_coordinator.components as *mut component_info_t)
                .offset(slot as isize))
            .name as *mut ::core::ffi::c_char,
            name,
        );
    } else if count < MAX_COMPONENTS {
        slot = count;
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g_coordinator.component_count,
            count + 1 as ::core::ffi::c_int,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot register component %s: maximum number of components reached\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
        pthread_mutex_unlock(&raw mut g_coordinator.mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut component: *mut component_info_t =
        (&raw mut g_coordinator.components as *mut component_info_t).offset(slot as isize);
    if reuse_idx >= 0 as ::core::ffi::c_int {
        (*component).generation =
            ((*component).generation + 1 as ::core::ffi::c_int) % MAX_COMPONENT_GENERATION;
    }
    let mut component_id: ::core::ffi::c_int = make_component_id(slot, (*component).generation);
    safe_strcpy(
        &raw mut (*component).name as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    (*component).r#type = r#type;
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*component).state,
        component_state_t::COMPONENT_RUNNING.0 as ::core::ffi::c_int,
    );
    (*component).context = context;
    (*component).priority = priority;
    g_coordinator.all_components_stopped = r#false != 0;
    pthread_mutex_unlock(&raw mut g_coordinator.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Registered component %s (ID: %d, slot: %d, type: %d, priority: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        name,
        component_id,
        slot,
        r#type.0,
        priority,
    );
    return component_id;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_component_state(
    mut component_id: ::core::ffi::c_int,
    mut state: component_state_t,
) {
    pthread_mutex_lock(&raw mut g_coordinator.mutex);
    let mut component: *mut component_info_t = resolve_component_locked(component_id);
    if component.is_null() {
        pthread_mutex_unlock(&raw mut g_coordinator.mutex);
        if component_id < 0 as ::core::ffi::c_int
            || component_id % MAX_COMPONENTS
                >= ::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut g_coordinator.component_count)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Invalid component ID: %d\0".as_ptr() as *const ::core::ffi::c_char,
                component_id,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Ignoring state %d for stale component ID %d (slot reused)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                state.0,
                component_id,
            );
        }
        return;
    }
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*component).state,
        state.0 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Updated component %s (ID: %d) state to %d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*component).name as *mut ::core::ffi::c_char,
        component_id,
        state.0,
    );
    if state.0 == component_state_t::COMPONENT_STOPPED.0 {
        let mut all_stopped: bool = r#true != 0;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i
            < ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g_coordinator.component_count,
            )
        {
            if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut g_coordinator.components as *mut component_info_t)
                    .offset(i as isize))
                .state,
            ) != component_state_t::COMPONENT_STOPPED.0 as ::core::ffi::c_int
            {
                all_stopped = r#false != 0;
                break;
            } else {
                i += 1;
            }
        }
        if all_stopped as ::core::ffi::c_int != 0 && !g_coordinator.all_components_stopped {
            g_coordinator.all_components_stopped = r#true != 0;
            pthread_cond_broadcast(&raw mut g_coordinator.all_stopped_cond);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"All components are now stopped\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    pthread_mutex_unlock(&raw mut g_coordinator.mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_component_state(
    mut component_id: ::core::ffi::c_int,
) -> component_state_t {
    pthread_mutex_lock(&raw mut g_coordinator.mutex);
    if component_id < 0 as ::core::ffi::c_int
        || component_id % MAX_COMPONENTS
            >= ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g_coordinator.component_count,
            )
    {
        pthread_mutex_unlock(&raw mut g_coordinator.mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid component ID: %d\0".as_ptr() as *const ::core::ffi::c_char,
            component_id,
        );
        return component_state_t::COMPONENT_STOPPED;
    }
    let mut component: *mut component_info_t = resolve_component_locked(component_id);
    let mut state: component_state_t = component_state_t(
        (if !component.is_null() {
            ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*component).state,
            )
        } else {
            component_state_t::COMPONENT_STOPPED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    pthread_mutex_unlock(&raw mut g_coordinator.mutex);
    return state;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_component_id_by_name(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut found: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    pthread_mutex_lock(&raw mut g_coordinator.mutex);
    let mut count: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g_coordinator.component_count);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut component: *mut component_info_t =
            (&raw mut g_coordinator.components as *mut component_info_t).offset(i as isize);
        if strcmp(&raw mut (*component).name as *mut ::core::ffi::c_char, name)
            == 0 as ::core::ffi::c_int
        {
            found = make_component_id(i, (*component).generation);
            if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*component).state,
            ) != component_state_t::COMPONENT_STOPPED.0 as ::core::ffi::c_int
            {
                break;
            }
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut g_coordinator.mutex);
    return found;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initiate_shutdown() {
    if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.coordinator_destroyed).load(::core::sync::atomic::Ordering::SeqCst) {
        return;
    }
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.shutdown_initiated).store(true, ::core::sync::atomic::Ordering::SeqCst);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Shutdown sequence initiated\0".as_ptr() as *const ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut g_coordinator.mutex);
    let mut priorities: [component_priority_t; 32] =
        [component_priority_t { id: 0, priority: 0 }; 32];
    memset(
        &raw mut priorities as *mut component_priority_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[component_priority_t; 32]>(),
    );
    let mut count: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g_coordinator.component_count);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        priorities[i as usize].id = i;
        priorities[i as usize].priority = g_coordinator.components[i as usize].priority;
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < count - 1 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < count - i_0 - 1 as ::core::ffi::c_int {
            if priorities[j as usize].priority
                < priorities[(j + 1 as ::core::ffi::c_int) as usize].priority
            {
                let mut temp: component_priority_t = priorities[j as usize];
                priorities[j as usize] = priorities[(j + 1 as ::core::ffi::c_int) as usize];
                priorities[(j + 1 as ::core::ffi::c_int) as usize] = temp;
            }
            j += 1;
        }
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < count {
        let mut id: ::core::ffi::c_int = priorities[i_1 as usize].id;
        let mut component: *mut component_info_t =
            (&raw mut g_coordinator.components as *mut component_info_t).offset(id as isize);
        if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*component).state,
        ) == component_state_t::COMPONENT_RUNNING.0 as ::core::ffi::c_int
        {
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*component).state,
                component_state_t::COMPONENT_STOPPING.0 as ::core::ffi::c_int,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Signaling component %s (ID: %d, priority: %d) to stop\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*component).name as *mut ::core::ffi::c_char,
                id,
                (*component).priority,
            );
        }
        i_1 += 1;
    }
    pthread_mutex_unlock(&raw mut g_coordinator.mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_shutdown_initiated() -> bool {
    return ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.shutdown_initiated).load(::core::sync::atomic::Ordering::SeqCst);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wait_for_all_components_stopped(
    mut timeout_seconds: ::core::ffi::c_int,
) -> bool {
    if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.coordinator_destroyed).load(::core::sync::atomic::Ordering::SeqCst) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Coordinator already destroyed, cannot wait for components\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    let mut timeout: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut timeout);
    timeout.tv_sec += timeout_seconds as __time_t;
    pthread_mutex_lock(&raw mut g_coordinator.mutex);
    if g_coordinator.all_components_stopped {
        pthread_mutex_unlock(&raw mut g_coordinator.mutex);
        return r#true != 0;
    }
    let mut component_count: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g_coordinator.component_count);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Waiting for %d components to stop (timeout: %d seconds):\0".as_ptr()
            as *const ::core::ffi::c_char,
        component_count,
        timeout_seconds,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < component_count {
        let mut state: component_state_t = component_state_t(::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(
            &raw mut (*(&raw mut g_coordinator.components as *mut component_info_t)
                .offset(i as isize))
            .state,
        ) as ::core::ffi::c_uint);
        if state.0 != component_state_t::COMPONENT_STOPPED.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Component %s (ID: %d) is in state %d\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(&raw mut g_coordinator.components as *mut component_info_t)
                    .offset(i as isize))
                .name as *mut ::core::ffi::c_char,
                i,
                state.0,
            );
        }
        i += 1;
    }
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !g_coordinator.all_components_stopped
        && result == 0 as ::core::ffi::c_int
        && !::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.coordinator_destroyed).load(::core::sync::atomic::Ordering::SeqCst)
    {
        result = pthread_cond_timedwait(
            &raw mut g_coordinator.all_stopped_cond,
            &raw mut g_coordinator.mutex,
            &raw mut timeout,
        );
    }
    let mut all_stopped: bool = g_coordinator.all_components_stopped;
    let mut destroyed: bool = ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_coordinator.coordinator_destroyed).load(::core::sync::atomic::Ordering::SeqCst);
    pthread_mutex_unlock(&raw mut g_coordinator.mutex);
    if !all_stopped && destroyed as ::core::ffi::c_int != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Coordinator cleaned up while waiting for components; proceeding\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    if !all_stopped {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Deadline or wait error before all components stopped (wait result: %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            result,
        );
    }
    return all_stopped;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_shutdown_coordinator() -> *mut shutdown_coordinator_t {
    return &raw mut g_coordinator;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
