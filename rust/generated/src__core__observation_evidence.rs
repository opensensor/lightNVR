unsafe extern "C" {
    pub type jpeg_encoder_cache;
    unsafe fn db_observation_insert_batch(
        batch: *const observation_batch_input_t,
        asset: *const observation_asset_input_t,
        result: *mut observation_insert_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_asset_transition(
        asset_uuid: *const ::core::ffi::c_char,
        from: observation_asset_state_t,
        to: observation_asset_state_t,
        reason: *const ::core::ffi::c_char,
        media: *const observation_asset_media_t,
        result: *mut observation_transition_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
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
    unsafe fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sem_init(
        __sem: *mut sem_t,
        __pshared: ::core::ffi::c_int,
        __value: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    unsafe fn sem_destroy(__sem: *mut sem_t) -> ::core::ffi::c_int;
    unsafe fn sem_timedwait(__sem: *mut sem_t, __abstime: *const timespec) -> ::core::ffi::c_int;
    unsafe fn sem_post(__sem: *mut sem_t) -> ::core::ffi::c_int;
    unsafe fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn register_component(
        name: *const ::core::ffi::c_char,
        r#type: component_type_t,
        context: *mut ::core::ffi::c_void,
        priority: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn update_component_state(component_id: ::core::ffi::c_int, state: component_state_t);
    unsafe fn get_stream_config_by_uuid(
        camera_uuid: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn jpeg_encoder_cache_create(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        quality: ::core::ffi::c_int,
    ) -> *mut jpeg_encoder_cache_t;
    unsafe fn jpeg_encoder_cache_encode_to_memory(
        encoder: *mut jpeg_encoder_cache_t,
        frame_data: *const ::core::ffi::c_uchar,
        out_data: *mut *mut ::core::ffi::c_uchar,
        out_size: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn jpeg_encoder_cache_destroy(encoder: *mut jpeg_encoder_cache_t);
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __mode_t = ::core::ffi::c_uint;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type intptr_t = isize;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const DB_OBS_OK: Self = Self(0);
    pub const DB_OBS_DUPLICATE: Self = Self(1);
    pub const DB_OBS_CAS_LOST: Self = Self(2);
    pub const DB_OBS_ERROR: Self = Self(-1);
    pub const DB_OBS_INVALID: Self = Self(-2);
    pub const DB_OBS_NOT_FOUND: Self = Self(-3);
    pub const DB_OBS_INVALID_CURSOR: Self = Self(-4);
    pub const DB_OBS_CURSOR_EXPIRED: Self = Self(-5);
    pub const DB_OBS_CONFLICT: Self = Self(-6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_asset_state_t(pub ::core::ffi::c_uint);
impl observation_asset_state_t {
    pub const OBS_ASSET_PENDING: Self = Self(0);
    pub const OBS_ASSET_READY: Self = Self(1);
    pub const OBS_ASSET_UNAVAILABLE: Self = Self(2);
    pub const OBS_ASSET_EXPIRED: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_object_input_t {
    pub object_index: ::core::ffi::c_int,
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_double,
    pub has_box: bool,
    pub box_x: ::core::ffi::c_double,
    pub box_y: ::core::ffi::c_double,
    pub box_width: ::core::ffi::c_double,
    pub box_height: ::core::ffi::c_double,
    pub has_track: bool,
    pub track_id: [::core::ffi::c_char; 65],
    pub track_namespace: [::core::ffi::c_char; 65],
    pub track_session: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_batch_input_t {
    pub batch_id: [::core::ffi::c_char; 37],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub stream_name: [::core::ffi::c_char; 64],
    pub observed_at_ms: int64_t,
    pub engine_key: [::core::ffi::c_char; 65],
    pub engine_type: [::core::ffi::c_char; 16],
    pub model: [::core::ffi::c_char; 129],
    pub model_version: [::core::ffi::c_char; 65],
    pub recording_id: int64_t,
    pub object_count: ::core::ffi::c_int,
    pub objects: [observation_object_input_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_asset_input_t {
    pub asset_uuid: [::core::ffi::c_char; 37],
    pub state: observation_asset_state_t,
    pub reason: [::core::ffi::c_char; 33],
    pub origin: [::core::ffi::c_char; 24],
    pub media_type: [::core::ffi::c_char; 24],
    pub captured_at_ms: int64_t,
    pub transform_json: [::core::ffi::c_char; 1024],
    pub expires_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_insert_result_t {
    pub asset_uuid: [::core::ffi::c_char; 37],
    pub object_count: ::core::ffi::c_int,
    pub observation_uuids: [[::core::ffi::c_char; 37]; 20],
    pub last_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_asset_media_t {
    pub path: *const ::core::ffi::c_char,
    pub bytes: int64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub expires_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_transition_result_t {
    pub observations_bumped: ::core::ffi::c_int,
    pub last_seq: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_evidence_result_t(pub ::core::ffi::c_int);
impl observation_evidence_result_t {
    pub const OBS_EVIDENCE_ACCEPTED: Self = Self(0);
    pub const OBS_EVIDENCE_DROPPED_QUEUE_FULL: Self = Self(1);
    pub const OBS_EVIDENCE_DROPPED_BUDGET: Self = Self(2);
    pub const OBS_EVIDENCE_DISABLED: Self = Self(3);
    pub const OBS_EVIDENCE_INVALID: Self = Self(-1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_transform_mode_t(pub ::core::ffi::c_uint);
impl observation_transform_mode_t {
    pub const OBS_TRANSFORM_NONE: Self = Self(0);
    pub const OBS_TRANSFORM_RESIZE: Self = Self(1);
    pub const OBS_TRANSFORM_LETTERBOX: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_evidence_transform_t {
    pub mode: observation_transform_mode_t,
    pub source_width: ::core::ffi::c_int,
    pub source_height: ::core::ffi::c_int,
    pub inference_width: ::core::ffi::c_int,
    pub inference_height: ::core::ffi::c_int,
    pub scale_x: ::core::ffi::c_double,
    pub scale_y: ::core::ffi::c_double,
    pub pad_x: ::core::ffi::c_double,
    pub pad_y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_evidence_config_t {
    pub storage_path: *const ::core::ffi::c_char,
    pub global_max_bytes: int64_t,
    pub global_max_items: ::core::ffi::c_int,
    pub per_camera_max_items: ::core::ffi::c_int,
    pub per_camera_max_bytes: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub lifecycle: atomic_int,
    pub inflight: atomic_int,
    pub storage_path: [::core::ffi::c_char; 512],
    pub global_max_bytes: int64_t,
    pub global_max_items: ::core::ffi::c_int,
    pub per_camera_max_items: ::core::ffi::c_int,
    pub per_camera_max_bytes: int64_t,
    pub start_lock: pthread_mutex_t,
    pub thread: pthread_t,
    pub thread_started: atomic_int,
    pub worker_running: atomic_int,
    pub component_id: ::core::ffi::c_int,
    pub head: *mut evidence_item_t,
    pub ring: *mut drop_ring_t,
    pub drop_pending: atomic_int,
    pub wake: sem_t,
    pub stop: atomic_int,
    pub drain_deadline_ns: int64_t,
    pub items: atomic_int,
    pub bytes: atomic_llong,
    pub high_water_items: atomic_uint,
    pub high_water_bytes: atomic_ullong,
    pub camera_high_water_items: atomic_uint,
    pub discard_max_bytes: int64_t,
    pub discard_bytes: atomic_llong,
    pub discard_deferred: atomic_ullong,
    pub discard_inline: atomic_ullong,
    pub submit_latency_count: atomic_ullong,
    pub submit_latency_sum_ns: atomic_ullong,
    pub submit_latency_max_ns: atomic_ullong,
    pub submit_latency_bucket: [atomic_ullong; 10],
    pub accepted: atomic_ullong,
    pub dropped_queue_full: atomic_ullong,
    pub dropped_budget: atomic_ullong,
    pub dropped_disabled: atomic_ullong,
    pub dropped_untracked: atomic_ullong,
    pub unavailable_submitted: atomic_ullong,
    pub ready: atomic_ullong,
    pub failed_encode: atomic_ullong,
    pub failed_write: atomic_ullong,
    pub failed_db: atomic_ullong,
    pub disabled_at_encode: atomic_ullong,
    pub interrupted: atomic_ullong,
    pub bytes_written: atomic_ullong,
    pub encoder: *mut jpeg_encoder_cache_t,
    pub encoder_w: ::core::ffi::c_int,
    pub encoder_h: ::core::ffi::c_int,
}
pub type jpeg_encoder_cache_t = jpeg_encoder_cache;
pub type atomic_ullong = ::core::ffi::c_ulonglong;
pub type atomic_llong = ::core::ffi::c_longlong;
pub type atomic_uint = ::core::ffi::c_uint;
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [::core::ffi::c_char; 32],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drop_ring_t {
    pub cells: [drop_cell_t; 256],
    pub enqueue_pos: atomic_size_t,
    pub dequeue_pos: size_t,
}
pub type atomic_size_t = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drop_cell_t {
    pub seq: atomic_size_t,
    pub record: drop_record_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drop_record_t {
    pub batch: observation_batch_input_t,
    pub asset_uuid: [::core::ffi::c_char; 37],
    pub reason: [::core::ffi::c_char; 33],
    pub discard: *mut uint8_t,
    pub discard_charge: size_t,
}
pub type evidence_item_t = evidence_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evidence_item {
    pub next: *mut evidence_item,
    pub is_jpeg: bool,
    pub slot: ::core::ffi::c_int,
    pub charge: size_t,
    pub data: *mut uint8_t,
    pub data_size: size_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub batch: observation_batch_input_t,
    pub asset: observation_asset_input_t,
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
pub struct camera_slot_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub items: atomic_int,
    pub bytes: atomic_llong,
    pub budget_exhausted: atomic_int,
}
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
pub struct observation_evidence_stage_t(pub ::core::ffi::c_uint);
impl observation_evidence_stage_t {
    pub const OBS_EVIDENCE_STAGE_ITEM_BEGIN: Self = Self(0);
    pub const OBS_EVIDENCE_STAGE_BEFORE_RENAME: Self = Self(1);
    pub const OBS_EVIDENCE_STAGE_ITEM_DONE: Self = Self(2);
    pub const OBS_EVIDENCE_STAGE_AFTER_INSERT: Self = Self(3);
    pub const OBS_EVIDENCE_STAGE_BEFORE_COMMIT: Self = Self(4);
}
pub type observation_evidence_stage_fn = Option<
    unsafe extern "C" fn(
        observation_evidence_stage_t,
        *const ::core::ffi::c_char,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type observation_evidence_enabled_fn =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> bool>;
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
pub struct observation_evidence_stats_t {
    pub initialized: bool,
    pub worker_running: bool,
    pub allocated: bool,
    pub accepted: uint64_t,
    pub dropped_queue_full: uint64_t,
    pub dropped_budget: uint64_t,
    pub dropped_disabled: uint64_t,
    pub dropped_untracked: uint64_t,
    pub unavailable_submitted: uint64_t,
    pub ready: uint64_t,
    pub failed_encode: uint64_t,
    pub failed_write: uint64_t,
    pub failed_db: uint64_t,
    pub disabled_at_encode: uint64_t,
    pub interrupted: uint64_t,
    pub bytes_written: uint64_t,
    pub backlog_items: uint32_t,
    pub backlog_bytes: uint64_t,
    pub drop_ring_depth: uint32_t,
    pub high_water_items: uint32_t,
    pub high_water_bytes: uint64_t,
    pub budget_exhausted_cameras: uint32_t,
    pub limit_items: uint32_t,
    pub limit_camera_items: uint32_t,
    pub limit_bytes: uint64_t,
    pub limit_camera_bytes: uint64_t,
    pub camera_high_water_items: uint32_t,
    pub submit_latency_count: uint64_t,
    pub submit_latency_sum_ns: uint64_t,
    pub submit_latency_max_ns: uint64_t,
    pub submit_latency_bucket: [uint64_t; 10],
    pub discard_deferred: uint64_t,
    pub discard_inline: uint64_t,
    pub discard_pending_bytes: uint64_t,
    pub limit_discard_bytes: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const LC_OFF: Self = Self(0);
    pub const LC_READY: Self = Self(1);
    pub const LC_RUNNING: Self = Self(2);
    pub const LC_STOPPING: Self = Self(3);
}
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const OBS_UUID_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const OBS_MAX_BATCH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const OBS_EVIDENCE_PER_CAMERA_DEPTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OBS_EVIDENCE_GLOBAL_DEPTH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const OBS_EVIDENCE_DROP_RING_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES: ::core::ffi::c_longlong = 32
    as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong;
pub const OBS_EVIDENCE_JPEG_QUALITY: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const OBS_EVIDENCE_DIR: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"observations\0") };
pub const OBS_EVIDENCE_DEFAULT_DRAIN_MS: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const RGB24_SWS_TAIL_PADDING: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const EVIDENCE_MAX_CAMERAS: ::core::ffi::c_int = MAX_STREAMS;
pub const EVIDENCE_WAIT_MS: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
static mut g_slots: [camera_slot_t; 1024] = [camera_slot_t {
    camera_uuid: [0; 37],
    items: 0,
    bytes: 0,
    budget_exhausted: 0,
}; 1024];
static mut g_slot_count: atomic_int = 0;
static mut g_slot_lock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_1::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut g_hook_lock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_1::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut g_stage_hook: observation_evidence_stage_fn = None;
static mut g_stage_ctx: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut g_enabled_probe: observation_evidence_enabled_fn = None;
static mut g_enabled_ctx: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut g: C2Rust_Unnamed_0 = C2Rust_Unnamed_0 {
    lifecycle: 0,
    inflight: 0,
    storage_path: [0; 512],
    global_max_bytes: 0,
    global_max_items: 0,
    per_camera_max_items: 0,
    per_camera_max_bytes: 0,
    start_lock: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0 as ::core::ffi::c_int,
            __count: 0 as ::core::ffi::c_uint,
            __owner: 0 as ::core::ffi::c_int,
            __nusers: 0 as ::core::ffi::c_uint,
            __kind: C2Rust_Unnamed_1::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
            __spins: 0 as ::core::ffi::c_short,
            __glibc_reserved: 0 as ::core::ffi::c_short,
            __list: __pthread_internal_list {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    thread: 0,
    thread_started: 0,
    worker_running: 0,
    component_id: -1 as ::core::ffi::c_int,
    head: ::core::ptr::null_mut::<evidence_item_t>(),
    ring: ::core::ptr::null_mut::<drop_ring_t>(),
    drop_pending: 0,
    wake: sem_t { __size: [0; 32] },
    stop: 0,
    drain_deadline_ns: 0,
    items: 0,
    bytes: 0,
    high_water_items: 0,
    high_water_bytes: 0,
    camera_high_water_items: 0,
    discard_max_bytes: 0,
    discard_bytes: 0,
    discard_deferred: 0,
    discard_inline: 0,
    submit_latency_count: 0,
    submit_latency_sum_ns: 0,
    submit_latency_max_ns: 0,
    submit_latency_bucket: [0; 10],
    accepted: 0,
    dropped_queue_full: 0,
    dropped_budget: 0,
    dropped_disabled: 0,
    dropped_untracked: 0,
    unavailable_submitted: 0,
    ready: 0,
    failed_encode: 0,
    failed_write: 0,
    failed_db: 0,
    disabled_at_encode: 0,
    interrupted: 0,
    bytes_written: 0,
    encoder: ::core::ptr::null_mut::<jpeg_encoder_cache_t>(),
    encoder_w: 0,
    encoder_h: 0,
};
unsafe extern "C" fn mono_ns() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut ts);
    return (ts.tv_sec as int64_t as ::core::ffi::c_longlong * 1000000000 as ::core::ffi::c_longlong
        + ts.tv_nsec as ::core::ffi::c_longlong) as int64_t;
}
unsafe extern "C" fn bump_max_uint(mut target: *mut atomic_uint, mut value: ::core::ffi::c_uint) {
    let mut current: ::core::ffi::c_uint = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(target);
    while value > current && {
        let c2rust_result = ::core::intrinsics::atomic_cxchgweak::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(target, *&raw mut current, value);
        *&raw mut current = c2rust_result.0;
        !c2rust_result.1
    } {}
}
unsafe extern "C" fn bump_max_ull(
    mut target: *mut atomic_ullong,
    mut value: ::core::ffi::c_ulonglong,
) {
    let mut current: ::core::ffi::c_ulonglong = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(target);
    while value > current && {
        let c2rust_result = ::core::intrinsics::atomic_cxchgweak::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(target, *&raw mut current, value);
        *&raw mut current = c2rust_result.0;
        !c2rust_result.1
    } {}
}
#[unsafe(no_mangle)]
pub static mut observation_evidence_submit_latency_bounds_ns: [uint64_t; 9] = [
    1000 as uint64_t,
    5000 as uint64_t,
    10000 as uint64_t,
    50000 as uint64_t,
    100000 as uint64_t,
    250000 as uint64_t,
    500000 as uint64_t,
    1000000 as uint64_t,
    10000000 as uint64_t,
];
unsafe extern "C" fn record_submit_latency(mut start_ns: int64_t) {
    let mut elapsed: int64_t = mono_ns() - start_ns;
    let mut ns: ::core::ffi::c_ulonglong = if elapsed > 0 as int64_t {
        elapsed as ::core::ffi::c_ulonglong
    } else {
        0 as ::core::ffi::c_ulonglong
    };
    let mut bucket: ::core::ffi::c_int =
        OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1 as ::core::ffi::c_int {
        if ns
            <= observation_evidence_submit_latency_bounds_ns[i as usize] as ::core::ffi::c_ulonglong
        {
            bucket = i;
            break;
        } else {
            i += 1;
        }
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        (&raw mut g.submit_latency_bucket as *mut atomic_ullong).offset(bucket as isize),
        1 as ::core::ffi::c_ulonglong,
    );
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut g.submit_latency_sum_ns,
        ns,
    );
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut g.submit_latency_count,
        1 as ::core::ffi::c_ulonglong,
    );
    bump_max_ull(&raw mut g.submit_latency_max_ns, ns);
}
unsafe extern "C" fn stage_hook(
    mut stage: observation_evidence_stage_t,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut g_hook_lock);
    let mut r#fn: observation_evidence_stage_fn = g_stage_hook;
    let mut ctx: *mut ::core::ffi::c_void = g_stage_ctx;
    pthread_mutex_unlock(&raw mut g_hook_lock);
    return if r#fn.is_some() {
        r#fn.expect("non-null function pointer")(stage, path, ctx)
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn camera_enabled(mut camera_uuid: *const ::core::ffi::c_char) -> bool {
    pthread_mutex_lock(&raw mut g_hook_lock);
    let mut r#fn: observation_evidence_enabled_fn = g_enabled_probe;
    let mut ctx: *mut ::core::ffi::c_void = g_enabled_ctx;
    pthread_mutex_unlock(&raw mut g_hook_lock);
    return if r#fn.is_some() {
        r#fn.expect("non-null function pointer")(camera_uuid, ctx) as ::core::ffi::c_int
    } else {
        r#true
    } != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_set_stage_hook(
    mut r#fn: observation_evidence_stage_fn,
    mut ctx: *mut ::core::ffi::c_void,
) {
    pthread_mutex_lock(&raw mut g_hook_lock);
    g_stage_hook = r#fn;
    g_stage_ctx = ctx;
    pthread_mutex_unlock(&raw mut g_hook_lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_set_enabled_probe(
    mut r#fn: observation_evidence_enabled_fn,
    mut ctx: *mut ::core::ffi::c_void,
) {
    pthread_mutex_lock(&raw mut g_hook_lock);
    g_enabled_probe = r#fn;
    g_enabled_ctx = ctx;
    pthread_mutex_unlock(&raw mut g_hook_lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_stream_enabled_probe(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut ctx: *mut ::core::ffi::c_void,
) -> bool {
    let mut stream: *mut stream_config_t =
        calloc(1 as size_t, ::core::mem::size_of::<stream_config_t>()) as *mut stream_config_t;
    if stream.is_null() {
        return r#true != 0;
    }
    let mut enabled: bool = get_stream_config_by_uuid(camera_uuid, stream)
        == 0 as ::core::ffi::c_int
        && (*stream).observation_evidence_enabled as ::core::ffi::c_int != 0;
    free(stream as *mut ::core::ffi::c_void);
    return enabled;
}
unsafe extern "C" fn slot_find(mut camera_uuid: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::Acquire },
    >(&raw mut g_slot_count);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if strcmp(
            &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset(i as isize)).camera_uuid
                as *mut ::core::ffi::c_char,
            camera_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return i;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn slot_get(mut camera_uuid: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut slot: ::core::ffi::c_int = slot_find(camera_uuid);
    if slot >= 0 as ::core::ffi::c_int {
        return slot;
    }
    pthread_mutex_lock(&raw mut g_slot_lock);
    slot = slot_find(camera_uuid);
    if slot < 0 as ::core::ffi::c_int {
        let mut count: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::Relaxed },
        >(&raw mut g_slot_count);
        if count < EVIDENCE_MAX_CAMERAS {
            let mut s: *mut camera_slot_t =
                (&raw mut g_slots as *mut camera_slot_t).offset(count as isize);
            safe_strcpy(
                &raw mut (*s).camera_uuid as *mut ::core::ffi::c_char,
                camera_uuid,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*s).items,
                0 as ::core::ffi::c_int,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*s).bytes,
                0 as ::core::ffi::c_longlong,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*s).budget_exhausted,
                0 as ::core::ffi::c_int,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::Release }>(
                &raw mut g_slot_count,
                count + 1 as ::core::ffi::c_int,
            );
            slot = count;
        }
    }
    pthread_mutex_unlock(&raw mut g_slot_lock);
    return slot;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_set_budget_exhausted(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut exhausted: bool,
) {
    if !lightnvr_uuid_is_valid(camera_uuid) {
        return;
    }
    let mut slot: ::core::ffi::c_int = if exhausted as ::core::ffi::c_int != 0 {
        slot_get(camera_uuid)
    } else {
        slot_find(camera_uuid)
    };
    if slot < 0 as ::core::ffi::c_int {
        if exhausted {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Evidence: camera table full; budget flag for %s not stored\0".as_ptr()
                    as *const ::core::ffi::c_char,
                camera_uuid,
            );
        }
        return;
    }
    let mut previous: bool =
        ::core::intrinsics::atomic_xchg::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset(slot as isize))
                .budget_exhausted,
            if exhausted as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        ) != 0 as ::core::ffi::c_int;
    if previous as ::core::ffi::c_int != exhausted as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: budget_exhausted %s for camera %s\0".as_ptr() as *const ::core::ffi::c_char,
            if exhausted as ::core::ffi::c_int != 0 {
                b"set\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"cleared\0".as_ptr() as *const ::core::ffi::c_char
            },
            camera_uuid,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_is_budget_exhausted(
    mut camera_uuid: *const ::core::ffi::c_char,
) -> bool {
    if camera_uuid.is_null() {
        return r#false != 0;
    }
    let mut slot: ::core::ffi::c_int = slot_find(camera_uuid);
    return slot >= 0 as ::core::ffi::c_int
        && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset(slot as isize))
                .budget_exhausted,
        ) != 0;
}
unsafe extern "C" fn format_number(
    mut value: ::core::ffi::c_double,
    mut out: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    if value.is_finite() as ::core::ffi::c_int == 0 {
        return r#false != 0;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        out,
        size,
        b"%.6g\0".as_ptr() as *const ::core::ffi::c_char,
        value,
    );
    return n > 0 as ::core::ffi::c_int && (n as size_t) < size;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_transform_json(
    mut transform: *const observation_evidence_transform_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    if out.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    *out.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut t: observation_evidence_transform_t = observation_evidence_transform_t {
        mode: observation_transform_mode_t::OBS_TRANSFORM_NONE,
        source_width: 0,
        source_height: 0,
        inference_width: 0,
        inference_height: 0,
        scale_x: 0.,
        scale_y: 0.,
        pad_x: 0.,
        pad_y: 0.,
    };
    if !transform.is_null() {
        t = *transform;
    } else {
        if width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        memset(
            &raw mut t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_evidence_transform_t>(),
        );
        t.mode = observation_transform_mode_t::OBS_TRANSFORM_NONE;
        t.inference_width = width;
        t.source_width = t.inference_width;
        t.inference_height = height;
        t.source_height = t.inference_height;
        t.scale_y = 1.0f64;
        t.scale_x = t.scale_y;
    }
    static mut modes: [*const ::core::ffi::c_char; 3] = [
        b"none\0".as_ptr() as *const ::core::ffi::c_char,
        b"resize\0".as_ptr() as *const ::core::ffi::c_char,
        b"letterbox\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if (t.mode.0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || t.mode.0 as ::core::ffi::c_int
            > observation_transform_mode_t::OBS_TRANSFORM_LETTERBOX.0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if t.source_width <= 0 as ::core::ffi::c_int
        || t.source_height <= 0 as ::core::ffi::c_int
        || t.inference_width <= 0 as ::core::ffi::c_int
        || t.inference_height <= 0 as ::core::ffi::c_int
        || !(t.scale_x > 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        || !(t.scale_y > 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        || !(t.pad_x >= 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        || !(t.pad_y >= 0 as ::core::ffi::c_int as ::core::ffi::c_double)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut sx: [::core::ffi::c_char; 32] = [0; 32];
    let mut sy: [::core::ffi::c_char; 32] = [0; 32];
    let mut px: [::core::ffi::c_char; 32] = [0; 32];
    let mut py: [::core::ffi::c_char; 32] = [0; 32];
    if !format_number(
        t.scale_x,
        &raw mut sx as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) || !format_number(
        t.scale_y,
        &raw mut sy as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) || !format_number(
        t.pad_x,
        &raw mut px as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) || !format_number(
        t.pad_y,
        &raw mut py as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) {
        return -1 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        out,
        out_size,
        b"{\"mode\":\"%s\",\"source_width\":%d,\"source_height\":%d,\"inference_width\":%d,\"inference_height\":%d,\"scale_x\":%s,\"scale_y\":%s,\"pad_x\":%s,\"pad_y\":%s}\0"
            .as_ptr() as *const ::core::ffi::c_char,
        modes[t.mode.0 as usize],
        t.source_width,
        t.source_height,
        t.inference_width,
        t.inference_height,
        &raw mut sx as *mut ::core::ffi::c_char,
        &raw mut sy as *mut ::core::ffi::c_char,
        &raw mut px as *mut ::core::ffi::c_char,
        &raw mut py as *mut ::core::ffi::c_char,
    );
    if n < 0 as ::core::ffi::c_int || n as size_t >= out_size {
        *out.offset(0isize) = '\0' as ::core::ffi::c_char;
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_relative_path(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut asset_uuid: *const ::core::ffi::c_char,
    mut observed_at_ms: int64_t,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    if out.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    *out.offset(0isize) = '\0' as ::core::ffi::c_char;
    if !lightnvr_uuid_is_valid(camera_uuid)
        || !lightnvr_uuid_is_valid(asset_uuid)
        || observed_at_ms < 0 as int64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut seconds: time_t = (observed_at_ms / 1000 as int64_t) as time_t;
    let mut tm: tm = tm {
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
    if gmtime_r(&raw mut seconds, &raw mut tm).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        out,
        out_size,
        b"%s/%s/%04d-%02d-%02d/%s.jpg\0".as_ptr() as *const ::core::ffi::c_char,
        OBS_EVIDENCE_DIR.as_ptr(),
        camera_uuid,
        tm.tm_year + 1900 as ::core::ffi::c_int,
        tm.tm_mon + 1 as ::core::ffi::c_int,
        tm.tm_mday,
        asset_uuid,
    );
    if n < 0 as ::core::ffi::c_int || n as size_t >= out_size {
        *out.offset(0isize) = '\0' as ::core::ffi::c_char;
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ring_create() -> *mut drop_ring_t {
    let mut ring: *mut drop_ring_t =
        calloc(1 as size_t, ::core::mem::size_of::<drop_ring_t>()) as *mut drop_ring_t;
    if ring.is_null() {
        return ::core::ptr::null_mut::<drop_ring_t>();
    }
    let mut i: size_t = 0 as size_t;
    while i < OBS_EVIDENCE_DROP_RING_SIZE as size_t {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
            &raw mut (*(&raw mut (*ring).cells as *mut drop_cell_t).offset(i as isize)).seq,
            i,
        );
        i = i.wrapping_add(1);
    }
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ring).enqueue_pos,
        0 as size_t,
    );
    return ring;
}
unsafe extern "C" fn ring_push(
    mut ring: *mut drop_ring_t,
    mut batch: *const observation_batch_input_t,
    mut asset_uuid: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut discard: *mut uint8_t,
    mut discard_charge: size_t,
) -> bool {
    if ring.is_null() {
        return r#false != 0;
    }
    let mut pos: size_t = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::Relaxed },
    >(&raw mut (*ring).enqueue_pos);
    let mut cell: *mut drop_cell_t = ::core::ptr::null_mut::<drop_cell_t>();
    loop {
        cell = (&raw mut (*ring).cells as *mut drop_cell_t).offset(
            (pos & (OBS_EVIDENCE_DROP_RING_SIZE - 1 as ::core::ffi::c_int) as size_t) as isize,
        );
        let mut seq: size_t = ::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::Acquire },
        >(&raw mut (*cell).seq);
        let mut diff: intptr_t = seq as intptr_t - pos as intptr_t;
        if diff == 0 as intptr_t {
            let c2rust_result = ::core::intrinsics::atomic_cxchgweak::<
                _,
                { ::core::intrinsics::AtomicOrdering::Relaxed },
                { ::core::intrinsics::AtomicOrdering::Relaxed },
            >(
                &raw mut (*ring).enqueue_pos,
                *&raw mut pos,
                pos.wrapping_add(1 as size_t),
            );
            *&raw mut pos = c2rust_result.0;
            if c2rust_result.1 {
                break;
            }
        } else if diff < 0 as intptr_t {
            return r#false != 0;
        } else {
            pos = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::Relaxed },
            >(&raw mut (*ring).enqueue_pos);
        }
    }
    (*cell).record.batch = *batch;
    safe_strcpy(
        &raw mut (*cell).record.asset_uuid as *mut ::core::ffi::c_char,
        asset_uuid,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*cell).record.reason as *mut ::core::ffi::c_char,
        reason,
        ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
        0 as size_t,
    );
    (*cell).record.discard = discard;
    (*cell).record.discard_charge = discard_charge;
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.drop_pending,
        1 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::Release }>(
        &raw mut (*cell).seq,
        pos.wrapping_add(1 as size_t),
    );
    return r#true != 0;
}
unsafe extern "C" fn ring_pop(mut ring: *mut drop_ring_t, mut out: *mut drop_record_t) -> bool {
    if ring.is_null() {
        return r#false != 0;
    }
    let mut cell: *mut drop_cell_t = (&raw mut (*ring).cells as *mut drop_cell_t).offset(
        ((*ring).dequeue_pos & (OBS_EVIDENCE_DROP_RING_SIZE - 1 as ::core::ffi::c_int) as size_t)
            as isize,
    );
    let mut seq: size_t = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::Acquire },
    >(&raw mut (*cell).seq);
    if (seq as intptr_t - (*ring).dequeue_pos.wrapping_add(1 as size_t) as intptr_t) < 0 as intptr_t
    {
        return r#false != 0;
    }
    *out = (*cell).record;
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::Release }>(
        &raw mut (*cell).seq,
        (*ring)
            .dequeue_pos
            .wrapping_add(OBS_EVIDENCE_DROP_RING_SIZE as size_t),
    );
    (*ring).dequeue_pos = (*ring).dequeue_pos.wrapping_add(1);
    return r#true != 0;
}
unsafe extern "C" fn release_item(mut item: *mut evidence_item_t) {
    if (*item).slot >= 0 as ::core::ffi::c_int {
        ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset((*item).slot as isize))
                .items,
            1 as ::core::ffi::c_int,
        );
        ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset((*item).slot as isize))
                .bytes,
            (*item).charge as ::core::ffi::c_longlong,
        );
    }
    ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.items,
        1 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.bytes,
        (*item).charge as ::core::ffi::c_longlong,
    );
    free((*item).data as *mut ::core::ffi::c_void);
    free(item as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn insert_unavailable(
    mut batch: *const observation_batch_input_t,
    mut asset_uuid: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut asset: observation_asset_input_t = observation_asset_input_t {
        asset_uuid: [0; 37],
        state: observation_asset_state_t::OBS_ASSET_PENDING,
        reason: [0; 33],
        origin: [0; 24],
        media_type: [0; 24],
        captured_at_ms: 0,
        transform_json: [0; 1024],
        expires_at_ms: 0,
    };
    memset(
        &raw mut asset as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_asset_input_t>(),
    );
    safe_strcpy(
        &raw mut asset.asset_uuid as *mut ::core::ffi::c_char,
        asset_uuid,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    asset.state = observation_asset_state_t::OBS_ASSET_UNAVAILABLE;
    safe_strcpy(
        &raw mut asset.reason as *mut ::core::ffi::c_char,
        reason,
        ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
        0 as size_t,
    );
    let mut result: observation_insert_result_t = observation_insert_result_t {
        asset_uuid: [0; 37],
        object_count: 0,
        observation_uuids: [[0; 37]; 20],
        last_seq: 0,
    };
    let mut rc: ::core::ffi::c_int =
        db_observation_insert_batch(batch, &raw mut asset, &raw mut result);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 && rc != C2Rust_Unnamed::DB_OBS_DUPLICATE.0 {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.failed_db,
            1 as ::core::ffi::c_ulonglong,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: recording batch %s as unavailable(%s) failed: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*batch).batch_id as *const ::core::ffi::c_char,
            reason,
            rc,
        );
    }
}
unsafe extern "C" fn process_drops() {
    let mut ring: *mut drop_ring_t = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.ring);
    let mut record: *mut drop_record_t = ::core::ptr::null_mut::<drop_record_t>();
    while !ring.is_null() {
        if record.is_null() {
            record = malloc(::core::mem::size_of::<drop_record_t>()) as *mut drop_record_t;
            if record.is_null() {
                return;
            }
        }
        if !ring_pop(ring, record) {
            break;
        }
        if !(*record).discard.is_null() {
            free((*record).discard as *mut ::core::ffi::c_void);
            ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.discard_bytes,
                (*record).discard_charge as ::core::ffi::c_longlong,
            );
        }
        insert_unavailable(
            &raw mut (*record).batch,
            &raw mut (*record).asset_uuid as *mut ::core::ffi::c_char,
            &raw mut (*record).reason as *mut ::core::ffi::c_char,
        );
        ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.drop_pending,
            1 as ::core::ffi::c_int,
        );
    }
    free(record as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn transition_failed(
    mut item: *mut evidence_item_t,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut result: observation_transition_result_t = observation_transition_result_t {
        observations_bumped: 0,
        last_seq: 0,
    };
    let mut rc: ::core::ffi::c_int = db_observation_asset_transition(
        &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
        observation_asset_state_t::OBS_ASSET_PENDING,
        observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
        reason,
        ::core::ptr::null::<observation_asset_media_t>(),
        &raw mut result,
    );
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 && rc != C2Rust_Unnamed::DB_OBS_CAS_LOST.0 {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.failed_db,
            1 as ::core::ffi::c_ulonglong,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: asset %s pending -> unavailable(%s) failed: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
            reason,
            rc,
        );
    }
}
unsafe extern "C" fn mkdir_parents(mut path: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 512] = [0; 512];
    if safe_strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        path,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut p: *mut ::core::ffi::c_char =
        (&raw mut buf as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize);
    while *p != 0 {
        if *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            *p = '\0' as ::core::ffi::c_char;
            if mkdir(&raw mut buf as *mut ::core::ffi::c_char, 0o750 as __mode_t)
                != 0 as ::core::ffi::c_int
                && *__errno_location() != EEXIST
            {
                return -1 as ::core::ffi::c_int;
            }
            *p = '/' as ::core::ffi::c_char;
        }
        p = p.offset(1);
    }
    if mkdir(&raw mut buf as *mut ::core::ffi::c_char, 0o750 as __mode_t) != 0 as ::core::ffi::c_int
        && *__errno_location() != EEXIST
    {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fsync_dir(mut dir: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = open(dir, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if fd < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = fsync(fd);
    close(fd);
    return rc;
}
unsafe extern "C" fn write_atomic(
    mut dir: *const ::core::ffi::c_char,
    mut final_path: *const ::core::ffi::c_char,
    mut data: *const uint8_t,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut tmp_path: [::core::ffi::c_char; 520] = [0; 520];
    let mut n: ::core::ffi::c_int = snprintf(
        &raw mut tmp_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 520]>(),
        b"%s.tmp\0".as_ptr() as *const ::core::ffi::c_char,
        final_path,
    );
    if n < 0 as ::core::ffi::c_int
        || n as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 520]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    if mkdir_parents(dir) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: cannot create %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            dir,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut fd: ::core::ffi::c_int = open(
        &raw mut tmp_path as *mut ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC | O_NOFOLLOW,
        0o640 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: cannot create %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut tmp_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut written: size_t = 0 as size_t;
    while written < size {
        let mut w: ssize_t = write(
            fd,
            data.offset(written as isize) as *const ::core::ffi::c_void,
            size.wrapping_sub(written),
        );
        if w < 0 as ssize_t && *__errno_location() == EINTR {
            continue;
        }
        if w <= 0 as ssize_t {
            break;
        }
        written = written.wrapping_add(w as size_t);
    }
    let mut ok: bool = written == size && fsync(fd) == 0 as ::core::ffi::c_int;
    if close(fd) != 0 as ::core::ffi::c_int {
        ok = r#false != 0;
    }
    if ok as ::core::ffi::c_int != 0
        && stage_hook(
            observation_evidence_stage_t::OBS_EVIDENCE_STAGE_BEFORE_RENAME,
            final_path,
        ) != 0 as ::core::ffi::c_int
    {
        ok = r#false != 0;
    }
    if !ok {
        unlink(&raw mut tmp_path as *mut ::core::ffi::c_char);
        return -1 as ::core::ffi::c_int;
    }
    if rename(&raw mut tmp_path as *mut ::core::ffi::c_char, final_path) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: rename %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut tmp_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        unlink(&raw mut tmp_path as *mut ::core::ffi::c_char);
        return -1 as ::core::ffi::c_int;
    }
    if fsync_dir(dir) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: fsync of %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            dir,
            strerror(*__errno_location()),
        );
        unlink(final_path);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn encode_rgb(
    mut item: *mut evidence_item_t,
    mut jpeg: *mut *mut ::core::ffi::c_uchar,
    mut jpeg_size: *mut size_t,
) -> ::core::ffi::c_int {
    if g.encoder.is_null() || g.encoder_w != (*item).width || g.encoder_h != (*item).height {
        if !g.encoder.is_null() {
            jpeg_encoder_cache_destroy(g.encoder);
        }
        g.encoder = jpeg_encoder_cache_create(
            (*item).width,
            (*item).height,
            3 as ::core::ffi::c_int,
            OBS_EVIDENCE_JPEG_QUALITY,
        );
        g.encoder_w = if !g.encoder.is_null() {
            (*item).width
        } else {
            0 as ::core::ffi::c_int
        };
        g.encoder_h = if !g.encoder.is_null() {
            (*item).height
        } else {
            0 as ::core::ffi::c_int
        };
        if g.encoder.is_null() {
            return -1 as ::core::ffi::c_int;
        }
    }
    return jpeg_encoder_cache_encode_to_memory(
        g.encoder,
        (*item).data as *const ::core::ffi::c_uchar,
        jpeg,
        jpeg_size,
    );
}
unsafe extern "C" fn clear_media_fields(mut asset: *mut observation_asset_input_t) {
    (*asset).origin[0usize] = '\0' as ::core::ffi::c_char;
    (*asset).media_type[0usize] = '\0' as ::core::ffi::c_char;
    (*asset).transform_json[0usize] = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn abandon_item(mut item: *mut evidence_item_t) {
    clear_media_fields(&raw mut (*item).asset);
    insert_unavailable(
        &raw mut (*item).batch,
        &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
        b"interrupted\0".as_ptr() as *const ::core::ffi::c_char,
    );
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.interrupted,
        1 as ::core::ffi::c_ulonglong,
    );
}
unsafe extern "C" fn process_item(mut item: *mut evidence_item_t) {
    let mut camera: *const ::core::ffi::c_char =
        &raw mut (*item).batch.camera_uuid as *mut ::core::ffi::c_char;
    if !camera_enabled(camera) {
        clear_media_fields(&raw mut (*item).asset);
        insert_unavailable(
            &raw mut (*item).batch,
            &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.disabled_at_encode,
            1 as ::core::ffi::c_ulonglong,
        );
        return;
    }
    let mut inserted: observation_insert_result_t = observation_insert_result_t {
        asset_uuid: [0; 37],
        object_count: 0,
        observation_uuids: [[0; 37]; 20],
        last_seq: 0,
    };
    let mut rc: ::core::ffi::c_int = db_observation_insert_batch(
        &raw mut (*item).batch,
        &raw mut (*item).asset,
        &raw mut inserted,
    );
    if rc == C2Rust_Unnamed::DB_OBS_DUPLICATE.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: batch %s already recorded; dropping its frame\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*item).batch.batch_id as *mut ::core::ffi::c_char,
        );
        return;
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.failed_db,
            1 as ::core::ffi::c_ulonglong,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: inserting batch %s failed: %d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*item).batch.batch_id as *mut ::core::ffi::c_char,
            rc,
        );
        return;
    }
    stage_hook(
        observation_evidence_stage_t::OBS_EVIDENCE_STAGE_AFTER_INSERT,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut relative: [::core::ffi::c_char; 512] = [0; 512];
    let mut final_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut dir: [::core::ffi::c_char; 512] = [0; 512];
    if observation_evidence_relative_path(
        camera,
        &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
        (*item).batch.observed_at_ms,
        &raw mut relative as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
        || snprintf(
            &raw mut final_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g.storage_path as *mut ::core::ffi::c_char,
            &raw mut relative as *mut ::core::ffi::c_char,
        ) >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
    {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.failed_write,
            1 as ::core::ffi::c_ulonglong,
        );
        transition_failed(
            item,
            b"write_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut dir as *mut ::core::ffi::c_char,
        &raw mut final_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut slash: *mut ::core::ffi::c_char = strrchr(
        &raw mut dir as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !slash.is_null() {
        *slash = '\0' as ::core::ffi::c_char;
    }
    let mut jpeg: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jpeg_size: size_t = 0 as size_t;
    if (*item).is_jpeg {
        jpeg = (*item).data as *mut ::core::ffi::c_uchar;
        jpeg_size = (*item).data_size;
    } else if encode_rgb(item, &raw mut jpeg, &raw mut jpeg_size) != 0 as ::core::ffi::c_int
        || jpeg.is_null()
        || jpeg_size == 0 as size_t
    {
        free(jpeg as *mut ::core::ffi::c_void);
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.failed_encode,
            1 as ::core::ffi::c_ulonglong,
        );
        transition_failed(
            item,
            b"encode_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else {
        free((*item).data as *mut ::core::ffi::c_void);
        (*item).data = ::core::ptr::null_mut::<uint8_t>();
    }
    let mut write_rc: ::core::ffi::c_int = write_atomic(
        &raw mut dir as *mut ::core::ffi::c_char,
        &raw mut final_path as *mut ::core::ffi::c_char,
        jpeg as *const uint8_t,
        jpeg_size,
    );
    if !(*item).is_jpeg {
        free(jpeg as *mut ::core::ffi::c_void);
    }
    if write_rc != 0 as ::core::ffi::c_int {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.failed_write,
            1 as ::core::ffi::c_ulonglong,
        );
        transition_failed(
            item,
            b"write_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut media: observation_asset_media_t = observation_asset_media_t {
        path: &raw mut relative as *mut ::core::ffi::c_char,
        bytes: jpeg_size as int64_t,
        width: (*item).width,
        height: (*item).height,
        expires_at_ms: 0 as int64_t,
    };
    let mut result: observation_transition_result_t = observation_transition_result_t {
        observations_bumped: 0,
        last_seq: 0,
    };
    stage_hook(
        observation_evidence_stage_t::OBS_EVIDENCE_STAGE_BEFORE_COMMIT,
        &raw mut final_path as *mut ::core::ffi::c_char,
    );
    rc = db_observation_asset_transition(
        &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
        observation_asset_state_t::OBS_ASSET_PENDING,
        observation_asset_state_t::OBS_ASSET_READY,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut media,
        &raw mut result,
    );
    if rc == C2Rust_Unnamed::DB_OBS_ERROR.0 {
        rc = db_observation_asset_transition(
            &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
            observation_asset_state_t::OBS_ASSET_PENDING,
            observation_asset_state_t::OBS_ASSET_READY,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut media,
            &raw mut result,
        );
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        unlink(&raw mut final_path as *mut ::core::ffi::c_char);
        fsync_dir(&raw mut dir as *mut ::core::ffi::c_char);
        if rc != C2Rust_Unnamed::DB_OBS_CAS_LOST.0 {
            ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.failed_db,
                1 as ::core::ffi::c_ulonglong,
            );
            transition_failed(
                item,
                b"write_failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evidence: asset %s pending -> ready failed: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
            rc,
        );
        return;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.ready,
        1 as ::core::ffi::c_ulonglong,
    );
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.bytes_written,
        jpeg_size as ::core::ffi::c_ulonglong,
    );
}
unsafe extern "C" fn take_all_fifo() -> *mut evidence_item_t {
    let mut list: *mut evidence_item_t =
        ::core::intrinsics::atomic_xchg::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.head,
            ::core::ptr::null_mut::<evidence_item_t>(),
        );
    let mut fifo: *mut evidence_item_t = ::core::ptr::null_mut::<evidence_item_t>();
    while !list.is_null() {
        let mut next: *mut evidence_item_t = (*list).next;
        (*list).next = fifo as *mut evidence_item;
        fifo = list;
        list = next;
    }
    return fifo;
}
unsafe extern "C" fn drain_expired() -> bool {
    return ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.stop,
    ) != 0
        && mono_ns()
            >= ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.drain_deadline_ns,
            );
}
unsafe extern "C" fn run_items(mut list: *mut evidence_item_t) {
    while !list.is_null() {
        let mut item: *mut evidence_item_t = list;
        list = (*list).next as *mut evidence_item_t;
        stage_hook(
            observation_evidence_stage_t::OBS_EVIDENCE_STAGE_ITEM_BEGIN,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if drain_expired() {
            abandon_item(item);
        } else {
            process_item(item);
        }
        stage_hook(
            observation_evidence_stage_t::OBS_EVIDENCE_STAGE_ITEM_DONE,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        release_item(item);
        process_drops();
    }
}
unsafe extern "C" fn evidence_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    g.component_id = register_component(
        b"observation_evidence\0".as_ptr() as *const ::core::ffi::c_char,
        component_type_t::COMPONENT_OTHER,
        NULL,
        20 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.worker_running,
        1 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Observation evidence worker started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    loop {
        process_drops();
        let mut list: *mut evidence_item_t = take_all_fifo();
        if !list.is_null() {
            run_items(list);
        } else {
            if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.stop,
            ) != 0
            {
                break;
            }
            let mut deadline: timespec = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            clock_gettime(CLOCK_REALTIME, &raw mut deadline);
            deadline.tv_nsec +=
                EVIDENCE_WAIT_MS as ::core::ffi::c_long * 1000000 as ::core::ffi::c_long;
            if deadline.tv_nsec >= 1000000000 as ::core::ffi::c_long {
                deadline.tv_sec += 1 as __time_t;
                deadline.tv_nsec -= 1000000000 as ::core::ffi::c_long;
            }
            while sem_timedwait(&raw mut g.wake, &raw mut deadline) != 0 as ::core::ffi::c_int
                && *__errno_location() == EINTR
            {}
        }
    }
    if !g.encoder.is_null() {
        jpeg_encoder_cache_destroy(g.encoder);
        g.encoder = ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
        g.encoder_h = 0 as ::core::ffi::c_int;
        g.encoder_w = g.encoder_h;
    }
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.worker_running,
        0 as ::core::ffi::c_int,
    );
    if g.component_id >= 0 as ::core::ffi::c_int {
        update_component_state(g.component_id, component_state_t::COMPONENT_STOPPED);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Observation evidence worker stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL;
}
unsafe extern "C" fn ensure_started() {
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.lifecycle,
    ) == C2Rust_Unnamed_2::LC_RUNNING.0 as ::core::ffi::c_int
    {
        return;
    }
    if pthread_mutex_trylock(&raw mut g.start_lock) != 0 as ::core::ffi::c_int {
        return;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.lifecycle,
    ) == C2Rust_Unnamed_2::LC_READY.0 as ::core::ffi::c_int
        && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.thread_started,
        ) == 0
    {
        let mut ring: *mut drop_ring_t = ring_create();
        if !ring.is_null() {
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.ring,
                ring,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.stop,
                0 as ::core::ffi::c_int,
            );
            if pthread_create(
                &raw mut g.thread,
                ::core::ptr::null::<pthread_attr_t>(),
                Some(
                    evidence_worker
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
                NULL,
            ) == 0 as ::core::ffi::c_int
            {
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut g.thread_started,
                    1 as ::core::ffi::c_int,
                );
                let mut expected: ::core::ffi::c_int =
                    C2Rust_Unnamed_2::LC_READY.0 as ::core::ffi::c_int;
                let c2rust_result = ::core::intrinsics::atomic_cxchg::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(
                    &raw mut g.lifecycle,
                    *&raw mut expected,
                    C2Rust_Unnamed_2::LC_RUNNING.0 as ::core::ffi::c_int,
                );
                *&raw mut expected = c2rust_result.0;
                c2rust_result.1;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Evidence: cannot start worker thread\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut g.ring,
                    ::core::ptr::null_mut::<drop_ring_t>(),
                );
                free(ring as *mut ::core::ffi::c_void);
            }
        }
    }
    pthread_mutex_unlock(&raw mut g.start_lock);
}
unsafe extern "C" fn enter() -> bool {
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.inflight,
        1 as ::core::ffi::c_int,
    );
    let mut lc: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.lifecycle);
    if lc == C2Rust_Unnamed_2::LC_READY.0 as ::core::ffi::c_int
        || lc == C2Rust_Unnamed_2::LC_RUNNING.0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.inflight,
        1 as ::core::ffi::c_int,
    );
    return r#false != 0;
}
unsafe extern "C" fn leave() {
    ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.inflight,
        1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn reserve_counter(
    mut counter: *mut atomic_int,
    mut limit: ::core::ffi::c_int,
) -> bool {
    if ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        counter,
        1 as ::core::ffi::c_int,
    ) + 1 as ::core::ffi::c_int
        > limit
    {
        ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            counter,
            1 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn reserve_bytes(
    mut counter: *mut atomic_llong,
    mut charge: ::core::ffi::c_longlong,
    mut limit: ::core::ffi::c_longlong,
) -> bool {
    if ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        counter, charge,
    ) + charge
        > limit
    {
        ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            counter, charge,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn reserve(mut slot: ::core::ffi::c_int, mut charge: size_t) -> bool {
    let mut c: ::core::ffi::c_longlong = charge as ::core::ffi::c_longlong;
    if !reserve_counter(&raw mut g.items, g.global_max_items) {
        return r#false != 0;
    }
    '_undo_items: {
        if reserve_bytes(
            &raw mut g.bytes,
            c,
            g.global_max_bytes as ::core::ffi::c_longlong,
        ) {
            's_39: {
                if slot >= 0 as ::core::ffi::c_int {
                    if reserve_counter(
                        &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset(slot as isize))
                            .items,
                        g.per_camera_max_items,
                    ) {
                        if !reserve_bytes(
                            &raw mut (*(&raw mut g_slots as *mut camera_slot_t)
                                .offset(slot as isize))
                            .bytes,
                            c,
                            g.per_camera_max_bytes as ::core::ffi::c_longlong,
                        ) {
                            ::core::intrinsics::atomic_xsub::<
                                _,
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(
                                &raw mut (*(&raw mut g_slots as *mut camera_slot_t)
                                    .offset(slot as isize))
                                .items,
                                1 as ::core::ffi::c_int,
                            );
                        } else {
                            bump_max_uint(
                                &raw mut g.camera_high_water_items,
                                ::core::intrinsics::atomic_load::<
                                    _,
                                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                                >(
                                    &raw mut (*(&raw mut g_slots as *mut camera_slot_t)
                                        .offset(slot as isize))
                                    .items,
                                ) as ::core::ffi::c_uint,
                            );
                            break 's_39;
                        }
                    }
                    ::core::intrinsics::atomic_xsub::<
                        _,
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut g.bytes, c);
                    break '_undo_items;
                }
            }
            return r#true != 0;
        }
    }
    ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.items,
        1 as ::core::ffi::c_int,
    );
    return r#false != 0;
}
unsafe extern "C" fn wake_worker() {
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.thread_started,
    ) != 0
    {
        sem_post(&raw mut g.wake);
    }
}
unsafe extern "C" fn record_drop(
    mut batch: *const observation_batch_input_t,
    mut asset_uuid: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut discard: *mut uint8_t,
    mut discard_size: size_t,
) -> bool {
    let mut charge: size_t = 0 as size_t;
    if !discard.is_null() {
        if reserve_bytes(
            &raw mut g.discard_bytes,
            discard_size as ::core::ffi::c_longlong,
            g.discard_max_bytes as ::core::ffi::c_longlong,
        ) {
            charge = discard_size;
        } else {
            free(discard as *mut ::core::ffi::c_void);
            discard = ::core::ptr::null_mut::<uint8_t>();
            ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.discard_inline,
                1 as ::core::ffi::c_ulonglong,
            );
        }
    }
    if ring_push(
        ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.ring,
        ),
        batch,
        asset_uuid,
        reason,
        discard,
        charge,
    ) {
        if !discard.is_null() {
            ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.discard_deferred,
                1 as ::core::ffi::c_ulonglong,
            );
        }
        wake_worker();
        return r#true != 0;
    }
    if !discard.is_null() {
        ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.discard_bytes,
            charge as ::core::ffi::c_longlong,
        );
        free(discard as *mut ::core::ffi::c_void);
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.discard_inline,
            1 as ::core::ffi::c_ulonglong,
        );
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.dropped_untracked,
        1 as ::core::ffi::c_ulonglong,
    );
    return r#false != 0;
}
unsafe extern "C" fn batch_valid(mut batch: *const observation_batch_input_t) -> bool {
    return !batch.is_null()
        && lightnvr_uuid_is_valid(&raw const (*batch).camera_uuid as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        && lightnvr_uuid_is_valid(&raw const (*batch).batch_id as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        && (*batch).object_count >= 1 as ::core::ffi::c_int
        && (*batch).object_count <= OBS_MAX_BATCH
        && (*batch).observed_at_ms >= 0 as int64_t;
}
unsafe extern "C" fn admit(
    mut batch: *const observation_batch_input_t,
    mut data: *mut uint8_t,
    mut data_size: size_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut is_jpeg: bool,
    mut transform: *const observation_evidence_transform_t,
    mut asset_uuid_out: *mut ::core::ffi::c_char,
) -> observation_evidence_result_t {
    if !asset_uuid_out.is_null() {
        *asset_uuid_out.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !enter() {
        free(data as *mut ::core::ffi::c_void);
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.dropped_disabled,
            1 as ::core::ffi::c_ulonglong,
        );
        return observation_evidence_result_t::OBS_EVIDENCE_DISABLED;
    }
    let mut result: observation_evidence_result_t =
        observation_evidence_result_t::OBS_EVIDENCE_INVALID;
    let mut transform_json: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut asset_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !batch_valid(batch)
        || data.is_null()
        || data_size == 0 as size_t
        || width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || observation_evidence_transform_json(
            transform,
            width,
            height,
            &raw mut transform_json as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ) != 0 as ::core::ffi::c_int
        || lightnvr_uuid_generate_v4(&raw mut asset_uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        free(data as *mut ::core::ffi::c_void);
        leave();
        return observation_evidence_result_t::OBS_EVIDENCE_INVALID;
    }
    ensure_started();
    let mut slot: ::core::ffi::c_int =
        slot_get(&raw const (*batch).camera_uuid as *const ::core::ffi::c_char);
    let mut charge: size_t = data_size
        .wrapping_add(
            (if is_jpeg as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                RGB24_SWS_TAIL_PADDING
            }) as size_t,
        )
        .wrapping_add(::core::mem::size_of::<evidence_item_t>());
    let mut item: *mut evidence_item_t = ::core::ptr::null_mut::<evidence_item_t>();
    if slot >= 0 as ::core::ffi::c_int
        && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset(slot as isize))
                .budget_exhausted,
        ) != 0
    {
        result = observation_evidence_result_t::OBS_EVIDENCE_DROPPED_BUDGET;
    } else if !reserve(slot, charge) {
        result = observation_evidence_result_t::OBS_EVIDENCE_DROPPED_QUEUE_FULL;
    } else {
        item = malloc(::core::mem::size_of::<evidence_item_t>()) as *mut evidence_item_t;
        if item.is_null() {
            if slot >= 0 as ::core::ffi::c_int {
                ::core::intrinsics::atomic_xsub::<
                    _,
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(
                    &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset(slot as isize))
                        .items,
                    1 as ::core::ffi::c_int,
                );
                ::core::intrinsics::atomic_xsub::<
                    _,
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(
                    &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset(slot as isize))
                        .bytes,
                    charge as ::core::ffi::c_longlong,
                );
            }
            ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.items,
                1 as ::core::ffi::c_int,
            );
            ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut g.bytes,
                charge as ::core::ffi::c_longlong,
            );
            result = observation_evidence_result_t::OBS_EVIDENCE_DROPPED_QUEUE_FULL;
        } else {
            result = observation_evidence_result_t::OBS_EVIDENCE_ACCEPTED;
        }
    }
    if result.0 != observation_evidence_result_t::OBS_EVIDENCE_ACCEPTED.0 {
        let mut budget: bool =
            result.0 == observation_evidence_result_t::OBS_EVIDENCE_DROPPED_BUDGET.0;
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            if budget as ::core::ffi::c_int != 0 {
                &raw mut g.dropped_budget
            } else {
                &raw mut g.dropped_queue_full
            },
            1 as ::core::ffi::c_ulonglong,
        );
        if record_drop(
            batch,
            &raw mut asset_uuid as *mut ::core::ffi::c_char,
            if budget as ::core::ffi::c_int != 0 {
                b"budget_exhausted\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"queue_full\0".as_ptr() as *const ::core::ffi::c_char
            },
            data,
            data_size,
        ) as ::core::ffi::c_int
            != 0
            && !asset_uuid_out.is_null()
        {
            safe_strcpy(
                asset_uuid_out,
                &raw mut asset_uuid as *mut ::core::ffi::c_char,
                OBS_UUID_SIZE as size_t,
                0 as size_t,
            );
        }
        leave();
        return result;
    }
    memset(
        item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        48 as size_t,
    );
    (*item).is_jpeg = is_jpeg;
    (*item).slot = slot;
    (*item).charge = charge;
    (*item).data = data;
    (*item).data_size = data_size;
    (*item).width = width;
    (*item).height = height;
    (*item).batch = *batch;
    memset(
        &raw mut (*item).asset as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_asset_input_t>(),
    );
    safe_strcpy(
        &raw mut (*item).asset.asset_uuid as *mut ::core::ffi::c_char,
        &raw mut asset_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*item).asset.state = observation_asset_state_t::OBS_ASSET_PENDING;
    safe_strcpy(
        &raw mut (*item).asset.origin as *mut ::core::ffi::c_char,
        if is_jpeg as ::core::ffi::c_int != 0 {
            b"submitted_image\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"inference_input\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*item).asset.media_type as *mut ::core::ffi::c_char,
        b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    (*item).asset.captured_at_ms = (*batch).observed_at_ms;
    safe_strcpy(
        &raw mut (*item).asset.transform_json as *mut ::core::ffi::c_char,
        &raw mut transform_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        0 as size_t,
    );
    let mut head: *mut evidence_item_t = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.head);
    loop {
        (*item).next = head as *mut evidence_item;
        let c2rust_result = ::core::intrinsics::atomic_cxchgweak::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut g.head, *&raw mut head, item);
        *&raw mut head = c2rust_result.0;
        if c2rust_result.1 {
            break;
        }
    }
    bump_max_uint(
        &raw mut g.high_water_items,
        ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.items,
        ) as ::core::ffi::c_uint,
    );
    bump_max_ull(
        &raw mut g.high_water_bytes,
        ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.bytes,
        ) as ::core::ffi::c_ulonglong,
    );
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.accepted,
        1 as ::core::ffi::c_ulonglong,
    );
    if !asset_uuid_out.is_null() {
        safe_strcpy(
            asset_uuid_out,
            &raw mut asset_uuid as *mut ::core::ffi::c_char,
            OBS_UUID_SIZE as size_t,
            0 as size_t,
        );
    }
    wake_worker();
    leave();
    return observation_evidence_result_t::OBS_EVIDENCE_ACCEPTED;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_submit(
    mut batch: *const observation_batch_input_t,
    mut rgb24: *mut uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut transform: *const observation_evidence_transform_t,
    mut asset_uuid_out: *mut ::core::ffi::c_char,
) -> observation_evidence_result_t {
    let mut start: int64_t = mono_ns();
    let mut size: size_t = if width > 0 as ::core::ffi::c_int && height > 0 as ::core::ffi::c_int {
        (width as size_t)
            .wrapping_mul(height as size_t)
            .wrapping_mul(3 as size_t)
    } else {
        0 as size_t
    };
    let mut rc: observation_evidence_result_t = admit(
        batch,
        rgb24,
        size,
        width,
        height,
        r#false != 0,
        transform,
        asset_uuid_out,
    );
    if rc.0 != observation_evidence_result_t::OBS_EVIDENCE_DISABLED.0 {
        record_submit_latency(start);
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_submit_jpeg(
    mut batch: *const observation_batch_input_t,
    mut jpeg: *mut uint8_t,
    mut jpeg_size: size_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut transform: *const observation_evidence_transform_t,
    mut asset_uuid_out: *mut ::core::ffi::c_char,
) -> observation_evidence_result_t {
    let mut start: int64_t = mono_ns();
    let mut rc: observation_evidence_result_t = admit(
        batch,
        jpeg,
        jpeg_size,
        width,
        height,
        r#true != 0,
        transform,
        asset_uuid_out,
    );
    if rc.0 != observation_evidence_result_t::OBS_EVIDENCE_DISABLED.0 {
        record_submit_latency(start);
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_submit_unavailable(
    mut batch: *const observation_batch_input_t,
    mut reason: *const ::core::ffi::c_char,
    mut asset_uuid_out: *mut ::core::ffi::c_char,
) -> observation_evidence_result_t {
    let mut start: int64_t = mono_ns();
    if !asset_uuid_out.is_null() {
        *asset_uuid_out.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !enter() {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.dropped_disabled,
            1 as ::core::ffi::c_ulonglong,
        );
        return observation_evidence_result_t::OBS_EVIDENCE_DISABLED;
    }
    let mut asset_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !batch_valid(batch)
        || reason.is_null()
        || strcmp(
            reason,
            b"no_matching_media\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || lightnvr_uuid_generate_v4(&raw mut asset_uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        leave();
        record_submit_latency(start);
        return observation_evidence_result_t::OBS_EVIDENCE_INVALID;
    }
    ensure_started();
    let mut result: observation_evidence_result_t =
        observation_evidence_result_t::OBS_EVIDENCE_DROPPED_QUEUE_FULL;
    if record_drop(
        batch,
        &raw mut asset_uuid as *mut ::core::ffi::c_char,
        reason,
        ::core::ptr::null_mut::<uint8_t>(),
        0 as size_t,
    ) {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.unavailable_submitted,
            1 as ::core::ffi::c_ulonglong,
        );
        if !asset_uuid_out.is_null() {
            safe_strcpy(
                asset_uuid_out,
                &raw mut asset_uuid as *mut ::core::ffi::c_char,
                OBS_UUID_SIZE as size_t,
                0 as size_t,
            );
        }
        result = observation_evidence_result_t::OBS_EVIDENCE_ACCEPTED;
    }
    leave();
    record_submit_latency(start);
    return result;
}
unsafe extern "C" fn reset_counters() {
    let mut counters: [*mut atomic_ullong; 19] = [
        &raw mut g.accepted,
        &raw mut g.dropped_queue_full,
        &raw mut g.dropped_budget,
        &raw mut g.dropped_disabled,
        &raw mut g.dropped_untracked,
        &raw mut g.unavailable_submitted,
        &raw mut g.ready,
        &raw mut g.failed_encode,
        &raw mut g.failed_write,
        &raw mut g.failed_db,
        &raw mut g.disabled_at_encode,
        &raw mut g.interrupted,
        &raw mut g.bytes_written,
        &raw mut g.high_water_bytes,
        &raw mut g.submit_latency_count,
        &raw mut g.submit_latency_sum_ns,
        &raw mut g.submit_latency_max_ns,
        &raw mut g.discard_deferred,
        &raw mut g.discard_inline,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*mut atomic_ullong; 19]>()
        .wrapping_div(::core::mem::size_of::<*mut atomic_ullong>())
    {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            counters[i],
            0 as ::core::ffi::c_ulonglong,
        );
        i = i.wrapping_add(1);
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            (&raw mut g.submit_latency_bucket as *mut atomic_ullong).offset(i_0 as isize),
            0 as ::core::ffi::c_ulonglong,
        );
        i_0 += 1;
    }
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.high_water_items,
        0 as ::core::ffi::c_uint,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.camera_high_water_items,
        0 as ::core::ffi::c_uint,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_init(
    mut config: *const observation_evidence_config_t,
) -> ::core::ffi::c_int {
    if config.is_null()
        || (*config).storage_path.is_null()
        || *(*config).storage_path.offset(0isize) == 0
        || (*config).global_max_bytes < 0 as int64_t
        || (*config).global_max_items < 0 as ::core::ffi::c_int
        || (*config).per_camera_max_items < 0 as ::core::ffi::c_int
        || (*config).per_camera_max_bytes < 0 as int64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    if strlen((*config).storage_path).wrapping_add(128 as size_t)
        >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g.start_lock);
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.lifecycle,
    ) != C2Rust_Unnamed_2::LC_OFF.0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut g.start_lock);
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut g.storage_path as *mut ::core::ffi::c_char,
        (*config).storage_path,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut len: size_t = strlen(&raw mut g.storage_path as *mut ::core::ffi::c_char);
    while len > 1 as size_t
        && g.storage_path[len.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        len = len.wrapping_sub(1);
        g.storage_path[len] = '\0' as ::core::ffi::c_char;
    }
    g.global_max_bytes = (if (*config).global_max_bytes > 0 as int64_t {
        (*config).global_max_bytes as ::core::ffi::c_longlong
    } else {
        OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES
    }) as int64_t;
    g.global_max_items = if (*config).global_max_items > 0 as ::core::ffi::c_int {
        (*config).global_max_items
    } else {
        OBS_EVIDENCE_GLOBAL_DEPTH
    };
    g.per_camera_max_items = if (*config).per_camera_max_items > 0 as ::core::ffi::c_int {
        (*config).per_camera_max_items
    } else {
        OBS_EVIDENCE_PER_CAMERA_DEPTH
    };
    g.per_camera_max_bytes = if (*config).per_camera_max_bytes > 0 as int64_t {
        (*config).per_camera_max_bytes
    } else {
        g.global_max_bytes
    };
    g.discard_max_bytes =
        (if g.global_max_bytes as ::core::ffi::c_longlong > OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES {
            g.global_max_bytes as ::core::ffi::c_longlong
        } else {
            OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES
        }) as int64_t;
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.discard_bytes,
        0 as ::core::ffi::c_longlong,
    );
    if sem_init(
        &raw mut g.wake,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_uint,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut g.start_lock);
        return -1 as ::core::ffi::c_int;
    }
    reset_counters();
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.items,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.bytes,
        0 as ::core::ffi::c_longlong,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.drop_pending,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.stop,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.lifecycle,
        C2Rust_Unnamed_2::LC_READY.0 as ::core::ffi::c_int,
    );
    pthread_mutex_unlock(&raw mut g.start_lock);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Observation evidence initialized (worker starts on first submission; queue %d items / %lld bytes, %d per camera)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        g.global_max_items,
        g.global_max_bytes as ::core::ffi::c_longlong,
        g.per_camera_max_items,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_shutdown(mut drain_timeout_ms: ::core::ffi::c_int) {
    let mut lc: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.lifecycle);
    if lc != C2Rust_Unnamed_2::LC_READY.0 as ::core::ffi::c_int
        && lc != C2Rust_Unnamed_2::LC_RUNNING.0 as ::core::ffi::c_int
        || {
            let c2rust_result = ::core::intrinsics::atomic_cxchg::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut g.lifecycle,
                *&raw mut lc,
                C2Rust_Unnamed_2::LC_STOPPING.0 as ::core::ffi::c_int,
            );
            *&raw mut lc = c2rust_result.0;
            !c2rust_result.1
        }
    {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.inflight,
    ) > 0 as ::core::ffi::c_int
        && i < 50000 as ::core::ffi::c_int
    {
        usleep(100 as __useconds_t);
        i += 1;
    }
    if drain_timeout_ms < 0 as ::core::ffi::c_int {
        drain_timeout_ms = OBS_EVIDENCE_DEFAULT_DRAIN_MS;
    }
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.drain_deadline_ns,
        (mono_ns() as ::core::ffi::c_longlong
            + drain_timeout_ms as int64_t as ::core::ffi::c_longlong
                * 1000000 as ::core::ffi::c_longlong) as int64_t,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.stop,
        1 as ::core::ffi::c_int,
    );
    pthread_mutex_lock(&raw mut g.start_lock);
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.thread_started,
    ) != 0
    {
        sem_post(&raw mut g.wake);
        pthread_join(
            g.thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.thread_started,
            0 as ::core::ffi::c_int,
        );
    }
    pthread_mutex_unlock(&raw mut g.start_lock);
    let mut list: *mut evidence_item_t = take_all_fifo();
    while !list.is_null() {
        let mut item: *mut evidence_item_t = list;
        list = (*list).next as *mut evidence_item_t;
        abandon_item(item);
        release_item(item);
    }
    process_drops();
    if !g.encoder.is_null() {
        jpeg_encoder_cache_destroy(g.encoder);
        g.encoder = ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    free(
        ::core::intrinsics::atomic_xchg::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.ring,
            ::core::ptr::null_mut::<drop_ring_t>(),
        ) as *mut ::core::ffi::c_void,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.drop_pending,
        0 as ::core::ffi::c_int,
    );
    sem_destroy(&raw mut g.wake);
    g.component_id = -1 as ::core::ffi::c_int;
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g.lifecycle,
        C2Rust_Unnamed_2::LC_OFF.0 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Observation evidence shut down (ready=%llu interrupted=%llu dropped_untracked=%llu)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.ready,
        ),
        ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.interrupted,
        ),
        ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.dropped_untracked,
        ),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_evidence_get_stats(
    mut out: *mut observation_evidence_stats_t,
) {
    if out.is_null() {
        return;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_evidence_stats_t>(),
    );
    let mut lc: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.lifecycle);
    (*out).initialized = lc == C2Rust_Unnamed_2::LC_READY.0 as ::core::ffi::c_int
        || lc == C2Rust_Unnamed_2::LC_RUNNING.0 as ::core::ffi::c_int;
    (*out).worker_running =
        ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.worker_running,
        ) != 0;
    (*out).allocated = !::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.ring)
    .is_null()
        || ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g.thread_started,
        ) != 0;
    (*out).accepted = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.accepted) as uint64_t;
    (*out).dropped_queue_full = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.dropped_queue_full) as uint64_t;
    (*out).dropped_budget = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.dropped_budget) as uint64_t;
    (*out).dropped_disabled = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.dropped_disabled) as uint64_t;
    (*out).dropped_untracked = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.dropped_untracked) as uint64_t;
    (*out).unavailable_submitted = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.unavailable_submitted) as uint64_t;
    (*out).ready = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.ready) as uint64_t;
    (*out).failed_encode = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.failed_encode) as uint64_t;
    (*out).failed_write = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.failed_write) as uint64_t;
    (*out).failed_db = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.failed_db) as uint64_t;
    (*out).disabled_at_encode = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.disabled_at_encode) as uint64_t;
    (*out).interrupted = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.interrupted) as uint64_t;
    (*out).bytes_written = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.bytes_written) as uint64_t;
    let mut items: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.items);
    let mut bytes: ::core::ffi::c_longlong = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.bytes);
    let mut drops: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.drop_pending);
    (*out).backlog_items = if items > 0 as ::core::ffi::c_int {
        items as uint32_t
    } else {
        0 as uint32_t
    };
    (*out).backlog_bytes = if bytes > 0 as ::core::ffi::c_longlong {
        bytes as uint64_t
    } else {
        0 as uint64_t
    };
    (*out).drop_ring_depth = if drops > 0 as ::core::ffi::c_int {
        drops as uint32_t
    } else {
        0 as uint32_t
    };
    (*out).high_water_items = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.high_water_items) as uint32_t;
    (*out).high_water_bytes = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.high_water_bytes) as uint64_t;
    (*out).camera_high_water_items = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.camera_high_water_items) as uint32_t;
    if (*out).initialized {
        (*out).limit_items = g.global_max_items as uint32_t;
        (*out).limit_camera_items = g.per_camera_max_items as uint32_t;
        (*out).limit_bytes = g.global_max_bytes as uint64_t;
        (*out).limit_camera_bytes = g.per_camera_max_bytes as uint64_t;
        (*out).limit_discard_bytes = g.discard_max_bytes as uint64_t;
    }
    (*out).discard_deferred = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.discard_deferred) as uint64_t;
    (*out).discard_inline = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.discard_inline) as uint64_t;
    let mut pending_discard: ::core::ffi::c_longlong = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.discard_bytes);
    (*out).discard_pending_bytes = if pending_discard > 0 as ::core::ffi::c_longlong {
        pending_discard as uint64_t
    } else {
        0 as uint64_t
    };
    (*out).submit_latency_count = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.submit_latency_count) as uint64_t;
    (*out).submit_latency_sum_ns = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.submit_latency_sum_ns) as uint64_t;
    (*out).submit_latency_max_ns = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut g.submit_latency_max_ns) as uint64_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS {
        (*out).submit_latency_bucket[i as usize] =
            ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                (&raw mut g.submit_latency_bucket as *mut atomic_ullong).offset(i as isize),
            ) as uint64_t;
        i += 1;
    }
    let mut count: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::Acquire },
    >(&raw mut g_slot_count);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < count {
        if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*(&raw mut g_slots as *mut camera_slot_t).offset(i_0 as isize))
                .budget_exhausted,
        ) != 0
        {
            (*out).budget_exhausted_cameras = (*out).budget_exhausted_cameras.wrapping_add(1);
        }
        i_0 += 1;
    }
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
