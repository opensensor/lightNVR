unsafe extern "C" {
    pub type fleet_selector;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_get_default(
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_mount_guard_active(target: *const storage_target_t) -> bool;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    unsafe fn db_fleet_camera_find_by_name(
        stream_name: *const ::core::ffi::c_char,
        camera: *mut fleet_camera_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_enrich_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    );
    unsafe fn db_storage_policy_list(
        policies: *mut storage_policy_t,
        max_count: ::core::ffi::c_int,
        enabled_only: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_policy_generation() -> uint64_t;
    unsafe fn db_storage_pool_allocate(
        uuid: *const ::core::ffi::c_char,
        exclude_target_uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_pool_result_t;
    unsafe fn storage_target_probe_and_publish(
        uuid: *const ::core::ffi::c_char,
        write_test: bool,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub target_type: [::core::ffi::c_char; 24],
    pub root_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub is_default: bool,
    pub storage_class: [::core::ffi::c_char; 16],
    pub reserve_bytes: uint64_t,
    pub high_watermark_pct: ::core::ffi::c_double,
    pub low_watermark_pct: ::core::ffi::c_double,
    pub health_status: [::core::ffi::c_char; 16],
    pub capacity_bytes: uint64_t,
    pub available_bytes: uint64_t,
    pub filesystem_device: uint64_t,
    pub last_probe_at: int64_t,
    pub last_success_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub recording_count: uint64_t,
    pub recording_bytes: uint64_t,
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 512],
    pub migration_bandwidth_bps: uint64_t,
    pub archival_window_start_minute: ::core::ffi::c_int,
    pub archival_window_end_minute: ::core::ffi::c_int,
    pub replica_count: uint64_t,
    pub replica_bytes: uint64_t,
    pub endpoint: [::core::ffi::c_char; 512],
    pub region: [::core::ffi::c_char; 64],
    pub bucket: [::core::ffi::c_char; 128],
    pub credential_ref: [::core::ffi::c_char; 128],
    pub archive_budget_bytes: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_target_result_t(pub ::core::ffi::c_int);
impl db_storage_target_result_t {
    pub const DB_STORAGE_TARGET_OK: Self = Self(0);
    pub const DB_STORAGE_TARGET_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_TARGET_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_TARGET_INVALID: Self = Self(-3);
    pub const DB_STORAGE_TARGET_STALE: Self = Self(-4);
    pub const DB_STORAGE_TARGET_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_TARGET_IN_USE: Self = Self(-6);
    pub const DB_STORAGE_TARGET_UNAVAILABLE: Self = Self(-7);
    pub const DB_STORAGE_TARGET_ERROR: Self = Self(-8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct storage_placement_status_t(pub ::core::ffi::c_uint);
impl storage_placement_status_t {
    pub const STORAGE_PLACEMENT_READY: Self = Self(0);
    pub const STORAGE_PLACEMENT_PAUSED: Self = Self(1);
    pub const STORAGE_PLACEMENT_FAILED: Self = Self(2);
    pub const STORAGE_PLACEMENT_ERROR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_placement_t {
    pub status: storage_placement_status_t,
    pub target_is_default: bool,
    pub target_uuid: [::core::ffi::c_char; 37],
    pub target_root: [::core::ffi::c_char; 512],
    pub object_key: [::core::ffi::c_char; 512],
    pub policy_uuid: [::core::ffi::c_char; 37],
    pub policy_version: int64_t,
    pub reason: [::core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_policy_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub enabled: bool,
    pub priority: ::core::ffi::c_int,
    pub selector_json: [::core::ffi::c_char; 8192],
    pub primary_target_uuid: [::core::ffi::c_char; 37],
    pub primary_pool_uuid: [::core::ffi::c_char; 37],
    pub fallback_mode: [::core::ffi::c_char; 16],
    pub fallback_target_uuid: [::core::ffi::c_char; 37],
    pub minimum_retention_days: ::core::ffi::c_int,
    pub desired_retention_days: ::core::ffi::c_int,
    pub maximum_retention_days: ::core::ffi::c_int,
    pub required_copy_count: ::core::ffi::c_int,
    pub replication_pool_uuid: [::core::ffi::c_char; 37],
    pub migration_after_days: ::core::ffi::c_int,
    pub migration_target_uuid: [::core::ffi::c_char; 37],
    pub pressure_priority: ::core::ffi::c_int,
    pub archive_after_seconds: ::core::ffi::c_int,
    pub hot_residency_seconds: ::core::ffi::c_int,
    pub archive_protected: bool,
    pub archive_on_pressure: bool,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
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
pub struct fallback_note_t {
    pub used: bool,
    pub stream_name: [::core::ffi::c_char; 256],
    pub detail: [::core::ffi::c_char; 256],
    pub logged_at: time_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_pool_result_t(pub ::core::ffi::c_int);
impl db_storage_pool_result_t {
    pub const DB_STORAGE_POOL_OK: Self = Self(0);
    pub const DB_STORAGE_POOL_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_POOL_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_POOL_INVALID: Self = Self(-3);
    pub const DB_STORAGE_POOL_STALE: Self = Self(-4);
    pub const DB_STORAGE_POOL_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_POOL_IN_USE: Self = Self(-6);
    pub const DB_STORAGE_POOL_UNAVAILABLE: Self = Self(-7);
    pub const DB_STORAGE_POOL_ERROR: Self = Self(-8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cached_assignment_t {
    pub used: bool,
    pub stream_name: [::core::ffi::c_char; 256],
    pub policy_index: ::core::ffi::c_int,
    pub evaluated_at: time_t,
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
pub struct cached_policy_t {
    pub policy: storage_policy_t,
    pub selector: *mut fleet_selector_t,
}
pub type fleet_selector_t = fleet_selector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector_explanation_t {
    pub clauses: [[::core::ffi::c_char; 160]; 64],
    pub clause_count: ::core::ffi::c_int,
}
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
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const STORAGE_POLICY_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PLACEMENT_ASSIGNMENT_TTL_SECONDS: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const PLACEMENT_FALLBACK_REPEAT_SECONDS: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
static mut placement_cache_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut policy_cache: [cached_policy_t; 128] = [cached_policy_t {
    policy: storage_policy_t {
        uuid: [0; 37],
        name: [0; 128],
        enabled: false,
        priority: 0,
        selector_json: [0; 8192],
        primary_target_uuid: [0; 37],
        primary_pool_uuid: [0; 37],
        fallback_mode: [0; 16],
        fallback_target_uuid: [0; 37],
        minimum_retention_days: 0,
        desired_retention_days: 0,
        maximum_retention_days: 0,
        required_copy_count: 0,
        replication_pool_uuid: [0; 37],
        migration_after_days: 0,
        migration_target_uuid: [0; 37],
        pressure_priority: 0,
        archive_after_seconds: 0,
        hot_residency_seconds: 0,
        archive_protected: false,
        archive_on_pressure: false,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    },
    selector: ::core::ptr::null_mut::<fleet_selector_t>(),
}; 128];
static mut policy_cache_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut policy_cache_generation: uint64_t = 0 as uint64_t;
static mut assignment_cache: [cached_assignment_t; 1024] = [cached_assignment_t {
    used: false,
    stream_name: [0; 256],
    policy_index: 0,
    evaluated_at: 0,
}; 1024];
static mut fallback_notes: [fallback_note_t; 1024] = [fallback_note_t {
    used: false,
    stream_name: [0; 256],
    detail: [0; 256],
    logged_at: 0,
}; 1024];
unsafe extern "C" fn describe_ineligible_primary(
    mut policy: *const storage_policy_t,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) {
    if (*policy).primary_pool_uuid[0usize] != 0 {
        snprintf(
            out,
            out_size,
            b"pool %s has no eligible member\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).primary_pool_uuid as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*policy).primary_target_uuid[0usize] == 0 {
        snprintf(
            out,
            out_size,
            b"no primary target configured\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut detail: storage_target_t = storage_target_t {
        uuid: [0; 37],
        name: [0; 128],
        target_type: [0; 24],
        root_path: [0; 512],
        enabled: false,
        is_default: false,
        storage_class: [0; 16],
        reserve_bytes: 0,
        high_watermark_pct: 0.,
        low_watermark_pct: 0.,
        health_status: [0; 16],
        capacity_bytes: 0,
        available_bytes: 0,
        filesystem_device: 0,
        last_probe_at: 0,
        last_success_at: 0,
        last_error: [0; 256],
        revision: 0,
        created_at: 0,
        updated_at: 0,
        recording_count: 0,
        recording_bytes: 0,
        mount_required: false,
        mount_guard_path: [0; 512],
        migration_bandwidth_bps: 0,
        archival_window_start_minute: 0,
        archival_window_end_minute: 0,
        replica_count: 0,
        replica_bytes: 0,
        endpoint: [0; 512],
        region: [0; 64],
        bucket: [0; 128],
        credential_ref: [0; 128],
        archive_budget_bytes: 0,
    };
    if db_storage_target_get(
        &raw const (*policy).primary_target_uuid as *const ::core::ffi::c_char,
        &raw mut detail,
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        snprintf(
            out,
            out_size,
            b"target %s not found\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).primary_target_uuid as *const ::core::ffi::c_char,
        );
        return;
    }
    snprintf(
        out,
        out_size,
        b"target \"%s\" enabled=%s type=%s health=%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut detail.name as *mut ::core::ffi::c_char,
        if detail.enabled as ::core::ffi::c_int != 0 {
            b"yes\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut detail.target_type as *mut ::core::ffi::c_char,
        if detail.health_status[0usize] as ::core::ffi::c_int != 0 {
            &raw mut detail.health_status as *mut ::core::ffi::c_char
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
        if detail.last_error[0usize] as ::core::ffi::c_int != 0 {
            b" last_error=\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut detail.last_error as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn find_fallback_note_locked(
    mut stream_name: *const ::core::ffi::c_char,
    mut allocate: bool,
) -> *mut fallback_note_t {
    let mut free_slot: *mut fallback_note_t = ::core::ptr::null_mut::<fallback_note_t>();
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < MAX_STREAMS {
        if fallback_notes[index as usize].used {
            if strcmp(
                &raw mut (*(&raw mut fallback_notes as *mut fallback_note_t).offset(index as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
            {
                return (&raw mut fallback_notes as *mut fallback_note_t).offset(index as isize);
            }
        } else if free_slot.is_null() {
            free_slot = (&raw mut fallback_notes as *mut fallback_note_t).offset(index as isize);
        }
        index += 1;
    }
    if !allocate || free_slot.is_null() {
        return ::core::ptr::null_mut::<fallback_note_t>();
    }
    memset(
        free_slot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fallback_note_t>(),
    );
    safe_strcpy(
        &raw mut (*free_slot).stream_name as *mut ::core::ffi::c_char,
        stream_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    return free_slot;
}
unsafe extern "C" fn note_primary_fallback(
    mut stream_name: *const ::core::ffi::c_char,
    mut policy: *const storage_policy_t,
    mut detail: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
) {
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_lock(&raw mut placement_cache_mutex);
    let mut note: *mut fallback_note_t = find_fallback_note_locked(stream_name, r#true != 0);
    let mut should_log: bool = note.is_null()
        || !(*note).used
        || strcmp(&raw mut (*note).detail as *mut ::core::ffi::c_char, detail)
            != 0 as ::core::ffi::c_int
        || now - (*note).logged_at >= PLACEMENT_FALLBACK_REPEAT_SECONDS as time_t;
    if !note.is_null() {
        (*note).used = r#true != 0;
        safe_strcpy(
            &raw mut (*note).detail as *mut ::core::ffi::c_char,
            detail,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        if should_log {
            (*note).logged_at = now;
        }
    }
    pthread_mutex_unlock(&raw mut placement_cache_mutex);
    if should_log {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Storage policy \"%s\": primary not eligible (%s); %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            &raw const (*policy).name as *const ::core::ffi::c_char,
            detail,
            outcome,
        );
    }
}
unsafe extern "C" fn clear_primary_fallback(
    mut stream_name: *const ::core::ffi::c_char,
    mut policy: *const storage_policy_t,
) {
    let mut was_falling_back: bool = r#false != 0;
    pthread_mutex_lock(&raw mut placement_cache_mutex);
    let mut note: *mut fallback_note_t = find_fallback_note_locked(stream_name, r#false != 0);
    if !note.is_null() && (*note).used as ::core::ffi::c_int != 0 {
        (*note).used = r#false != 0;
        was_falling_back = r#true != 0;
    }
    pthread_mutex_unlock(&raw mut placement_cache_mutex);
    if was_falling_back {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Storage policy \"%s\": primary target eligible again; new segments record there\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            &raw const (*policy).name as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn clear_policy_cache_locked() {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < policy_cache_count {
        fleet_selector_free(policy_cache[index as usize].selector);
        policy_cache[index as usize].selector = ::core::ptr::null_mut::<fleet_selector_t>();
        index += 1;
    }
    memset(
        &raw mut policy_cache as *mut cached_policy_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[cached_policy_t; 128]>(),
    );
    policy_cache_count = 0 as ::core::ffi::c_int;
    memset(
        &raw mut assignment_cache as *mut cached_assignment_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[cached_assignment_t; 1024]>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_placement_cache_invalidate() {
    pthread_mutex_lock(&raw mut placement_cache_mutex);
    clear_policy_cache_locked();
    policy_cache_generation = 0 as uint64_t;
    memset(
        &raw mut fallback_notes as *mut fallback_note_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[fallback_note_t; 1024]>(),
    );
    pthread_mutex_unlock(&raw mut placement_cache_mutex);
}
unsafe extern "C" fn reload_policies_locked(mut generation: uint64_t) -> ::core::ffi::c_int {
    clear_policy_cache_locked();
    let mut policies: *mut storage_policy_t = calloc(
        STORAGE_POLICY_MAX_COUNT as size_t,
        ::core::mem::size_of::<storage_policy_t>(),
    ) as *mut storage_policy_t;
    if policies.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int =
        db_storage_policy_list(policies, STORAGE_POLICY_MAX_COUNT, r#true != 0);
    if count < 0 as ::core::ffi::c_int {
        free(policies as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut json: *mut cJSON = cJSON_Parse(
            &raw mut (*policies.offset(index as isize)).selector_json as *mut ::core::ffi::c_char,
        );
        let mut error: [::core::ffi::c_char; 256] = [
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
        let mut selector: *mut fleet_selector_t = if !json.is_null() {
            fleet_selector_parse(
                json,
                &raw mut error as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            )
        } else {
            ::core::ptr::null_mut::<fleet_selector_t>()
        };
        cJSON_Delete(json);
        if selector.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Skipping invalid storage policy %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*policies.offset(index as isize)).uuid as *mut ::core::ffi::c_char,
                if error[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut error as *mut ::core::ffi::c_char
                } else {
                    b"invalid selector JSON\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        } else {
            let mut loaded: *const storage_policy_t = policies.offset(index as isize);
            if (*loaded).migration_target_uuid[0usize] as ::core::ffi::c_int != 0
                && (*loaded).migration_after_days <= 0 as ::core::ffi::c_int
                && (*loaded).archive_after_seconds < 0 as ::core::ffi::c_int
                && !(*loaded).archive_protected
                && !(*loaded).archive_on_pressure
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Storage policy \"%s\" names archive target %s but has no trigger (migration_after_days, archive_after_seconds, archive_protected or archive_on_pressure); no recordings will be moved\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*loaded).name as *const ::core::ffi::c_char,
                    &raw const (*loaded).migration_target_uuid
                        as *const ::core::ffi::c_char,
                );
            }
            policy_cache[policy_cache_count as usize].policy = *policies.offset(index as isize);
            policy_cache[policy_cache_count as usize].selector = selector;
            policy_cache_count += 1;
        }
        index += 1;
    }
    free(policies as *mut ::core::ffi::c_void);
    policy_cache_generation = generation;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn find_assignment_locked(
    mut stream_name: *const ::core::ffi::c_char,
    mut now: time_t,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < MAX_STREAMS {
        if !assignment_cache[index as usize].used
            || strcmp(
                &raw mut (*(&raw mut assignment_cache as *mut cached_assignment_t)
                    .offset(index as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) != 0 as ::core::ffi::c_int
        {
            index += 1;
        } else {
            if now - assignment_cache[index as usize].evaluated_at
                <= PLACEMENT_ASSIGNMENT_TTL_SECONDS as time_t
            {
                return assignment_cache[index as usize].policy_index;
            }
            assignment_cache[index as usize].used = r#false != 0;
            return -2 as ::core::ffi::c_int;
        }
    }
    return -2 as ::core::ffi::c_int;
}
unsafe extern "C" fn store_assignment_locked(
    mut stream_name: *const ::core::ffi::c_char,
    mut policy_index: ::core::ffi::c_int,
    mut now: time_t,
) {
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut oldest: time_t = now;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < MAX_STREAMS {
        if assignment_cache[index as usize].used as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut assignment_cache as *mut cached_assignment_t)
                    .offset(index as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            slot = index;
            break;
        } else if !assignment_cache[index as usize].used {
            slot = index;
            break;
        } else {
            if assignment_cache[index as usize].evaluated_at <= oldest {
                oldest = assignment_cache[index as usize].evaluated_at;
                slot = index;
            }
            index += 1;
        }
    }
    if slot < 0 as ::core::ffi::c_int {
        return;
    }
    assignment_cache[slot as usize].used = r#true != 0;
    safe_strcpy(
        &raw mut (*(&raw mut assignment_cache as *mut cached_assignment_t).offset(slot as isize))
            .stream_name as *mut ::core::ffi::c_char,
        stream_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    assignment_cache[slot as usize].policy_index = policy_index;
    assignment_cache[slot as usize].evaluated_at = now;
}
unsafe extern "C" fn resolve_policy(
    mut stream_name: *const ::core::ffi::c_char,
    mut policy: *mut storage_policy_t,
    mut matched: *mut bool,
) -> ::core::ffi::c_int {
    *matched = r#false != 0;
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut generation: uint64_t = db_storage_policy_generation();
    pthread_mutex_lock(&raw mut placement_cache_mutex);
    if policy_cache_generation != generation
        && reload_policies_locked(generation) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut placement_cache_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut cached_index: ::core::ffi::c_int = find_assignment_locked(stream_name, now);
    if cached_index >= 0 as ::core::ffi::c_int && cached_index < policy_cache_count {
        *policy = policy_cache[cached_index as usize].policy;
        *matched = r#true != 0;
        pthread_mutex_unlock(&raw mut placement_cache_mutex);
        return 0 as ::core::ffi::c_int;
    }
    if cached_index == -1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut placement_cache_mutex);
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut placement_cache_mutex);
    let mut camera: fleet_camera_t = fleet_camera_t {
        camera_uuid: [0; 37],
        name: [0; 256],
        address: [0; 512],
        legacy_tags: [0; 256],
        location_uuid: [0; 37],
        location_name: [0; 128],
        location_path: [0; 1024],
        location_ancestor_uuids: [[0; 37]; 32],
        location_depth: 0,
        tags: [fleet_camera_tag_t {
            uuid: [0; 37],
            label: [0; 256],
        }; 64],
        tag_count: 0,
        manufacturer: [0; 128],
        model: [0; 128],
        enabled: false,
        streaming_enabled: false,
        record: false,
        detection_based_recording: false,
        is_onvif: false,
        ptz_enabled: false,
        backchannel_enabled: false,
        health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
        availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
        health_changed_at: 0,
        first_video_at: 0,
        last_frame_ts: 0,
        last_recording_at: 0,
        current_fps: 0.,
        recording_active: false,
    };
    let mut camera_result: ::core::ffi::c_int =
        db_fleet_camera_find_by_name(stream_name, &raw mut camera);
    let mut selected: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if camera_result == 0 as ::core::ffi::c_int {
        fleet_camera_enrich_runtime_health(&raw mut camera, 1 as ::core::ffi::c_int);
        pthread_mutex_lock(&raw mut placement_cache_mutex);
        generation = db_storage_policy_generation();
        if policy_cache_generation != generation
            && reload_policies_locked(generation) != 0 as ::core::ffi::c_int
        {
            pthread_mutex_unlock(&raw mut placement_cache_mutex);
            return -1 as ::core::ffi::c_int;
        }
        let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index < policy_cache_count {
            if fleet_selector_matches(
                policy_cache[index as usize].selector,
                &raw mut camera,
                ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
            ) {
                selected = index;
                break;
            } else {
                index += 1;
            }
        }
        store_assignment_locked(stream_name, selected, now);
        if selected >= 0 as ::core::ffi::c_int {
            *policy = policy_cache[selected as usize].policy;
            *matched = r#true != 0;
        }
        pthread_mutex_unlock(&raw mut placement_cache_mutex);
        return 0 as ::core::ffi::c_int;
    }
    if camera_result < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut placement_cache_mutex);
    store_assignment_locked(stream_name, -1 as ::core::ffi::c_int, now);
    pthread_mutex_unlock(&raw mut placement_cache_mutex);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn target_is_eligible(
    mut uuid: *const ::core::ffi::c_char,
    mut target: *mut storage_target_t,
) -> bool {
    if db_storage_target_get(uuid, target).0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        return r#false != 0;
    }
    if !(*target).enabled
        || strcmp(
            &raw mut (*target).target_type as *mut ::core::ffi::c_char,
            b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if (*target).mount_required {
        static mut missing_mount_error: [::core::ffi::c_char; 26] = unsafe {
            ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"Required mount is absent:\0",
            )
        };
        let mut was_missing: bool = strncmp(
            &raw mut (*target).last_error as *mut ::core::ffi::c_char,
            &raw const missing_mount_error as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 26]>().wrapping_sub(1 as size_t),
        ) == 0 as ::core::ffi::c_int;
        if !db_storage_target_mount_guard_active(target) {
            if !was_missing {
                storage_target_probe_and_publish(uuid, r#false != 0, target);
            }
            return r#false != 0;
        }
        if was_missing as ::core::ffi::c_int != 0
            && storage_target_probe_and_publish(uuid, r#false != 0, target).0
                != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        {
            return r#false != 0;
        }
    }
    return strcmp(
        &raw mut (*target).health_status as *mut ::core::ffi::c_char,
        b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn default_target(mut target: *mut storage_target_t) -> bool {
    let mut configured: storage_target_t = storage_target_t {
        uuid: [0; 37],
        name: [0; 128],
        target_type: [0; 24],
        root_path: [0; 512],
        enabled: false,
        is_default: false,
        storage_class: [0; 16],
        reserve_bytes: 0,
        high_watermark_pct: 0.,
        low_watermark_pct: 0.,
        health_status: [0; 16],
        capacity_bytes: 0,
        available_bytes: 0,
        filesystem_device: 0,
        last_probe_at: 0,
        last_success_at: 0,
        last_error: [0; 256],
        revision: 0,
        created_at: 0,
        updated_at: 0,
        recording_count: 0,
        recording_bytes: 0,
        mount_required: false,
        mount_guard_path: [0; 512],
        migration_bandwidth_bps: 0,
        archival_window_start_minute: 0,
        archival_window_end_minute: 0,
        replica_count: 0,
        replica_bytes: 0,
        endpoint: [0; 512],
        region: [0; 64],
        bucket: [0; 128],
        credential_ref: [0; 128],
        archive_budget_bytes: 0,
    };
    if db_storage_target_get_default(&raw mut configured).0
        != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        return r#false != 0;
    }
    return target_is_eligible(&raw mut configured.uuid as *mut ::core::ffi::c_char, target);
}
unsafe extern "C" fn ready(
    mut placement: *mut storage_placement_t,
    mut target: *const storage_target_t,
    mut reason: *const ::core::ffi::c_char,
) {
    (*placement).status = storage_placement_status_t::STORAGE_PLACEMENT_READY;
    (*placement).target_is_default = (*target).is_default;
    safe_strcpy(
        &raw mut (*placement).target_uuid as *mut ::core::ffi::c_char,
        &raw const (*target).uuid as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*placement).target_root as *mut ::core::ffi::c_char,
        &raw const (*target).root_path as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*placement).reason as *mut ::core::ffi::c_char,
        reason,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_placement_select(
    mut stream_name: *const ::core::ffi::c_char,
    mut placement: *mut storage_placement_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || placement.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        placement as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_placement_t>(),
    );
    (*placement).status = storage_placement_status_t::STORAGE_PLACEMENT_ERROR;
    let mut policy: storage_policy_t = storage_policy_t {
        uuid: [0; 37],
        name: [0; 128],
        enabled: false,
        priority: 0,
        selector_json: [0; 8192],
        primary_target_uuid: [0; 37],
        primary_pool_uuid: [0; 37],
        fallback_mode: [0; 16],
        fallback_target_uuid: [0; 37],
        minimum_retention_days: 0,
        desired_retention_days: 0,
        maximum_retention_days: 0,
        required_copy_count: 0,
        replication_pool_uuid: [0; 37],
        migration_after_days: 0,
        migration_target_uuid: [0; 37],
        pressure_priority: 0,
        archive_after_seconds: 0,
        hot_residency_seconds: 0,
        archive_protected: false,
        archive_on_pressure: false,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut matched: bool = r#false != 0;
    if resolve_policy(stream_name, &raw mut policy, &raw mut matched) != 0 as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut (*placement).reason as *mut ::core::ffi::c_char,
            b"policy-lookup-failed\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut target: storage_target_t = storage_target_t {
        uuid: [0; 37],
        name: [0; 128],
        target_type: [0; 24],
        root_path: [0; 512],
        enabled: false,
        is_default: false,
        storage_class: [0; 16],
        reserve_bytes: 0,
        high_watermark_pct: 0.,
        low_watermark_pct: 0.,
        health_status: [0; 16],
        capacity_bytes: 0,
        available_bytes: 0,
        filesystem_device: 0,
        last_probe_at: 0,
        last_success_at: 0,
        last_error: [0; 256],
        revision: 0,
        created_at: 0,
        updated_at: 0,
        recording_count: 0,
        recording_bytes: 0,
        mount_required: false,
        mount_guard_path: [0; 512],
        migration_bandwidth_bps: 0,
        archival_window_start_minute: 0,
        archival_window_end_minute: 0,
        replica_count: 0,
        replica_bytes: 0,
        endpoint: [0; 512],
        region: [0; 64],
        bucket: [0; 128],
        credential_ref: [0; 128],
        archive_budget_bytes: 0,
    };
    if !matched {
        if !default_target(&raw mut target) {
            (*placement).status = storage_placement_status_t::STORAGE_PLACEMENT_PAUSED;
            safe_strcpy(
                &raw mut (*placement).reason as *mut ::core::ffi::c_char,
                b"default-unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            return 0 as ::core::ffi::c_int;
        }
        ready(
            placement,
            &raw mut target,
            b"default-target\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut (*placement).policy_uuid as *mut ::core::ffi::c_char,
        &raw mut policy.uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*placement).policy_version = policy.revision;
    let mut primary_available: bool = if policy.primary_pool_uuid[0usize] as ::core::ffi::c_int != 0
    {
        (db_storage_pool_allocate(
            &raw mut policy.primary_pool_uuid as *mut ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut target,
        )
        .0 == db_storage_pool_result_t::DB_STORAGE_POOL_OK.0) as ::core::ffi::c_int
    } else {
        target_is_eligible(
            &raw mut policy.primary_target_uuid as *mut ::core::ffi::c_char,
            &raw mut target,
        ) as ::core::ffi::c_int
    } != 0;
    if primary_available {
        clear_primary_fallback(stream_name, &raw mut policy);
        let mut reason: [::core::ffi::c_char; 64] = [0; 64];
        snprintf(
            &raw mut reason as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            if policy.primary_pool_uuid[0usize] as ::core::ffi::c_int != 0 {
                b"policy-pool:%s\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"policy-primary:%s\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut policy.uuid as *mut ::core::ffi::c_char,
        );
        ready(
            placement,
            &raw mut target,
            &raw mut reason as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut detail: [::core::ffi::c_char; 256] = [0; 256];
    describe_ineligible_primary(
        &raw mut policy,
        &raw mut detail as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if strcmp(
        &raw mut policy.fallback_mode as *mut ::core::ffi::c_char,
        b"pause\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*placement).status = storage_placement_status_t::STORAGE_PLACEMENT_PAUSED;
        safe_strcpy(
            &raw mut (*placement).reason as *mut ::core::ffi::c_char,
            b"policy-pause\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        note_primary_fallback(
            stream_name,
            &raw mut policy,
            &raw mut detail as *mut ::core::ffi::c_char,
            b"recording paused (fallback_mode=pause)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut policy.fallback_mode as *mut ::core::ffi::c_char,
        b"fail\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*placement).status = storage_placement_status_t::STORAGE_PLACEMENT_FAILED;
        safe_strcpy(
            &raw mut (*placement).reason as *mut ::core::ffi::c_char,
            b"policy-fail\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        note_primary_fallback(
            stream_name,
            &raw mut policy,
            &raw mut detail as *mut ::core::ffi::c_char,
            b"recording failed (fallback_mode=fail)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut available: bool = if strcmp(
        &raw mut policy.fallback_mode as *mut ::core::ffi::c_char,
        b"target\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        target_is_eligible(
            &raw mut policy.fallback_target_uuid as *mut ::core::ffi::c_char,
            &raw mut target,
        ) as ::core::ffi::c_int
    } else {
        default_target(&raw mut target) as ::core::ffi::c_int
    } != 0;
    if !available {
        (*placement).status = storage_placement_status_t::STORAGE_PLACEMENT_PAUSED;
        safe_strcpy(
            &raw mut (*placement).reason as *mut ::core::ffi::c_char,
            b"fallback-unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        note_primary_fallback(
            stream_name,
            &raw mut policy,
            &raw mut detail as *mut ::core::ffi::c_char,
            b"fallback target unavailable as well; recording paused\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut reason_0: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut reason_0 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        if strcmp(
            &raw mut policy.fallback_mode as *mut ::core::ffi::c_char,
            b"target\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            b"policy-fallback:%s\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"policy-default:%s\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut policy.uuid as *mut ::core::ffi::c_char,
    );
    let mut outcome: [::core::ffi::c_char; 192] = [0; 192];
    snprintf(
        &raw mut outcome as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
        b"recording to %s target \"%s\" instead\0".as_ptr() as *const ::core::ffi::c_char,
        if strcmp(
            &raw mut policy.fallback_mode as *mut ::core::ffi::c_char,
            b"target\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            b"fallback\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"default\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut target.name as *mut ::core::ffi::c_char,
    );
    note_primary_fallback(
        stream_name,
        &raw mut policy,
        &raw mut detail as *mut ::core::ffi::c_char,
        &raw mut outcome as *mut ::core::ffi::c_char,
    );
    ready(
        placement,
        &raw mut target,
        &raw mut reason_0 as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
