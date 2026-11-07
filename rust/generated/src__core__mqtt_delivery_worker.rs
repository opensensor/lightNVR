unsafe extern "C" {
    pub type mqtt_destination_client;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn event_envelope_validate(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_outbox_enqueue(
        event: *const event_envelope_t,
        destination: *const ::core::ffi::c_char,
        topic: *const ::core::ffi::c_char,
        limits: *const event_outbox_limits_t,
        row_id: *mut int64_t,
        shed_count: *mut ::core::ffi::c_int,
    ) -> event_outbox_enqueue_result_t;
    unsafe fn db_event_outbox_claim_due(
        destination: *const ::core::ffi::c_char,
        now: int64_t,
        lease_seconds: ::core::ffi::c_int,
        item: *mut event_outbox_item_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_outbox_mark_delivered(
        row_id: int64_t,
        delivered_at: int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_outbox_mark_retry(
        row_id: int64_t,
        next_attempt_at: int64_t,
        error: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_outbox_mark_dead(
        row_id: int64_t,
        failed_at: int64_t,
        error: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_outbox_expire(
        now: int64_t,
        expired_count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_outbox_prune_terminal(
        updated_before: int64_t,
        limit: ::core::ffi::c_int,
        deleted_count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_outbox_item_clear(item: *mut event_outbox_item_t);
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
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn log_set_thread_context(
        component: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    );
    unsafe fn mqtt_is_connected() -> bool;
    unsafe fn mqtt_publish_raw_confirmed(
        topic: *const ::core::ffi::c_char,
        payload: *const ::core::ffi::c_char,
        retain: bool,
        timeout_ms: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_destination_count() -> ::core::ffi::c_int;
    unsafe fn db_event_destination_list(
        destinations: *mut event_destination_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_destination_make_key(
        uuid: *const ::core::ffi::c_char,
        key: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_destination_generation() -> uint64_t;
    unsafe fn mqtt_destination_client_create(
        profile: *const event_destination_t,
    ) -> *mut mqtt_destination_client_t;
    unsafe fn mqtt_destination_client_destroy(client: *mut mqtt_destination_client_t);
    unsafe fn mqtt_destination_client_is_connected(client: *mut mqtt_destination_client_t) -> bool;
    unsafe fn mqtt_destination_client_publish_confirmed(
        client: *mut mqtt_destination_client_t,
        topic: *const ::core::ffi::c_char,
        payload: *const ::core::ffi::c_char,
        retain: bool,
        timeout_ms: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
pub struct event_outbox_enqueue_result_t(pub ::core::ffi::c_int);
impl event_outbox_enqueue_result_t {
    pub const EVENT_OUTBOX_ENQUEUED: Self = Self(0);
    pub const EVENT_OUTBOX_DUPLICATE: Self = Self(1);
    pub const EVENT_OUTBOX_FULL: Self = Self(2);
    pub const EVENT_OUTBOX_ERROR: Self = Self(-1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_outbox_limits_t {
    pub max_rows: int64_t,
    pub max_bytes: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_outbox_item_t {
    pub row_id: int64_t,
    pub event_id: [::core::ffi::c_char; 37],
    pub event_source: [::core::ffi::c_char; 128],
    pub event_type: [::core::ffi::c_char; 96],
    pub subject: [::core::ffi::c_char; 96],
    pub destination: [::core::ffi::c_char; 64],
    pub topic: [::core::ffi::c_char; 512],
    pub envelope_json: *mut ::core::ffi::c_char,
    pub envelope_bytes: size_t,
    pub severity: event_severity_t,
    pub attempt_count: ::core::ffi::c_int,
    pub next_attempt_at: int64_t,
    pub expires_at: int64_t,
    pub created_at: int64_t,
}
pub type mqtt_delivery_publish_fn = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        bool,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub struct mqtt_delivery_worker_state_t {
    pub mutex: pthread_mutex_t,
    pub wake: pthread_cond_t,
    pub thread: pthread_t,
    pub thread_started: bool,
    pub shutdown_in_progress: bool,
    pub stop_requested: bool,
    pub stats: mqtt_delivery_worker_stats_t,
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
pub struct managed_destination_slot_t {
    pub profile: event_destination_t,
    pub destination_key: [::core::ffi::c_char; 64],
    pub client: *mut mqtt_destination_client_t,
    pub next_create_attempt_at: int64_t,
}
pub type mqtt_destination_client_t = mqtt_destination_client;
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
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_DESTINATION_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_TOPIC_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_DEFAULT_LEASE_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_TERMINAL_RETENTION_SECONDS: ::core::ffi::c_longlong =
    24 as ::core::ffi::c_longlong * 60 as ::core::ffi::c_longlong * 60 as ::core::ffi::c_longlong;
pub const MQTT_EVENT_OUTBOX_DESTINATION: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"mqtt:default\0") };
pub const MQTT_DELIVERY_ACK_TIMEOUT_MS: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
pub const MQTT_DELIVERY_MAX_BACKOFF_SECONDS: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
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
pub const EVENT_DESTINATION_MAX_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_KEY_PREFIX: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"mqtt:\0") };
pub const MQTT_DELIVERY_POLL_SECONDS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MQTT_DESTINATION_CREATE_RETRY_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MQTT_DELIVERY_EXPIRE_INTERVAL_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MQTT_DELIVERY_PRUNE_INTERVAL_SECONDS: ::core::ffi::c_int =
    60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int;
pub const MQTT_DELIVERY_PRUNE_BATCH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
static mut WORKER: mqtt_delivery_worker_state_t = mqtt_delivery_worker_state_t {
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
    thread: 0,
    thread_started: false,
    shutdown_in_progress: false,
    stop_requested: false,
    stats: mqtt_delivery_worker_stats_t {
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
    },
};
unsafe extern "C" fn increment(mut counter: *mut uint64_t, mut amount: uint64_t) {
    pthread_mutex_lock(&raw mut WORKER.mutex);
    *counter = (*counter).wrapping_add(amount);
    pthread_mutex_unlock(&raw mut WORKER.mutex);
}
unsafe extern "C" fn event_jitter_seed(mut item: *const event_outbox_item_t) -> uint32_t {
    let mut hash: uint32_t = 2166136261 as uint32_t;
    let mut cursor: *const ::core::ffi::c_uchar =
        &raw const (*item).event_id as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        hash ^= *cursor as uint32_t;
        hash =
            (hash as ::core::ffi::c_uint).wrapping_mul(16777619 as ::core::ffi::c_uint) as uint32_t;
        cursor = cursor.offset(1);
    }
    hash ^= (*item).attempt_count as uint32_t;
    hash = (hash as ::core::ffi::c_uint).wrapping_mul(16777619 as ::core::ffi::c_uint) as uint32_t;
    return hash;
}
unsafe extern "C" fn valid_destination_key(mut destination: *const ::core::ffi::c_char) -> bool {
    if destination.is_null() {
        return r#false != 0;
    }
    if strcmp(destination, MQTT_EVENT_OUTBOX_DESTINATION.as_ptr()) == 0 as ::core::ffi::c_int {
        return r#true != 0;
    }
    let mut prefix_length: size_t = strlen(EVENT_DESTINATION_KEY_PREFIX.as_ptr());
    return strncmp(
        destination,
        EVENT_DESTINATION_KEY_PREFIX.as_ptr(),
        prefix_length,
    ) == 0 as ::core::ffi::c_int
        && lightnvr_uuid_is_valid(destination.offset(prefix_length as isize))
            as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn append_topic_text(
    mut topic: *mut ::core::ffi::c_char,
    mut topic_size: size_t,
    mut length: *mut size_t,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut value_length: size_t = strlen(value);
    if value_length >= topic_size
        || *length
            > topic_size
                .wrapping_sub(value_length)
                .wrapping_sub(1 as size_t)
    {
        return r#false != 0;
    }
    memcpy(
        topic.offset(*length as isize) as *mut ::core::ffi::c_void,
        value as *const ::core::ffi::c_void,
        value_length,
    );
    *length = (*length).wrapping_add(value_length);
    *topic.offset(*length as isize) = '\0' as ::core::ffi::c_char;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_topic_expand(
    mut topic_template: *const ::core::ffi::c_char,
    mut event: *const event_envelope_t,
    mut topic: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !topic.is_null() {
        *topic.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if topic_template.is_null()
        || event.is_null()
        || topic.is_null()
        || *topic_template.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || event_envelope_validate(
            event,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut subject_id: *const ::core::ffi::c_char = strrchr(
        &raw const (*event).subject as *const ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    subject_id = if !subject_id.is_null() {
        subject_id.offset(1 as ::core::ffi::c_int as isize)
    } else {
        &raw const (*event).subject as *const ::core::ffi::c_char
    };
    if *subject_id.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut length: size_t = 0 as size_t;
    let mut saw_type: bool = r#false != 0;
    let mut saw_subject_id: bool = r#false != 0;
    let mut index: size_t = 0 as size_t;
    while *topic_template.offset(index as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut value: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut token_length: size_t = 0 as size_t;
        if strncmp(
            topic_template.offset(index as isize),
            b"{type}\0".as_ptr() as *const ::core::ffi::c_char,
            6 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            value = &raw const (*event).r#type as *const ::core::ffi::c_char;
            token_length = 6 as size_t;
            saw_type = r#true != 0;
        } else if strncmp(
            topic_template.offset(index as isize),
            b"{subject_id}\0".as_ptr() as *const ::core::ffi::c_char,
            12 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            value = subject_id;
            token_length = 12 as size_t;
            saw_subject_id = r#true != 0;
        }
        if !value.is_null() {
            if !append_topic_text(
                topic,
                EVENT_OUTBOX_TOPIC_MAX as size_t,
                &raw mut length,
                value,
            ) {
                *topic.offset(0isize) = '\0' as ::core::ffi::c_char;
                return -1 as ::core::ffi::c_int;
            }
            index = index.wrapping_add(token_length);
        } else {
            if *topic_template.offset(index as isize) as ::core::ffi::c_int
                == '{' as ::core::ffi::c_int
                || *topic_template.offset(index as isize) as ::core::ffi::c_int
                    == '}' as ::core::ffi::c_int
                || *topic_template.offset(index as isize) as ::core::ffi::c_int
                    == '+' as ::core::ffi::c_int
                || *topic_template.offset(index as isize) as ::core::ffi::c_int
                    == '#' as ::core::ffi::c_int
            {
                *topic.offset(0isize) = '\0' as ::core::ffi::c_char;
                return -1 as ::core::ffi::c_int;
            }
            let mut literal: [::core::ffi::c_char; 2] = [
                *topic_template.offset(index as isize),
                '\0' as ::core::ffi::c_char,
            ];
            if !append_topic_text(
                topic,
                EVENT_OUTBOX_TOPIC_MAX as size_t,
                &raw mut length,
                &raw mut literal as *mut ::core::ffi::c_char,
            ) {
                *topic.offset(0isize) = '\0' as ::core::ffi::c_char;
                return -1 as ::core::ffi::c_int;
            }
            index = index.wrapping_add(1);
        }
    }
    if !saw_type
        || !saw_subject_id
        || length == 0 as size_t
        || *topic.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        || *topic.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        *topic.offset(0isize) = '\0' as ::core::ffi::c_char;
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_backoff_seconds(
    mut attempt_count: ::core::ffi::c_int,
    mut jitter_seed: uint32_t,
) -> ::core::ffi::c_int {
    if attempt_count < 1 as ::core::ffi::c_int {
        attempt_count = 1 as ::core::ffi::c_int;
    }
    let mut ceiling: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut attempt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while attempt < attempt_count && ceiling < MQTT_DELIVERY_MAX_BACKOFF_SECONDS {
        if ceiling > MQTT_DELIVERY_MAX_BACKOFF_SECONDS / 2 as ::core::ffi::c_int {
            ceiling = MQTT_DELIVERY_MAX_BACKOFF_SECONDS;
        } else {
            ceiling *= 2 as ::core::ffi::c_int;
        }
        attempt += 1;
    }
    let mut floor: ::core::ffi::c_int = ceiling - ceiling / 4 as ::core::ffi::c_int;
    let mut range: ::core::ffi::c_int = ceiling - floor + 1 as ::core::ffi::c_int;
    return floor + jitter_seed.wrapping_rem(range as uint32_t) as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_worker_enqueue(
    mut event: *const event_envelope_t,
    mut topic_prefix: *const ::core::ffi::c_char,
    mut row_id: *mut int64_t,
) -> event_outbox_enqueue_result_t {
    if !row_id.is_null() {
        *row_id = 0 as int64_t;
    }
    if event.is_null()
        || topic_prefix.is_null()
        || *topic_prefix.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        increment(&raw mut WORKER.stats.enqueue_errors, 1 as uint64_t);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    let mut topic_template: [::core::ffi::c_char; 512] = [0; 512];
    let mut template_length: ::core::ffi::c_int = snprintf(
        &raw mut topic_template as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/v1/events/{type}/{subject_id}\0".as_ptr() as *const ::core::ffi::c_char,
        topic_prefix,
    );
    if template_length < 0 as ::core::ffi::c_int
        || template_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        increment(&raw mut WORKER.stats.enqueue_errors, 1 as uint64_t);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    return mqtt_delivery_worker_enqueue_destination(
        event,
        MQTT_EVENT_OUTBOX_DESTINATION.as_ptr(),
        &raw mut topic_template as *mut ::core::ffi::c_char,
        row_id,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_worker_enqueue_destination(
    mut event: *const event_envelope_t,
    mut destination: *const ::core::ffi::c_char,
    mut topic_template: *const ::core::ffi::c_char,
    mut row_id: *mut int64_t,
) -> event_outbox_enqueue_result_t {
    if !row_id.is_null() {
        *row_id = 0 as int64_t;
    }
    if event.is_null()
        || !valid_destination_key(destination)
        || strlen(destination) >= EVENT_OUTBOX_DESTINATION_MAX as size_t
        || topic_template.is_null()
        || *topic_template.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        increment(&raw mut WORKER.stats.enqueue_errors, 1 as uint64_t);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    let mut topic: [::core::ffi::c_char; 512] = [0; 512];
    if mqtt_delivery_topic_expand(
        topic_template,
        event,
        &raw mut topic as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        increment(&raw mut WORKER.stats.enqueue_errors, 1 as uint64_t);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    let mut shed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut result: event_outbox_enqueue_result_t = db_event_outbox_enqueue(
        event,
        destination,
        &raw mut topic as *mut ::core::ffi::c_char,
        ::core::ptr::null::<event_outbox_limits_t>(),
        row_id,
        &raw mut shed,
    );
    pthread_mutex_lock(&raw mut WORKER.mutex);
    if result.0 == event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED.0 {
        WORKER.stats.enqueued = WORKER.stats.enqueued.wrapping_add(1);
        WORKER.stats.priority_shed = WORKER.stats.priority_shed.wrapping_add(shed as uint64_t);
        pthread_cond_signal(&raw mut WORKER.wake);
    } else if result.0 == event_outbox_enqueue_result_t::EVENT_OUTBOX_DUPLICATE.0 {
        WORKER.stats.duplicates = WORKER.stats.duplicates.wrapping_add(1);
    } else if result.0 == event_outbox_enqueue_result_t::EVENT_OUTBOX_FULL.0 {
        WORKER.stats.rejected_full = WORKER.stats.rejected_full.wrapping_add(1);
    } else {
        WORKER.stats.enqueue_errors = WORKER.stats.enqueue_errors.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut WORKER.mutex);
    return result;
}
unsafe extern "C" fn process_destination_once(
    mut destination: *const ::core::ffi::c_char,
    mut now: int64_t,
    mut broker_connected: bool,
    mut publish: mqtt_delivery_publish_fn,
    mut context: *mut ::core::ffi::c_void,
    mut expire_first: bool,
) -> ::core::ffi::c_int {
    if !valid_destination_key(destination) || publish.is_none() {
        return -1 as ::core::ffi::c_int;
    }
    if now <= 0 as int64_t {
        now = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    }
    if expire_first {
        let mut expired: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if db_event_outbox_expire(now, &raw mut expired) != 0 as ::core::ffi::c_int {
            increment(&raw mut WORKER.stats.outcome_errors, 1 as uint64_t);
            return -1 as ::core::ffi::c_int;
        }
        if expired > 0 as ::core::ffi::c_int {
            increment(&raw mut WORKER.stats.expired, expired as uint64_t);
        }
    }
    if !broker_connected {
        increment(&raw mut WORKER.stats.disconnected_polls, 1 as uint64_t);
        return 0 as ::core::ffi::c_int;
    }
    let mut item: event_outbox_item_t = event_outbox_item_t {
        row_id: 0,
        event_id: [0; 37],
        event_source: [0; 128],
        event_type: [0; 96],
        subject: [0; 96],
        destination: [0; 64],
        topic: [0; 512],
        envelope_json: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        envelope_bytes: 0,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        attempt_count: 0,
        next_attempt_at: 0,
        expires_at: 0,
        created_at: 0,
    };
    let mut claim: ::core::ffi::c_int = db_event_outbox_claim_due(
        destination,
        now,
        EVENT_OUTBOX_DEFAULT_LEASE_SECONDS,
        &raw mut item,
    );
    if claim <= 0 as ::core::ffi::c_int {
        if claim < 0 as ::core::ffi::c_int {
            increment(&raw mut WORKER.stats.outcome_errors, 1 as uint64_t);
        }
        return claim;
    }
    increment(&raw mut WORKER.stats.attempted, 1 as uint64_t);
    let mut publish_result: ::core::ffi::c_int = publish.expect("non-null function pointer")(
        &raw mut item.topic as *mut ::core::ffi::c_char,
        item.envelope_json,
        r#false != 0,
        MQTT_DELIVERY_ACK_TIMEOUT_MS,
        context,
    );
    let mut outcome_at: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    if outcome_at < now {
        outcome_at = now;
    }
    let mut result: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if publish_result == 0 as ::core::ffi::c_int {
        if db_event_outbox_mark_delivered(item.row_id, outcome_at) != 0 as ::core::ffi::c_int {
            increment(&raw mut WORKER.stats.outcome_errors, 1 as uint64_t);
            result = -1 as ::core::ffi::c_int;
        } else {
            increment(&raw mut WORKER.stats.delivered, 1 as uint64_t);
        }
    } else {
        let mut delay: ::core::ffi::c_int =
            mqtt_delivery_backoff_seconds(item.attempt_count, event_jitter_seed(&raw mut item));
        let mut next_attempt_at: int64_t = outcome_at + delay as int64_t;
        if next_attempt_at >= item.expires_at {
            if db_event_outbox_mark_dead(
                item.row_id,
                outcome_at,
                b"MQTT retry would exceed event expiry\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                increment(&raw mut WORKER.stats.outcome_errors, 1 as uint64_t);
                result = -1 as ::core::ffi::c_int;
            } else {
                increment(&raw mut WORKER.stats.dead, 1 as uint64_t);
            }
        } else if db_event_outbox_mark_retry(
            item.row_id,
            next_attempt_at,
            b"MQTT publish was not acknowledged\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            increment(&raw mut WORKER.stats.outcome_errors, 1 as uint64_t);
            result = -1 as ::core::ffi::c_int;
        } else {
            increment(&raw mut WORKER.stats.retried, 1 as uint64_t);
        }
    }
    db_event_outbox_item_clear(&raw mut item);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_worker_process_destination_once(
    mut destination: *const ::core::ffi::c_char,
    mut now: int64_t,
    mut broker_connected: bool,
    mut publish: mqtt_delivery_publish_fn,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return process_destination_once(
        destination,
        now,
        broker_connected,
        publish,
        context,
        r#true != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_worker_process_once(
    mut now: int64_t,
    mut broker_connected: bool,
    mut publish: mqtt_delivery_publish_fn,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return mqtt_delivery_worker_process_destination_once(
        MQTT_EVENT_OUTBOX_DESTINATION.as_ptr(),
        now,
        broker_connected,
        publish,
        context,
    );
}
static mut MANAGED_SLOTS: *mut managed_destination_slot_t =
    ::core::ptr::null_mut::<managed_destination_slot_t>();
static mut MANAGED_SLOT_COUNT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut MANAGED_GENERATION: uint64_t = 0 as uint64_t;
unsafe extern "C" fn mqtt_confirmed_publish(
    mut topic: *const ::core::ffi::c_char,
    mut payload: *const ::core::ffi::c_char,
    mut retain: bool,
    mut timeout_ms: ::core::ffi::c_int,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return mqtt_publish_raw_confirmed(topic, payload, retain, timeout_ms);
}
unsafe extern "C" fn managed_confirmed_publish(
    mut topic: *const ::core::ffi::c_char,
    mut payload: *const ::core::ffi::c_char,
    mut retain: bool,
    mut timeout_ms: ::core::ffi::c_int,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return mqtt_destination_client_publish_confirmed(
        context as *mut mqtt_destination_client_t,
        topic,
        payload,
        retain,
        timeout_ms,
    );
}
unsafe extern "C" fn clear_managed_slots() {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < MANAGED_SLOT_COUNT {
        mqtt_destination_client_destroy((*MANAGED_SLOTS.offset(index as isize)).client);
        index += 1;
    }
    free(MANAGED_SLOTS as *mut ::core::ffi::c_void);
    MANAGED_SLOTS = ::core::ptr::null_mut::<managed_destination_slot_t>();
    MANAGED_SLOT_COUNT = 0 as ::core::ffi::c_int;
    MANAGED_GENERATION = 0 as uint64_t;
    pthread_mutex_lock(&raw mut WORKER.mutex);
    WORKER.stats.managed_profiles = 0 as uint64_t;
    WORKER.stats.managed_connected = 0 as uint64_t;
    pthread_mutex_unlock(&raw mut WORKER.mutex);
}
unsafe extern "C" fn count_profile_error() {
    increment(&raw mut WORKER.stats.profile_errors, 1 as uint64_t);
}
unsafe extern "C" fn create_managed_client(
    mut slot: *mut managed_destination_slot_t,
    mut now: int64_t,
) {
    if slot.is_null() || !(*slot).client.is_null() || now < (*slot).next_create_attempt_at {
        return;
    }
    (*slot).client = mqtt_destination_client_create(&raw mut (*slot).profile);
    (*slot).next_create_attempt_at = now + MQTT_DESTINATION_CREATE_RETRY_SECONDS as int64_t;
    if (*slot).client.is_null() {
        count_profile_error();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Managed MQTT destination '%s' will retry initialization\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*slot).profile.name as *mut ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn reload_managed_slots(
    mut generation: uint64_t,
    mut now: int64_t,
) -> ::core::ffi::c_int {
    let mut total: ::core::ffi::c_int = db_event_destination_count();
    if total < 0 as ::core::ffi::c_int || total > EVENT_DESTINATION_MAX_COUNT {
        return -1 as ::core::ffi::c_int;
    }
    let mut profiles: *mut event_destination_t = (if total > 0 as ::core::ffi::c_int {
        calloc(
            total as size_t,
            ::core::mem::size_of::<event_destination_t>(),
        )
    } else {
        NULL
    }) as *mut event_destination_t;
    if total > 0 as ::core::ffi::c_int && profiles.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int {
        db_event_destination_list(profiles, total)
    } else {
        0 as ::core::ffi::c_int
    };
    if count < 0 as ::core::ffi::c_int {
        free(profiles as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut enabled_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        if (*profiles.offset(index as isize)).enabled {
            enabled_count += 1;
        }
        index += 1;
    }
    let mut slots: *mut managed_destination_slot_t = (if enabled_count > 0 as ::core::ffi::c_int {
        calloc(
            enabled_count as size_t,
            ::core::mem::size_of::<managed_destination_slot_t>(),
        )
    } else {
        NULL
    }) as *mut managed_destination_slot_t;
    if enabled_count > 0 as ::core::ffi::c_int && slots.is_null() {
        free(profiles as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut slot_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut invalid_profile: bool = r#false != 0;
    let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_0 < count {
        if (*profiles.offset(index_0 as isize)).enabled {
            let c2rust_fresh0 = slot_index;
            slot_index += 1;
            let mut slot: *mut managed_destination_slot_t = slots.offset(c2rust_fresh0 as isize);
            (*slot).profile = *profiles.offset(index_0 as isize);
            if db_event_destination_make_key(
                &raw mut (*profiles.offset(index_0 as isize)).uuid as *mut ::core::ffi::c_char,
                &raw mut (*slot).destination_key as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                invalid_profile = r#true != 0;
            }
        }
        index_0 += 1;
    }
    free(profiles as *mut ::core::ffi::c_void);
    if invalid_profile {
        free(slots as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    clear_managed_slots();
    MANAGED_SLOTS = slots;
    MANAGED_SLOT_COUNT = enabled_count;
    MANAGED_GENERATION = generation;
    let mut index_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_1 < MANAGED_SLOT_COUNT {
        create_managed_client(MANAGED_SLOTS.offset(index_1 as isize), now);
        index_1 += 1;
    }
    pthread_mutex_lock(&raw mut WORKER.mutex);
    WORKER.stats.managed_profiles = MANAGED_SLOT_COUNT as uint64_t;
    WORKER.stats.profile_reloads = WORKER.stats.profile_reloads.wrapping_add(1);
    pthread_mutex_unlock(&raw mut WORKER.mutex);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ensure_managed_slots(mut now: int64_t) -> bool {
    let mut generation: uint64_t = db_event_destination_generation();
    if MANAGED_GENERATION != generation
        && reload_managed_slots(generation, now) != 0 as ::core::ffi::c_int
    {
        count_profile_error();
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn update_managed_connected_stats() {
    let mut connected: uint64_t = 0 as uint64_t;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < MANAGED_SLOT_COUNT {
        if mqtt_destination_client_is_connected((*MANAGED_SLOTS.offset(index as isize)).client) {
            connected = connected.wrapping_add(1);
        }
        index += 1;
    }
    pthread_mutex_lock(&raw mut WORKER.mutex);
    WORKER.stats.managed_connected = connected;
    pthread_mutex_unlock(&raw mut WORKER.mutex);
}
unsafe extern "C" fn stop_requested() -> bool {
    pthread_mutex_lock(&raw mut WORKER.mutex);
    let mut stop: bool = WORKER.stop_requested;
    pthread_mutex_unlock(&raw mut WORKER.mutex);
    return stop;
}
unsafe extern "C" fn wait_for_work() {
    let mut deadline: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut deadline);
    deadline.tv_sec += MQTT_DELIVERY_POLL_SECONDS as __time_t;
    pthread_mutex_lock(&raw mut WORKER.mutex);
    if !WORKER.stop_requested {
        let mut rc: ::core::ffi::c_int = pthread_cond_timedwait(
            &raw mut WORKER.wake,
            &raw mut WORKER.mutex,
            &raw mut deadline,
        );
    }
    pthread_mutex_unlock(&raw mut WORKER.mutex);
}
unsafe extern "C" fn maintain_outbox(
    mut now: int64_t,
    mut last_expire_at: *mut int64_t,
    mut last_prune_at: *mut int64_t,
) -> bool {
    if now < *last_expire_at
        || now - *last_expire_at >= MQTT_DELIVERY_EXPIRE_INTERVAL_SECONDS as int64_t
    {
        let mut expired: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if db_event_outbox_expire(now, &raw mut expired) != 0 as ::core::ffi::c_int {
            increment(&raw mut WORKER.stats.outcome_errors, 1 as uint64_t);
            return r#false != 0;
        }
        *last_expire_at = now;
        if expired > 0 as ::core::ffi::c_int {
            increment(&raw mut WORKER.stats.expired, expired as uint64_t);
        }
    }
    if now as ::core::ffi::c_longlong > EVENT_OUTBOX_TERMINAL_RETENTION_SECONDS
        && (now < *last_prune_at
            || now - *last_prune_at >= MQTT_DELIVERY_PRUNE_INTERVAL_SECONDS as int64_t)
    {
        let mut deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if db_event_outbox_prune_terminal(
            (now as ::core::ffi::c_longlong - EVENT_OUTBOX_TERMINAL_RETENTION_SECONDS) as int64_t,
            MQTT_DELIVERY_PRUNE_BATCH,
            &raw mut deleted,
        ) != 0 as ::core::ffi::c_int
        {
            increment(&raw mut WORKER.stats.outcome_errors, 1 as uint64_t);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Could not prune delivered event outbox rows\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            increment(&raw mut WORKER.stats.pruned, deleted as uint64_t);
            if deleted > 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Pruned %d terminal event outbox rows\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    deleted,
                );
            }
        }
        *last_prune_at = if deleted >= MQTT_DELIVERY_PRUNE_BATCH {
            0 as int64_t
        } else {
            now
        };
    }
    return r#true != 0;
}
unsafe extern "C" fn mqtt_delivery_thread(
    mut unused: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"MQTTDelivery\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut last_expire_at: int64_t = 0 as int64_t;
    let mut last_prune_at: int64_t = 0 as int64_t;
    while !stop_requested() {
        let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
        let mut processed_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !ensure_managed_slots(now) {
            wait_for_work();
        } else if !maintain_outbox(now, &raw mut last_expire_at, &raw mut last_prune_at) {
            wait_for_work();
        } else {
            let mut processed: ::core::ffi::c_int = process_destination_once(
                MQTT_EVENT_OUTBOX_DESTINATION.as_ptr(),
                now,
                mqtt_is_connected(),
                Some(
                    mqtt_confirmed_publish
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            bool,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                NULL,
                r#false != 0,
            );
            if processed > 0 as ::core::ffi::c_int {
                processed_count += processed;
            }
            let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while index < MANAGED_SLOT_COUNT {
                let mut slot: *mut managed_destination_slot_t =
                    MANAGED_SLOTS.offset(index as isize);
                create_managed_client(slot, now);
                processed = process_destination_once(
                    &raw mut (*slot).destination_key as *mut ::core::ffi::c_char,
                    now,
                    mqtt_destination_client_is_connected((*slot).client),
                    Some(
                        managed_confirmed_publish
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                bool,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    (*slot).client as *mut ::core::ffi::c_void,
                    r#false != 0,
                );
                if processed > 0 as ::core::ffi::c_int {
                    processed_count += processed;
                }
                index += 1;
            }
            update_managed_connected_stats();
            if processed_count == 0 as ::core::ffi::c_int {
                wait_for_work();
            }
        }
    }
    clear_managed_slots();
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_worker_start() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut WORKER.mutex);
    while WORKER.shutdown_in_progress {
        pthread_cond_wait(&raw mut WORKER.wake, &raw mut WORKER.mutex);
    }
    if WORKER.thread_started {
        pthread_mutex_unlock(&raw mut WORKER.mutex);
        return 0 as ::core::ffi::c_int;
    }
    WORKER.stop_requested = r#false != 0;
    if pthread_create(
        &raw mut WORKER.thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            mqtt_delivery_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut WORKER.mutex);
        return -1 as ::core::ffi::c_int;
    }
    WORKER.thread_started = r#true != 0;
    WORKER.stats.running = r#true != 0;
    pthread_mutex_unlock(&raw mut WORKER.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT durable delivery worker started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_worker_shutdown() {
    pthread_mutex_lock(&raw mut WORKER.mutex);
    while WORKER.shutdown_in_progress {
        pthread_cond_wait(&raw mut WORKER.wake, &raw mut WORKER.mutex);
    }
    if !WORKER.thread_started {
        WORKER.stats.running = r#false != 0;
        pthread_mutex_unlock(&raw mut WORKER.mutex);
        return;
    }
    WORKER.shutdown_in_progress = r#true != 0;
    WORKER.stop_requested = r#true != 0;
    pthread_cond_broadcast(&raw mut WORKER.wake);
    let mut thread: pthread_t = WORKER.thread;
    pthread_mutex_unlock(&raw mut WORKER.mutex);
    pthread_join(thread, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    pthread_mutex_lock(&raw mut WORKER.mutex);
    WORKER.thread_started = r#false != 0;
    WORKER.shutdown_in_progress = r#false != 0;
    WORKER.stats.running = r#false != 0;
    pthread_cond_broadcast(&raw mut WORKER.wake);
    pthread_mutex_unlock(&raw mut WORKER.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT durable delivery worker stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_delivery_worker_get_stats(
    mut stats: *mut mqtt_delivery_worker_stats_t,
) {
    if stats.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut WORKER.mutex);
    *stats = WORKER.stats;
    pthread_mutex_unlock(&raw mut WORKER.mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
