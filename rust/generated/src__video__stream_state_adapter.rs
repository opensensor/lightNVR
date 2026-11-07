unsafe extern "C" {
    pub type stream_handle_s;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn create_stream_state(config: *const stream_config_t) -> *mut stream_state_manager_t;
    unsafe fn get_stream_state_by_name(
        name: *const ::core::ffi::c_char,
    ) -> *mut stream_state_manager_t;
    unsafe fn update_stream_state_config(
        state: *mut stream_state_manager_t,
        config: *const stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn set_stream_feature(
        state: *mut stream_state_manager_t,
        feature: *const ::core::ffi::c_char,
        enabled: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn start_stream_with_state(state: *mut stream_state_manager_t) -> ::core::ffi::c_int;
    unsafe fn stop_stream_with_state(
        state: *mut stream_state_manager_t,
        wait_for_completion: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_operational_state(state: *mut stream_state_manager_t) -> stream_state_t;
    unsafe fn get_stream_statistics(
        state: *mut stream_state_manager_t,
        stats: *mut stream_stats_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_state_count() -> ::core::ffi::c_int;
    unsafe fn get_stream_state_by_index(index: ::core::ffi::c_int) -> *mut stream_state_manager_t;
    unsafe fn remove_stream_state(state: *mut stream_state_manager_t) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
pub type uint8_t = u8;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_config_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub url: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub priority: ::core::ffi::c_int,
    pub record: bool,
    pub segment_duration: ::core::ffi::c_int,
    pub detection_based_recording: bool,
    pub detection_model: [::core::ffi::c_char; 512],
    pub detection_interval: ::core::ffi::c_int,
    pub detection_threshold: ::core::ffi::c_float,
    pub pre_detection_buffer: ::core::ffi::c_int,
    pub post_detection_buffer: ::core::ffi::c_int,
    pub detection_api_url: [::core::ffi::c_char; 512],
    pub detection_object_filter: [::core::ffi::c_char; 16],
    pub detection_object_filter_list: [::core::ffi::c_char; 256],
    pub streaming_enabled: bool,
    pub protocol: stream_protocol_t,
    pub record_audio: bool,
    pub audio_voice_enhancement: bool,
    pub onvif_username: [::core::ffi::c_char; 64],
    pub onvif_password: [::core::ffi::c_char; 64],
    pub onvif_profile: [::core::ffi::c_char; 64],
    pub onvif_port: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub is_onvif: bool,
    pub backchannel_enabled: bool,
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: ::core::ffi::c_int,
    pub tier_critical_multiplier: ::core::ffi::c_double,
    pub tier_important_multiplier: ::core::ffi::c_double,
    pub tier_ephemeral_multiplier: ::core::ffi::c_double,
    pub storage_priority: ::core::ffi::c_int,
    pub ptz_enabled: bool,
    pub ptz_max_x: ::core::ffi::c_int,
    pub ptz_max_y: ::core::ffi::c_int,
    pub ptz_max_z: ::core::ffi::c_int,
    pub ptz_has_home: bool,
    pub record_on_schedule: bool,
    pub recording_schedule: [uint8_t; 168],
    pub detection_record_on_schedule: bool,
    pub detection_recording_schedule: [uint8_t; 168],
    pub tags: [::core::ffi::c_char; 256],
    pub admin_url: [::core::ffi::c_char; 512],
    pub privacy_mode: bool,
    pub motion_trigger_source: [::core::ffi::c_char; 256],
    pub go2rtc_source_override: [::core::ffi::c_char; 8192],
    pub sub_stream_url: [::core::ffi::c_char; 512],
    pub detection_url: [::core::ffi::c_char; 512],
    pub publish_url: [::core::ffi::c_char; 512],
    pub playback_transport: [::core::ffi::c_char; 24],
    pub eptz_config: [::core::ffi::c_char; 1024],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_status_t(pub ::core::ffi::c_uint);
impl stream_status_t {
    pub const STREAM_STATUS_UNKNOWN: Self = Self(0);
    pub const STREAM_STATUS_STOPPED: Self = Self(1);
    pub const STREAM_STATUS_STARTING: Self = Self(2);
    pub const STREAM_STATUS_RUNNING: Self = Self(3);
    pub const STREAM_STATUS_STOPPING: Self = Self(4);
    pub const STREAM_STATUS_ERROR: Self = Self(5);
    pub const STREAM_STATUS_RECONNECTING: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_stats_t {
    pub bytes_received: uint64_t,
    pub frames_received: uint64_t,
    pub frames_dropped: uint64_t,
    pub errors: uint64_t,
    pub reconnects: uint64_t,
    pub bitrate: ::core::ffi::c_double,
    pub fps: ::core::ffi::c_double,
    pub uptime: uint64_t,
    pub last_frame_time: uint64_t,
}
pub type stream_handle_t = *mut stream_handle_s;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_state_t(pub ::core::ffi::c_uint);
impl stream_state_t {
    pub const STREAM_STATE_INACTIVE: Self = Self(0);
    pub const STREAM_STATE_STARTING: Self = Self(1);
    pub const STREAM_STATE_ACTIVE: Self = Self(2);
    pub const STREAM_STATE_STOPPING: Self = Self(3);
    pub const STREAM_STATE_ERROR: Self = Self(4);
    pub const STREAM_STATE_RECONNECTING: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_features_t {
    pub streaming_enabled: bool,
    pub recording_enabled: bool,
    pub detection_enabled: bool,
    pub motion_detection_enabled: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_protocol_state_t {
    pub protocol: stream_protocol_t,
    pub is_multicast: bool,
    pub reconnect_attempts: ::core::ffi::c_int,
    pub last_reconnect_time: int64_t,
    pub buffer_size: ::core::ffi::c_int,
    pub timeout_ms: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_timestamp_state_t {
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub expected_next_pts: int64_t,
    pub pts_discontinuity_count: int64_t,
    pub timestamps_initialized: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_state_manager_t {
    pub name: [::core::ffi::c_char; 256],
    pub state: stream_state_t,
    pub features: stream_features_t,
    pub protocol_state: stream_protocol_state_t,
    pub timestamp_state: stream_timestamp_state_t,
    pub stats: stream_stats_t,
    pub config: stream_config_t,
    pub mutex: pthread_mutex_t,
    pub state_mutex: pthread_mutex_t,
    pub ref_count: ::core::ffi::c_int,
    pub component_refs: [::core::ffi::c_int; 6],
    pub reader_ctx: *mut ::core::ffi::c_void,
    pub hls_ctx: *mut ::core::ffi::c_void,
    pub mp4_ctx: *mut ::core::ffi::c_void,
    pub detection_ctx: *mut ::core::ffi::c_void,
    pub callbacks_enabled: bool,
    pub last_error_message: [::core::ffi::c_char; 256],
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
pub struct stream_handle_mapping_t {
    pub handle: stream_handle_t,
    pub state: *mut stream_state_manager_t,
}
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_HANDLE_MAPPINGS: ::core::ffi::c_int = MAX_STREAMS;
static mut handle_mappings: [stream_handle_mapping_t; 1024] = [stream_handle_mapping_t {
    handle: ::core::ptr::null_mut::<stream_handle_s>(),
    state: ::core::ptr::null_mut::<stream_state_manager_t>(),
}; 1024];
static mut mappings_mutex: pthread_mutex_t = pthread_mutex_t {
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
};
static mut adapter_initialized: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_stream_state_adapter() -> ::core::ffi::c_int {
    if adapter_initialized {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut mappings_mutex);
    memset(
        &raw mut handle_mappings as *mut stream_handle_mapping_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[stream_handle_mapping_t; 1024]>(),
    );
    adapter_initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut mappings_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream state adapter initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_stream_state_adapter() {
    if !adapter_initialized {
        return;
    }
    pthread_mutex_lock(&raw mut mappings_mutex);
    memset(
        &raw mut handle_mappings as *mut stream_handle_mapping_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[stream_handle_mapping_t; 1024]>(),
    );
    adapter_initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut mappings_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream state adapter shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn add_handle_mapping(
    mut handle: stream_handle_t,
    mut state: *mut stream_state_manager_t,
) -> ::core::ffi::c_int {
    if handle.is_null() || state.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut mappings_mutex);
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_HANDLE_MAPPINGS {
        if handle_mappings[i as usize].handle.is_null()
            && handle_mappings[i as usize].state.is_null()
        {
            slot = i;
            break;
        } else {
            i += 1;
        }
    }
    if slot == -1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut mappings_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No available slots for handle mapping\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    handle_mappings[slot as usize].handle = handle;
    handle_mappings[slot as usize].state = state;
    pthread_mutex_unlock(&raw mut mappings_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Added handle mapping for stream '%s' in slot %d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        slot,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn remove_handle_mapping_by_handle(
    mut handle: stream_handle_t,
) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut mappings_mutex);
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_HANDLE_MAPPINGS {
        if handle_mappings[i as usize].handle == handle {
            slot = i;
            break;
        } else {
            i += 1;
        }
    }
    if slot == -1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut mappings_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Handle mapping not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    handle_mappings[slot as usize].handle = ::core::ptr::null_mut::<stream_handle_s>();
    handle_mappings[slot as usize].state = ::core::ptr::null_mut::<stream_state_manager_t>();
    pthread_mutex_unlock(&raw mut mappings_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Removed handle mapping from slot %d\0".as_ptr() as *const ::core::ffi::c_char,
        slot,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_handle_to_state(
    mut handle: stream_handle_t,
) -> *mut stream_state_manager_t {
    if handle.is_null() || !adapter_initialized {
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    pthread_mutex_lock(&raw mut mappings_mutex);
    let mut state: *mut stream_state_manager_t = ::core::ptr::null_mut::<stream_state_manager_t>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_HANDLE_MAPPINGS {
        if handle_mappings[i as usize].handle == handle {
            state = handle_mappings[i as usize].state;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut mappings_mutex);
    return state;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_state_to_handle(
    mut state: *const stream_state_manager_t,
) -> stream_handle_t {
    if state.is_null() || !adapter_initialized {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    pthread_mutex_lock(&raw mut mappings_mutex);
    let mut handle: stream_handle_t = ::core::ptr::null_mut::<stream_handle_s>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_HANDLE_MAPPINGS {
        if handle_mappings[i as usize].state == state as *mut stream_state_manager_t {
            handle = handle_mappings[i as usize].handle;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut mappings_mutex);
    return handle;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_stream_adapter(mut config: *const stream_config_t) -> stream_handle_t {
    if config.is_null() || !adapter_initialized {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    let mut state: *mut stream_state_manager_t = create_stream_state(config);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create stream state for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).name as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    let mut handle: stream_handle_t = state as stream_handle_t;
    if add_handle_mapping(handle, state) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to add handle mapping for stream '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*config).name as *const ::core::ffi::c_char,
        );
        remove_stream_state(state);
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Added stream '%s' through adapter\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).name as *const ::core::ffi::c_char,
    );
    return handle;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_stream_adapter(mut handle: stream_handle_t) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in remove_stream_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    remove_handle_mapping_by_handle(handle);
    let mut result: ::core::ffi::c_int = remove_stream_state(state);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Removed stream '%s' through adapter\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_stream_adapter(mut handle: stream_handle_t) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in start_stream_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return start_stream_with_state(state);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_stream_adapter(mut handle: stream_handle_t) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in stop_stream_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return stop_stream_with_state(state, r#true != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_status_adapter(mut handle: stream_handle_t) -> stream_status_t {
    if handle.is_null() || !adapter_initialized {
        return stream_status_t::STREAM_STATUS_UNKNOWN;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in get_stream_status_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return stream_status_t::STREAM_STATUS_UNKNOWN;
    }
    let mut state_value: stream_state_t = get_stream_operational_state(state);
    match state_value {
        stream_state_t::STREAM_STATE_INACTIVE => {
            return stream_status_t::STREAM_STATUS_STOPPED;
        }
        stream_state_t::STREAM_STATE_STARTING => {
            return stream_status_t::STREAM_STATUS_STARTING;
        }
        stream_state_t::STREAM_STATE_ACTIVE => {
            return stream_status_t::STREAM_STATUS_RUNNING;
        }
        stream_state_t::STREAM_STATE_STOPPING => {
            return stream_status_t::STREAM_STATUS_STOPPING;
        }
        stream_state_t::STREAM_STATE_ERROR => return stream_status_t::STREAM_STATUS_ERROR,
        stream_state_t::STREAM_STATE_RECONNECTING => {
            return stream_status_t::STREAM_STATUS_RECONNECTING;
        }
        _ => return stream_status_t::STREAM_STATUS_UNKNOWN,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_stats_adapter(
    mut handle: stream_handle_t,
    mut stats: *mut stream_stats_t,
) -> ::core::ffi::c_int {
    if handle.is_null() || stats.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in get_stream_stats_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return get_stream_statistics(state, stats);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_config_adapter(
    mut handle: stream_handle_t,
    mut config: *mut stream_config_t,
) -> ::core::ffi::c_int {
    if handle.is_null() || config.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in get_stream_config_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    memcpy(
        config as *mut ::core::ffi::c_void,
        &raw mut (*state).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*state).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_priority_adapter(
    mut handle: stream_handle_t,
    mut priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in set_stream_priority_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut updated_config: stream_config_t = stream_config_t {
        camera_uuid: [0; 37],
        location_uuid: [0; 37],
        name: [0; 256],
        url: [0; 512],
        enabled: false,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        priority: 0,
        record: false,
        segment_duration: 0,
        detection_based_recording: false,
        detection_model: [0; 512],
        detection_interval: 0,
        detection_threshold: 0.,
        pre_detection_buffer: 0,
        post_detection_buffer: 0,
        detection_api_url: [0; 512],
        detection_object_filter: [0; 16],
        detection_object_filter_list: [0; 256],
        streaming_enabled: false,
        protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
        record_audio: false,
        audio_voice_enhancement: false,
        onvif_username: [0; 64],
        onvif_password: [0; 64],
        onvif_profile: [0; 64],
        onvif_port: 0,
        onvif_discovery_enabled: false,
        is_onvif: false,
        backchannel_enabled: false,
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
        tier_critical_multiplier: 0.,
        tier_important_multiplier: 0.,
        tier_ephemeral_multiplier: 0.,
        storage_priority: 0,
        ptz_enabled: false,
        ptz_max_x: 0,
        ptz_max_y: 0,
        ptz_max_z: 0,
        ptz_has_home: false,
        record_on_schedule: false,
        recording_schedule: [0; 168],
        detection_record_on_schedule: false,
        detection_recording_schedule: [0; 168],
        tags: [0; 256],
        admin_url: [0; 512],
        privacy_mode: false,
        motion_trigger_source: [0; 256],
        go2rtc_source_override: [0; 8192],
        sub_stream_url: [0; 512],
        detection_url: [0; 512],
        publish_url: [0; 512],
        playback_transport: [0; 24],
        eptz_config: [0; 1024],
    };
    pthread_mutex_lock(&raw mut (*state).mutex);
    memcpy(
        &raw mut updated_config as *mut ::core::ffi::c_void,
        &raw mut (*state).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*state).mutex);
    updated_config.priority = priority;
    update_stream_state_config(state, &raw mut updated_config);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set priority for stream '%s' to %d through adapter\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        priority,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_recording_adapter(
    mut handle: stream_handle_t,
    mut enable: bool,
) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in set_stream_recording_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return set_stream_feature(
        state,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        enable,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_detection_recording_adapter(
    mut handle: stream_handle_t,
    mut enable: bool,
    mut model_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in set_stream_detection_recording_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !model_path.is_null() {
        let mut updated_config: stream_config_t = stream_config_t {
            camera_uuid: [0; 37],
            location_uuid: [0; 37],
            name: [0; 256],
            url: [0; 512],
            enabled: false,
            width: 0,
            height: 0,
            fps: 0,
            codec: [0; 16],
            priority: 0,
            record: false,
            segment_duration: 0,
            detection_based_recording: false,
            detection_model: [0; 512],
            detection_interval: 0,
            detection_threshold: 0.,
            pre_detection_buffer: 0,
            post_detection_buffer: 0,
            detection_api_url: [0; 512],
            detection_object_filter: [0; 16],
            detection_object_filter_list: [0; 256],
            streaming_enabled: false,
            protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
            record_audio: false,
            audio_voice_enhancement: false,
            onvif_username: [0; 64],
            onvif_password: [0; 64],
            onvif_profile: [0; 64],
            onvif_port: 0,
            onvif_discovery_enabled: false,
            is_onvif: false,
            backchannel_enabled: false,
            retention_days: 0,
            detection_retention_days: 0,
            max_storage_mb: 0,
            tier_critical_multiplier: 0.,
            tier_important_multiplier: 0.,
            tier_ephemeral_multiplier: 0.,
            storage_priority: 0,
            ptz_enabled: false,
            ptz_max_x: 0,
            ptz_max_y: 0,
            ptz_max_z: 0,
            ptz_has_home: false,
            record_on_schedule: false,
            recording_schedule: [0; 168],
            detection_record_on_schedule: false,
            detection_recording_schedule: [0; 168],
            tags: [0; 256],
            admin_url: [0; 512],
            privacy_mode: false,
            motion_trigger_source: [0; 256],
            go2rtc_source_override: [0; 8192],
            sub_stream_url: [0; 512],
            detection_url: [0; 512],
            publish_url: [0; 512],
            playback_transport: [0; 24],
            eptz_config: [0; 1024],
        };
        pthread_mutex_lock(&raw mut (*state).mutex);
        memcpy(
            &raw mut updated_config as *mut ::core::ffi::c_void,
            &raw mut (*state).config as *const ::core::ffi::c_void,
            ::core::mem::size_of::<stream_config_t>(),
        );
        pthread_mutex_unlock(&raw mut (*state).mutex);
        safe_strcpy(
            &raw mut updated_config.detection_model as *mut ::core::ffi::c_char,
            model_path,
            MAX_PATH_LENGTH as size_t,
            0 as size_t,
        );
        update_stream_state_config(state, &raw mut updated_config);
    }
    return set_stream_feature(
        state,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
        enable,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_detection_params_adapter(
    mut handle: stream_handle_t,
    mut interval: ::core::ffi::c_int,
    mut threshold: ::core::ffi::c_float,
    mut pre_buffer: ::core::ffi::c_int,
    mut post_buffer: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in set_stream_detection_params_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut updated_config: stream_config_t = stream_config_t {
        camera_uuid: [0; 37],
        location_uuid: [0; 37],
        name: [0; 256],
        url: [0; 512],
        enabled: false,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        priority: 0,
        record: false,
        segment_duration: 0,
        detection_based_recording: false,
        detection_model: [0; 512],
        detection_interval: 0,
        detection_threshold: 0.,
        pre_detection_buffer: 0,
        post_detection_buffer: 0,
        detection_api_url: [0; 512],
        detection_object_filter: [0; 16],
        detection_object_filter_list: [0; 256],
        streaming_enabled: false,
        protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
        record_audio: false,
        audio_voice_enhancement: false,
        onvif_username: [0; 64],
        onvif_password: [0; 64],
        onvif_profile: [0; 64],
        onvif_port: 0,
        onvif_discovery_enabled: false,
        is_onvif: false,
        backchannel_enabled: false,
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
        tier_critical_multiplier: 0.,
        tier_important_multiplier: 0.,
        tier_ephemeral_multiplier: 0.,
        storage_priority: 0,
        ptz_enabled: false,
        ptz_max_x: 0,
        ptz_max_y: 0,
        ptz_max_z: 0,
        ptz_has_home: false,
        record_on_schedule: false,
        recording_schedule: [0; 168],
        detection_record_on_schedule: false,
        detection_recording_schedule: [0; 168],
        tags: [0; 256],
        admin_url: [0; 512],
        privacy_mode: false,
        motion_trigger_source: [0; 256],
        go2rtc_source_override: [0; 8192],
        sub_stream_url: [0; 512],
        detection_url: [0; 512],
        publish_url: [0; 512],
        playback_transport: [0; 24],
        eptz_config: [0; 1024],
    };
    pthread_mutex_lock(&raw mut (*state).mutex);
    memcpy(
        &raw mut updated_config as *mut ::core::ffi::c_void,
        &raw mut (*state).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*state).mutex);
    if interval > 0 as ::core::ffi::c_int {
        updated_config.detection_interval = interval;
    }
    if threshold >= 0.0f32 && threshold <= 1.0f32 {
        updated_config.detection_threshold = threshold;
    }
    if pre_buffer >= 0 as ::core::ffi::c_int {
        updated_config.pre_detection_buffer = pre_buffer;
    }
    if post_buffer >= 0 as ::core::ffi::c_int {
        updated_config.post_detection_buffer = post_buffer;
    }
    update_stream_state_config(state, &raw mut updated_config);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set detection parameters for stream '%s' through adapter\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut (*state).mutex);
    let mut detection_enabled: bool = (*state).features.detection_enabled;
    let mut current_state: stream_state_t = (*state).state;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    if detection_enabled as ::core::ffi::c_int != 0
        && current_state.0 == stream_state_t::STREAM_STATE_ACTIVE.0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Restarting stream '%s' to apply detection parameter changes\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        stop_stream_with_state(state, r#true != 0);
        start_stream_with_state(state);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_streaming_enabled_adapter(
    mut handle: stream_handle_t,
    mut enabled: bool,
) -> ::core::ffi::c_int {
    if handle.is_null() || !adapter_initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = stream_handle_to_state(handle);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find state for handle in set_stream_streaming_enabled_adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return set_stream_feature(
        state,
        b"streaming\0".as_ptr() as *const ::core::ffi::c_char,
        enabled,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_by_name_adapter(
    mut name: *const ::core::ffi::c_char,
) -> stream_handle_t {
    if name.is_null() || !adapter_initialized {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    let mut state: *mut stream_state_manager_t = get_stream_state_by_name(name);
    if state.is_null() {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    return stream_state_to_handle(state);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_by_index_adapter(
    mut index: ::core::ffi::c_int,
) -> stream_handle_t {
    if index < 0 as ::core::ffi::c_int || !adapter_initialized {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    let mut state: *mut stream_state_manager_t = get_stream_state_by_index(index);
    if state.is_null() {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    return stream_state_to_handle(state);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_active_stream_count_adapter() -> ::core::ffi::c_int {
    if !adapter_initialized {
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut total: ::core::ffi::c_int = get_stream_state_count();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < total {
        let mut state: *mut stream_state_manager_t = get_stream_state_by_index(i);
        if !state.is_null()
            && get_stream_operational_state(state).0 == stream_state_t::STREAM_STATE_ACTIVE.0
        {
            count += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_total_stream_count_adapter() -> ::core::ffi::c_int {
    if !adapter_initialized {
        return 0 as ::core::ffi::c_int;
    }
    return get_stream_state_count();
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
