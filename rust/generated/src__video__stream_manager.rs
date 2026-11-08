unsafe extern "C" {
    pub type stream_handle_s;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
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
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
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
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn stop_mp4_recording(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_mp4_recording_runtime_info(
        stream_name: *const ::core::ffi::c_char,
        info: *mut recording_runtime_info_t,
    ) -> ::core::ffi::c_int;
    unsafe fn stream_start_hls(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stream_stop_hls(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stop_recording(stream_name: *const ::core::ffi::c_char);
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
    unsafe fn update_stream_config(
        name: *const ::core::ffi::c_char,
        stream: *const stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_all_stream_configs(
        streams_0: *mut stream_config_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn start_unified_detection_thread(
        stream_name: *const ::core::ffi::c_char,
        model_path: *const ::core::ffi::c_char,
        threshold: ::core::ffi::c_float,
        pre_buffer_seconds: ::core::ffi::c_int,
        post_buffer_seconds: ::core::ffi::c_int,
        annotation_only: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn stop_unified_detection_thread(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn is_unified_detection_running(stream_name: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_integration_start_recording(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_integration_stop_recording(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_integration_register_stream(stream_name: *const ::core::ffi::c_char) -> bool;
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
pub union pthread_mutexattr_t {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
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
    pub observation_evidence_enabled: bool,
    pub observation_labels: [::core::ffi::c_char; 512],
    pub observation_max_age_days: ::core::ffi::c_int,
    pub observation_max_bytes: int64_t,
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_runtime_info_t {
    pub active: bool,
    pub initializing: bool,
    pub trigger_type: [::core::ffi::c_char; 16],
    pub recording_id: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_t {
    pub config: stream_config_t,
    pub status: stream_status_t,
    pub stats: stream_stats_t,
    pub mutex: pthread_mutex_t,
    pub recording_enabled: bool,
    pub detection_recording_enabled: bool,
    pub last_detection_time: time_t,
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
pub struct stream_features_t {
    pub streaming_enabled: bool,
    pub recording_enabled: bool,
    pub detection_enabled: bool,
    pub motion_detection_enabled: bool,
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
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut streams: *mut stream_t = ::core::ptr::null_mut::<stream_t>();
static mut streams_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut initialized: bool = r#false != 0;
static mut schedule_monitor_thread: pthread_t = 0;
static mut schedule_monitor_running: bool = r#false != 0;
static mut schedule_monitor_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut schedule_monitor_cond: pthread_cond_t = PTHREAD_COND_INITIALIZER;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_recording_scheduled(mut config: *const stream_config_t) -> bool {
    if !(*config).record_on_schedule {
        return r#true != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut tm_info: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    localtime_r(&raw mut now, &raw mut tm_info);
    let mut index: ::core::ffi::c_int =
        tm_info.tm_wday * 24 as ::core::ffi::c_int + tm_info.tm_hour;
    if index < 0 as ::core::ffi::c_int || index >= 168 as ::core::ffi::c_int {
        return r#true != 0;
    }
    return (*config).recording_schedule[index as usize] as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_detection_recording_scheduled(
    mut config: *const stream_config_t,
) -> bool {
    if config.is_null() || !(*config).detection_record_on_schedule {
        return r#true != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut tm_info: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    localtime_r(&raw mut now, &raw mut tm_info);
    let mut index: ::core::ffi::c_int =
        tm_info.tm_wday * 24 as ::core::ffi::c_int + tm_info.tm_hour;
    if index < 0 as ::core::ffi::c_int || index >= 168 as ::core::ffi::c_int {
        return r#true != 0;
    }
    return (*config).detection_recording_schedule[index as usize] as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn schedule_monitor_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"StreamScheduler\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording schedule monitor thread started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    while schedule_monitor_running {
        let mut ts: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &raw mut ts);
        ts.tv_sec += 60 as __time_t;
        pthread_mutex_lock(&raw mut schedule_monitor_mutex);
        if schedule_monitor_running {
            pthread_cond_timedwait(
                &raw mut schedule_monitor_cond,
                &raw mut schedule_monitor_mutex,
                &raw mut ts,
            );
        }
        pthread_mutex_unlock(&raw mut schedule_monitor_mutex);
        if !schedule_monitor_running || !initialized {
            break;
        }
        let mut db_streams: *mut stream_config_t = calloc(
            (if streams_capacity > 0 as ::core::ffi::c_int {
                streams_capacity
            } else {
                32 as ::core::ffi::c_int
            }) as size_t,
            ::core::mem::size_of::<stream_config_t>(),
        ) as *mut stream_config_t;
        if db_streams.is_null() {
            sleep(60 as ::core::ffi::c_uint);
        } else {
            let mut count: ::core::ffi::c_int = get_all_stream_configs(
                db_streams,
                if streams_capacity > 0 as ::core::ffi::c_int {
                    streams_capacity
                } else {
                    32 as ::core::ffi::c_int
                },
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < count {
                if !((*db_streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
                    == '\0' as ::core::ffi::c_int
                    || !(*db_streams.offset(i as isize)).enabled)
                {
                    if (*db_streams.offset(i as isize)).record as ::core::ffi::c_int != 0
                        && (*db_streams.offset(i as isize)).record_on_schedule as ::core::ffi::c_int
                            != 0
                    {
                        let mut should_record: bool =
                            is_recording_scheduled(db_streams.offset(i as isize));
                        let mut runtime_info: recording_runtime_info_t = recording_runtime_info_t {
                            active: false,
                            initializing: false,
                            trigger_type: [0; 16],
                            recording_id: 0,
                        };
                        let mut is_recording: bool = get_mp4_recording_runtime_info(
                            &raw mut (*db_streams.offset(i as isize)).name
                                as *mut ::core::ffi::c_char,
                            &raw mut runtime_info,
                        ) == 0 as ::core::ffi::c_int;
                        if should_record as ::core::ffi::c_int != 0 && !is_recording {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Schedule: starting recording for stream '%s'\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                            go2rtc_integration_start_recording(
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                        } else if !should_record
                            && is_recording as ::core::ffi::c_int != 0
                            && strcmp(
                                &raw mut runtime_info.trigger_type as *mut ::core::ffi::c_char,
                                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                        {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Schedule: stopping recording for stream '%s'\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                            go2rtc_integration_stop_recording(
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                            stop_recording(
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    if (*db_streams.offset(i as isize)).detection_based_recording
                        as ::core::ffi::c_int
                        != 0
                        && (*db_streams.offset(i as isize)).detection_record_on_schedule
                            as ::core::ffi::c_int
                            != 0
                    {
                        let mut should_detect: bool =
                            is_detection_recording_scheduled(db_streams.offset(i as isize));
                        let mut is_detecting: bool = is_unified_detection_running(
                            &raw mut (*db_streams.offset(i as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                        if should_detect as ::core::ffi::c_int != 0 && !is_detecting {
                            let mut annotation_only: bool = (*db_streams.offset(i as isize)).record;
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Detection schedule: starting detection for stream '%s'\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                            start_unified_detection_thread(
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(i as isize)).detection_model
                                    as *mut ::core::ffi::c_char,
                                (*db_streams.offset(i as isize)).detection_threshold,
                                (*db_streams.offset(i as isize)).pre_detection_buffer,
                                (*db_streams.offset(i as isize)).post_detection_buffer,
                                annotation_only,
                            );
                        } else if !should_detect && is_detecting as ::core::ffi::c_int != 0 {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Detection schedule: stopping detection for stream '%s'\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                            stop_unified_detection_thread(
                                &raw mut (*db_streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                }
                i += 1;
            }
            free(db_streams as *mut ::core::ffi::c_void);
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording schedule monitor thread exiting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_stream_manager(
    mut max_streams: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if initialized {
        return 0 as ::core::ffi::c_int;
    }
    if max_streams < 1 as ::core::ffi::c_int {
        max_streams = 1 as ::core::ffi::c_int;
    }
    if max_streams > MAX_STREAMS {
        max_streams = MAX_STREAMS;
    }
    streams = calloc(max_streams as size_t, ::core::mem::size_of::<stream_t>()) as *mut stream_t;
    if streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"init_stream_manager: failed to allocate stream array (%d slots)\0".as_ptr()
                as *const ::core::ffi::c_char,
            max_streams,
        );
        return -1 as ::core::ffi::c_int;
    }
    streams_capacity = max_streams;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < streams_capacity {
        pthread_mutex_init(
            &raw mut (*streams.offset(i as isize)).mutex,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
        (*streams.offset(i as isize)).status = stream_status_t::STREAM_STATUS_STOPPED;
        memset(
            &raw mut (*streams.offset(i as isize)).stats as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stream_stats_t>(),
        );
        (*streams.offset(i as isize)).recording_enabled = r#false != 0;
        (*streams.offset(i as isize)).detection_recording_enabled = r#false != 0;
        i += 1;
    }
    let mut db_streams: *mut stream_config_t = calloc(
        streams_capacity as size_t,
        ::core::mem::size_of::<stream_config_t>(),
    ) as *mut stream_config_t;
    if db_streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"init_stream_manager: out of memory for db_streams\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(streams as *mut ::core::ffi::c_void);
        streams = ::core::ptr::null_mut::<stream_t>();
        streams_capacity = 0 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = get_all_stream_configs(db_streams, streams_capacity);
    if count > 0 as ::core::ffi::c_int {
        if count > streams_capacity {
            count = streams_capacity;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < count && i_0 < streams_capacity {
            if (*db_streams.offset(i_0 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                memcpy(
                    &raw mut (*streams.offset(i_0 as isize)).config as *mut ::core::ffi::c_void,
                    db_streams.offset(i_0 as isize) as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<stream_config_t>(),
                );
                (*streams.offset(i_0 as isize)).recording_enabled =
                    (*db_streams.offset(i_0 as isize)).record;
                (*streams.offset(i_0 as isize)).detection_recording_enabled =
                    (*db_streams.offset(i_0 as isize)).detection_based_recording;
            }
            i_0 += 1;
        }
    }
    free(db_streams as *mut ::core::ffi::c_void);
    initialized = r#true != 0;
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < streams_capacity {
        if (*streams.offset(i_1 as isize)).config.name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            let mut state: *const stream_state_manager_t = get_stream_state_by_name(
                &raw mut (*streams.offset(i_1 as isize)).config.name as *mut ::core::ffi::c_char,
            );
            if state.is_null() {
                state = create_stream_state(&raw mut (*streams.offset(i_1 as isize)).config);
                if state.is_null() {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to create stream state for '%s' during initialization\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*streams.offset(i_1 as isize)).config.name
                            as *mut ::core::ffi::c_char,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Created stream state for '%s' during initialization\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*streams.offset(i_1 as isize)).config.name
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            go2rtc_integration_register_stream(
                &raw mut (*streams.offset(i_1 as isize)).config.name as *mut ::core::ffi::c_char,
            );
        }
        i_1 += 1;
    }
    schedule_monitor_running = r#true != 0;
    if pthread_create(
        &raw mut schedule_monitor_thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            schedule_monitor_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create recording schedule monitor thread\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        schedule_monitor_running = r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream manager initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_stream_manager() {
    if !initialized {
        return;
    }
    if schedule_monitor_running {
        schedule_monitor_running = r#false != 0;
        pthread_mutex_lock(&raw mut schedule_monitor_mutex);
        pthread_cond_signal(&raw mut schedule_monitor_cond);
        pthread_mutex_unlock(&raw mut schedule_monitor_mutex);
        pthread_join(
            schedule_monitor_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Recording schedule monitor thread stopped\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < streams_capacity {
        if (*streams.offset(i as isize)).config.name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && (*streams.offset(i as isize)).status.0 == stream_status_t::STREAM_STATUS_RUNNING.0
        {
            let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
            safe_strcpy(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut (*streams.offset(i as isize)).config.name as *mut ::core::ffi::c_char,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            let mut streaming_enabled: bool =
                (*streams.offset(i as isize)).config.streaming_enabled;
            let mut recording_enabled: bool = (*streams.offset(i as isize)).config.record;
            let mut detection_enabled: bool = (*streams.offset(i as isize))
                .config
                .detection_based_recording;
            if streaming_enabled as ::core::ffi::c_int != 0
                || detection_enabled as ::core::ffi::c_int != 0
            {
                stream_stop_hls(&raw mut stream_name as *mut ::core::ffi::c_char);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stopped HLS streaming for '%s' during shutdown\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
            if recording_enabled {
                stop_mp4_recording(&raw mut stream_name as *mut ::core::ffi::c_char);
                stop_recording(&raw mut stream_name as *mut ::core::ffi::c_char);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stopped recording for '%s' during shutdown\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
            (*streams.offset(i as isize)).status = stream_status_t::STREAM_STATUS_STOPPED;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < streams_capacity {
        pthread_mutex_destroy(&raw mut (*streams.offset(i_0 as isize)).mutex);
        i_0 += 1;
    }
    free(streams as *mut ::core::ffi::c_void);
    streams = ::core::ptr::null_mut::<stream_t>();
    streams_capacity = 0 as ::core::ffi::c_int;
    initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream manager shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_by_name(
    mut name: *const ::core::ffi::c_char,
) -> stream_handle_t {
    if name.is_null() || !initialized {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < streams_capacity {
        if (*streams.offset(i as isize)).config.name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && strcmp(
                &raw mut (*streams.offset(i as isize)).config.name as *mut ::core::ffi::c_char,
                name,
            ) == 0 as ::core::ffi::c_int
        {
            return streams.offset(i as isize) as stream_handle_t;
        }
        i += 1;
    }
    let mut db_config: stream_config_t = stream_config_t {
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
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    if get_stream_config_by_name(name, &raw mut db_config) == 0 as ::core::ffi::c_int {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < streams_capacity {
            if (*streams.offset(i_0 as isize)).config.name[0usize] as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
            {
                memcpy(
                    &raw mut (*streams.offset(i_0 as isize)).config as *mut ::core::ffi::c_void,
                    &raw mut db_config as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<stream_config_t>(),
                );
                (*streams.offset(i_0 as isize)).status = stream_status_t::STREAM_STATUS_STOPPED;
                (*streams.offset(i_0 as isize)).recording_enabled = db_config.record;
                (*streams.offset(i_0 as isize)).detection_recording_enabled =
                    db_config.detection_based_recording;
                return streams.offset(i_0 as isize) as stream_handle_t;
            }
            i_0 += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No available slots for stream from database: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
    }
    return ::core::ptr::null_mut::<stream_handle_s>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_config(
    mut stream: stream_handle_t,
    mut config: *mut stream_config_t,
) -> ::core::ffi::c_int {
    if stream.is_null() || config.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = stream as *mut stream_t;
    if get_stream_config_by_name(
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        config,
    ) == 0 as ::core::ffi::c_int
    {
        pthread_mutex_lock(&raw mut (*s).mutex);
        memcpy(
            &raw mut (*s).config as *mut ::core::ffi::c_void,
            config as *const ::core::ffi::c_void,
            ::core::mem::size_of::<stream_config_t>(),
        );
        pthread_mutex_unlock(&raw mut (*s).mutex);
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Database unavailable for stream %s, using in-memory configuration\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut (*s).mutex);
    memcpy(
        config as *mut ::core::ffi::c_void,
        &raw mut (*s).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_status(mut stream: stream_handle_t) -> stream_status_t {
    if stream.is_null() {
        return stream_status_t::STREAM_STATUS_UNKNOWN;
    }
    let mut s: *mut stream_t = stream as *mut stream_t;
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut (*s).config.name as *mut ::core::ffi::c_char);
    if !state.is_null() {
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
            stream_state_t::STREAM_STATE_ERROR => {
                return stream_status_t::STREAM_STATUS_ERROR;
            }
            stream_state_t::STREAM_STATE_RECONNECTING => {
                return stream_status_t::STREAM_STATUS_RECONNECTING;
            }
            _ => return stream_status_t::STREAM_STATUS_UNKNOWN,
        }
    }
    pthread_mutex_lock(&raw mut (*s).mutex);
    let mut status: stream_status_t = (*s).status;
    pthread_mutex_unlock(&raw mut (*s).mutex);
    return status;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_detection_recording(
    mut stream: stream_handle_t,
    mut enabled: bool,
    mut model_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if stream.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = stream as *mut stream_t;
    pthread_mutex_lock(&raw mut (*s).mutex);
    let mut was_enabled: bool = (*s).config.detection_based_recording;
    let mut now_disabled: bool = was_enabled as ::core::ffi::c_int != 0 && !enabled;
    let mut now_enabled: bool = !was_enabled && enabled as ::core::ffi::c_int != 0;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    (*s).config.detection_based_recording = enabled;
    if !model_path.is_null() {
        safe_strcpy(
            &raw mut (*s).config.detection_model as *mut ::core::ffi::c_char,
            model_path,
            MAX_PATH_LENGTH as size_t,
            0 as size_t,
        );
    }
    let mut config_copy: stream_config_t = stream_config_t {
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
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    memcpy(
        &raw mut config_copy as *mut ::core::ffi::c_void,
        &raw mut (*s).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    if update_stream_config(
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
        &raw mut config_copy,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update stream configuration in database for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut config_copy.name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        update_stream_state_config(state, &raw mut config_copy);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated stream state configuration for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
    }
    if now_disabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detection disabled for stream %s, stopping unified detection thread\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        if stop_unified_detection_thread(&raw mut stream_name as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to stop unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully stopped unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
    } else if now_enabled as ::core::ffi::c_int != 0
        && is_detection_recording_scheduled(&raw mut config_copy) as ::core::ffi::c_int != 0
    {
        let mut annotation_only: bool = config_copy.record;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detection enabled for stream %s, starting unified detection thread with model %s (annotation_only=%s)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            &raw mut config_copy.detection_model as *mut ::core::ffi::c_char,
            if annotation_only as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if start_unified_detection_thread(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            &raw mut config_copy.detection_model as *mut ::core::ffi::c_char,
            config_copy.detection_threshold,
            config_copy.pre_detection_buffer,
            config_copy.post_detection_buffer,
            annotation_only,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to start unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully started unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set detection recording for stream %s: enabled=%s, model=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
        if enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !model_path.is_null() {
            model_path
        } else {
            b"none\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_detection_params(
    mut stream: stream_handle_t,
    mut interval: ::core::ffi::c_int,
    mut threshold: ::core::ffi::c_float,
    mut pre_buffer: ::core::ffi::c_int,
    mut post_buffer: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = stream as *mut stream_t;
    pthread_mutex_lock(&raw mut (*s).mutex);
    if interval > 0 as ::core::ffi::c_int {
        (*s).config.detection_interval = interval;
    }
    if threshold >= 0.0f32 && threshold <= 1.0f32 {
        (*s).config.detection_threshold = threshold;
    }
    if pre_buffer >= 0 as ::core::ffi::c_int {
        (*s).config.pre_detection_buffer = pre_buffer;
    }
    if post_buffer >= 0 as ::core::ffi::c_int {
        (*s).config.post_detection_buffer = post_buffer;
    }
    let mut config_copy: stream_config_t = stream_config_t {
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
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    memcpy(
        &raw mut config_copy as *mut ::core::ffi::c_void,
        &raw mut (*s).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    if update_stream_config(
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
        &raw mut config_copy,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update stream configuration in database for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut config_copy.name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        update_stream_state_config(state, &raw mut config_copy);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated stream state configuration for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set detection parameters for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_stream(mut config: *const stream_config_t) -> stream_handle_t {
    if config.is_null() || !initialized {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < streams_capacity {
        if (*streams.offset(i as isize)).config.name[0usize] as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
        {
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
            b"No available slots for new stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < streams_capacity {
        if i_0 != slot
            && (*streams.offset(i_0 as isize)).config.name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            && strcmp(
                &raw mut (*streams.offset(i_0 as isize)).config.name as *mut ::core::ffi::c_char,
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
            return ::core::ptr::null_mut::<stream_handle_s>();
        }
        i_0 += 1;
    }
    pthread_mutex_lock(&raw mut (*streams.offset(slot as isize)).mutex);
    memcpy(
        &raw mut (*streams.offset(slot as isize)).config as *mut ::core::ffi::c_void,
        config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    (*streams.offset(slot as isize)).status = stream_status_t::STREAM_STATUS_STOPPED;
    memset(
        &raw mut (*streams.offset(slot as isize)).stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_stats_t>(),
    );
    (*streams.offset(slot as isize)).recording_enabled = (*config).record;
    (*streams.offset(slot as isize)).detection_recording_enabled =
        (*config).detection_based_recording;
    pthread_mutex_unlock(&raw mut (*streams.offset(slot as isize)).mutex);
    let mut state: *const stream_state_manager_t =
        get_stream_state_by_name(&raw const (*config).name as *const ::core::ffi::c_char);
    if state.is_null() {
        state = create_stream_state(config);
        if state.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create stream state for '%s', some features may not work correctly\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw const (*config).name as *const ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Created stream state for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*config).name as *const ::core::ffi::c_char,
            );
        }
    }
    go2rtc_integration_register_stream(&raw const (*config).name as *const ::core::ffi::c_char);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Added stream '%s' in slot %d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).name as *const ::core::ffi::c_char,
        slot,
    );
    return streams.offset(slot as isize) as stream_handle_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_stream(mut handle: stream_handle_t) -> ::core::ffi::c_int {
    if handle.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    if (*s).status.0 == stream_status_t::STREAM_STATUS_RUNNING.0 {
        stop_stream(handle);
    }
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < streams_capacity {
        if streams.offset(i as isize) == s {
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
            b"Stream not found in array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    pthread_mutex_lock(&raw mut (*s).mutex);
    let mut stream_name_for_cleanup: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name_for_cleanup as *mut ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    memset(
        &raw mut (*s).config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_config_t>(),
    );
    (*s).status = stream_status_t::STREAM_STATUS_STOPPED;
    memset(
        &raw mut (*s).stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_stats_t>(),
    );
    (*s).recording_enabled = r#false != 0;
    (*s).detection_recording_enabled = r#false != 0;
    pthread_mutex_unlock(&raw mut (*s).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Removed stream '%s' from slot %d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        slot,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_stream(mut handle: stream_handle_t) -> ::core::ffi::c_int {
    if handle.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    pthread_mutex_lock(&raw mut (*s).mutex);
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using new state management system to start stream '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        let mut result: ::core::ffi::c_int = start_stream_with_state(state);
        if result == 0 as ::core::ffi::c_int {
            pthread_mutex_lock(&raw mut (*s).mutex);
            (*s).status = stream_status_t::STREAM_STATUS_RUNNING;
            pthread_mutex_unlock(&raw mut (*s).mutex);
        } else {
            pthread_mutex_lock(&raw mut (*s).mutex);
            (*s).status = stream_status_t::STREAM_STATUS_ERROR;
            pthread_mutex_unlock(&raw mut (*s).mutex);
        }
        return result;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Falling back to legacy system to start stream '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut (*s).mutex);
    if (*s).status.0 == stream_status_t::STREAM_STATUS_RUNNING.0 {
        pthread_mutex_unlock(&raw mut (*s).mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s' is already running\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    (*s).status = stream_status_t::STREAM_STATUS_STARTING;
    let mut streaming_enabled: bool = (*s).config.streaming_enabled;
    let mut recording_enabled: bool = (*s).config.record as ::core::ffi::c_int != 0
        && is_recording_scheduled(&raw mut (*s).config) as ::core::ffi::c_int != 0;
    if (*s).config.record as ::core::ffi::c_int != 0 && !recording_enabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s': recording deferred \xE2\x80\x94 outside scheduled window\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        );
    }
    let mut detection_enabled: bool = (*s).config.detection_based_recording;
    pthread_mutex_unlock(&raw mut (*s).mutex);
    let mut any_component_started: bool = r#false != 0;
    let mut need_hls: bool = streaming_enabled as ::core::ffi::c_int != 0
        || detection_enabled as ::core::ffi::c_int != 0;
    if need_hls {
        let mut hls_result: ::core::ffi::c_int =
            stream_start_hls(&raw mut stream_name as *mut ::core::ffi::c_char);
        if hls_result != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to start HLS stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        } else {
            if streaming_enabled as ::core::ffi::c_int != 0
                && detection_enabled as ::core::ffi::c_int != 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Started HLS streaming for '%s' (streaming + detection keepalive)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            } else if detection_enabled {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Started HLS for '%s' as go2rtc keepalive for detection\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Started HLS streaming for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
            any_component_started = r#true != 0;
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Streaming and detection disabled for '%s', not starting HLS stream\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    if recording_enabled {
        let mut mp4_result: ::core::ffi::c_int = 0;
        mp4_result =
            go2rtc_integration_start_recording(&raw mut stream_name as *mut ::core::ffi::c_char);
        if mp4_result != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to start MP4 recording for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Started recording for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            any_component_started = r#true != 0;
        }
    }
    pthread_mutex_lock(&raw mut (*s).mutex);
    if any_component_started {
        (*s).status = stream_status_t::STREAM_STATUS_RUNNING;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s' is now running\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    } else {
        (*s).status = stream_status_t::STREAM_STATUS_ERROR;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start any components for stream '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*s).mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut (*s).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_stream(mut handle: stream_handle_t) -> ::core::ffi::c_int {
    if handle.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    pthread_mutex_lock(&raw mut (*s).mutex);
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using new state management system to stop stream '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        let mut result: ::core::ffi::c_int = stop_stream_with_state(state, r#true != 0);
        if result == 0 as ::core::ffi::c_int {
            pthread_mutex_lock(&raw mut (*s).mutex);
            (*s).status = stream_status_t::STREAM_STATUS_STOPPED;
            pthread_mutex_unlock(&raw mut (*s).mutex);
        }
        return result;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Falling back to legacy system to stop stream '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut (*s).mutex);
    if (*s).status.0 == stream_status_t::STREAM_STATUS_STOPPED.0 {
        pthread_mutex_unlock(&raw mut (*s).mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream '%s' is already stopped\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    (*s).status = stream_status_t::STREAM_STATUS_STOPPING;
    let mut streaming_enabled: bool = (*s).config.streaming_enabled;
    let mut recording_enabled: bool = (*s).config.record;
    let mut detection_enabled: bool = (*s).config.detection_based_recording;
    pthread_mutex_unlock(&raw mut (*s).mutex);
    if streaming_enabled as ::core::ffi::c_int != 0 || detection_enabled as ::core::ffi::c_int != 0
    {
        let mut result_0: ::core::ffi::c_int =
            stream_stop_hls(&raw mut stream_name as *mut ::core::ffi::c_char);
        if result_0 != 0 as ::core::ffi::c_int {
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
    }
    if recording_enabled {
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
        stop_recording(&raw mut stream_name as *mut ::core::ffi::c_char);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopped recording metadata for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    pthread_mutex_lock(&raw mut (*s).mutex);
    (*s).status = stream_status_t::STREAM_STATUS_STOPPED;
    pthread_mutex_unlock(&raw mut (*s).mutex);
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
pub unsafe extern "C" fn get_stream_by_index(mut index: ::core::ffi::c_int) -> stream_handle_t {
    if index < 0 as ::core::ffi::c_int || index >= streams_capacity || !initialized {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    if (*streams.offset(index as isize)).config.name[0usize] as ::core::ffi::c_int
        == '\0' as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<stream_handle_s>();
    }
    return streams.offset(index as isize) as stream_handle_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_active_stream_count() -> ::core::ffi::c_int {
    if !initialized {
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < streams_capacity {
        if (*streams.offset(i as isize)).config.name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && ((*streams.offset(i as isize)).status.0 == stream_status_t::STREAM_STATUS_RUNNING.0
                || (*streams.offset(i as isize)).status.0
                    == stream_status_t::STREAM_STATUS_RECONNECTING.0
                || (*streams.offset(i as isize)).status.0
                    == stream_status_t::STREAM_STATUS_STARTING.0)
        {
            count += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_total_stream_count() -> ::core::ffi::c_int {
    if !initialized {
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < streams_capacity {
        if (*streams.offset(i as isize)).config.name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            count += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_capacity() -> ::core::ffi::c_int {
    if !initialized {
        return 0 as ::core::ffi::c_int;
    }
    return streams_capacity;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_stats(
    mut handle: stream_handle_t,
    mut stats: *mut stream_stats_t,
) -> ::core::ffi::c_int {
    if handle.is_null() || stats.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    pthread_mutex_lock(&raw mut (*s).mutex);
    memcpy(
        stats as *mut ::core::ffi::c_void,
        &raw mut (*s).stats as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_stats_t>(),
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_priority(
    mut handle: stream_handle_t,
    mut priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if handle.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    pthread_mutex_lock(&raw mut (*s).mutex);
    (*s).config.priority = priority;
    let mut config_copy: stream_config_t = stream_config_t {
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
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    memcpy(
        &raw mut config_copy as *mut ::core::ffi::c_void,
        &raw mut (*s).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    if update_stream_config(
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
        &raw mut config_copy,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update stream configuration in database for stream %s (in-memory updated)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
    }
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut config_copy.name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        update_stream_state_config(state, &raw mut config_copy);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated stream state configuration for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set priority for stream '%s' to %d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
        priority,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_recording(
    mut handle: stream_handle_t,
    mut enable: bool,
) -> ::core::ffi::c_int {
    if handle.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    pthread_mutex_lock(&raw mut (*s).mutex);
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using new state management system to set recording for stream '%s' to %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            if enable as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        let mut result: ::core::ffi::c_int = set_stream_feature(
            state,
            b"recording\0".as_ptr() as *const ::core::ffi::c_char,
            enable,
        );
        if result == 0 as ::core::ffi::c_int {
            pthread_mutex_lock(&raw mut (*s).mutex);
            (*s).config.record = enable;
            (*s).recording_enabled = enable;
            pthread_mutex_unlock(&raw mut (*s).mutex);
        }
        return result;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Falling back to legacy system to set recording for stream '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut (*s).mutex);
    (*s).config.record = enable;
    (*s).recording_enabled = enable;
    let mut config_copy: stream_config_t = stream_config_t {
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
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    memcpy(
        &raw mut config_copy as *mut ::core::ffi::c_void,
        &raw mut (*s).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    if update_stream_config(
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
        &raw mut config_copy,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update stream configuration in database for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    state = get_stream_state_by_name(&raw mut config_copy.name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        update_stream_state_config(state, &raw mut config_copy);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated stream state configuration for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set recording for stream '%s' to %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        if enable as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_last_detection_time(
    mut handle: stream_handle_t,
    mut time_0: time_t,
) -> ::core::ffi::c_int {
    if handle.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    pthread_mutex_lock(&raw mut (*s).mutex);
    (*s).last_detection_time = time_0;
    pthread_mutex_unlock(&raw mut (*s).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set last detection time for stream '%s' to %ld\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        time_0,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_streaming_enabled(
    mut handle: stream_handle_t,
    mut enabled: bool,
) -> ::core::ffi::c_int {
    if handle.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    pthread_mutex_lock(&raw mut (*s).mutex);
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using new state management system to set streaming for stream '%s' to %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            if enabled as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        let mut result: ::core::ffi::c_int = set_stream_feature(
            state,
            b"streaming\0".as_ptr() as *const ::core::ffi::c_char,
            enabled,
        );
        if result == 0 as ::core::ffi::c_int {
            pthread_mutex_lock(&raw mut (*s).mutex);
            (*s).config.streaming_enabled = enabled;
            pthread_mutex_unlock(&raw mut (*s).mutex);
        }
        return result;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Falling back to legacy system to set streaming for stream '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut (*s).mutex);
    (*s).config.streaming_enabled = enabled;
    let mut config_copy: stream_config_t = stream_config_t {
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
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    memcpy(
        &raw mut config_copy as *mut ::core::ffi::c_void,
        &raw mut (*s).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    if update_stream_config(
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
        &raw mut config_copy,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update stream configuration in database for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    state = get_stream_state_by_name(&raw mut config_copy.name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        update_stream_state_config(state, &raw mut config_copy);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated stream state configuration for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set streaming enabled for stream '%s' to %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        if enabled as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_onvif_flag(
    mut handle: stream_handle_t,
    mut is_onvif: bool,
) -> ::core::ffi::c_int {
    if handle.is_null() || !initialized {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut stream_t = handle as *mut stream_t;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    pthread_mutex_lock(&raw mut (*s).mutex);
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*s).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Setting ONVIF flag for stream '%s' to %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        if is_onvif as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    pthread_mutex_lock(&raw mut (*s).mutex);
    if (*s).config.is_onvif as ::core::ffi::c_int == is_onvif as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut (*s).mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF flag for stream '%s' is already set to %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            if is_onvif as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return 0 as ::core::ffi::c_int;
    }
    (*s).config.is_onvif = is_onvif;
    let mut config_copy: stream_config_t = stream_config_t {
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
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    memcpy(
        &raw mut config_copy as *mut ::core::ffi::c_void,
        &raw mut (*s).config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    pthread_mutex_unlock(&raw mut (*s).mutex);
    if update_stream_config(
        &raw mut config_copy.name as *mut ::core::ffi::c_char,
        &raw mut config_copy,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update stream configuration in database for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut config_copy.name as *mut ::core::ffi::c_char);
    if !state.is_null() {
        update_stream_state_config(state, &raw mut config_copy);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated stream state configuration for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config_copy.name as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set ONVIF flag for stream '%s' to %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        if is_onvif as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
