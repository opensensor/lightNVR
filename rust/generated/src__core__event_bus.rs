unsafe extern "C" {
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn event_envelope_serialize(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn event_envelope_clone(
        destination: *mut event_envelope_t,
        source: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_envelope_clear(event: *mut event_envelope_t);
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
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
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
pub type time_t = __time_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_bus_result_t(pub ::core::ffi::c_int);
impl event_bus_result_t {
    pub const EVENT_BUS_OK: Self = Self(0);
    pub const EVENT_BUS_NOT_RUNNING: Self = Self(-1);
    pub const EVENT_BUS_INVALID_EVENT: Self = Self(-2);
    pub const EVENT_BUS_QUEUE_FULL: Self = Self(-3);
    pub const EVENT_BUS_ALLOCATION_FAILED: Self = Self(-4);
}
pub type event_bus_handler_t = Option<
    unsafe extern "C" fn(*const event_envelope_t, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
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
pub struct event_bus_state_t {
    pub mutex: pthread_mutex_t,
    pub wake: pthread_cond_t,
    pub idle: pthread_cond_t,
    pub worker: pthread_t,
    pub initialized: bool,
    pub running: bool,
    pub drain_on_shutdown: bool,
    pub head: *mut event_node_t,
    pub tail: *mut event_node_t,
    pub subscribers: [event_subscription_t; 8],
    pub subscriber_count: ::core::ffi::c_int,
    pub stats: event_bus_stats_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_subscription_t {
    pub name: [::core::ffi::c_char; 48],
    pub handler: event_bus_handler_t,
    pub context: *mut ::core::ffi::c_void,
}
pub type event_node_t = event_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_node {
    pub event: event_envelope_t,
    pub serialized_size: size_t,
    pub next: *mut event_node,
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
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EVENT_ENVELOPE_MAX_BYTES: ::core::ffi::c_uint =
    (96 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
pub const EVENT_BUS_DEFAULT_MAX_EVENTS: ::core::ffi::c_uint = 1024 as ::core::ffi::c_uint;
pub const EVENT_BUS_DEFAULT_MAX_BYTES: ::core::ffi::c_uint = (8 as ::core::ffi::c_uint)
    .wrapping_mul(1024 as ::core::ffi::c_uint)
    .wrapping_mul(1024 as ::core::ffi::c_uint);
pub const EVENT_BUS_MAX_SUBSCRIBERS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const EVENT_BUS_SUBSCRIBER_NAME_MAX: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
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
static mut BUS: event_bus_state_t = event_bus_state_t {
    mutex: pthread_mutex_t {
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
    idle: PTHREAD_COND_INITIALIZER,
    worker: 0,
    initialized: false,
    running: false,
    drain_on_shutdown: false,
    head: ::core::ptr::null_mut::<event_node_t>(),
    tail: ::core::ptr::null_mut::<event_node_t>(),
    subscribers: [event_subscription_t {
        name: [0; 48],
        handler: None,
        context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    }; 8],
    subscriber_count: 0,
    stats: event_bus_stats_t {
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
    },
};
unsafe extern "C" fn set_error(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut message: *const ::core::ffi::c_char,
) {
    if error.is_null() || error_size == 0 as size_t {
        return;
    }
    snprintf(
        error,
        error_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !message.is_null() {
            message
        } else {
            b"event bus error\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn free_node(mut node: *mut event_node_t) {
    if node.is_null() {
        return;
    }
    event_envelope_clear(&raw mut (*node).event);
    free(node as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn remove_node_locked(
    mut previous: *mut event_node_t,
    mut node: *mut event_node_t,
) {
    if node.is_null() {
        return;
    }
    if !previous.is_null() {
        (*previous).next = (*node).next;
    } else {
        BUS.head = (*node).next as *mut event_node_t;
    }
    if BUS.tail == node {
        BUS.tail = previous;
    }
    BUS.stats.queued_events = BUS.stats.queued_events.wrapping_sub(1);
    BUS.stats.queued_bytes = BUS.stats.queued_bytes.wrapping_sub((*node).serialized_size);
    free_node(node);
}
unsafe extern "C" fn queue_has_capacity_locked(mut event_size: size_t) -> bool {
    return BUS.stats.queued_events < BUS.stats.max_events
        && event_size <= BUS.stats.max_bytes.wrapping_sub(BUS.stats.queued_bytes);
}
unsafe extern "C" fn shed_lower_priority_locked(
    mut incoming_severity: event_severity_t,
    mut event_size: size_t,
) -> bool {
    while !queue_has_capacity_locked(event_size) {
        let mut previous: *mut event_node_t = ::core::ptr::null_mut::<event_node_t>();
        let mut candidate_previous: *mut event_node_t = ::core::ptr::null_mut::<event_node_t>();
        let mut candidate: *mut event_node_t = ::core::ptr::null_mut::<event_node_t>();
        let mut node: *mut event_node_t = BUS.head;
        while !node.is_null() {
            if (*node).event.severity.0 < incoming_severity.0 {
                candidate = node;
                candidate_previous = previous;
                break;
            } else {
                previous = node;
                node = (*node).next as *mut event_node_t;
            }
        }
        if candidate.is_null() {
            return r#false != 0;
        }
        remove_node_locked(candidate_previous, candidate);
        BUS.stats.dropped_events = BUS.stats.dropped_events.wrapping_add(1);
        BUS.stats.priority_shed_events = BUS.stats.priority_shed_events.wrapping_add(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn pop_node_locked() -> *mut event_node_t {
    let mut node: *mut event_node_t = BUS.head;
    if node.is_null() {
        return ::core::ptr::null_mut::<event_node_t>();
    }
    BUS.head = (*node).next as *mut event_node_t;
    if BUS.head.is_null() {
        BUS.tail = ::core::ptr::null_mut::<event_node_t>();
    }
    (*node).next = ::core::ptr::null_mut::<event_node>();
    BUS.stats.queued_events = BUS.stats.queued_events.wrapping_sub(1);
    BUS.stats.queued_bytes = BUS.stats.queued_bytes.wrapping_sub((*node).serialized_size);
    BUS.stats.active_dispatches = BUS.stats.active_dispatches.wrapping_add(1);
    return node;
}
unsafe extern "C" fn event_bus_worker(
    mut unused: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    pthread_mutex_lock(&raw mut BUS.mutex);
    loop {
        while BUS.head.is_null() && BUS.running as ::core::ffi::c_int != 0 {
            pthread_cond_wait(&raw mut BUS.wake, &raw mut BUS.mutex);
        }
        if !BUS.running && (!BUS.drain_on_shutdown || BUS.head.is_null()) {
            break;
        }
        let mut node: *mut event_node_t = pop_node_locked();
        let mut subscribers: [event_subscription_t; 8] = [event_subscription_t {
            name: [0; 48],
            handler: None,
            context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        }; 8];
        let mut subscriber_count: ::core::ffi::c_int = BUS.subscriber_count;
        if subscriber_count > 0 as ::core::ffi::c_int {
            memcpy(
                &raw mut subscribers as *mut event_subscription_t as *mut ::core::ffi::c_void,
                &raw mut BUS.subscribers as *mut event_subscription_t as *const ::core::ffi::c_void,
                (subscriber_count as size_t)
                    .wrapping_mul(::core::mem::size_of::<event_subscription_t>()),
            );
        }
        pthread_mutex_unlock(&raw mut BUS.mutex);
        let mut failures: uint64_t = 0 as uint64_t;
        let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index < subscriber_count {
            if subscribers[index as usize]
                .handler
                .expect("non-null function pointer")(
                &raw mut (*node).event,
                subscribers[index as usize].context,
            ) != 0 as ::core::ffi::c_int
            {
                failures = failures.wrapping_add(1);
            }
            index += 1;
        }
        free_node(node);
        pthread_mutex_lock(&raw mut BUS.mutex);
        BUS.stats.dispatched_events = BUS.stats.dispatched_events.wrapping_add(1);
        BUS.stats.callback_deliveries = BUS
            .stats
            .callback_deliveries
            .wrapping_add(subscriber_count as uint64_t);
        BUS.stats.handler_failures = BUS.stats.handler_failures.wrapping_add(failures);
        BUS.stats.active_dispatches = BUS.stats.active_dispatches.wrapping_sub(1);
        if BUS.head.is_null() && BUS.stats.active_dispatches == 0 as size_t {
            pthread_cond_broadcast(&raw mut BUS.idle);
        }
    }
    if BUS.head.is_null() && BUS.stats.active_dispatches == 0 as size_t {
        pthread_cond_broadcast(&raw mut BUS.idle);
    }
    pthread_mutex_unlock(&raw mut BUS.mutex);
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_bus_subscribe(
    mut name: *const ::core::ffi::c_char,
    mut handler: event_bus_handler_t,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if name.is_null()
        || *name.offset(0isize) == 0
        || strlen(name) >= EVENT_BUS_SUBSCRIBER_NAME_MAX as size_t
        || handler.is_none()
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut BUS.mutex);
    if BUS.initialized as ::core::ffi::c_int != 0
        || BUS.subscriber_count >= EVENT_BUS_MAX_SUBSCRIBERS
    {
        pthread_mutex_unlock(&raw mut BUS.mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < BUS.subscriber_count {
        if strcmp(
            &raw mut (*(&raw mut BUS.subscribers as *mut event_subscription_t)
                .offset(index as isize))
            .name as *mut ::core::ffi::c_char,
            name,
        ) == 0 as ::core::ffi::c_int
        {
            pthread_mutex_unlock(&raw mut BUS.mutex);
            return -1 as ::core::ffi::c_int;
        }
        index += 1;
    }
    let c2rust_fresh0 = BUS.subscriber_count;
    BUS.subscriber_count += 1;
    let mut subscription: *mut event_subscription_t =
        (&raw mut BUS.subscribers as *mut event_subscription_t).offset(c2rust_fresh0 as isize);
    memset(
        subscription as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_subscription_t>(),
    );
    snprintf(
        &raw mut (*subscription).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
    (*subscription).handler = handler;
    (*subscription).context = context;
    pthread_mutex_unlock(&raw mut BUS.mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_bus_unsubscribe(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if name.is_null() || *name.offset(0isize) == 0 {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut BUS.mutex);
    if BUS.initialized {
        pthread_mutex_unlock(&raw mut BUS.mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < BUS.subscriber_count {
        if strcmp(
            &raw mut (*(&raw mut BUS.subscribers as *mut event_subscription_t)
                .offset(index as isize))
            .name as *mut ::core::ffi::c_char,
            name,
        ) == 0 as ::core::ffi::c_int
        {
            let mut r#move: ::core::ffi::c_int = index + 1 as ::core::ffi::c_int;
            while r#move < BUS.subscriber_count {
                BUS.subscribers[(r#move - 1 as ::core::ffi::c_int) as usize] =
                    BUS.subscribers[r#move as usize];
                r#move += 1;
            }
            BUS.subscriber_count -= 1;
            memset(
                (&raw mut BUS.subscribers as *mut event_subscription_t)
                    .offset(BUS.subscriber_count as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<event_subscription_t>(),
            );
            pthread_mutex_unlock(&raw mut BUS.mutex);
            return 0 as ::core::ffi::c_int;
        }
        index += 1;
    }
    pthread_mutex_unlock(&raw mut BUS.mutex);
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_bus_init(
    mut max_events: size_t,
    mut max_bytes: size_t,
) -> ::core::ffi::c_int {
    if max_events == 0 as size_t {
        max_events = EVENT_BUS_DEFAULT_MAX_EVENTS as size_t;
    }
    if max_bytes == 0 as size_t {
        max_bytes = EVENT_BUS_DEFAULT_MAX_BYTES as size_t;
    }
    if max_events == 0 as size_t || max_bytes < EVENT_ENVELOPE_MAX_BYTES as size_t {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut BUS.mutex);
    if BUS.initialized {
        pthread_mutex_unlock(&raw mut BUS.mutex);
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut BUS.stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_bus_stats_t>(),
    );
    BUS.stats.max_events = max_events;
    BUS.stats.max_bytes = max_bytes;
    BUS.stats.subscriber_count = BUS.subscriber_count;
    BUS.stats.running = r#true != 0;
    BUS.running = r#true != 0;
    BUS.drain_on_shutdown = r#false != 0;
    BUS.initialized = r#true != 0;
    if pthread_create(
        &raw mut BUS.worker,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            event_bus_worker
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        BUS.running = r#false != 0;
        BUS.initialized = r#false != 0;
        BUS.stats.running = r#false != 0;
        pthread_mutex_unlock(&raw mut BUS.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut BUS.mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_bus_publish(
    mut event: *const event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> event_bus_result_t {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    pthread_mutex_lock(&raw mut BUS.mutex);
    let mut running: bool =
        BUS.initialized as ::core::ffi::c_int != 0 && BUS.running as ::core::ffi::c_int != 0;
    pthread_mutex_unlock(&raw mut BUS.mutex);
    if !running {
        set_error(
            error,
            error_size,
            b"event bus is not running\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return event_bus_result_t::EVENT_BUS_NOT_RUNNING;
    }
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
    let mut serialized: *mut ::core::ffi::c_char = event_envelope_serialize(
        event,
        &raw mut validation_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if serialized.is_null() {
        pthread_mutex_lock(&raw mut BUS.mutex);
        BUS.stats.rejected_events = BUS.stats.rejected_events.wrapping_add(1);
        pthread_mutex_unlock(&raw mut BUS.mutex);
        set_error(
            error,
            error_size,
            &raw mut validation_error as *mut ::core::ffi::c_char,
        );
        return event_bus_result_t::EVENT_BUS_INVALID_EVENT;
    }
    let mut serialized_size: size_t = strlen(serialized);
    free(serialized as *mut ::core::ffi::c_void);
    let mut node: *mut event_node_t =
        calloc(1 as size_t, ::core::mem::size_of::<event_node_t>()) as *mut event_node_t;
    if node.is_null()
        || event_envelope_clone(
            &raw mut (*node).event,
            event,
            &raw mut validation_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) != 0 as ::core::ffi::c_int
    {
        free_node(node);
        pthread_mutex_lock(&raw mut BUS.mutex);
        BUS.stats.rejected_events = BUS.stats.rejected_events.wrapping_add(1);
        pthread_mutex_unlock(&raw mut BUS.mutex);
        set_error(
            error,
            error_size,
            b"event queue allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return event_bus_result_t::EVENT_BUS_ALLOCATION_FAILED;
    }
    (*node).serialized_size = serialized_size;
    pthread_mutex_lock(&raw mut BUS.mutex);
    if !BUS.initialized || !BUS.running {
        pthread_mutex_unlock(&raw mut BUS.mutex);
        free_node(node);
        set_error(
            error,
            error_size,
            b"event bus stopped before enqueue\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return event_bus_result_t::EVENT_BUS_NOT_RUNNING;
    }
    let mut has_capacity: bool = queue_has_capacity_locked(serialized_size);
    if !has_capacity && (*node).event.severity.0 >= event_severity_t::EVENT_SEVERITY_ERROR.0 {
        has_capacity = shed_lower_priority_locked((*node).event.severity, serialized_size);
    }
    if !has_capacity {
        BUS.stats.dropped_events = BUS.stats.dropped_events.wrapping_add(1);
        pthread_mutex_unlock(&raw mut BUS.mutex);
        free_node(node);
        set_error(
            error,
            error_size,
            b"event bus queue is full\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return event_bus_result_t::EVENT_BUS_QUEUE_FULL;
    }
    if !BUS.tail.is_null() {
        (*BUS.tail).next = node as *mut event_node;
    } else {
        BUS.head = node;
    }
    BUS.tail = node;
    BUS.stats.queued_events = BUS.stats.queued_events.wrapping_add(1);
    BUS.stats.queued_bytes = BUS.stats.queued_bytes.wrapping_add(serialized_size);
    BUS.stats.accepted_events = BUS.stats.accepted_events.wrapping_add(1);
    pthread_cond_signal(&raw mut BUS.wake);
    pthread_mutex_unlock(&raw mut BUS.mutex);
    return event_bus_result_t::EVENT_BUS_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_bus_wait_until_idle(
    mut timeout_ms: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut deadline: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut deadline) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    deadline.tv_sec += timeout_ms.wrapping_div(1000 as ::core::ffi::c_uint) as time_t as __time_t;
    deadline.tv_nsec += timeout_ms.wrapping_rem(1000 as ::core::ffi::c_uint) as ::core::ffi::c_long
        * 1000000 as ::core::ffi::c_long;
    if deadline.tv_nsec >= 1000000000 as ::core::ffi::c_long {
        deadline.tv_sec += 1;
        deadline.tv_nsec -= 1000000000 as ::core::ffi::c_long;
    }
    pthread_mutex_lock(&raw mut BUS.mutex);
    if !BUS.initialized {
        pthread_mutex_unlock(&raw mut BUS.mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while BUS.stats.queued_events > 0 as size_t || BUS.stats.active_dispatches > 0 as size_t {
        let mut wait_result: ::core::ffi::c_int =
            pthread_cond_timedwait(&raw mut BUS.idle, &raw mut BUS.mutex, &raw mut deadline);
        if wait_result == ETIMEDOUT {
            result = -1 as ::core::ffi::c_int;
            break;
        } else {
            if wait_result == 0 as ::core::ffi::c_int {
                continue;
            }
            result = -1 as ::core::ffi::c_int;
            break;
        }
    }
    pthread_mutex_unlock(&raw mut BUS.mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_bus_get_stats(mut stats: *mut event_bus_stats_t) {
    if stats.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut BUS.mutex);
    *stats = BUS.stats;
    (*stats).subscriber_count = BUS.subscriber_count;
    (*stats).running =
        BUS.initialized as ::core::ffi::c_int != 0 && BUS.running as ::core::ffi::c_int != 0;
    pthread_mutex_unlock(&raw mut BUS.mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_bus_shutdown(mut drain: bool) {
    pthread_mutex_lock(&raw mut BUS.mutex);
    if !BUS.initialized {
        pthread_mutex_unlock(&raw mut BUS.mutex);
        return;
    }
    BUS.running = r#false != 0;
    BUS.drain_on_shutdown = drain;
    BUS.stats.running = r#false != 0;
    if !drain {
        while !BUS.head.is_null() {
            let mut node: *mut event_node_t = BUS.head;
            BUS.head = (*node).next as *mut event_node_t;
            BUS.stats.queued_events = BUS.stats.queued_events.wrapping_sub(1);
            BUS.stats.queued_bytes = BUS.stats.queued_bytes.wrapping_sub((*node).serialized_size);
            BUS.stats.dropped_events = BUS.stats.dropped_events.wrapping_add(1);
            free_node(node);
        }
        BUS.tail = ::core::ptr::null_mut::<event_node_t>();
    }
    pthread_cond_broadcast(&raw mut BUS.wake);
    pthread_mutex_unlock(&raw mut BUS.mutex);
    pthread_join(
        BUS.worker,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    pthread_mutex_lock(&raw mut BUS.mutex);
    while !BUS.head.is_null() {
        let mut node_0: *mut event_node_t = BUS.head;
        BUS.head = (*node_0).next as *mut event_node_t;
        free_node(node_0);
    }
    BUS.tail = ::core::ptr::null_mut::<event_node_t>();
    BUS.stats.queued_events = 0 as size_t;
    BUS.stats.queued_bytes = 0 as size_t;
    BUS.initialized = r#false != 0;
    BUS.drain_on_shutdown = r#false != 0;
    pthread_cond_broadcast(&raw mut BUS.idle);
    pthread_mutex_unlock(&raw mut BUS.mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
