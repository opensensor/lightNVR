unsafe extern "C" {
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
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
}
pub type size_t = usize;
pub type int64_t = i64;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint64_t = u64;
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
pub struct mqtt_presence_runtime_t {
    pub lock: pthread_mutex_t,
    pub configured: bool,
    pub connected: bool,
    pub ever_connected: bool,
    pub topic: [::core::ffi::c_char; 512],
    pub installation_uuid: [::core::ffi::c_char; 37],
    pub run_id: [::core::ffi::c_char; 37],
    pub boot_id: [::core::ffi::c_char; 64],
    pub visibility: [::core::ffi::c_char; 96],
    pub version: [::core::ffi::c_char; 32],
    pub sequence: uint64_t,
    pub connections: uint64_t,
    pub reconnects: uint64_t,
    pub disconnects: uint64_t,
    pub publish_attempts: uint64_t,
    pub publish_successes: uint64_t,
    pub publish_failures: uint64_t,
    pub last_publish_at_ms: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const MQTT_PRESENCE_TOPIC_MAX: ::core::ffi::c_uint = 512 as ::core::ffi::c_uint;
static mut runtime: mqtt_presence_runtime_t = mqtt_presence_runtime_t {
    lock: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0 as ::core::ffi::c_int,
            __count: 0 as ::core::ffi::c_uint,
            __owner: 0 as ::core::ffi::c_int,
            __nusers: 0 as ::core::ffi::c_uint,
            __kind: C2Rust_Unnamed::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
            __spins: 0 as ::core::ffi::c_short,
            __glibc_reserved: 0 as ::core::ffi::c_short,
            __list: __pthread_internal_list {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    configured: false,
    connected: false,
    ever_connected: false,
    topic: [0; 512],
    installation_uuid: [0; 37],
    run_id: [0; 37],
    boot_id: [0; 64],
    visibility: [0; 96],
    version: [0; 32],
    sequence: 0,
    connections: 0,
    reconnects: 0,
    disconnects: 0,
    publish_attempts: 0,
    publish_successes: 0,
    publish_failures: 0,
    last_publish_at_ms: 0,
};
unsafe extern "C" fn safe_token(
    mut value: *const ::core::ffi::c_char,
    mut capacity: size_t,
) -> bool {
    if value.is_null() || *value.offset(0isize) == 0 || strlen(value) >= capacity {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if (*cursor as ::core::ffi::c_uint) < 0x20 as ::core::ffi::c_uint
            || *cursor as ::core::ffi::c_uint == 0x7f as ::core::ffi::c_uint
            || *cursor as ::core::ffi::c_int == '"' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn safe_topic_prefix(mut prefix: *const ::core::ffi::c_char) -> bool {
    if !safe_token(
        prefix,
        MQTT_PRESENCE_TOPIC_MAX.wrapping_sub(48 as ::core::ffi::c_uint) as size_t,
    ) {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_char = prefix;
    while *cursor != 0 {
        if *cursor as ::core::ffi::c_int == '#' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '+' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
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
unsafe extern "C" fn increment_saturated(mut value: uint64_t) -> uint64_t {
    return if value == UINT64_MAX as uint64_t {
        value
    } else {
        value.wrapping_add(1 as uint64_t)
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_configure(
    mut topic_prefix: *const ::core::ffi::c_char,
    mut installation_uuid: *const ::core::ffi::c_char,
    mut run_id: *const ::core::ffi::c_char,
    mut boot_id: *const ::core::ffi::c_char,
    mut visibility_scope: *const ::core::ffi::c_char,
    mut version: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !safe_topic_prefix(topic_prefix)
        || !lightnvr_uuid_is_valid(installation_uuid)
        || !lightnvr_uuid_is_valid(run_id)
        || !safe_token(boot_id, ::core::mem::size_of::<[::core::ffi::c_char; 64]>())
        || !safe_token(
            visibility_scope,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        )
        || !safe_token(version, ::core::mem::size_of::<[::core::ffi::c_char; 32]>())
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut prefix_length: size_t = strlen(topic_prefix);
    while prefix_length > 0 as size_t
        && *topic_prefix.offset(prefix_length.wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        prefix_length = prefix_length.wrapping_sub(1);
    }
    if prefix_length == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut topic: [::core::ffi::c_char; 512] = [0; 512];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%.*s/v1/status/%s\0".as_ptr() as *const ::core::ffi::c_char,
        prefix_length as ::core::ffi::c_int,
        topic_prefix,
        installation_uuid,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut runtime.lock);
    let mut same_run: bool = runtime.configured as ::core::ffi::c_int != 0
        && strcmp(&raw mut runtime.run_id as *mut ::core::ffi::c_char, run_id)
            == 0 as ::core::ffi::c_int;
    if !same_run {
        runtime.connected = r#false != 0;
        runtime.ever_connected = r#false != 0;
        runtime.sequence = 0 as uint64_t;
        runtime.connections = 0 as uint64_t;
        runtime.reconnects = 0 as uint64_t;
        runtime.disconnects = 0 as uint64_t;
        runtime.publish_attempts = 0 as uint64_t;
        runtime.publish_successes = 0 as uint64_t;
        runtime.publish_failures = 0 as uint64_t;
        runtime.last_publish_at_ms = 0 as int64_t;
    }
    snprintf(
        &raw mut runtime.topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut topic as *mut ::core::ffi::c_char,
    );
    snprintf(
        &raw mut runtime.installation_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        installation_uuid,
    );
    snprintf(
        &raw mut runtime.run_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        run_id,
    );
    snprintf(
        &raw mut runtime.boot_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        boot_id,
    );
    snprintf(
        &raw mut runtime.visibility as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        visibility_scope,
    );
    snprintf(
        &raw mut runtime.version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        version,
    );
    runtime.configured = r#true != 0;
    pthread_mutex_unlock(&raw mut runtime.lock);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn build_locked(
    mut state: mqtt_presence_state_t,
    mut operational_state: mqtt_operational_state_t,
    mut active_incidents: size_t,
    mut observed_at_ms: int64_t,
    mut sequence: uint64_t,
    mut topic: *mut ::core::ffi::c_char,
    mut topic_size: size_t,
    mut payload: *mut ::core::ffi::c_char,
    mut payload_size: size_t,
) -> ::core::ffi::c_int {
    let mut state_name: *const ::core::ffi::c_char = presence_state_name(state);
    let mut operational_name: *const ::core::ffi::c_char =
        operational_state_name(operational_state);
    if !runtime.configured
        || state_name.is_null()
        || operational_name.is_null()
        || observed_at_ms <= 0 as int64_t
        || topic.is_null()
        || topic_size == 0 as size_t
        || payload.is_null()
        || payload_size == 0 as size_t
        || strlen(&raw mut runtime.topic as *mut ::core::ffi::c_char) >= topic_size
    {
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        topic,
        topic_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut runtime.topic as *mut ::core::ffi::c_char,
    );
    let mut written: ::core::ffi::c_int = snprintf(
        payload,
        payload_size,
        b"{\"schema_version\":1,\"installation_uuid\":\"%s\",\"run_id\":\"%s\",\"boot_id\":\"%s\",\"visibility_scope\":\"%s\",\"version\":\"%s\",\"sequence\":%lu,\"timestamp_ms\":%ld,\"state\":\"%s\",\"overall_state\":\"%s\",\"active_incidents\":%zu}\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut runtime.installation_uuid as *mut ::core::ffi::c_char,
        &raw mut runtime.run_id as *mut ::core::ffi::c_char,
        &raw mut runtime.boot_id as *mut ::core::ffi::c_char,
        &raw mut runtime.visibility as *mut ::core::ffi::c_char,
        &raw mut runtime.version as *mut ::core::ffi::c_char,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_build_will(
    mut configured_at_ms: int64_t,
    mut topic: *mut ::core::ffi::c_char,
    mut topic_size: size_t,
    mut payload: *mut ::core::ffi::c_char,
    mut payload_size: size_t,
) -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut runtime.lock);
    let mut result: ::core::ffi::c_int = build_locked(
        mqtt_presence_state_t::MQTT_PRESENCE_OFFLINE,
        mqtt_operational_state_t::MQTT_OPERATIONAL_UNKNOWN,
        0 as size_t,
        configured_at_ms,
        0 as uint64_t,
        topic,
        topic_size,
        payload,
        payload_size,
    );
    pthread_mutex_unlock(&raw mut runtime.lock);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_build(
    mut state: mqtt_presence_state_t,
    mut operational_state: mqtt_operational_state_t,
    mut active_incidents: size_t,
    mut observed_at_ms: int64_t,
    mut topic: *mut ::core::ffi::c_char,
    mut topic_size: size_t,
    mut payload: *mut ::core::ffi::c_char,
    mut payload_size: size_t,
) -> ::core::ffi::c_int {
    if state.0 != mqtt_presence_state_t::MQTT_PRESENCE_ONLINE.0
        && state.0 != mqtt_presence_state_t::MQTT_PRESENCE_STOPPING.0
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut runtime.lock);
    let mut next: uint64_t = increment_saturated(runtime.sequence);
    let mut result: ::core::ffi::c_int = build_locked(
        state,
        operational_state,
        active_incidents,
        observed_at_ms,
        next,
        topic,
        topic_size,
        payload,
        payload_size,
    );
    if result == 0 as ::core::ffi::c_int {
        runtime.sequence = next;
    }
    pthread_mutex_unlock(&raw mut runtime.lock);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_record_connected() {
    pthread_mutex_lock(&raw mut runtime.lock);
    runtime.connections = increment_saturated(runtime.connections);
    if runtime.ever_connected {
        runtime.reconnects = increment_saturated(runtime.reconnects);
    }
    runtime.ever_connected = r#true != 0;
    runtime.connected = r#true != 0;
    pthread_mutex_unlock(&raw mut runtime.lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_record_disconnected() {
    pthread_mutex_lock(&raw mut runtime.lock);
    if runtime.connected {
        runtime.disconnects = increment_saturated(runtime.disconnects);
    }
    runtime.connected = r#false != 0;
    pthread_mutex_unlock(&raw mut runtime.lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_record_publish(
    mut succeeded: bool,
    mut observed_at_ms: int64_t,
) {
    pthread_mutex_lock(&raw mut runtime.lock);
    runtime.publish_attempts = increment_saturated(runtime.publish_attempts);
    if succeeded {
        runtime.publish_successes = increment_saturated(runtime.publish_successes);
    } else {
        runtime.publish_failures = increment_saturated(runtime.publish_failures);
    }
    if observed_at_ms > 0 as int64_t {
        runtime.last_publish_at_ms = observed_at_ms;
    }
    pthread_mutex_unlock(&raw mut runtime.lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_get_stats(mut stats: *mut mqtt_presence_stats_t) {
    if stats.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut runtime.lock);
    (*stats).configured = runtime.configured;
    (*stats).connected = runtime.connected;
    (*stats).sequence = runtime.sequence;
    (*stats).connections = runtime.connections;
    (*stats).reconnects = runtime.reconnects;
    (*stats).disconnects = runtime.disconnects;
    (*stats).publish_attempts = runtime.publish_attempts;
    (*stats).publish_successes = runtime.publish_successes;
    (*stats).publish_failures = runtime.publish_failures;
    (*stats).last_publish_at_ms = runtime.last_publish_at_ms;
    pthread_mutex_unlock(&raw mut runtime.lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_copy_topic(
    mut topic: *mut ::core::ffi::c_char,
    mut topic_size: size_t,
) -> ::core::ffi::c_int {
    if topic.is_null() || topic_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut runtime.lock);
    if !runtime.configured
        || strlen(&raw mut runtime.topic as *mut ::core::ffi::c_char) >= topic_size
    {
        *topic.offset(0isize) = '\0' as ::core::ffi::c_char;
        pthread_mutex_unlock(&raw mut runtime.lock);
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        topic,
        topic_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut runtime.topic as *mut ::core::ffi::c_char,
    );
    pthread_mutex_unlock(&raw mut runtime.lock);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_copy_installation_uuid(
    mut uuid: *mut ::core::ffi::c_char,
    mut uuid_size: size_t,
) -> ::core::ffi::c_int {
    if uuid.is_null() || uuid_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut runtime.lock);
    if !runtime.configured
        || strlen(&raw mut runtime.installation_uuid as *mut ::core::ffi::c_char) >= uuid_size
    {
        *uuid.offset(0isize) = '\0' as ::core::ffi::c_char;
        pthread_mutex_unlock(&raw mut runtime.lock);
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        uuid,
        uuid_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut runtime.installation_uuid as *mut ::core::ffi::c_char,
    );
    pthread_mutex_unlock(&raw mut runtime.lock);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_presence_reset() {
    pthread_mutex_lock(&raw mut runtime.lock);
    runtime.configured = r#false != 0;
    runtime.connected = r#false != 0;
    runtime.ever_connected = r#false != 0;
    memset(
        &raw mut runtime.topic as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    memset(
        &raw mut runtime.installation_uuid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    memset(
        &raw mut runtime.run_id as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    memset(
        &raw mut runtime.boot_id as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    memset(
        &raw mut runtime.visibility as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
    );
    memset(
        &raw mut runtime.version as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    runtime.sequence = 0 as uint64_t;
    runtime.connections = 0 as uint64_t;
    runtime.reconnects = 0 as uint64_t;
    runtime.disconnects = 0 as uint64_t;
    runtime.publish_attempts = 0 as uint64_t;
    runtime.publish_successes = 0 as uint64_t;
    runtime.publish_failures = 0 as uint64_t;
    runtime.last_publish_at_ms = 0 as int64_t;
    pthread_mutex_unlock(&raw mut runtime.lock);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
