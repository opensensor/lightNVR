unsafe extern "C" {
    unsafe fn cJSON_CreateNull() -> *mut cJSON;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_AddBoolToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        boolean: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddNumberToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        number: ::core::ffi::c_double,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddObjectToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn db_observation_diagnostics_get(
        out: *mut observation_db_diagnostics_t,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_evidence_get_stats(out: *mut observation_evidence_stats_t);
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn observation_relay_get_stats(stats: *mut observation_relay_stats_t);
    unsafe fn observation_retention_budget_exhausted_count() -> ::core::ffi::c_int;
    unsafe fn observation_retention_camera_status(
        out: *mut observation_retention_camera_status_t,
        max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_meta_t {
    pub epoch_uuid: [::core::ffi::c_char; 37],
    pub oldest_seq: int64_t,
    pub head_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_db_diagnostics_t {
    pub meta: observation_meta_t,
    pub has_oldest_observed_at: bool,
    pub oldest_observed_at_ms: int64_t,
    pub observations: int64_t,
    pub publications_pending: int64_t,
    pub publications_failed: int64_t,
    pub publications_expired: int64_t,
    pub oldest_pending_created_at_ms: int64_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_relay_stats_t {
    pub running: bool,
    pub passes: uint64_t,
    pub relayed: uint64_t,
    pub duplicates: uint64_t,
    pub relayed_no_route: uint64_t,
    pub expired: uint64_t,
    pub failed: uint64_t,
    pub enqueue_full: uint64_t,
    pub enqueue_errors: uint64_t,
    pub mark_errors: uint64_t,
    pub read_errors: uint64_t,
    pub cap_pauses: uint64_t,
    pub backoffs: uint64_t,
    pub sidecar_writes: uint64_t,
    pub sidecar_errors: uint64_t,
    pub epoch_regenerations: uint64_t,
    pub last_relayed_seq: int64_t,
    pub oldest_pending_created_at_ms: int64_t,
    pub last_pass_pending: ::core::ffi::c_int,
    pub outbox_type_pending: int64_t,
    pub consecutive_failures: ::core::ffi::c_int,
    pub backoff_until_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_retention_camera_status_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub retained_bytes: int64_t,
    pub max_bytes: int64_t,
    pub max_age_days: ::core::ffi::c_int,
    pub budget_exhausted: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_diagnostics_snapshot {
    pub collected_at_ms: int64_t,
    pub evidence: observation_evidence_stats_t,
    pub relay: observation_relay_stats_t,
    pub db_available: bool,
    pub db: observation_db_diagnostics_t,
    pub cameras: *mut observation_retention_camera_status_t,
    pub camera_count: ::core::ffi::c_int,
    pub budget_exhausted_cameras: ::core::ffi::c_int,
}
pub type observation_diagnostics_t = observation_diagnostics_snapshot;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OBS_SNAPSHOT_RETRY_AFTER_MAX: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
unsafe extern "C" fn wall_ms() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut ts);
    return ts.tv_sec as int64_t * 1000 as int64_t + ts.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn collect_cameras(mut out: *mut observation_diagnostics_t) {
    let mut capacity: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    let mut attempt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while attempt < 8 as ::core::ffi::c_int {
        let mut rows: *mut observation_retention_camera_status_t = calloc(
            capacity as size_t,
            ::core::mem::size_of::<observation_retention_camera_status_t>(),
        )
            as *mut observation_retention_camera_status_t;
        if rows.is_null() {
            return;
        }
        let mut count: ::core::ffi::c_int = observation_retention_camera_status(rows, capacity);
        if count < capacity || attempt == 7 as ::core::ffi::c_int {
            if count <= 0 as ::core::ffi::c_int {
                free(rows as *mut ::core::ffi::c_void);
                return;
            }
            (*out).cameras = rows;
            (*out).camera_count = count;
            return;
        }
        free(rows as *mut ::core::ffi::c_void);
        capacity *= 2 as ::core::ffi::c_int;
        attempt += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_diagnostics_collect(
    mut out: *mut observation_diagnostics_t,
    mut now_ms: int64_t,
) -> ::core::ffi::c_int {
    if out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_diagnostics_t>(),
    );
    (*out).collected_at_ms = if now_ms > 0 as int64_t {
        now_ms
    } else {
        wall_ms()
    };
    observation_evidence_get_stats(&raw mut (*out).evidence);
    observation_relay_get_stats(&raw mut (*out).relay);
    (*out).db_available =
        db_observation_diagnostics_get(&raw mut (*out).db) == C2Rust_Unnamed::DB_OBS_OK.0;
    collect_cameras(out);
    (*out).budget_exhausted_cameras = observation_retention_budget_exhausted_count();
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_diagnostics_free(
    mut diagnostics: *mut observation_diagnostics_t,
) {
    if diagnostics.is_null() {
        return;
    }
    free((*diagnostics).cameras as *mut ::core::ffi::c_void);
    (*diagnostics).cameras = ::core::ptr::null_mut::<observation_retention_camera_status_t>();
    (*diagnostics).camera_count = 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_diagnostics_relay_lag_seconds(
    mut diagnostics: *const observation_diagnostics_t,
) -> ::core::ffi::c_double {
    if diagnostics.is_null() || !(*diagnostics).db_available {
        return 0.0f64;
    }
    let mut oldest: int64_t = (*diagnostics).db.oldest_pending_created_at_ms;
    if oldest <= 0 as int64_t || (*diagnostics).collected_at_ms <= oldest {
        return 0.0f64;
    }
    return ((*diagnostics).collected_at_ms - oldest) as ::core::ffi::c_double / 1000.0f64;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_snapshot_retry_after_seconds(
    mut asset_age_ms: int64_t,
    mut items_ahead: uint64_t,
) -> ::core::ffi::c_int {
    if asset_age_ms < 5000 as int64_t {
        return 1 as ::core::ffi::c_int;
    }
    if items_ahead >= (OBS_SNAPSHOT_RETRY_AFTER_MAX / 2 as ::core::ffi::c_int) as uint64_t {
        return OBS_SNAPSHOT_RETRY_AFTER_MAX;
    }
    let mut seconds: ::core::ffi::c_int =
        (2 as uint64_t).wrapping_mul(items_ahead) as ::core::ffi::c_int;
    return if seconds < 1 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        seconds
    };
}
unsafe extern "C" fn add_u64(
    mut object: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut value: uint64_t,
) {
    cJSON_AddNumberToObject(object, name, value as ::core::ffi::c_double);
}
unsafe extern "C" fn time_or_null(mut present: bool, mut ms: int64_t) -> *mut cJSON {
    if !present || ms <= 0 as int64_t {
        return cJSON_CreateNull();
    }
    let mut seconds: time_t = (ms / 1000 as int64_t) as time_t;
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
        return cJSON_CreateNull();
    }
    let mut text: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%04d-%02d-%02dT%02d:%02d:%02d.%03dZ\0".as_ptr() as *const ::core::ffi::c_char,
        tm.tm_year + 1900 as ::core::ffi::c_int,
        tm.tm_mon + 1 as ::core::ffi::c_int,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        (ms % 1000 as int64_t) as ::core::ffi::c_int,
    );
    return cJSON_CreateString(&raw mut text as *mut ::core::ffi::c_char);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_diagnostics_to_json(
    mut d: *const observation_diagnostics_t,
) -> *mut cJSON {
    if d.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut e: *const observation_evidence_stats_t = &raw const (*d).evidence;
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut active: bool = (*e).allocated as ::core::ffi::c_int != 0
        || (*e).accepted != 0
        || (*e).dropped_queue_full != 0
        || (*e).dropped_budget != 0
        || (*e).dropped_untracked != 0
        || (*e).unavailable_submitted != 0
        || (*d).db_available as ::core::ffi::c_int != 0 && (*d).db.observations > 0 as int64_t;
    cJSON_AddBoolToObject(
        root,
        b"active\0".as_ptr() as *const ::core::ffi::c_char,
        active as cJSON_bool,
    );
    add_u64(
        root,
        b"accepted\0".as_ptr() as *const ::core::ffi::c_char,
        (*e).accepted,
    );
    let mut dropped: *mut cJSON =
        cJSON_AddObjectToObject(root, b"dropped\0".as_ptr() as *const ::core::ffi::c_char);
    if !dropped.is_null() {
        add_u64(
            dropped,
            b"queue_full\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).dropped_queue_full,
        );
        add_u64(
            dropped,
            b"budget_exhausted\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).dropped_budget,
        );
        add_u64(
            dropped,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).dropped_disabled,
        );
        add_u64(
            dropped,
            b"untracked\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).dropped_untracked,
        );
    }
    let mut evidence: *mut cJSON =
        cJSON_AddObjectToObject(root, b"evidence\0".as_ptr() as *const ::core::ffi::c_char);
    if !evidence.is_null() {
        cJSON_AddBoolToObject(
            evidence,
            b"queue_allocated\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).allocated as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            evidence,
            b"worker_running\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).worker_running as cJSON_bool,
        );
        add_u64(
            evidence,
            b"backlog_items\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).backlog_items as uint64_t,
        );
        add_u64(
            evidence,
            b"backlog_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).backlog_bytes,
        );
        add_u64(
            evidence,
            b"high_water_items\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).high_water_items as uint64_t,
        );
        add_u64(
            evidence,
            b"high_water_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).high_water_bytes,
        );
        add_u64(
            evidence,
            b"drop_ring_depth\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).drop_ring_depth as uint64_t,
        );
        add_u64(
            evidence,
            b"ready\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).ready,
        );
        add_u64(
            evidence,
            b"unavailable_submitted\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).unavailable_submitted,
        );
        add_u64(
            evidence,
            b"bytes_written\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).bytes_written,
        );
        let mut failures: *mut cJSON = cJSON_AddObjectToObject(
            evidence,
            b"failures\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !failures.is_null() {
            add_u64(
                failures,
                b"encode_failed\0".as_ptr() as *const ::core::ffi::c_char,
                (*e).failed_encode,
            );
            add_u64(
                failures,
                b"write_failed\0".as_ptr() as *const ::core::ffi::c_char,
                (*e).failed_write,
            );
            add_u64(
                failures,
                b"db_error\0".as_ptr() as *const ::core::ffi::c_char,
                (*e).failed_db,
            );
            add_u64(
                failures,
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
                (*e).disabled_at_encode,
            );
            add_u64(
                failures,
                b"interrupted\0".as_ptr() as *const ::core::ffi::c_char,
                (*e).interrupted,
            );
        }
    }
    let mut history: *mut cJSON =
        cJSON_AddObjectToObject(root, b"history\0".as_ptr() as *const ::core::ffi::c_char);
    if !history.is_null() {
        cJSON_AddBoolToObject(
            history,
            b"available\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).db_available as cJSON_bool,
        );
        cJSON_AddNumberToObject(
            history,
            b"observations\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).db.observations as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            history,
            b"oldest_seq\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).db.meta.oldest_seq as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            history,
            b"head_seq\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).db.meta.head_seq as ::core::ffi::c_double,
        );
        cJSON_AddItemToObject(
            history,
            b"oldest_observed_at\0".as_ptr() as *const ::core::ffi::c_char,
            time_or_null(
                (*d).db_available as ::core::ffi::c_int != 0
                    && (*d).db.has_oldest_observed_at as ::core::ffi::c_int != 0,
                (*d).db.oldest_observed_at_ms,
            ),
        );
        add_u64(
            history,
            b"epoch_regenerations\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).relay.epoch_regenerations,
        );
    }
    let mut publications: *mut cJSON = cJSON_AddObjectToObject(
        root,
        b"publications\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !publications.is_null() {
        cJSON_AddNumberToObject(
            publications,
            b"pending\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).db.publications_pending as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            publications,
            b"failed\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).db.publications_failed as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            publications,
            b"expired\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).db.publications_expired as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            publications,
            b"relay_lag_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            observation_diagnostics_relay_lag_seconds(d),
        );
        cJSON_AddBoolToObject(
            publications,
            b"relay_running\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).relay.running as cJSON_bool,
        );
        add_u64(
            publications,
            b"relayed\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).relay.relayed,
        );
        add_u64(
            publications,
            b"relayed_no_route\0".as_ptr() as *const ::core::ffi::c_char,
            (*d).relay.relayed_no_route,
        );
    }
    cJSON_AddNumberToObject(
        root,
        b"budget_exhausted_cameras\0".as_ptr() as *const ::core::ffi::c_char,
        (*d).budget_exhausted_cameras as ::core::ffi::c_double,
    );
    let mut cameras: *mut cJSON =
        cJSON_AddArrayToObject(root, b"cameras\0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !cameras.is_null() && i < (*d).camera_count {
        let mut c: *const observation_retention_camera_status_t = (*d).cameras.offset(i as isize);
        let mut item: *mut cJSON = cJSON_CreateObject();
        if item.is_null() {
            break;
        }
        cJSON_AddStringToObject(
            item,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*c).camera_uuid as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            item,
            b"retained_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            (*c).retained_bytes as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"max_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            (*c).max_bytes as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"max_age_days\0".as_ptr() as *const ::core::ffi::c_char,
            (*c).max_age_days as ::core::ffi::c_double,
        );
        cJSON_AddBoolToObject(
            item,
            b"budget_exhausted\0".as_ptr() as *const ::core::ffi::c_char,
            (*c).budget_exhausted as cJSON_bool,
        );
        cJSON_AddItemToArray(cameras, item);
        i += 1;
    }
    return root;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
