use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type fleet_selector;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn event_severity_name(severity: event_severity_t) -> *const ::core::ffi::c_char;
    unsafe fn event_envelope_validate(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_destination_get_by_key(
        key: *const ::core::ffi::c_char,
        destination: *mut event_destination_t,
    ) -> db_event_destination_result_t;
    unsafe fn db_event_destination_generation() -> uint64_t;
    unsafe fn db_event_route_validate(
        route: *const event_route_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> db_event_route_result_t;
    unsafe fn db_event_route_count() -> ::core::ffi::c_int;
    unsafe fn db_event_route_list(
        routes: *mut event_route_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_route_generation() -> uint64_t;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn fleet_selector_matches(
        selector: *const fleet_selector_t,
        camera: *const fleet_camera_t,
        explanation: *mut fleet_selector_explanation_t,
    ) -> bool;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn db_event_route_suppression_check(
        route: *const event_route_t,
        event_type: *const ::core::ffi::c_char,
        subject: *const ::core::ffi::c_char,
        now: int64_t,
    ) -> event_suppression_result_t;
    unsafe fn db_event_route_suppression_record_allowed(
        route_uuid: *const ::core::ffi::c_char,
        route_revision: int64_t,
        event_type: *const ::core::ffi::c_char,
        subject: *const ::core::ffi::c_char,
        event_id: *const ::core::ffi::c_char,
        now: int64_t,
    ) -> event_suppression_result_t;
    unsafe fn db_event_route_suppression_prune(
        updated_before: int64_t,
        limit: ::core::ffi::c_int,
        deleted_count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_enrich_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int32_t = i32;
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type cJSON_bool = ::core::ffi::c_int;
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
pub struct db_event_destination_result_t(pub ::core::ffi::c_int);
impl db_event_destination_result_t {
    pub const DB_EVENT_DESTINATION_OK: Self = Self(0);
    pub const DB_EVENT_DESTINATION_NOT_FOUND: Self = Self(-1);
    pub const DB_EVENT_DESTINATION_CONFLICT: Self = Self(-2);
    pub const DB_EVENT_DESTINATION_INVALID: Self = Self(-3);
    pub const DB_EVENT_DESTINATION_STALE: Self = Self(-4);
    pub const DB_EVENT_DESTINATION_LIMIT: Self = Self(-5);
    pub const DB_EVENT_DESTINATION_IN_USE: Self = Self(-6);
    pub const DB_EVENT_DESTINATION_ERROR: Self = Self(-7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub destination_key: [::core::ffi::c_char; 128],
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub predicate_json: [::core::ffi::c_char; 4096],
    pub schedule_json: [::core::ffi::c_char; 8192],
    pub event_types: [[::core::ffi::c_char; 96]; 32],
    pub event_type_count: ::core::ffi::c_int,
    pub debounce_seconds: ::core::ffi::c_int,
    pub cooldown_seconds: ::core::ffi::c_int,
    pub grouping_window_seconds: ::core::ffi::c_int,
    pub max_events_per_minute: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_event_route_result_t(pub ::core::ffi::c_int);
impl db_event_route_result_t {
    pub const DB_EVENT_ROUTE_OK: Self = Self(0);
    pub const DB_EVENT_ROUTE_NOT_FOUND: Self = Self(-1);
    pub const DB_EVENT_ROUTE_CONFLICT: Self = Self(-2);
    pub const DB_EVENT_ROUTE_INVALID: Self = Self(-3);
    pub const DB_EVENT_ROUTE_STALE: Self = Self(-4);
    pub const DB_EVENT_ROUTE_LIMIT: Self = Self(-5);
    pub const DB_EVENT_ROUTE_ERROR: Self = Self(-6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_router_result_t(pub ::core::ffi::c_int);
impl event_router_result_t {
    pub const EVENT_ROUTER_ERROR: Self = Self(-1);
    pub const EVENT_ROUTER_NO_MATCH: Self = Self(0);
    pub const EVENT_ROUTER_MATCH: Self = Self(1);
    pub const EVENT_ROUTER_DEFAULT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_router_stats_t {
    pub cache_reloads: uint64_t,
    pub events_evaluated: uint64_t,
    pub default_events: uint64_t,
    pub matched_events: uint64_t,
    pub unmatched_events: uint64_t,
    pub evaluation_errors: uint64_t,
    pub routes_considered: uint64_t,
    pub type_rejections: uint64_t,
    pub scope_rejections: uint64_t,
    pub predicate_rejections: uint64_t,
    pub schedule_rejections: uint64_t,
    pub destination_disabled_rejections: uint64_t,
    pub debounce_suppressions: uint64_t,
    pub cooldown_suppressions: uint64_t,
    pub grouping_suppressions: uint64_t,
    pub rate_suppressions: uint64_t,
    pub suppression_errors: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_delivery_plan_entry_t {
    pub route_uuid: [::core::ffi::c_char; 37],
    pub route_revision: int64_t,
    pub destination_key: [::core::ffi::c_char; 128],
    pub topic_template: [::core::ffi::c_char; 512],
    pub suppression_pending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_delivery_plan_t {
    pub event_id: [::core::ffi::c_char; 37],
    pub event_type: [::core::ffi::c_char; 96],
    pub subject: [::core::ffi::c_char; 96],
    pub entries: *mut event_route_delivery_plan_entry_t,
    pub count: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_router_state_t {
    pub mutex: pthread_mutex_t,
    pub routes: *mut compiled_route_t,
    pub route_count: ::core::ffi::c_int,
    pub definition_count: ::core::ffi::c_int,
    pub route_generation: uint64_t,
    pub destination_generation: uint64_t,
    pub inventory: *mut fleet_camera_t,
    pub inventory_count: ::core::ffi::c_int,
    pub inventory_loaded_at: int64_t,
    pub inventory_loaded: bool,
    pub last_suppression_prune_at: int64_t,
    pub stats: event_router_stats_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub address: [::core::ffi::c_char; 512],
    pub legacy_tags: [::core::ffi::c_char; 256],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub location_name: [::core::ffi::c_char; 128],
    pub location_path: [::core::ffi::c_char; 1024],
    pub location_ancestor_uuids: [[::core::ffi::c_char; 37]; 32],
    pub location_depth: ::core::ffi::c_int,
    pub tags: [fleet_camera_tag_t; 64],
    pub tag_count: ::core::ffi::c_int,
    pub manufacturer: [::core::ffi::c_char; 128],
    pub model: [::core::ffi::c_char; 128],
    pub enabled: bool,
    pub streaming_enabled: bool,
    pub record: bool,
    pub detection_based_recording: bool,
    pub is_onvif: bool,
    pub ptz_enabled: bool,
    pub backchannel_enabled: bool,
    pub health: fleet_health_state_t,
    pub availability: fleet_availability_state_t,
    pub health_changed_at: int64_t,
    pub first_video_at: int64_t,
    pub last_frame_ts: int64_t,
    pub last_recording_at: int64_t,
    pub current_fps: ::core::ffi::c_double,
    pub recording_active: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_availability_state_t(pub ::core::ffi::c_uint);
impl fleet_availability_state_t {
    pub const FLEET_AVAILABILITY_LIVE: Self = Self(0);
    pub const FLEET_AVAILABILITY_OFFLINE: Self = Self(1);
    pub const FLEET_AVAILABILITY_NEVER_CONNECTED: Self = Self(2);
    pub const FLEET_AVAILABILITY_DISABLED: Self = Self(3);
    pub const FLEET_AVAILABILITY_COUNT: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_health_state_t(pub ::core::ffi::c_uint);
impl fleet_health_state_t {
    pub const FLEET_HEALTH_UNKNOWN: Self = Self(0);
    pub const FLEET_HEALTH_UP: Self = Self(1);
    pub const FLEET_HEALTH_DEGRADED: Self = Self(2);
    pub const FLEET_HEALTH_DOWN: Self = Self(3);
    pub const FLEET_HEALTH_DISABLED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compiled_route_t {
    pub route: event_route_t,
    pub destination: event_destination_t,
    pub selector: *mut fleet_selector_t,
    pub detection: compiled_detection_predicate_t,
    pub health: compiled_health_predicate_t,
    pub windows: [compiled_schedule_window_t; 64],
    pub window_count: ::core::ffi::c_int,
    pub timezone: compiled_timezone_t,
    pub valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compiled_timezone_t {
    pub transitions: *mut int64_t,
    pub transition_types: *mut ::core::ffi::c_uchar,
    pub transition_count: ::core::ffi::c_int,
    pub offsets: [int32_t; 256],
    pub is_dst: [::core::ffi::c_uchar; 256],
    pub type_count: ::core::ffi::c_int,
    pub default_type: ::core::ffi::c_int,
    pub utc: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compiled_schedule_window_t {
    pub days: ::core::ffi::c_uchar,
    pub start_minute: ::core::ffi::c_int,
    pub end_minute: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compiled_health_predicate_t {
    pub conditions: [[::core::ffi::c_char; 128]; 64],
    pub condition_count: ::core::ffi::c_int,
    pub severities: [[::core::ffi::c_char; 128]; 64],
    pub severity_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compiled_detection_predicate_t {
    pub labels: [[::core::ffi::c_char; 128]; 64],
    pub label_count: ::core::ffi::c_int,
    pub zones: [[::core::ffi::c_char; 128]; 64],
    pub zone_count: ::core::ffi::c_int,
    pub has_min_confidence: bool,
    pub min_confidence: ::core::ffi::c_double,
}
pub type fleet_selector_t = fleet_selector;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_suppression_result_t(pub ::core::ffi::c_int);
impl event_suppression_result_t {
    pub const EVENT_SUPPRESSION_ERROR: Self = Self(-1);
    pub const EVENT_SUPPRESSION_STALE: Self = Self(-2);
    pub const EVENT_SUPPRESSION_PERMIT: Self = Self(0);
    pub const EVENT_SUPPRESSION_DEBOUNCE: Self = Self(1);
    pub const EVENT_SUPPRESSION_COOLDOWN: Self = Self(2);
    pub const EVENT_SUPPRESSION_GROUPING: Self = Self(3);
    pub const EVENT_SUPPRESSION_RATE: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector_explanation_t {
    pub clauses: [[::core::ffi::c_char; 160]; 64],
    pub clause_count: ::core::ffi::c_int,
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
pub struct tzif_counts_t {
    pub ttisgmt_count: ::core::ffi::c_int,
    pub ttisstd_count: ::core::ffi::c_int,
    pub leap_count: ::core::ffi::c_int,
    pub time_count: ::core::ffi::c_int,
    pub type_count: ::core::ffi::c_int,
    pub character_count: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
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
pub const INT32_MIN: ::core::ffi::c_int =
    -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const INT64_MIN: ::core::ffi::c_long =
    -9223372036854775807 as ::core::ffi::c_long - 1 as ::core::ffi::c_long;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const EVENT_ROUTE_MAX_COUNT: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const EVENT_ROUTE_DEFAULT_DESTINATION: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"mqtt:default\0") };
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const EVENT_ROUTE_SUPPRESSION_RETENTION_SECONDS: ::core::ffi::c_longlong = 30
    as ::core::ffi::c_longlong
    * 24 as ::core::ffi::c_longlong
    * 60 as ::core::ffi::c_longlong
    * 60 as ::core::ffi::c_longlong;
pub const DETECTION_EVENT_TYPE: [::core::ffi::c_char; 32] = unsafe {
    ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
        *b"io.lightnvr.detection.object.v1\0",
    )
};
pub const HEALTH_ALERT_EVENT_TYPE: [::core::ffi::c_char; 35] = unsafe {
    ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
        *b"io.lightnvr.system.health_alert.v1\0",
    )
};
pub const HEALTH_RECOVERED_EVENT_TYPE: [::core::ffi::c_char; 39] = unsafe {
    ::core::mem::transmute::<[u8; 39], [::core::ffi::c_char; 39]>(
        *b"io.lightnvr.system.health_recovered.v1\0",
    )
};
pub const ROUTER_FILTER_MAX_VALUES: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const ROUTER_FILTER_VALUE_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const ROUTER_SCHEDULE_MAX_WINDOWS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const ROUTER_INVENTORY_TTL_SECONDS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ROUTER_SUPPRESSION_PRUNE_INTERVAL_SECONDS: ::core::ffi::c_longlong =
    24 as ::core::ffi::c_longlong * 60 as ::core::ffi::c_longlong * 60 as ::core::ffi::c_longlong;
pub const ROUTER_SUPPRESSION_PRUNE_BATCH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const ROUTER_TZ_MAX_FILE_BYTES: ::core::ffi::c_uint =
    (1024 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
pub const ROUTER_TZ_MAX_TRANSITIONS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const ROUTER_TZ_MAX_TYPES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut ROUTER: event_router_state_t = event_router_state_t {
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
    routes: ::core::ptr::null_mut::<compiled_route_t>(),
    route_count: 0,
    definition_count: 0,
    route_generation: 0,
    destination_generation: 0,
    inventory: ::core::ptr::null_mut::<fleet_camera_t>(),
    inventory_count: 0,
    inventory_loaded_at: 0,
    inventory_loaded: false,
    last_suppression_prune_at: 0,
    stats: event_router_stats_t {
        cache_reloads: 0,
        events_evaluated: 0,
        default_events: 0,
        matched_events: 0,
        unmatched_events: 0,
        evaluation_errors: 0,
        routes_considered: 0,
        type_rejections: 0,
        scope_rejections: 0,
        predicate_rejections: 0,
        schedule_rejections: 0,
        destination_disabled_rejections: 0,
        debounce_suppressions: 0,
        cooldown_suppressions: 0,
        grouping_suppressions: 0,
        rate_suppressions: 0,
        suppression_errors: 0,
    },
};
unsafe extern "C" fn read_be32(mut input: *const ::core::ffi::c_uchar) -> uint32_t {
    return (*input.offset(0isize) as uint32_t) << 24 as ::core::ffi::c_int
        | (*input.offset(1isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*input.offset(2isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *input.offset(3isize) as uint32_t;
}
unsafe extern "C" fn read_be32_signed(mut input: *const ::core::ffi::c_uchar) -> int32_t {
    let mut value: uint32_t = read_be32(input);
    if value <= INT32_MAX as uint32_t {
        return value as int32_t;
    }
    let mut magnitude: uint32_t = (!value).wrapping_add(1 as uint32_t);
    if magnitude == 0x80000000 as uint32_t {
        return INT32_MIN as int32_t;
    }
    return -(magnitude as int32_t);
}
unsafe extern "C" fn read_be64_signed(mut input: *const ::core::ffi::c_uchar) -> int64_t {
    let mut value: uint64_t = 0 as uint64_t;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < 8 as ::core::ffi::c_int {
        value = value << 8 as ::core::ffi::c_int | *input.offset(index as isize) as uint64_t;
        index += 1;
    }
    if value <= INT64_MAX as uint64_t {
        return value as int64_t;
    }
    let mut magnitude: uint64_t = (!value).wrapping_add(1 as uint64_t);
    if magnitude == (1 as uint64_t) << 63 as ::core::ffi::c_int {
        return INT64_MIN as int64_t;
    }
    return -(magnitude as int64_t);
}
unsafe extern "C" fn parse_tzif_header(
    mut bytes: *const ::core::ffi::c_uchar,
    mut size: size_t,
    mut offset: size_t,
    mut counts: *mut tzif_counts_t,
    mut version: *mut ::core::ffi::c_char,
) -> bool {
    if bytes.is_null()
        || counts.is_null()
        || offset > size
        || size.wrapping_sub(offset) < 44 as size_t
        || memcmp(
            bytes.offset(offset as isize) as *const ::core::ffi::c_void,
            b"TZif\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if !version.is_null() {
        *version = *bytes.offset(offset.wrapping_add(4 as size_t) as isize) as ::core::ffi::c_char;
    }
    let mut raw: [uint32_t; 6] = [0; 6];
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < 6 as ::core::ffi::c_int {
        raw[index as usize] = read_be32(
            bytes
                .offset(offset as isize)
                .offset(20 as ::core::ffi::c_int as isize)
                .offset((index as size_t).wrapping_mul(4 as size_t) as isize),
        );
        if raw[index as usize] > INT_MAX as uint32_t {
            return r#false != 0;
        }
        index += 1;
    }
    (*counts).ttisgmt_count = raw[0usize] as ::core::ffi::c_int;
    (*counts).ttisstd_count = raw[1usize] as ::core::ffi::c_int;
    (*counts).leap_count = raw[2usize] as ::core::ffi::c_int;
    (*counts).time_count = raw[3usize] as ::core::ffi::c_int;
    (*counts).type_count = raw[4usize] as ::core::ffi::c_int;
    (*counts).character_count = raw[5usize] as ::core::ffi::c_int;
    return (*counts).time_count >= 0 as ::core::ffi::c_int
        && (*counts).time_count <= ROUTER_TZ_MAX_TRANSITIONS
        && (*counts).type_count >= 1 as ::core::ffi::c_int
        && (*counts).type_count <= ROUTER_TZ_MAX_TYPES
        && (*counts).character_count >= 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_size(
    mut total: *mut size_t,
    mut count: size_t,
    mut width: size_t,
) -> bool {
    if total.is_null()
        || count > 0 as size_t
            && width
                > (SIZE_MAX as size_t)
                    .wrapping_sub(*total)
                    .wrapping_div(count)
    {
        return r#false != 0;
    }
    *total = (*total).wrapping_add(count.wrapping_mul(width));
    return r#true != 0;
}
unsafe extern "C" fn tzif_block_size(
    mut counts: *const tzif_counts_t,
    mut time_width: size_t,
    mut size: *mut size_t,
) -> bool {
    let mut result: size_t = 0 as size_t;
    let mut valid: bool = add_size(&raw mut result, (*counts).time_count as size_t, time_width)
        as ::core::ffi::c_int
        != 0
        && add_size(&raw mut result, (*counts).time_count as size_t, 1 as size_t)
            as ::core::ffi::c_int
            != 0
        && add_size(&raw mut result, (*counts).type_count as size_t, 6 as size_t)
            as ::core::ffi::c_int
            != 0
        && add_size(
            &raw mut result,
            (*counts).character_count as size_t,
            1 as size_t,
        ) as ::core::ffi::c_int
            != 0
        && add_size(
            &raw mut result,
            (*counts).leap_count as size_t,
            time_width.wrapping_add(4 as size_t),
        ) as ::core::ffi::c_int
            != 0
        && add_size(
            &raw mut result,
            (*counts).ttisstd_count as size_t,
            1 as size_t,
        ) as ::core::ffi::c_int
            != 0
        && add_size(
            &raw mut result,
            (*counts).ttisgmt_count as size_t,
            1 as size_t,
        ) as ::core::ffi::c_int
            != 0;
    if valid {
        *size = result;
    }
    return valid;
}
unsafe extern "C" fn timezone_clear(mut timezone: *mut compiled_timezone_t) {
    if timezone.is_null() {
        return;
    }
    free((*timezone).transitions as *mut ::core::ffi::c_void);
    free((*timezone).transition_types as *mut ::core::ffi::c_void);
    memset(
        timezone as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<compiled_timezone_t>(),
    );
}
unsafe extern "C" fn timezone_name_is_safe(mut name: *const ::core::ffi::c_char) -> bool {
    if name.is_null()
        || *name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *name.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut component: *const ::core::ffi::c_char = name;
    let mut p: *const ::core::ffi::c_char = name;
    loop {
        if *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            let mut len: size_t = p.offset_from(component) as size_t;
            if len == 0 as size_t {
                return r#false != 0;
            }
            if len == 1 as size_t
                && *component.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                || len == 2 as size_t
                    && *component.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                    && *component.offset(1isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                break;
            }
            component = p.offset(1 as ::core::ffi::c_int as isize);
        } else if !(*(*__ctype_b_loc())
            .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || *p as ::core::ffi::c_int == '_' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '+' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
        p = p.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn timezone_load_tzif(
    mut name: *const ::core::ffi::c_char,
    mut timezone: *mut compiled_timezone_t,
) -> bool {
    if name.is_null() || timezone.is_null() {
        return r#false != 0;
    }
    if !timezone_name_is_safe(name) {
        return r#false != 0;
    }
    memset(
        timezone as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<compiled_timezone_t>(),
    );
    if strcmp(name, b"UTC\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcmp(name, b"Etc/UTC\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(name, b"GMT\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        (*timezone).utc = r#true != 0;
        (*timezone).type_count = 1 as ::core::ffi::c_int;
        return r#true != 0;
    }
    let mut path: [::core::ffi::c_char; 256] = [0; 256];
    let mut path_length: ::core::ffi::c_int = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"/usr/share/zoneinfo/%s\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
    if path_length < 0 as ::core::ffi::c_int
        || path_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
    {
        return r#false != 0;
    }
    let mut file: *mut FILE = fopen(
        &raw mut path as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if file.is_null() {
        return r#false != 0;
    }
    if fseek(file, 0 as ::core::ffi::c_long, SEEK_END) != 0 as ::core::ffi::c_int {
        fclose(file);
        return r#false != 0;
    }
    let mut file_length: ::core::ffi::c_long = ftell(file);
    if file_length < 44 as ::core::ffi::c_long
        || file_length as ::core::ffi::c_ulong > ROUTER_TZ_MAX_FILE_BYTES as ::core::ffi::c_ulong
        || fseek(file, 0 as ::core::ffi::c_long, SEEK_SET) != 0 as ::core::ffi::c_int
    {
        fclose(file);
        return r#false != 0;
    }
    let mut size: size_t = file_length as size_t;
    let mut bytes: *mut ::core::ffi::c_uchar = malloc(size) as *mut ::core::ffi::c_uchar;
    if bytes.is_null()
        || fread(bytes as *mut ::core::ffi::c_void, 1 as size_t, size, file) as size_t != size
    {
        free(bytes as *mut ::core::ffi::c_void);
        fclose(file);
        return r#false != 0;
    }
    fclose(file);
    let mut counts: tzif_counts_t = tzif_counts_t {
        ttisgmt_count: 0,
        ttisstd_count: 0,
        leap_count: 0,
        time_count: 0,
        type_count: 0,
        character_count: 0,
    };
    let mut version: ::core::ffi::c_char = '\0' as ::core::ffi::c_char;
    let mut header_offset: size_t = 0 as size_t;
    let mut time_width: size_t = 4 as size_t;
    if !parse_tzif_header(bytes, size, 0 as size_t, &raw mut counts, &raw mut version) {
        free(bytes as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    if version as ::core::ffi::c_int == '2' as ::core::ffi::c_int
        || version as ::core::ffi::c_int == '3' as ::core::ffi::c_int
        || version as ::core::ffi::c_int == '4' as ::core::ffi::c_int
    {
        let mut first_block: size_t = 0 as size_t;
        if !tzif_block_size(&raw mut counts, 4 as size_t, &raw mut first_block)
            || first_block > size.wrapping_sub(44 as size_t)
        {
            free(bytes as *mut ::core::ffi::c_void);
            return r#false != 0;
        }
        header_offset = (44 as size_t).wrapping_add(first_block);
        if !parse_tzif_header(
            bytes,
            size,
            header_offset,
            &raw mut counts,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ) {
            free(bytes as *mut ::core::ffi::c_void);
            return r#false != 0;
        }
        time_width = 8 as size_t;
    }
    let mut block_size: size_t = 0 as size_t;
    let mut data_offset: size_t = header_offset.wrapping_add(44 as size_t);
    if !tzif_block_size(&raw mut counts, time_width, &raw mut block_size)
        || data_offset > size
        || block_size > size.wrapping_sub(data_offset)
    {
        free(bytes as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    if counts.time_count < 0 as ::core::ffi::c_int {
        free(bytes as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    let mut transition_count: size_t = counts.time_count as size_t;
    if transition_count > ROUTER_TZ_MAX_TRANSITIONS as size_t {
        free(bytes as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    if transition_count > 0 as size_t {
        (*timezone).transitions =
            calloc(transition_count, ::core::mem::size_of::<int64_t>()) as *mut int64_t;
        (*timezone).transition_types = calloc(
            transition_count,
            ::core::mem::size_of::<::core::ffi::c_uchar>(),
        ) as *mut ::core::ffi::c_uchar;
        if (*timezone).transitions.is_null() || (*timezone).transition_types.is_null() {
            free(bytes as *mut ::core::ffi::c_void);
            timezone_clear(timezone);
            return r#false != 0;
        }
    }
    let mut transition_data: *const ::core::ffi::c_uchar = bytes.offset(data_offset as isize);
    let mut type_indices: *const ::core::ffi::c_uchar =
        transition_data.offset((counts.time_count as size_t).wrapping_mul(time_width) as isize);
    let mut type_data: *const ::core::ffi::c_uchar =
        type_indices.offset(counts.time_count as isize);
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < counts.time_count {
        *(*timezone).transitions.offset(index as isize) = if time_width == 8 as size_t {
            read_be64_signed(
                transition_data.offset((index as size_t).wrapping_mul(8 as size_t) as isize),
            )
        } else {
            read_be32_signed(
                transition_data.offset((index as size_t).wrapping_mul(4 as size_t) as isize),
            ) as int64_t
        };
        *(*timezone).transition_types.offset(index as isize) = *type_indices.offset(index as isize);
        if *type_indices.offset(index as isize) as ::core::ffi::c_int >= counts.type_count {
            free(bytes as *mut ::core::ffi::c_void);
            timezone_clear(timezone);
            return r#false != 0;
        }
        index += 1;
    }
    (*timezone).type_count = counts.type_count;
    (*timezone).transition_count = counts.time_count;
    (*timezone).default_type = 0 as ::core::ffi::c_int;
    let mut default_found: bool = r#false != 0;
    let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_0 < counts.type_count {
        (*timezone).offsets[index_0 as usize] = read_be32_signed(
            type_data.offset((index_0 as size_t).wrapping_mul(6 as size_t) as isize),
        );
        (*timezone).is_dst[index_0 as usize] = *type_data.offset(
            (index_0 as size_t)
                .wrapping_mul(6 as size_t)
                .wrapping_add(4 as size_t) as isize,
        );
        if !default_found
            && (*timezone).is_dst[index_0 as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*timezone).default_type = index_0;
            default_found = r#true != 0;
        }
        index_0 += 1;
    }
    free(bytes as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn timezone_offset_at(
    mut timezone: *const compiled_timezone_t,
    mut epoch: int64_t,
) -> int32_t {
    if timezone.is_null()
        || (*timezone).utc as ::core::ffi::c_int != 0
        || (*timezone).transition_count == 0 as ::core::ffi::c_int
    {
        return if !timezone.is_null() && (*timezone).type_count > 0 as ::core::ffi::c_int {
            (*timezone).offsets[(*timezone).default_type as usize]
        } else {
            0 as int32_t
        };
    }
    let mut low: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut high: ::core::ffi::c_int = (*timezone).transition_count;
    while low < high {
        let mut middle: ::core::ffi::c_int = low + (high - low) / 2 as ::core::ffi::c_int;
        if *(*timezone).transitions.offset(middle as isize) <= epoch {
            low = middle + 1 as ::core::ffi::c_int;
        } else {
            high = middle;
        }
    }
    let mut r#type: ::core::ffi::c_int = if low == 0 as ::core::ffi::c_int {
        (*timezone).default_type
    } else {
        *(*timezone)
            .transition_types
            .offset((low - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
    };
    return (*timezone).offsets[r#type as usize];
}
unsafe extern "C" fn copy_filter_values(
    mut array: *const cJSON,
    mut values: *mut [::core::ffi::c_char; 128],
    mut count: *mut ::core::ffi::c_int,
) -> bool {
    let mut size: ::core::ffi::c_int = if cJSON_IsArray(array) != 0 {
        cJSON_GetArraySize(array)
    } else {
        0 as ::core::ffi::c_int
    };
    if count.is_null() || size < 0 as ::core::ffi::c_int || size > ROUTER_FILTER_MAX_VALUES {
        return r#false != 0;
    }
    *count = 0 as ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < size {
        let mut item: *const cJSON = cJSON_GetArrayItem(array, index);
        if cJSON_IsString(item) == 0
            || (*item).valuestring.is_null()
            || strlen((*item).valuestring) >= ROUTER_FILTER_VALUE_MAX as size_t
        {
            return r#false != 0;
        }
        let c2rust_fresh2 = *count;
        *count += 1;
        safe_strcpy(
            &raw mut *values.offset(c2rust_fresh2 as isize) as *mut ::core::ffi::c_char,
            (*item).valuestring,
            ROUTER_FILTER_VALUE_MAX as size_t,
            0 as size_t,
        );
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn compile_predicate(mut compiled: *mut compiled_route_t) -> bool {
    let mut root: *mut cJSON =
        cJSON_Parse(&raw mut (*compiled).route.predicate_json as *mut ::core::ffi::c_char);
    if cJSON_IsObject(root) == 0 {
        cJSON_Delete(root);
        return r#false != 0;
    }
    let mut detection: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut valid: bool = r#true != 0;
    if !detection.is_null() {
        let mut labels: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            detection,
            b"labels_any\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut zones: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            detection,
            b"zone_ids_any\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut confidence: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            detection,
            b"min_confidence\0".as_ptr() as *const ::core::ffi::c_char,
        );
        valid = copy_filter_values(
            labels,
            &raw mut (*compiled).detection.labels as *mut [::core::ffi::c_char; 128],
            &raw mut (*compiled).detection.label_count,
        ) as ::core::ffi::c_int
            != 0
            && copy_filter_values(
                zones,
                &raw mut (*compiled).detection.zones as *mut [::core::ffi::c_char; 128],
                &raw mut (*compiled).detection.zone_count,
            ) as ::core::ffi::c_int
                != 0;
        if valid as ::core::ffi::c_int != 0 && !confidence.is_null() {
            if cJSON_IsNumber(confidence) == 0 {
                valid = r#false != 0;
            } else {
                (*compiled).detection.has_min_confidence = r#true != 0;
                (*compiled).detection.min_confidence = (*confidence).valuedouble;
            }
        }
    }
    let mut health: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(root, b"health\0".as_ptr() as *const ::core::ffi::c_char);
    if valid as ::core::ffi::c_int != 0 && !health.is_null() {
        let mut conditions: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            health,
            b"condition_codes_any\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut severities: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            health,
            b"severities_any\0".as_ptr() as *const ::core::ffi::c_char,
        );
        valid = copy_filter_values(
            conditions,
            &raw mut (*compiled).health.conditions as *mut [::core::ffi::c_char; 128],
            &raw mut (*compiled).health.condition_count,
        ) as ::core::ffi::c_int
            != 0
            && copy_filter_values(
                severities,
                &raw mut (*compiled).health.severities as *mut [::core::ffi::c_char; 128],
                &raw mut (*compiled).health.severity_count,
            ) as ::core::ffi::c_int
                != 0;
    }
    cJSON_Delete(root);
    return valid;
}
unsafe extern "C" fn clock_minute(mut value: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if value.is_null()
        || strlen(value) != 5 as size_t
        || *value.offset(2isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < 5 as ::core::ffi::c_int {
        if index != 2 as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(*value.offset(index as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
        {
            return -1 as ::core::ffi::c_int;
        }
        index += 1;
    }
    let mut hour: ::core::ffi::c_int = (*value.offset(0isize) as ::core::ffi::c_int
        - '0' as ::core::ffi::c_int)
        * 10 as ::core::ffi::c_int
        + *value.offset(1isize) as ::core::ffi::c_int
        - '0' as ::core::ffi::c_int;
    let mut minute: ::core::ffi::c_int = (*value.offset(3isize) as ::core::ffi::c_int
        - '0' as ::core::ffi::c_int)
        * 10 as ::core::ffi::c_int
        + *value.offset(4isize) as ::core::ffi::c_int
        - '0' as ::core::ffi::c_int;
    if hour > 23 as ::core::ffi::c_int || minute > 59 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    return hour * 60 as ::core::ffi::c_int + minute;
}
unsafe extern "C" fn compile_schedule(mut compiled: *mut compiled_route_t) -> bool {
    let mut root: *mut cJSON =
        cJSON_Parse(&raw mut (*compiled).route.schedule_json as *mut ::core::ffi::c_char);
    let mut timezone: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"timezone\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut windows: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(root, b"windows\0".as_ptr() as *const ::core::ffi::c_char);
    let mut count: ::core::ffi::c_int = if cJSON_IsArray(windows) != 0 {
        cJSON_GetArraySize(windows)
    } else {
        -1 as ::core::ffi::c_int
    };
    if cJSON_IsObject(root) == 0
        || cJSON_IsString(timezone) == 0
        || count < 0 as ::core::ffi::c_int
        || count > ROUTER_SCHEDULE_MAX_WINDOWS
    {
        cJSON_Delete(root);
        return r#false != 0;
    }
    (*compiled).window_count = count;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut window: *const cJSON = cJSON_GetArrayItem(windows, index);
        let mut days: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            window,
            b"days\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut start: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            window,
            b"start\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut end: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            window,
            b"end\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut day_count: ::core::ffi::c_int = if cJSON_IsArray(days) != 0 {
            cJSON_GetArraySize(days)
        } else {
            -1 as ::core::ffi::c_int
        };
        if cJSON_IsString(start) == 0
            || cJSON_IsString(end) == 0
            || day_count < 1 as ::core::ffi::c_int
        {
            cJSON_Delete(root);
            return r#false != 0;
        }
        let mut destination: *mut compiled_schedule_window_t = (&raw mut (*compiled).windows
            as *mut compiled_schedule_window_t)
            .offset(index as isize);
        (*destination).start_minute = clock_minute((*start).valuestring);
        (*destination).end_minute = clock_minute((*end).valuestring);
        if (*destination).start_minute < 0 as ::core::ffi::c_int
            || (*destination).end_minute < 0 as ::core::ffi::c_int
            || (*destination).start_minute == (*destination).end_minute
        {
            cJSON_Delete(root);
            return r#false != 0;
        }
        let mut day_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while day_index < day_count {
            let mut day: *const cJSON = cJSON_GetArrayItem(days, day_index);
            if cJSON_IsNumber(day) == 0
                || (*day).valueint < 0 as ::core::ffi::c_int
                || (*day).valueint > 6 as ::core::ffi::c_int
            {
                cJSON_Delete(root);
                return r#false != 0;
            }
            (*destination).days = ((*destination).days as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_uint) << (*day).valueint) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            day_index += 1;
        }
        index += 1;
    }
    let mut valid: bool = count == 0 as ::core::ffi::c_int
        || timezone_load_tzif((*timezone).valuestring, &raw mut (*compiled).timezone)
            as ::core::ffi::c_int
            != 0;
    if !valid {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Event route %s uses unavailable timezone %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*compiled).route.uuid as *mut ::core::ffi::c_char,
            (*timezone).valuestring,
        );
    }
    cJSON_Delete(root);
    return valid;
}
unsafe extern "C" fn compile_route(
    mut compiled: *mut compiled_route_t,
    mut route: *const event_route_t,
) -> bool {
    memset(
        compiled as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<compiled_route_t>(),
    );
    (*compiled).route = *route;
    if db_event_route_validate(
        route,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
    )
    .0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0
    {
        return r#false != 0;
    }
    if strcmp(
        &raw const (*route).destination_key as *const ::core::ffi::c_char,
        EVENT_ROUTE_DEFAULT_DESTINATION.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        (*compiled).destination.enabled = r#true != 0;
    } else if db_event_destination_get_by_key(
        &raw const (*route).destination_key as *const ::core::ffi::c_char,
        &raw mut (*compiled).destination,
    )
    .0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0
    {
        return r#false != 0;
    }
    if strcmp(
        &raw const (*route).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut selector_json: *mut cJSON =
            cJSON_Parse(&raw const (*route).selector_json as *const ::core::ffi::c_char);
        (*compiled).selector = fleet_selector_parse(
            selector_json,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        );
        cJSON_Delete(selector_json);
        if (*compiled).selector.is_null() {
            return r#false != 0;
        }
    }
    return compile_predicate(compiled) as ::core::ffi::c_int != 0
        && compile_schedule(compiled) as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn compiled_route_clear(mut route: *mut compiled_route_t) {
    if route.is_null() {
        return;
    }
    fleet_selector_free((*route).selector);
    timezone_clear(&raw mut (*route).timezone);
    memset(
        route as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<compiled_route_t>(),
    );
}
unsafe extern "C" fn clear_routes_locked() {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < ROUTER.route_count {
        compiled_route_clear(ROUTER.routes.offset(index as isize));
        index += 1;
    }
    free(ROUTER.routes as *mut ::core::ffi::c_void);
    ROUTER.routes = ::core::ptr::null_mut::<compiled_route_t>();
    ROUTER.route_count = 0 as ::core::ffi::c_int;
    ROUTER.definition_count = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn reload_routes_locked(
    mut route_generation: uint64_t,
    mut destination_generation: uint64_t,
) -> bool {
    let mut total: ::core::ffi::c_int = db_event_route_count();
    if total < 0 as ::core::ffi::c_int || total > EVENT_ROUTE_MAX_COUNT {
        return r#false != 0;
    }
    let mut routes: *mut event_route_t = (if total > 0 as ::core::ffi::c_int {
        calloc(total as size_t, ::core::mem::size_of::<event_route_t>())
    } else {
        NULL
    }) as *mut event_route_t;
    if total > 0 as ::core::ffi::c_int && routes.is_null() {
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int {
        db_event_route_list(routes, total)
    } else {
        0 as ::core::ffi::c_int
    };
    if count < 0 as ::core::ffi::c_int {
        free(routes as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    let mut enabled_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        if (*routes.offset(index as isize)).enabled {
            enabled_count += 1;
        }
        index += 1;
    }
    let mut compiled: *mut compiled_route_t = (if enabled_count > 0 as ::core::ffi::c_int {
        calloc(
            enabled_count as size_t,
            ::core::mem::size_of::<compiled_route_t>(),
        )
    } else {
        NULL
    }) as *mut compiled_route_t;
    if enabled_count > 0 as ::core::ffi::c_int && compiled.is_null() {
        free(routes as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    let mut compiled_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_0 < count {
        if (*routes.offset(index_0 as isize)).enabled {
            let c2rust_fresh1 = compiled_count;
            compiled_count += 1;
            let mut destination: *mut compiled_route_t = compiled.offset(c2rust_fresh1 as isize);
            (*destination).valid = compile_route(destination, routes.offset(index_0 as isize));
        }
        index_0 += 1;
    }
    free(routes as *mut ::core::ffi::c_void);
    clear_routes_locked();
    ROUTER.routes = compiled;
    ROUTER.route_count = compiled_count;
    ROUTER.definition_count = count;
    ROUTER.route_generation = route_generation;
    ROUTER.destination_generation = destination_generation;
    ROUTER.stats.cache_reloads = ROUTER.stats.cache_reloads.wrapping_add(1);
    return r#true != 0;
}
unsafe extern "C" fn ensure_routes_locked() -> bool {
    let mut route_generation: uint64_t = db_event_route_generation();
    let mut destination_generation: uint64_t = db_event_destination_generation();
    return ROUTER.route_generation == route_generation
        && ROUTER.destination_generation == destination_generation
        || reload_routes_locked(route_generation, destination_generation) as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn route_has_type(
    mut route: *const event_route_t,
    mut r#type: *const ::core::ffi::c_char,
) -> bool {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*route).event_type_count {
        if strcmp(
            &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                .offset(index as isize) as *const ::core::ffi::c_char,
            r#type,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        index += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn string_matches(
    mut value: *const ::core::ffi::c_char,
    mut candidates: *const [::core::ffi::c_char; 128],
    mut count: ::core::ffi::c_int,
) -> bool {
    if count == 0 as ::core::ffi::c_int {
        return r#true != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        if strcmp(
            if !value.is_null() {
                value
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw const *candidates.offset(index as isize) as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        index += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn predicate_matches(
    mut route: *const compiled_route_t,
    mut event: *const event_envelope_t,
) -> bool {
    let mut has_detection: bool = (*route).detection.label_count > 0 as ::core::ffi::c_int
        || (*route).detection.zone_count > 0 as ::core::ffi::c_int
        || (*route).detection.has_min_confidence as ::core::ffi::c_int != 0;
    let mut has_health: bool = (*route).health.condition_count > 0 as ::core::ffi::c_int
        || (*route).health.severity_count > 0 as ::core::ffi::c_int;
    if has_health {
        if strcmp(
            &raw const (*event).r#type as *const ::core::ffi::c_char,
            HEALTH_ALERT_EVENT_TYPE.as_ptr(),
        ) != 0 as ::core::ffi::c_int
            && strcmp(
                &raw const (*event).r#type as *const ::core::ffi::c_char,
                HEALTH_RECOVERED_EVENT_TYPE.as_ptr(),
            ) != 0 as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        let mut code: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            (*event).data,
            b"code\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut severity_value: *const ::core::ffi::c_char = event_severity_name((*event).severity);
        if strcmp(
            &raw const (*event).r#type as *const ::core::ffi::c_char,
            HEALTH_RECOVERED_EVENT_TYPE.as_ptr(),
        ) == 0 as ::core::ffi::c_int
        {
            let mut previous: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                (*event).data,
                b"previous_severity\0".as_ptr() as *const ::core::ffi::c_char,
            );
            severity_value = if cJSON_IsString(previous) != 0 {
                (*previous).valuestring
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
        return cJSON_IsString(code) != 0
            && string_matches(
                (*code).valuestring,
                &raw const (*route).health.conditions as *const [::core::ffi::c_char; 128],
                (*route).health.condition_count,
            ) as ::core::ffi::c_int
                != 0
            && string_matches(
                severity_value,
                &raw const (*route).health.severities as *const [::core::ffi::c_char; 128],
                (*route).health.severity_count,
            ) as ::core::ffi::c_int
                != 0;
    }
    if !has_detection {
        return r#true != 0;
    }
    if strcmp(
        &raw const (*event).r#type as *const ::core::ffi::c_char,
        DETECTION_EVENT_TYPE.as_ptr(),
    ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut detections: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        (*event).data,
        b"detections\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
    item = if !detections.is_null() {
        (*detections).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        let mut label: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"label\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut zone: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"zone_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut confidence: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !(cJSON_IsString(label) == 0 || cJSON_IsNumber(confidence) == 0) {
            if !(!string_matches(
                (*label).valuestring,
                &raw const (*route).detection.labels as *const [::core::ffi::c_char; 128],
                (*route).detection.label_count,
            ) || !string_matches(
                if cJSON_IsString(zone) != 0 {
                    (*zone).valuestring
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw const (*route).detection.zones as *const [::core::ffi::c_char; 128],
                (*route).detection.zone_count,
            ) || (*route).detection.has_min_confidence as ::core::ffi::c_int != 0
                && (*confidence).valuedouble < (*route).detection.min_confidence)
            {
                return r#true != 0;
            }
        }
        item = (*item).next;
    }
    return r#false != 0;
}
unsafe extern "C" fn schedule_matches(
    mut route: *const compiled_route_t,
    mut epoch: int64_t,
) -> bool {
    if (*route).window_count == 0 as ::core::ffi::c_int {
        return r#true != 0;
    }
    let mut offset: int32_t = timezone_offset_at(&raw const (*route).timezone, epoch);
    if offset > 0 as int32_t && epoch > INT64_MAX as int64_t - offset as int64_t
        || offset < 0 as int32_t && epoch < INT64_MIN as int64_t - offset as int64_t
    {
        return r#false != 0;
    }
    let mut local_epoch: time_t = (epoch + offset as int64_t) as time_t;
    if local_epoch as int64_t != epoch + offset as int64_t {
        return r#false != 0;
    }
    let mut local: tm = tm {
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
    if gmtime_r(&raw mut local_epoch, &raw mut local).is_null() {
        return r#false != 0;
    }
    let mut minute: ::core::ffi::c_int = local.tm_hour * 60 as ::core::ffi::c_int + local.tm_min;
    let mut today: ::core::ffi::c_uchar =
        ((1 as ::core::ffi::c_uint) << local.tm_wday) as ::core::ffi::c_uchar;
    let mut previous_day: ::core::ffi::c_int =
        (local.tm_wday + 6 as ::core::ffi::c_int) % 7 as ::core::ffi::c_int;
    let mut previous: ::core::ffi::c_uchar =
        ((1 as ::core::ffi::c_uint) << previous_day) as ::core::ffi::c_uchar;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*route).window_count {
        let mut window: *const compiled_schedule_window_t = (&raw const (*route).windows
            as *const compiled_schedule_window_t)
            .offset(index as isize);
        if (*window).start_minute < (*window).end_minute {
            if (*window).days as ::core::ffi::c_int & today as ::core::ffi::c_int != 0
                && minute >= (*window).start_minute
                && minute < (*window).end_minute
            {
                return r#true != 0;
            }
        } else if (*window).days as ::core::ffi::c_int & today as ::core::ffi::c_int != 0
            && minute >= (*window).start_minute
            || (*window).days as ::core::ffi::c_int & previous as ::core::ffi::c_int != 0
                && minute < (*window).end_minute
        {
            return r#true != 0;
        }
        index += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn camera_uuid_from_subject(
    mut subject: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    static mut prefix: [::core::ffi::c_char; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"camera/\0") };
    if subject.is_null()
        || strncmp(
            subject,
            &raw const prefix as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut uuid: *const ::core::ffi::c_char = subject
        .offset(::core::mem::size_of::<[::core::ffi::c_char; 8]>() as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    return if lightnvr_uuid_is_valid(uuid) as ::core::ffi::c_int != 0 {
        uuid
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
unsafe extern "C" fn refresh_inventory_locked(mut now: int64_t) -> bool {
    if ROUTER.inventory_loaded as ::core::ffi::c_int != 0
        && now >= ROUTER.inventory_loaded_at
        && now - ROUTER.inventory_loaded_at < ROUTER_INVENTORY_TTL_SECONDS as int64_t
    {
        return r#true != 0;
    }
    let mut inventory: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load(&raw mut inventory, &raw mut count) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    fleet_camera_enrich_runtime_health(inventory, count);
    free(ROUTER.inventory as *mut ::core::ffi::c_void);
    ROUTER.inventory = inventory;
    ROUTER.inventory_count = count;
    ROUTER.inventory_loaded_at = now;
    ROUTER.inventory_loaded = r#true != 0;
    return r#true != 0;
}
unsafe extern "C" fn find_camera_locked(
    mut uuid: *const ::core::ffi::c_char,
    mut now: int64_t,
    mut camera: *mut *const fleet_camera_t,
) -> ::core::ffi::c_int {
    if !camera.is_null() {
        *camera = ::core::ptr::null::<fleet_camera_t>();
    }
    if uuid.is_null() || camera.is_null() || !refresh_inventory_locked(now) {
        return -1 as ::core::ffi::c_int;
    }
    let mut low: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut high: ::core::ffi::c_int = ROUTER.inventory_count;
    while low < high {
        let mut middle: ::core::ffi::c_int = low + (high - low) / 2 as ::core::ffi::c_int;
        let mut comparison: ::core::ffi::c_int = strcmp(
            &raw mut (*ROUTER.inventory.offset(middle as isize)).camera_uuid
                as *mut ::core::ffi::c_char,
            uuid,
        );
        if comparison < 0 as ::core::ffi::c_int {
            low = middle + 1 as ::core::ffi::c_int;
        } else {
            high = middle;
        }
    }
    if low >= ROUTER.inventory_count
        || strcmp(
            &raw mut (*ROUTER.inventory.offset(low as isize)).camera_uuid
                as *mut ::core::ffi::c_char,
            uuid,
        ) != 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    *camera = ROUTER.inventory.offset(low as isize);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn route_uses_suppression(mut route: *const event_route_t) -> bool {
    return (*route).debounce_seconds > 0 as ::core::ffi::c_int
        || (*route).cooldown_seconds > 0 as ::core::ffi::c_int
        || (*route).grouping_window_seconds > 0 as ::core::ffi::c_int
        || (*route).max_events_per_minute > 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn delivery_plan_append(
    mut plan: *mut event_route_delivery_plan_t,
    mut compiled: *const compiled_route_t,
    mut suppression_pending: bool,
) -> bool {
    if plan.is_null() {
        return r#true != 0;
    }
    if (*plan).count == (*plan).capacity {
        let mut capacity: size_t = if (*plan).capacity == 0 as size_t {
            4 as size_t
        } else {
            (*plan).capacity.wrapping_mul(2 as size_t)
        };
        if capacity > EVENT_ROUTE_MAX_COUNT as size_t {
            capacity = EVENT_ROUTE_MAX_COUNT as size_t;
        }
        if capacity <= (*plan).capacity {
            return r#false != 0;
        }
        let mut entries: *mut event_route_delivery_plan_entry_t = realloc(
            (*plan).entries as *mut ::core::ffi::c_void,
            capacity.wrapping_mul(::core::mem::size_of::<event_route_delivery_plan_entry_t>()),
        )
            as *mut event_route_delivery_plan_entry_t;
        if entries.is_null() {
            return r#false != 0;
        }
        (*plan).entries = entries;
        (*plan).capacity = capacity;
    }
    let c2rust_fresh0 = (*plan).count;
    (*plan).count = (*plan).count.wrapping_add(1);
    let mut entry: *mut event_route_delivery_plan_entry_t =
        (*plan).entries.offset(c2rust_fresh0 as isize);
    memset(
        entry as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_route_delivery_plan_entry_t>(),
    );
    let mut route: *const event_route_t = &raw const (*compiled).route;
    safe_strcpy(
        &raw mut (*entry).route_uuid as *mut ::core::ffi::c_char,
        &raw const (*route).uuid as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*entry).route_revision = (*route).revision;
    safe_strcpy(
        &raw mut (*entry).destination_key as *mut ::core::ffi::c_char,
        &raw const (*route).destination_key as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    if strcmp(
        &raw const (*route).destination_key as *const ::core::ffi::c_char,
        EVENT_ROUTE_DEFAULT_DESTINATION.as_ptr(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut (*entry).topic_template as *mut ::core::ffi::c_char,
            &raw const (*compiled).destination.topic_template as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    (*entry).suppression_pending = suppression_pending;
    return r#true != 0;
}
unsafe extern "C" fn count_suppression_locked(mut result: event_suppression_result_t) {
    match result {
        event_suppression_result_t::EVENT_SUPPRESSION_DEBOUNCE => {
            ROUTER.stats.debounce_suppressions = ROUTER.stats.debounce_suppressions.wrapping_add(1);
        }
        event_suppression_result_t::EVENT_SUPPRESSION_COOLDOWN => {
            ROUTER.stats.cooldown_suppressions = ROUTER.stats.cooldown_suppressions.wrapping_add(1);
        }
        event_suppression_result_t::EVENT_SUPPRESSION_GROUPING => {
            ROUTER.stats.grouping_suppressions = ROUTER.stats.grouping_suppressions.wrapping_add(1);
        }
        event_suppression_result_t::EVENT_SUPPRESSION_RATE => {
            ROUTER.stats.rate_suppressions = ROUTER.stats.rate_suppressions.wrapping_add(1);
        }
        _ => {}
    };
}
unsafe extern "C" fn maybe_prune_suppression_locked(mut now: int64_t) {
    if now <= 0 as int64_t
        || ROUTER.last_suppression_prune_at > 0 as int64_t
            && now >= ROUTER.last_suppression_prune_at
            && ((now - ROUTER.last_suppression_prune_at) as ::core::ffi::c_longlong)
                < ROUTER_SUPPRESSION_PRUNE_INTERVAL_SECONDS
    {
        return;
    }
    let mut deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_int = db_event_route_suppression_prune(
        (now as ::core::ffi::c_longlong - EVENT_ROUTE_SUPPRESSION_RETENTION_SECONDS) as int64_t,
        ROUTER_SUPPRESSION_PRUNE_BATCH,
        &raw mut deleted,
    );
    if result == 0 as ::core::ffi::c_int {
        ROUTER.last_suppression_prune_at = now;
        if deleted > 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Pruned %d expired event route suppression states\0".as_ptr()
                    as *const ::core::ffi::c_char,
                deleted,
            );
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not prune expired event route suppression states\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_router_evaluate_delivery(
    mut event: *const event_envelope_t,
    mut plan: *mut event_route_delivery_plan_t,
) -> event_router_result_t {
    event_route_delivery_plan_clear(plan);
    if event.is_null()
        || event_envelope_validate(
            event,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return event_router_result_t::EVENT_ROUTER_ERROR;
    }
    if !plan.is_null() {
        safe_strcpy(
            &raw mut (*plan).event_id as *mut ::core::ffi::c_char,
            &raw const (*event).id as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*plan).event_type as *mut ::core::ffi::c_char,
            &raw const (*event).r#type as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*plan).subject as *mut ::core::ffi::c_char,
            &raw const (*event).subject as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            0 as size_t,
        );
    }
    pthread_mutex_lock(&raw mut ROUTER.mutex);
    ROUTER.stats.events_evaluated = ROUTER.stats.events_evaluated.wrapping_add(1);
    if !ensure_routes_locked() {
        ROUTER.stats.evaluation_errors = ROUTER.stats.evaluation_errors.wrapping_add(1);
        pthread_mutex_unlock(&raw mut ROUTER.mutex);
        return event_router_result_t::EVENT_ROUTER_ERROR;
    }
    if ROUTER.definition_count == 0 as ::core::ffi::c_int {
        ROUTER.stats.default_events = ROUTER.stats.default_events.wrapping_add(1);
        pthread_mutex_unlock(&raw mut ROUTER.mutex);
        return event_router_result_t::EVENT_ROUTER_DEFAULT;
    }
    let mut relevant_error: bool = r#false != 0;
    let mut plan_error: bool = r#false != 0;
    let mut should_enqueue: bool = r#false != 0;
    let mut camera: *const fleet_camera_t = ::core::ptr::null::<fleet_camera_t>();
    let mut camera_resolved: bool = r#false != 0;
    let mut camera_uuid: *const ::core::ffi::c_char =
        camera_uuid_from_subject(&raw const (*event).subject as *const ::core::ffi::c_char);
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    maybe_prune_suppression_locked(now);
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < ROUTER.route_count {
        let mut route: *mut compiled_route_t = ROUTER.routes.offset(index as isize);
        ROUTER.stats.routes_considered = ROUTER.stats.routes_considered.wrapping_add(1);
        's_90: {
            if !route_has_type(
                &raw mut (*route).route,
                &raw const (*event).r#type as *const ::core::ffi::c_char,
            ) {
                ROUTER.stats.type_rejections = ROUTER.stats.type_rejections.wrapping_add(1);
            } else if !(*route).valid {
                relevant_error = r#true != 0;
            } else if !(*route).destination.enabled {
                ROUTER.stats.destination_disabled_rejections =
                    ROUTER.stats.destination_disabled_rejections.wrapping_add(1);
            } else {
                if !(*route).selector.is_null() {
                    if camera_uuid.is_null() {
                        ROUTER.stats.scope_rejections =
                            ROUTER.stats.scope_rejections.wrapping_add(1);
                        break 's_90;
                    } else {
                        if !camera_resolved {
                            let mut camera_result: ::core::ffi::c_int =
                                find_camera_locked(camera_uuid, now, &raw mut camera);
                            if camera_result < 0 as ::core::ffi::c_int {
                                relevant_error = r#true != 0;
                            }
                            camera_resolved = r#true != 0;
                        }
                        if camera.is_null() {
                            ROUTER.stats.scope_rejections =
                                ROUTER.stats.scope_rejections.wrapping_add(1);
                            break 's_90;
                        } else if !fleet_selector_matches(
                            (*route).selector,
                            camera,
                            ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
                        ) {
                            ROUTER.stats.scope_rejections =
                                ROUTER.stats.scope_rejections.wrapping_add(1);
                            break 's_90;
                        }
                    }
                }
                if !predicate_matches(route, event) {
                    ROUTER.stats.predicate_rejections =
                        ROUTER.stats.predicate_rejections.wrapping_add(1);
                } else if !schedule_matches(route, (*event).occurred_at as int64_t) {
                    ROUTER.stats.schedule_rejections =
                        ROUTER.stats.schedule_rejections.wrapping_add(1);
                } else if !route_uses_suppression(&raw mut (*route).route) {
                    if !delivery_plan_append(plan, route, r#false != 0) {
                        relevant_error = r#true != 0;
                        plan_error = r#true != 0;
                    } else {
                        should_enqueue = r#true != 0;
                    }
                } else {
                    let mut suppression: event_suppression_result_t =
                        db_event_route_suppression_check(
                            &raw mut (*route).route,
                            &raw const (*event).r#type as *const ::core::ffi::c_char,
                            &raw const (*event).subject as *const ::core::ffi::c_char,
                            now,
                        );
                    if suppression.0 == event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0 {
                        if !delivery_plan_append(plan, route, r#true != 0) {
                            relevant_error = r#true != 0;
                            plan_error = r#true != 0;
                        } else {
                            should_enqueue = r#true != 0;
                        }
                    } else if suppression.0 > event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0
                    {
                        count_suppression_locked(suppression);
                    } else {
                        relevant_error = r#true != 0;
                        ROUTER.stats.suppression_errors =
                            ROUTER.stats.suppression_errors.wrapping_add(1);
                    }
                }
            }
        }
        index += 1;
    }
    if should_enqueue as ::core::ffi::c_int != 0 && !plan_error {
        ROUTER.stats.matched_events = ROUTER.stats.matched_events.wrapping_add(1);
        pthread_mutex_unlock(&raw mut ROUTER.mutex);
        return event_router_result_t::EVENT_ROUTER_MATCH;
    }
    let mut result: event_router_result_t =
        event_router_result_t(if relevant_error as ::core::ffi::c_int != 0 {
            event_router_result_t::EVENT_ROUTER_ERROR.0
        } else {
            event_router_result_t::EVENT_ROUTER_NO_MATCH.0
        });
    if result.0 == event_router_result_t::EVENT_ROUTER_ERROR.0 {
        ROUTER.stats.evaluation_errors = ROUTER.stats.evaluation_errors.wrapping_add(1);
    } else {
        ROUTER.stats.unmatched_events = ROUTER.stats.unmatched_events.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut ROUTER.mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_router_evaluate(
    mut event: *const event_envelope_t,
) -> event_router_result_t {
    return event_router_evaluate_delivery(
        event,
        ::core::ptr::null_mut::<event_route_delivery_plan_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_router_record_destination_enqueued(
    mut event: *const event_envelope_t,
    mut plan: *const event_route_delivery_plan_t,
    mut destination_key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if event.is_null()
        || plan.is_null()
        || destination_key.is_null()
        || *destination_key.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strcmp(
            &raw const (*event).id as *const ::core::ffi::c_char,
            &raw const (*plan).event_id as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*event).r#type as *const ::core::ffi::c_char,
            &raw const (*plan).event_type as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*event).subject as *const ::core::ffi::c_char,
            &raw const (*plan).subject as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut failed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    let mut index: size_t = 0 as size_t;
    while index < (*plan).count {
        let mut entry: *const event_route_delivery_plan_entry_t =
            (*plan).entries.offset(index as isize);
        if !(!(*entry).suppression_pending
            || strcmp(
                &raw const (*entry).destination_key as *const ::core::ffi::c_char,
                destination_key,
            ) != 0 as ::core::ffi::c_int)
        {
            let mut result: event_suppression_result_t = db_event_route_suppression_record_allowed(
                &raw const (*entry).route_uuid as *const ::core::ffi::c_char,
                (*entry).route_revision,
                &raw const (*event).r#type as *const ::core::ffi::c_char,
                &raw const (*event).subject as *const ::core::ffi::c_char,
                &raw const (*event).id as *const ::core::ffi::c_char,
                now,
            );
            if result.0 != event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0
                && result.0 != event_suppression_result_t::EVENT_SUPPRESSION_STALE.0
            {
                failed = 1 as ::core::ffi::c_int;
            }
        }
        index = index.wrapping_add(1);
    }
    return if failed != 0 {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_route_delivery_plan_clear(
    mut plan: *mut event_route_delivery_plan_t,
) {
    if plan.is_null() {
        return;
    }
    free((*plan).entries as *mut ::core::ffi::c_void);
    memset(
        plan as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_route_delivery_plan_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_router_get_stats(mut stats: *mut event_router_stats_t) {
    if stats.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut ROUTER.mutex);
    *stats = ROUTER.stats;
    pthread_mutex_unlock(&raw mut ROUTER.mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_router_shutdown() {
    pthread_mutex_lock(&raw mut ROUTER.mutex);
    clear_routes_locked();
    free(ROUTER.inventory as *mut ::core::ffi::c_void);
    ROUTER.inventory = ::core::ptr::null_mut::<fleet_camera_t>();
    ROUTER.inventory_count = 0 as ::core::ffi::c_int;
    ROUTER.inventory_loaded_at = 0 as int64_t;
    ROUTER.inventory_loaded = r#false != 0;
    ROUTER.last_suppression_prune_at = 0 as int64_t;
    ROUTER.route_generation = 0 as uint64_t;
    ROUTER.destination_generation = 0 as uint64_t;
    memset(
        &raw mut ROUTER.stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_router_stats_t>(),
    );
    pthread_mutex_unlock(&raw mut ROUTER.mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
