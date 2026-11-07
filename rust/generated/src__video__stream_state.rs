unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn is_recording_scheduled(config: *const stream_config_t) -> bool;
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
    unsafe fn stop_mp4_recording(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn reset_timestamp_tracker(stream_name: *const ::core::ffi::c_char);
    unsafe fn remove_timestamp_tracker(stream_name: *const ::core::ffi::c_char);
    unsafe fn go2rtc_integration_start_recording(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_record_error(
        stream_name: *const ::core::ffi::c_char,
        error_type: *const ::core::ffi::c_char,
    );
    unsafe fn metrics_record_reconnect(stream_name: *const ::core::ffi::c_char);
    unsafe fn metrics_set_configured_fps(
        stream_name: *const ::core::ffi::c_char,
        fps: ::core::ffi::c_double,
    );
    unsafe fn stream_start_hls(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stream_stop_hls(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_recording_state(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub union pthread_mutexattr_t {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_component_t(pub ::core::ffi::c_uint);
impl stream_component_t {
    pub const STREAM_COMPONENT_READER: Self = Self(0);
    pub const STREAM_COMPONENT_HLS: Self = Self(1);
    pub const STREAM_COMPONENT_MP4: Self = Self(2);
    pub const STREAM_COMPONENT_DETECTION: Self = Self(3);
    pub const STREAM_COMPONENT_API: Self = Self(4);
    pub const STREAM_COMPONENT_OTHER: Self = Self(5);
    pub const STREAM_COMPONENT_COUNT: Self = Self(6);
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
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const STREAM_ERR_MODEL_LOAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const AV_NOPTS_VALUE: int64_t = 0x8000000000000000 as ::core::ffi::c_ulong as int64_t;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
static mut stream_states: *mut *mut stream_state_manager_t =
    ::core::ptr::null_mut::<*mut stream_state_manager_t>();
static mut states_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut states_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut initialized: bool = r#false != 0;
static mut states_shutting_down: bool = r#false != 0;
unsafe extern "C" fn dispose_stream_state(mut state: *mut stream_state_manager_t) {
    pthread_mutex_destroy(&raw mut (*state).mutex);
    pthread_mutex_destroy(&raw mut (*state).state_mutex);
    free(state as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_stream_state_manager(
    mut max_streams: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if max_streams < 1 as ::core::ffi::c_int {
        max_streams = 1 as ::core::ffi::c_int;
    }
    if max_streams > MAX_STREAMS {
        max_streams = MAX_STREAMS;
    }
    pthread_mutex_lock(&raw mut states_mutex);
    if initialized {
        pthread_mutex_unlock(&raw mut states_mutex);
        return 0 as ::core::ffi::c_int;
    }
    stream_states = calloc(
        max_streams as size_t,
        ::core::mem::size_of::<*mut stream_state_manager_t>(),
    ) as *mut *mut stream_state_manager_t;
    if stream_states.is_null() {
        pthread_mutex_unlock(&raw mut states_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"init_stream_state_manager: failed to allocate states array\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    states_capacity = max_streams;
    initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut states_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream state manager initialized (%d slots)\0".as_ptr() as *const ::core::ffi::c_char,
        max_streams,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_stream_state_manager() {
    pthread_mutex_lock(&raw mut states_mutex);
    if !initialized || states_shutting_down as ::core::ffi::c_int != 0 {
        pthread_mutex_unlock(&raw mut states_mutex);
        return;
    }
    states_shutting_down = r#true != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < states_capacity {
        if !(*stream_states.offset(i as isize)).is_null() {
            let mut state: *mut stream_state_manager_t = *stream_states.offset(i as isize);
            let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
            safe_strcpy(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut (*state).name as *mut ::core::ffi::c_char,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            if (*state).state.0 == stream_state_t::STREAM_STATE_ACTIVE.0
                || (*state).state.0 == stream_state_t::STREAM_STATE_STARTING.0
                || (*state).state.0 == stream_state_t::STREAM_STATE_RECONNECTING.0
                || (*state).state.0 == stream_state_t::STREAM_STATE_STOPPING.0
            {
                pthread_mutex_unlock(&raw mut states_mutex);
                stop_stream_with_state(state, r#false != 0);
                pthread_mutex_lock(&raw mut states_mutex);
            }
            dispose_stream_state(state);
            *stream_states.offset(i as isize) = ::core::ptr::null_mut::<stream_state_manager_t>();
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cleaned up stream state for '%s' during shutdown\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        i += 1;
    }
    free(stream_states as *mut ::core::ffi::c_void);
    stream_states = ::core::ptr::null_mut::<*mut stream_state_manager_t>();
    states_capacity = 0 as ::core::ffi::c_int;
    initialized = r#false != 0;
    states_shutting_down = r#false != 0;
    pthread_mutex_unlock(&raw mut states_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream state manager shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_stream_state(
    mut config: *const stream_config_t,
) -> *mut stream_state_manager_t {
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters or state manager not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    pthread_mutex_lock(&raw mut states_mutex);
    if !initialized || states_shutting_down as ::core::ffi::c_int != 0 {
        pthread_mutex_unlock(&raw mut states_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"State manager is not initialized or is shutting down\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < states_capacity {
        if (*stream_states.offset(i as isize)).is_null() {
            slot = i;
            break;
        } else {
            i += 1;
        }
    }
    if slot == -1 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No available slots for new stream state\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut states_mutex);
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < states_capacity {
        if i_0 != slot
            && !(*stream_states.offset(i_0 as isize)).is_null()
            && strcmp(
                &raw mut (**stream_states.offset(i_0 as isize)).name as *mut ::core::ffi::c_char,
                &raw const (*config).name as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream with name '%s' already exists\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*config).name as *const ::core::ffi::c_char,
            );
            pthread_mutex_unlock(&raw mut states_mutex);
            return ::core::ptr::null_mut::<stream_state_manager_t>();
        }
        i_0 += 1;
    }
    let mut state: *mut stream_state_manager_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<stream_state_manager_t>(),
    ) as *mut stream_state_manager_t;
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for stream state\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut states_mutex);
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    if pthread_mutex_init(
        &raw mut (*state).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize stream state mutex\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(state as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut states_mutex);
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    if pthread_mutex_init(
        &raw mut (*state).state_mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize stream state transition mutex\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_destroy(&raw mut (*state).mutex);
        free(state as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut states_mutex);
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    memcpy(
        &raw mut (*state).config as *mut ::core::ffi::c_void,
        config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    safe_strcpy(
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        &raw const (*config).name as *const ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    (*state).state = stream_state_t::STREAM_STATE_INACTIVE;
    (*state).features.streaming_enabled = (*config).streaming_enabled;
    (*state).features.recording_enabled = (*config).record;
    (*state).features.detection_enabled = (*config).detection_based_recording;
    (*state).features.motion_detection_enabled = r#false != 0;
    (*state).protocol_state.protocol = (*config).protocol;
    (*state).protocol_state.is_multicast = r#false != 0;
    (*state).protocol_state.reconnect_attempts = 0 as ::core::ffi::c_int;
    (*state).protocol_state.last_reconnect_time = 0 as ::core::ffi::c_long;
    (*state).protocol_state.buffer_size = 0 as ::core::ffi::c_int;
    (*state).protocol_state.timeout_ms = 0 as ::core::ffi::c_int;
    (*state).timestamp_state.last_pts = AV_NOPTS_VALUE;
    (*state).timestamp_state.last_dts = AV_NOPTS_VALUE;
    (*state).timestamp_state.expected_next_pts = AV_NOPTS_VALUE;
    (*state).timestamp_state.pts_discontinuity_count = 0 as int64_t;
    (*state).timestamp_state.timestamps_initialized = r#false != 0;
    memset(
        &raw mut (*state).stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_stats_t>(),
    );
    (*state).ref_count = 1 as ::core::ffi::c_int;
    memset(
        &raw mut (*state).component_refs as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_int; 6]>(),
    );
    (*state).component_refs
        [stream_component_t::STREAM_COMPONENT_API.0 as ::core::ffi::c_int as usize] =
        1 as ::core::ffi::c_int;
    (*state).callbacks_enabled = r#true != 0;
    (*state).reader_ctx = NULL;
    (*state).hls_ctx = NULL;
    (*state).mp4_ctx = NULL;
    (*state).detection_ctx = NULL;
    *stream_states.offset(slot as isize) = state;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Created stream state for '%s' in slot %d with initial reference count 1\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*config).name as *const ::core::ffi::c_char,
        slot,
    );
    pthread_mutex_unlock(&raw mut states_mutex);
    return state;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_state_by_name(
    mut name: *const ::core::ffi::c_char,
) -> *mut stream_state_manager_t {
    if name.is_null() || !initialized {
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    pthread_mutex_lock(&raw mut states_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < states_capacity {
        if !(*stream_states.offset(i as isize)).is_null()
            && strcmp(
                &raw mut (**stream_states.offset(i as isize)).name as *mut ::core::ffi::c_char,
                name,
            ) == 0 as ::core::ffi::c_int
        {
            pthread_mutex_unlock(&raw mut states_mutex);
            return *stream_states.offset(i as isize);
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut states_mutex);
    return ::core::ptr::null_mut::<stream_state_manager_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_stream_state_config(
    mut state: *mut stream_state_manager_t,
    mut config: *const stream_config_t,
) -> ::core::ffi::c_int {
    if state.is_null() || config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for update_stream_config\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    let mut protocol_changing: bool = (*state).config.protocol.0 != (*config).protocol.0;
    let mut streaming_changing: bool = (*state).config.streaming_enabled as ::core::ffi::c_int
        != (*config).streaming_enabled as ::core::ffi::c_int;
    let mut recording_changing: bool =
        (*state).config.record as ::core::ffi::c_int != (*config).record as ::core::ffi::c_int;
    let mut detection_changing: bool = (*state).config.detection_based_recording
        as ::core::ffi::c_int
        != (*config).detection_based_recording as ::core::ffi::c_int;
    let mut onvif_changing: bool =
        (*state).config.is_onvif as ::core::ffi::c_int != (*config).is_onvif as ::core::ffi::c_int;
    if onvif_changing {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF flag changed for stream '%s': %s -> %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
            if (*state).config.is_onvif as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            if (*config).is_onvif as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut old_state: stream_state_t = (*state).state;
    let mut old_protocol: stream_protocol_t = (*state).config.protocol;
    memcpy(
        &raw mut (*state).config as *mut ::core::ffi::c_void,
        config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    (*state).features.streaming_enabled = (*config).streaming_enabled;
    (*state).features.recording_enabled = (*config).record;
    (*state).features.detection_enabled = (*config).detection_based_recording;
    if protocol_changing {
        (*state).protocol_state.protocol = (*config).protocol;
        (*state).protocol_state.reconnect_attempts = 0 as ::core::ffi::c_int;
        (*state).protocol_state.last_reconnect_time = 0 as ::core::ffi::c_long;
        if (*config).protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
            (*state).protocol_state.buffer_size = 16777216 as ::core::ffi::c_int;
            (*state).protocol_state.timeout_ms = 10000 as ::core::ffi::c_int;
        } else {
            (*state).protocol_state.buffer_size = 8388608 as ::core::ffi::c_int;
            (*state).protocol_state.timeout_ms = 5000 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Protocol changed for stream '%s': %s -> %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
            if old_protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
                b"UDP\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"TCP\0".as_ptr() as *const ::core::ffi::c_char
            },
            if (*config).protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
                b"UDP\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"TCP\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    pthread_mutex_unlock(&raw mut (*state).mutex);
    if old_state.0 == stream_state_t::STREAM_STATE_ACTIVE.0
        && (protocol_changing as ::core::ffi::c_int != 0
            || streaming_changing as ::core::ffi::c_int != 0
            || recording_changing as ::core::ffi::c_int != 0
            || detection_changing as ::core::ffi::c_int != 0
            || onvif_changing as ::core::ffi::c_int != 0)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Restarting stream '%s' due to configuration changes\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        stop_stream_with_state(state, r#false != 0);
        start_stream_with_state(state);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_stream_protocol(
    mut state: *mut stream_state_manager_t,
    mut protocol: stream_protocol_t,
) -> ::core::ffi::c_int {
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for update_stream_protocol\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    if (*state).config.protocol.0 == protocol.0 {
        pthread_mutex_unlock(&raw mut (*state).mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Protocol for stream '%s' already set to %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
            if protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
                b"UDP\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"TCP\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return 0 as ::core::ffi::c_int;
    }
    (*state).config.protocol = protocol;
    (*state).protocol_state.protocol = protocol;
    (*state).protocol_state.reconnect_attempts = 0 as ::core::ffi::c_int;
    (*state).protocol_state.last_reconnect_time = 0 as ::core::ffi::c_long;
    if protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
        (*state).protocol_state.buffer_size = 16777216 as ::core::ffi::c_int;
        (*state).protocol_state.timeout_ms = 10000 as ::core::ffi::c_int;
    } else {
        (*state).protocol_state.buffer_size = 8388608 as ::core::ffi::c_int;
        (*state).protocol_state.timeout_ms = 5000 as ::core::ffi::c_int;
    }
    let mut current_state: stream_state_t = (*state).state;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Protocol updated for stream '%s' to %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        if protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
            b"UDP\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"TCP\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if current_state.0 == stream_state_t::STREAM_STATE_ACTIVE.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Restarting stream '%s' due to protocol change\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        stop_stream_with_state(state, r#false != 0);
        start_stream_with_state(state);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_feature(
    mut state: *mut stream_state_manager_t,
    mut feature: *const ::core::ffi::c_char,
    mut enabled: bool,
) -> ::core::ffi::c_int {
    if state.is_null() || feature.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for set_stream_feature\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    let mut changed: bool = r#false != 0;
    if strcmp(
        feature,
        b"streaming\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if (*state).features.streaming_enabled as ::core::ffi::c_int
            != enabled as ::core::ffi::c_int
        {
            (*state).features.streaming_enabled = enabled;
            (*state).config.streaming_enabled = enabled;
            changed = r#true != 0;
        }
    } else if strcmp(
        feature,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if (*state).features.recording_enabled as ::core::ffi::c_int
            != enabled as ::core::ffi::c_int
        {
            (*state).features.recording_enabled = enabled;
            (*state).config.record = enabled;
            changed = r#true != 0;
        }
    } else if strcmp(
        feature,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if (*state).features.detection_enabled as ::core::ffi::c_int
            != enabled as ::core::ffi::c_int
        {
            (*state).features.detection_enabled = enabled;
            (*state).config.detection_based_recording = enabled;
            changed = r#true != 0;
        }
    } else if strcmp(
        feature,
        b"motion_detection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if (*state).features.motion_detection_enabled as ::core::ffi::c_int
            != enabled as ::core::ffi::c_int
        {
            (*state).features.motion_detection_enabled = enabled;
            changed = r#true != 0;
        }
    } else {
        pthread_mutex_unlock(&raw mut (*state).mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unknown feature '%s' for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            feature,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut current_state: stream_state_t = (*state).state;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    if strcmp(
        feature,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut actively_recording: ::core::ffi::c_int =
            get_recording_state(&raw mut (*state).name as *mut ::core::ffi::c_char);
        if enabled {
            if current_state.0 == stream_state_t::STREAM_STATE_ACTIVE.0
                && actively_recording != 1 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Starting recording for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*state).name as *mut ::core::ffi::c_char,
                );
                go2rtc_integration_start_recording(
                    &raw mut (*state).name as *mut ::core::ffi::c_char,
                );
            } else if current_state.0 != stream_state_t::STREAM_STATE_ACTIVE.0 {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stream '%s' is not ACTIVE (state=%d), recording will start when stream becomes active\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*state).name as *mut ::core::ffi::c_char,
                    current_state.0,
                );
            }
        } else if actively_recording == 1 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stopping recording for stream '%s' (state=%d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*state).name as *mut ::core::ffi::c_char,
                current_state.0,
            );
            stop_mp4_recording(&raw mut (*state).name as *mut ::core::ffi::c_char);
        }
    }
    if changed {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Feature '%s' %s for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            feature,
            if enabled as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s' current_state=%d (ACTIVE=%d), applying feature change\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
            current_state.0,
            stream_state_t::STREAM_STATE_ACTIVE.0 as ::core::ffi::c_int,
        );
        if strcmp(
            feature,
            b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            if current_state.0 == stream_state_t::STREAM_STATE_ACTIVE.0 {
                if strcmp(
                    feature,
                    b"streaming\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if enabled {
                        stream_start_hls(&raw mut (*state).name as *mut ::core::ffi::c_char);
                    } else {
                        stream_stop_hls(&raw mut (*state).name as *mut ::core::ffi::c_char);
                    }
                } else if strcmp(
                    feature,
                    b"detection\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        feature,
                        b"motion_detection\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Restarting stream '%s' to apply detection settings\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*state).name as *mut ::core::ffi::c_char,
                    );
                    stop_stream_with_state(state, r#false != 0);
                    start_stream_with_state(state);
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stream '%s' is not ACTIVE (state=%d), skipping non-recording feature application\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*state).name as *mut ::core::ffi::c_char,
                    current_state.0,
                );
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_stream_with_state(
    mut state: *mut stream_state_manager_t,
) -> ::core::ffi::c_int {
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for start_stream_with_state\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    if (*state).state.0 == stream_state_t::STREAM_STATE_ACTIVE.0 {
        pthread_mutex_unlock(&raw mut (*state).mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s' is already running\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    (*state).state = stream_state_t::STREAM_STATE_STARTING;
    let mut streaming_enabled: bool = (*state).features.streaming_enabled;
    let mut recording_enabled: bool = (*state).features.recording_enabled;
    let mut detection_enabled: bool = (*state).features.detection_enabled;
    let mut protocol: stream_protocol_t = (*state).protocol_state.protocol;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting stream '%s' (protocol: %s, streaming: %s, recording: %s, detection: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        if protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
            b"UDP\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"TCP\0".as_ptr() as *const ::core::ffi::c_char
        },
        if streaming_enabled as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
        if recording_enabled as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
        if detection_enabled as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut any_component_started: bool = r#false != 0;
    if streaming_enabled {
        let mut hls_result: ::core::ffi::c_int =
            stream_start_hls(&raw mut (*state).name as *mut ::core::ffi::c_char);
        if hls_result != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to start HLS stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*state).name as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Started HLS streaming for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*state).name as *mut ::core::ffi::c_char,
            );
            any_component_started = r#true != 0;
        }
    }
    if recording_enabled {
        let mut schedule_ok: bool = is_recording_scheduled(&raw mut (*state).config);
        if !schedule_ok {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream '%s': recording deferred \xE2\x80\x94 outside scheduled window\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*state).name as *mut ::core::ffi::c_char,
            );
            any_component_started = r#true != 0;
        } else {
            let mut mp4_result: ::core::ffi::c_int = go2rtc_integration_start_recording(
                &raw mut (*state).name as *mut ::core::ffi::c_char,
            );
            if mp4_result != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to start MP4 recording for '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*state).name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Started recording for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*state).name as *mut ::core::ffi::c_char,
                );
                any_component_started = r#true != 0;
            }
        }
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    if any_component_started {
        (*state).state = stream_state_t::STREAM_STATE_ACTIVE;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s' is now running\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        if (*state).config.fps > 0 as ::core::ffi::c_int {
            metrics_set_configured_fps(
                &raw mut (*state).name as *mut ::core::ffi::c_char,
                (*state).config.fps as ::core::ffi::c_double,
            );
        }
    } else {
        (*state).state = stream_state_t::STREAM_STATE_ERROR;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start any components for stream '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*state).mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut (*state).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_stream_with_state(
    mut state: *mut stream_state_manager_t,
    mut wait_for_completion: bool,
) -> ::core::ffi::c_int {
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for stop_stream_with_state\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).state_mutex);
    if (*state).state.0 == stream_state_t::STREAM_STATE_INACTIVE.0 {
        pthread_mutex_unlock(&raw mut (*state).state_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s' is already stopped\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*state).state.0 == stream_state_t::STREAM_STATE_STOPPING.0 {
        pthread_mutex_unlock(&raw mut (*state).state_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s' is already in the process of stopping\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        if wait_for_completion {
            return wait_for_stream_stop(state, 10000 as ::core::ffi::c_int);
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut old_state: stream_state_t = (*state).state;
    (*state).state = stream_state_t::STREAM_STATE_STOPPING;
    let mut streaming_enabled: bool = (*state).features.streaming_enabled;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    if !streaming_enabled {
        (*state).callbacks_enabled = r#false != 0;
    }
    pthread_mutex_unlock(&raw mut (*state).state_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream '%s' transitioning from %d to STOPPING state\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        old_state.0,
    );
    let mut result: ::core::ffi::c_int =
        stream_stop_hls(&raw mut stream_name as *mut ::core::ffi::c_char);
    if result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to stop HLS stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopped HLS streaming for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    let mut mp4_result: ::core::ffi::c_int =
        stop_mp4_recording(&raw mut stream_name as *mut ::core::ffi::c_char);
    if mp4_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to stop MP4 recording for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopped MP4 recording for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    reset_timestamp_tracker(&raw mut stream_name as *mut ::core::ffi::c_char);
    pthread_mutex_lock(&raw mut (*state).state_mutex);
    (*state).state = stream_state_t::STREAM_STATE_INACTIVE;
    (*state).callbacks_enabled = r#true != 0;
    pthread_mutex_unlock(&raw mut (*state).state_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stopped stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_operational_state(
    mut state: *mut stream_state_manager_t,
) -> stream_state_t {
    if state.is_null() {
        return stream_state_t::STREAM_STATE_INACTIVE;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    let mut current_state: stream_state_t = (*state).state;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    return current_state;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_statistics(
    mut state: *mut stream_state_manager_t,
    mut stats: *mut stream_stats_t,
) -> ::core::ffi::c_int {
    if state.is_null() || stats.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    memcpy(
        stats as *mut ::core::ffi::c_void,
        &raw mut (*state).stats as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_stats_t>(),
    );
    pthread_mutex_unlock(&raw mut (*state).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_stream_error(
    mut state: *mut stream_state_manager_t,
    mut error_code: ::core::ffi::c_int,
    mut error_message: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if state.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    (*state).stats.errors = (*state).stats.errors.wrapping_add(1);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream '%s' error: %s (code: %d)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        error_message,
        error_code,
    );
    if !error_message.is_null() {
        snprintf(
            &raw mut (*state).last_error_message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            error_message,
        );
    } else {
        (*state).last_error_message[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut error_type: *const ::core::ffi::c_char = b"io\0".as_ptr() as *const ::core::ffi::c_char;
    if error_code == -ETIMEDOUT {
        error_type = b"timeout\0".as_ptr() as *const ::core::ffi::c_char;
    } else if error_code == -EINVAL {
        error_type = b"protocol\0".as_ptr() as *const ::core::ffi::c_char;
    } else if error_code == -EIO {
        error_type = b"io\0".as_ptr() as *const ::core::ffi::c_char;
    } else if error_code == STREAM_ERR_MODEL_LOAD {
        error_type = b"model\0".as_ptr() as *const ::core::ffi::c_char;
    }
    metrics_record_error(
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        error_type,
    );
    let mut permanent_error: bool = error_code == STREAM_ERR_MODEL_LOAD;
    let mut should_reconnect: bool = r#false != 0;
    if (*state).state.0 == stream_state_t::STREAM_STATE_ACTIVE.0 && !permanent_error {
        should_reconnect = r#true != 0;
        (*state).state = stream_state_t::STREAM_STATE_RECONNECTING;
        (*state).protocol_state.reconnect_attempts += 1;
        (*state).protocol_state.last_reconnect_time = time(::core::ptr::null_mut::<time_t>());
        (*state).stats.reconnects = (*state).stats.reconnects.wrapping_add(1);
        metrics_record_reconnect(&raw mut (*state).name as *mut ::core::ffi::c_char);
    } else {
        (*state).state = stream_state_t::STREAM_STATE_ERROR;
    }
    pthread_mutex_unlock(&raw mut (*state).mutex);
    if should_reconnect {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempting to reconnect stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        stop_stream_with_state(state, r#false != 0);
        sleep(1 as ::core::ffi::c_uint);
        start_stream_with_state(state);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_stream_timestamps(
    mut state: *mut stream_state_manager_t,
    mut pts: int64_t,
    mut dts: int64_t,
) -> ::core::ffi::c_int {
    if state.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    (*state).timestamp_state.last_pts = pts;
    (*state).timestamp_state.last_dts = dts;
    if !(*state).timestamp_state.timestamps_initialized {
        (*state).timestamp_state.timestamps_initialized = r#true != 0;
        (*state).timestamp_state.expected_next_pts = pts;
    } else {
        let mut frame_duration: int64_t = 0 as int64_t;
        if (*state).config.fps > 0 as ::core::ffi::c_int {
            frame_duration = (90000 as ::core::ffi::c_int / (*state).config.fps) as int64_t;
        } else {
            frame_duration = 3000 as int64_t;
        }
        (*state).timestamp_state.expected_next_pts = pts + frame_duration;
    }
    pthread_mutex_unlock(&raw mut (*state).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_state_count() -> ::core::ffi::c_int {
    if !initialized {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut states_mutex);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < states_capacity {
        if !(*stream_states.offset(i as isize)).is_null() {
            count += 1;
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut states_mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_state_by_index(
    mut index: ::core::ffi::c_int,
) -> *mut stream_state_manager_t {
    if index < 0 as ::core::ffi::c_int || index >= states_capacity || !initialized {
        return ::core::ptr::null_mut::<stream_state_manager_t>();
    }
    pthread_mutex_lock(&raw mut states_mutex);
    let mut state: *mut stream_state_manager_t = *stream_states.offset(index as isize);
    pthread_mutex_unlock(&raw mut states_mutex);
    return state;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_stream_state(
    mut state: *mut stream_state_manager_t,
) -> ::core::ffi::c_int {
    if state.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut states_mutex);
    if !initialized || states_shutting_down as ::core::ffi::c_int != 0 {
        pthread_mutex_unlock(&raw mut states_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < states_capacity {
        if *stream_states.offset(i as isize) == state {
            slot = i;
            break;
        } else {
            i += 1;
        }
    }
    if slot == -1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut states_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream state not found in array\0".as_ptr() as *const ::core::ffi::c_char,
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
    pthread_mutex_lock(&raw mut (*state).mutex);
    let mut ref_count: ::core::ffi::c_int = (*state).ref_count;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    if ref_count > 1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut states_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot remove stream state for '%s' with reference count %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            ref_count,
        );
        return -1 as ::core::ffi::c_int;
    }
    *stream_states.offset(slot as isize) = ::core::ptr::null_mut::<stream_state_manager_t>();
    if (*state).state.0 == stream_state_t::STREAM_STATE_ACTIVE.0
        || (*state).state.0 == stream_state_t::STREAM_STATE_STARTING.0
        || (*state).state.0 == stream_state_t::STREAM_STATE_RECONNECTING.0
        || (*state).state.0 == stream_state_t::STREAM_STATE_STOPPING.0
    {
        pthread_mutex_unlock(&raw mut states_mutex);
        stop_stream_with_state(state, r#true != 0);
        pthread_mutex_lock(&raw mut states_mutex);
    }
    remove_timestamp_tracker(&raw mut stream_name as *mut ::core::ffi::c_char);
    dispose_stream_state(state);
    pthread_mutex_unlock(&raw mut states_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Removed stream state for '%s' from slot %d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        slot,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_state_add_ref(
    mut state: *mut stream_state_manager_t,
    mut component: stream_component_t,
) -> ::core::ffi::c_int {
    if state.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if component.0 < 0 as ::core::ffi::c_uint
        || component.0 >= stream_component_t::STREAM_COMPONENT_COUNT.0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid component type %d for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            component.0,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    (*state).component_refs[component.0 as usize] += 1;
    (*state).ref_count += 1;
    let mut new_ref_count: ::core::ffi::c_int = (*state).ref_count;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Added reference to stream '%s' for component %d, new ref count: %d\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        component.0,
        new_ref_count,
    );
    return new_ref_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_state_release_ref(
    mut state: *mut stream_state_manager_t,
    mut component: stream_component_t,
) -> ::core::ffi::c_int {
    if state.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if component.0 < 0 as ::core::ffi::c_uint
        || component.0 >= stream_component_t::STREAM_COMPONENT_COUNT.0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid component type %d for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            component.0,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    if (*state).component_refs[component.0 as usize] <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Component %d has no references to release for stream '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            component.0,
            &raw mut (*state).name as *mut ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*state).mutex);
        return (*state).ref_count;
    }
    (*state).component_refs[component.0 as usize] -= 1;
    (*state).ref_count -= 1;
    let mut new_ref_count: ::core::ffi::c_int = (*state).ref_count;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Released reference to stream '%s' for component %d, new ref count: %d\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*state).name as *mut ::core::ffi::c_char,
        component.0,
        new_ref_count,
    );
    return new_ref_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_state_get_ref_count(
    mut state: *mut stream_state_manager_t,
) -> ::core::ffi::c_int {
    if state.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    let mut ref_count: ::core::ffi::c_int = (*state).ref_count;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    return ref_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_stream_state_stopping(mut state: *mut stream_state_manager_t) -> bool {
    if state.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut (*state).state_mutex);
    let mut stopping: bool = (*state).state.0 == stream_state_t::STREAM_STATE_STOPPING.0;
    pthread_mutex_unlock(&raw mut (*state).state_mutex);
    return stopping;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wait_for_stream_stop(
    mut state: *mut stream_state_manager_t,
    mut timeout_ms: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if state.is_null() {
        return -2 as ::core::ffi::c_int;
    }
    let mut end_time: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&raw mut end_time, NULL);
    end_time.tv_sec += (timeout_ms / 1000 as ::core::ffi::c_int) as __time_t;
    end_time.tv_usec += (timeout_ms % 1000 as ::core::ffi::c_int) as ::core::ffi::c_long
        * 1000 as ::core::ffi::c_long;
    if end_time.tv_usec >= 1000000 as __suseconds_t {
        end_time.tv_sec += 1 as __time_t;
        end_time.tv_usec -= 1000000 as __suseconds_t;
    }
    loop {
        pthread_mutex_lock(&raw mut (*state).state_mutex);
        let mut is_inactive: bool = (*state).state.0 == stream_state_t::STREAM_STATE_INACTIVE.0;
        pthread_mutex_unlock(&raw mut (*state).state_mutex);
        if is_inactive {
            return 0 as ::core::ffi::c_int;
        }
        let mut current_time: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        gettimeofday(&raw mut current_time, NULL);
        if current_time.tv_sec > end_time.tv_sec
            || current_time.tv_sec == end_time.tv_sec && current_time.tv_usec >= end_time.tv_usec
        {
            return -1 as ::core::ffi::c_int;
        }
        usleep(10000 as __useconds_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_callbacks_enabled(
    mut state: *mut stream_state_manager_t,
    mut enabled: bool,
) -> ::core::ffi::c_int {
    if state.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    (*state).callbacks_enabled = enabled;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Callbacks %s for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
        if enabled as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut (*state).name as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn are_stream_callbacks_enabled(
    mut state: *mut stream_state_manager_t,
) -> bool {
    if state.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut (*state).mutex);
    let mut enabled: bool = (*state).callbacks_enabled;
    pthread_mutex_unlock(&raw mut (*state).mutex);
    return enabled;
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
