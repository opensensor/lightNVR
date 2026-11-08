unsafe extern "C" {
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemReferenceToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_Duplicate(item: *const cJSON, recurse: cJSON_bool) -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __ctype_tolower_loc() -> *mut *const __int32_t;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn observation_asset_state_parse(
        name: *const ::core::ffi::c_char,
        state: *mut observation_asset_state_t,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_reason_is_valid(
        state: observation_asset_state_t,
        reason: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_sensitivity_t(pub ::core::ffi::c_uint);
impl event_sensitivity_t {
    pub const EVENT_SENSITIVITY_OPERATIONAL: Self = Self(0);
    pub const EVENT_SENSITIVITY_INTERNAL: Self = Self(1);
    pub const EVENT_SENSITIVITY_RESTRICTED: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_media_policy_t(pub ::core::ffi::c_uint);
impl event_media_policy_t {
    pub const EVENT_MEDIA_FORBIDDEN: Self = Self(0);
    pub const EVENT_MEDIA_REFERENCE_ALLOWED: Self = Self(1);
    pub const EVENT_MEDIA_BINARY_OPT_IN: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_expected_rate_t(pub ::core::ffi::c_uint);
impl event_expected_rate_t {
    pub const EVENT_RATE_LOW: Self = Self(0);
    pub const EVENT_RATE_MEDIUM: Self = Self(1);
    pub const EVENT_RATE_HIGH: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_subject_kind_t(pub ::core::ffi::c_uint);
impl event_subject_kind_t {
    pub const EVENT_SUBJECT_CAMERA: Self = Self(0);
    pub const EVENT_SUBJECT_STORAGE: Self = Self(1);
    pub const EVENT_SUBJECT_SYSTEM: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_type_definition_t {
    pub r#type: *const ::core::ffi::c_char,
    pub family: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub severity: event_severity_t,
    pub dynamic_severity: bool,
    pub minimum_severity: event_severity_t,
    pub maximum_severity: event_severity_t,
    pub sensitivity: event_sensitivity_t,
    pub media_policy: event_media_policy_t,
    pub expected_rate: event_expected_rate_t,
    pub subject_kind: event_subject_kind_t,
    pub default_expiry_seconds: ::core::ffi::c_int,
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
pub struct observation_asset_state_t(pub ::core::ffi::c_uint);
impl observation_asset_state_t {
    pub const OBS_ASSET_PENDING: Self = Self(0);
    pub const OBS_ASSET_READY: Self = Self(1);
    pub const OBS_ASSET_UNAVAILABLE: Self = Self(2);
    pub const OBS_ASSET_EXPIRED: Self = Self(3);
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
pub const cJSON_Number: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const cJSON_String: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const cJSON_Array: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const cJSON_Object: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const EVENT_TYPE_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_SOURCE_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const EVENT_SUBJECT_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_TIME_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EVENT_DATA_MAX_BYTES: ::core::ffi::c_uint =
    (64 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
pub const EVENT_ENVELOPE_MAX_BYTES: ::core::ffi::c_uint =
    (96 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -128 as ::core::ffi::c_int && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const OBS_LABEL_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
static mut EVENT_TYPES: [event_type_definition_t; 14] = [
    event_type_definition_t {
        r#type: b"io.lightnvr.detection.object.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"detection\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"One or more objects observed by a camera\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_OPERATIONAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_REFERENCE_ALLOWED,
        expected_rate: event_expected_rate_t::EVENT_RATE_HIGH,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_CAMERA,
        default_expiry_seconds: 3600 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.detection.observation_changed.v1\0".as_ptr()
            as *const ::core::ffi::c_char,
        family: b"detection\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"A durable object observation was created or changed\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_OPERATIONAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_REFERENCE_ALLOWED,
        expected_rate: event_expected_rate_t::EVENT_RATE_HIGH,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_CAMERA,
        default_expiry_seconds: 3600 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.recognition.license_plate.v1\0".as_ptr()
            as *const ::core::ffi::c_char,
        family: b"recognition\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"A protected license-plate read was stored\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_RESTRICTED,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_HIGH,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_CAMERA,
        default_expiry_seconds: 3600 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.camera.offline.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Camera connectivity declared offline\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_WARNING,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_OPERATIONAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_CAMERA,
        default_expiry_seconds: 86400 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.camera.recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Camera connectivity recovered after an offline state\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_OPERATIONAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_CAMERA,
        default_expiry_seconds: 86400 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.stream.degraded.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"A camera stream crossed a degraded health threshold\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_WARNING,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_OPERATIONAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_CAMERA,
        default_expiry_seconds: 86400 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.stream.recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"A degraded camera stream returned to healthy\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_OPERATIONAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_CAMERA,
        default_expiry_seconds: 86400 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.stream.recording_gap.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"A gap was detected in a camera recording\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_WARNING,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_OPERATIONAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_REFERENCE_ALLOWED,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_CAMERA,
        default_expiry_seconds: 604800 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.storage.pressure.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"storage\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Storage usage crossed an operational threshold\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_CRITICAL,
        dynamic_severity: r#true != 0,
        minimum_severity: event_severity_t::EVENT_SEVERITY_WARNING,
        maximum_severity: event_severity_t::EVENT_SEVERITY_CRITICAL,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_INTERNAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_STORAGE,
        default_expiry_seconds: 604800 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.storage.recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"storage\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Storage returned to normal after pressure\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_INTERNAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_STORAGE,
        default_expiry_seconds: 604800 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.storage.target_unavailable.v1\0".as_ptr()
            as *const ::core::ffi::c_char,
        family: b"storage\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"A configured storage target became unavailable\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_ERROR,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_INTERNAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_STORAGE,
        default_expiry_seconds: 604800 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.storage.target_recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"storage\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"An unavailable storage target became usable again\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_INTERNAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_STORAGE,
        default_expiry_seconds: 604800 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.system.health_alert.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"system_health\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"A host or hardware health incident changed state\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_WARNING,
        dynamic_severity: r#true != 0,
        minimum_severity: event_severity_t::EVENT_SEVERITY_WARNING,
        maximum_severity: event_severity_t::EVENT_SEVERITY_CRITICAL,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_INTERNAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_SYSTEM,
        default_expiry_seconds: 604800 as ::core::ffi::c_int,
    },
    event_type_definition_t {
        r#type: b"io.lightnvr.system.health_recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
        family: b"system_health\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"A host or hardware health incident recovered\0".as_ptr()
            as *const ::core::ffi::c_char,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        dynamic_severity: false,
        minimum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        maximum_severity: event_severity_t::EVENT_SEVERITY_INFO,
        sensitivity: event_sensitivity_t::EVENT_SENSITIVITY_INTERNAL,
        media_policy: event_media_policy_t::EVENT_MEDIA_FORBIDDEN,
        expected_rate: event_expected_rate_t::EVENT_RATE_LOW,
        subject_kind: event_subject_kind_t::EVENT_SUBJECT_SYSTEM,
        default_expiry_seconds: 604800 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn set_error(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut message: *const ::core::ffi::c_char,
) {
    if error.is_null()
        || error_size == 0 as size_t
        || *error.offset(0isize) as ::core::ffi::c_int != 0
    {
        return;
    }
    snprintf(
        error,
        error_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        message,
    );
}
unsafe extern "C" fn valid_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
    mut allow_empty: bool,
) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strnlen(value, maximum);
    if !allow_empty && length == 0 as size_t || length == maximum {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn format_event_time(
    mut occurred_at: time_t,
    mut output: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut utc: tm = tm {
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
    if gmtime_r(&raw mut occurred_at, &raw mut utc).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return if strftime(
        output,
        EVENT_TIME_MAX as size_t,
        b"%Y-%m-%dT%H:%M:%SZ\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut utc,
    ) > 0 as size_t
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn valid_event_time(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() || strnlen(value, EVENT_TIME_MAX as size_t) != 20 as size_t {
        return r#false != 0;
    }
    let separators: [::core::ffi::c_int; 6] = [
        4 as ::core::ffi::c_int,
        7 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
        13 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
    ];
    let expected: [::core::ffi::c_char; 6] = [
        '-' as ::core::ffi::c_char,
        '-' as ::core::ffi::c_char,
        'T' as ::core::ffi::c_char,
        ':' as ::core::ffi::c_char,
        ':' as ::core::ffi::c_char,
        'Z' as ::core::ffi::c_char,
    ];
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[::core::ffi::c_int; 6]>()
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>())
    {
        if *value.offset(separators[index] as isize) as ::core::ffi::c_int
            != expected[index] as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        index = index.wrapping_add(1);
    }
    let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_0 < 19 as ::core::ffi::c_int {
        if !(index_0 == 4 as ::core::ffi::c_int
            || index_0 == 7 as ::core::ffi::c_int
            || index_0 == 10 as ::core::ffi::c_int
            || index_0 == 13 as ::core::ffi::c_int
            || index_0 == 16 as ::core::ffi::c_int)
        {
            if *(*__ctype_b_loc()).offset(*value.offset(index_0 as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
            {
                return r#false != 0;
            }
        }
        index_0 += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn contains_case_insensitive(
    mut value: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    if value.is_null() || needle.is_null() || *needle.offset(0isize) == 0 {
        return r#false != 0;
    }
    let mut needle_length: size_t = strlen(needle);
    let mut start: *const ::core::ffi::c_char = value;
    while *start != 0 {
        let mut index: size_t = 0 as size_t;
        while index < needle_length
            && *start.offset(index as isize) as ::core::ffi::c_int != 0
            && ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                    if 0 as ::core::ffi::c_int != 0 {
                        let mut __c: ::core::ffi::c_int = *start.offset(index as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int;
                        __res = (if __c < -128 as ::core::ffi::c_int
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*start.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*start.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) == ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                    if 0 as ::core::ffi::c_int != 0 {
                        let mut __c: ::core::ffi::c_int = *needle.offset(index as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int;
                        __res = (if __c < -128 as ::core::ffi::c_int
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*needle.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*needle.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            })
        {
            index = index.wrapping_add(1);
        }
        if index == needle_length {
            return r#true != 0;
        }
        start = start.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn forbidden_data_key(mut key: *const ::core::ffi::c_char) -> bool {
    if key.is_null() {
        return r#false != 0;
    }
    if contains_case_insensitive(key, b"password\0".as_ptr() as *const ::core::ffi::c_char)
        as ::core::ffi::c_int
        != 0
        || contains_case_insensitive(key, b"secret\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"credential\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(
            key,
            b"authorization\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"cookie\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"api_key\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"apikey\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"plate\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"token\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
    {
        return r#true != 0;
    }
    let mut length: size_t = strlen(key);
    return contains_case_insensitive(key, b"_path\0".as_ptr() as *const ::core::ffi::c_char)
        as ::core::ffi::c_int
        != 0
        || length == 4 as size_t
            && contains_case_insensitive(key, b"path\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0;
}
unsafe extern "C" fn has_case_insensitive_suffix(
    mut value: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> bool {
    if value.is_null() || suffix.is_null() {
        return r#false != 0;
    }
    let mut value_length: size_t = strlen(value);
    let mut suffix_length: size_t = strlen(suffix);
    if suffix_length > value_length {
        return r#false != 0;
    }
    return contains_case_insensitive(
        value
            .offset(value_length as isize)
            .offset(-(suffix_length as isize)),
        suffix,
    );
}
unsafe extern "C" fn looks_like_raw_filesystem_path(
    mut value: *const ::core::ffi::c_char,
    mut property_name: *const ::core::ffi::c_char,
) -> bool {
    if value.is_null() || *value.offset(0isize) == 0 {
        return r#false != 0;
    }
    if strncasecmp(
        value,
        b"file://\0".as_ptr() as *const ::core::ffi::c_char,
        7 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if *(*__ctype_b_loc())
        .offset(*value.offset(0isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISalpha.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
        && *value.offset(1isize) as ::core::ffi::c_int == ':' as ::core::ffi::c_int
        && (*value.offset(2isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            || *value.offset(2isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int)
    {
        return r#true != 0;
    }
    if *value.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int {
        return r#false != 0;
    }
    return property_name.is_null()
        || !has_case_insensitive_suffix(
            property_name,
            b"_url\0".as_ptr() as *const ::core::ffi::c_char,
        );
}
unsafe extern "C" fn validate_data_node(
    mut node: *const cJSON,
    mut depth: ::core::ffi::c_int,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if node.is_null() || depth > 16 as ::core::ffi::c_int {
        set_error(
            error,
            error_size,
            b"event data exceeds maximum depth\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !(*node).string.is_null() {
        if !valid_text((*node).string, 128 as size_t, r#false != 0) {
            set_error(
                error,
                error_size,
                b"event data contains an invalid key\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if forbidden_data_key((*node).string) {
            set_error(
                error,
                error_size,
                b"event data contains a sensitive or filesystem key\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if cJSON_IsObject(node) != 0 || cJSON_IsArray(node) != 0 {
        let mut child: *const cJSON = (*node).child;
        while !child.is_null() {
            if validate_data_node(child, depth + 1 as ::core::ffi::c_int, error, error_size)
                != 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            child = (*child).next;
        }
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsString(node) != 0 {
        if !valid_text(
            (*node).valuestring,
            EVENT_DATA_MAX_BYTES as size_t,
            r#true != 0,
        ) {
            set_error(
                error,
                error_size,
                b"event data contains invalid text\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if looks_like_raw_filesystem_path((*node).valuestring, (*node).string) {
            set_error(
                error,
                error_size,
                b"event data contains a raw filesystem path\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsNumber(node) != 0 {
        if (*node).valuedouble.is_finite() as ::core::ffi::c_int == 0 {
            set_error(
                error,
                error_size,
                b"event data contains a non-finite number\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsBool(node) != 0 || cJSON_IsNull(node) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    set_error(
        error,
        error_size,
        b"event data contains an unsupported value\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn required_field(
    mut data: *const cJSON,
    mut name: *const ::core::ffi::c_char,
    mut expected_type: ::core::ffi::c_int,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> *const cJSON {
    let mut field: *const cJSON = cJSON_GetObjectItemCaseSensitive(data, name);
    let mut valid: bool = !field.is_null();
    if expected_type == cJSON_String {
        valid = cJSON_IsString(field) != 0;
    }
    if expected_type == cJSON_Number {
        valid = cJSON_IsNumber(field) != 0;
    }
    if expected_type == cJSON_Array {
        valid = cJSON_IsArray(field) != 0;
    }
    if expected_type == cJSON_Object {
        valid = cJSON_IsObject(field) != 0;
    }
    if !valid {
        let mut message: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"event data is missing required field '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        set_error(
            error,
            error_size,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null::<cJSON>();
    }
    return field;
}
unsafe extern "C" fn valid_health_token(mut value: *const ::core::ffi::c_char) -> bool {
    if !valid_text(value, 65 as size_t, r#false != 0) {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
            && *cursor as ::core::ffi::c_int != '.' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '_' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '-' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn system_subject_matches_scope(
    mut subject: *const ::core::ffi::c_char,
    mut scope: *const ::core::ffi::c_char,
) -> bool {
    if subject.is_null() || scope.is_null() {
        return r#false != 0;
    }
    if strcmp(scope, b"host\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return strcmp(
            subject,
            b"system/host\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    }
    if strcmp(scope, b"container\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return strcmp(
            subject,
            b"system/container\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    }
    if strcmp(scope, b"process\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return strcmp(
            subject,
            b"system/process\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    }
    if strcmp(
        scope,
        b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(scope, b"device\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return strcmp(
            subject,
            b"system/storage\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    }
    return r#false != 0;
}
unsafe extern "C" fn valid_rfc3339_utc(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strnlen(value, 40 as size_t);
    if length < 20 as size_t || length >= 40 as size_t {
        return r#false != 0;
    }
    let mut pattern: *const ::core::ffi::c_char =
        b"dddd-dd-ddTdd:dd:dd\0".as_ptr() as *const ::core::ffi::c_char;
    let mut index: size_t = 0 as size_t;
    while index < 19 as size_t {
        if *pattern.offset(index as isize) as ::core::ffi::c_int == 'd' as ::core::ffi::c_int {
            if *(*__ctype_b_loc()).offset(*value.offset(index as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
            {
                return r#false != 0;
            }
        } else if *value.offset(index as isize) as ::core::ffi::c_int
            != *pattern.offset(index as isize) as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        index = index.wrapping_add(1);
    }
    let mut cursor: size_t = 19 as size_t;
    if *value.offset(cursor as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
        let mut digits: size_t = 0 as size_t;
        cursor = cursor.wrapping_add(1);
        while *(*__ctype_b_loc()).offset(
            *value.offset(cursor as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
        ) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            && digits < 9 as size_t
        {
            cursor = cursor.wrapping_add(1);
            digits = digits.wrapping_add(1);
        }
        if digits == 0 as size_t {
            return r#false != 0;
        }
    }
    return *value.offset(cursor as isize) as ::core::ffi::c_int == 'Z' as ::core::ffi::c_int
        && *value.offset(cursor.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn valid_observation_label(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strnlen(value, OBS_LABEL_SIZE as size_t);
    if length == 0 as size_t || length >= OBS_LABEL_SIZE as size_t {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn validate_observation_changed(
    mut event: *const event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    let mut data: *const cJSON = (*event).data;
    let mut observation_id: *const cJSON = required_field(
        data,
        b"observation_id\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_String,
        error,
        error_size,
    );
    let mut revision: *const cJSON = required_field(
        data,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_Number,
        error,
        error_size,
    );
    let mut camera_uuid: *const cJSON = required_field(
        data,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_String,
        error,
        error_size,
    );
    let mut label: *const cJSON = required_field(
        data,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_String,
        error,
        error_size,
    );
    let mut observed_at: *const cJSON = required_field(
        data,
        b"observed_at\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_String,
        error,
        error_size,
    );
    let mut cause: *const cJSON = required_field(
        data,
        b"change_cause\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_String,
        error,
        error_size,
    );
    let mut state: *const cJSON = required_field(
        data,
        b"evidence_state\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_String,
        error,
        error_size,
    );
    let mut asset_id: *const cJSON = required_field(
        data,
        b"asset_id\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_String,
        error,
        error_size,
    );
    if observation_id.is_null()
        || revision.is_null()
        || camera_uuid.is_null()
        || label.is_null()
        || observed_at.is_null()
        || cause.is_null()
        || state.is_null()
        || asset_id.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut reason: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        data,
        b"evidence_reason\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if reason.is_null() {
        set_error(
            error,
            error_size,
            b"event data is missing required field 'evidence_reason'\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !lightnvr_uuid_is_valid((*observation_id).valuestring)
        || !lightnvr_uuid_is_valid((*asset_id).valuestring)
        || !lightnvr_uuid_is_valid((*camera_uuid).valuestring)
    {
        set_error(
            error,
            error_size,
            b"observation identifiers must be UUIDs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(
        (&raw const (*event).subject as *const ::core::ffi::c_char)
            .offset(7 as ::core::ffi::c_int as isize),
        (*camera_uuid).valuestring,
    ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"observation camera_uuid must match the event subject\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*revision).valuedouble < 1 as ::core::ffi::c_int as ::core::ffi::c_double
        || (*revision).valuedouble > INT_MAX as ::core::ffi::c_double
        || (*revision).valuedouble != (*revision).valueint as ::core::ffi::c_double
    {
        set_error(
            error,
            error_size,
            b"observation revision must be an integer >= 1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !valid_observation_label((*label).valuestring) {
        set_error(
            error,
            error_size,
            b"observation label must be 1..31 bytes\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !valid_rfc3339_utc((*observed_at).valuestring) {
        set_error(
            error,
            error_size,
            b"observation observed_at must be RFC 3339 UTC\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*cause).valuestring,
        b"created\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && strcmp(
            (*cause).valuestring,
            b"evidence\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            (*cause).valuestring,
            b"classification\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"observation change_cause is unknown\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut evidence_state: observation_asset_state_t =
        observation_asset_state_t::OBS_ASSET_PENDING;
    if observation_asset_state_parse((*state).valuestring, &raw mut evidence_state)
        != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"observation evidence_state is unknown\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if cJSON_IsNull(reason) == 0 && cJSON_IsString(reason) == 0 {
        set_error(
            error,
            error_size,
            b"observation evidence_reason must be a string or null\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut reason_text: *const ::core::ffi::c_char = if cJSON_IsString(reason) != 0 {
        (*reason).valuestring
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if !reason_text.is_null()
        && *reason_text.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"observation evidence_reason must not be empty\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !observation_reason_is_valid(evidence_state, reason_text) {
        set_error(
            error,
            error_size,
            b"observation evidence_reason is inconsistent with evidence_state\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut snapshot: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        data,
        b"snapshot_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if evidence_state.0 == observation_asset_state_t::OBS_ASSET_READY.0 {
        let mut expected: [::core::ffi::c_char; 96] = [0; 96];
        snprintf(
            &raw mut expected as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            b"/api/observations/%s/snapshot\0".as_ptr() as *const ::core::ffi::c_char,
            (*observation_id).valuestring,
        );
        if cJSON_IsString(snapshot) == 0
            || strcmp(
                (*snapshot).valuestring,
                &raw mut expected as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            set_error(
                error,
                error_size,
                b"ready observation events need their own snapshot_url\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if !snapshot.is_null() {
        set_error(
            error,
            error_size,
            b"snapshot_url is only allowed when evidence is ready\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn validate_type_data(
    mut event: *const event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    let mut r#type: *const ::core::ffi::c_char =
        &raw const (*event).r#type as *const ::core::ffi::c_char;
    let mut data: *const cJSON = (*event).data;
    if strcmp(
        r#type,
        b"io.lightnvr.detection.observation_changed.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return validate_observation_changed(event, error, error_size);
    }
    if strcmp(
        r#type,
        b"io.lightnvr.detection.object.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut count: *const cJSON = required_field(
            data,
            b"count\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        let mut detections: *const cJSON = required_field(
            data,
            b"detections\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Array,
            error,
            error_size,
        );
        if count.is_null() || detections.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        let mut count_value: ::core::ffi::c_int = (*count).valueint;
        if (*count).valuedouble != count_value as ::core::ffi::c_double
            || count_value <= 0 as ::core::ffi::c_int
            || count_value > 1024 as ::core::ffi::c_int
            || cJSON_GetArraySize(detections) != count_value
        {
            set_error(
                error,
                error_size,
                b"detection count must match a non-empty detections array\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        let mut detection: *const cJSON = (*detections).child;
        while !detection.is_null() {
            if cJSON_IsObject(detection) == 0 {
                set_error(
                    error,
                    error_size,
                    b"each detection must be a JSON object\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            let mut label: *const cJSON = required_field(
                detection,
                b"label\0".as_ptr() as *const ::core::ffi::c_char,
                cJSON_String,
                error,
                error_size,
            );
            let mut confidence: *const cJSON = required_field(
                detection,
                b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
                cJSON_Number,
                error,
                error_size,
            );
            if label.is_null()
                || confidence.is_null()
                || *(*label).valuestring.offset(0isize) == 0
                || (*confidence).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                || (*confidence).valuedouble > 1 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                set_error(
                    error,
                    error_size,
                    b"detection values must use normalized ranges\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            let mut x: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                detection,
                b"x\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut y: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                detection,
                b"y\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut width: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                detection,
                b"width\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut height: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                detection,
                b"height\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut any_box: bool =
                !x.is_null() || !y.is_null() || !width.is_null() || !height.is_null();
            let mut complete_box: bool = cJSON_IsNumber(x) != 0
                && cJSON_IsNumber(y) != 0
                && cJSON_IsNumber(width) != 0
                && cJSON_IsNumber(height) != 0;
            if any_box as ::core::ffi::c_int != 0 && !complete_box
                || complete_box as ::core::ffi::c_int != 0
                    && ((*x).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        || (*x).valuedouble > 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        || (*y).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        || (*y).valuedouble > 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        || (*width).valuedouble <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        || (*width).valuedouble > 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        || (*height).valuedouble
                            <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        || (*height).valuedouble > 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            {
                set_error(
                    error,
                    error_size,
                    b"detection bounding boxes must be complete and normalized\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            let mut track_id: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                detection,
                b"track_id\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !track_id.is_null()
                && (cJSON_IsNumber(track_id) == 0
                    || (*track_id).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    || (*track_id).valuedouble > INT_MAX as ::core::ffi::c_double
                    || (*track_id).valuedouble != (*track_id).valueint as ::core::ffi::c_double)
            {
                set_error(
                    error,
                    error_size,
                    b"detection track_id must be a non-negative integer\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            let mut zone_id: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                detection,
                b"zone_id\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !zone_id.is_null()
                && (cJSON_IsString(zone_id) == 0
                    || *(*zone_id).valuestring.offset(0isize) as ::core::ffi::c_int
                        == '\0' as ::core::ffi::c_int)
            {
                set_error(
                    error,
                    error_size,
                    b"detection zone_id must be a non-empty string\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            detection = (*detection).next;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.recognition.license_plate.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut read_id: *const cJSON = required_field(
            data,
            b"read_id\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut stream_name: *const cJSON = required_field(
            data,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut source: *const cJSON = required_field(
            data,
            b"source\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        if read_id.is_null() || stream_name.is_null() || source.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        if !lightnvr_uuid_is_valid((*read_id).valuestring)
            || !valid_text((*stream_name).valuestring, 256 as size_t, r#false != 0)
            || !valid_text((*source).valuestring, 64 as size_t, r#false != 0)
        {
            set_error(
                error,
                error_size,
                b"protected license-plate event data is invalid\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.camera.offline.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut reason: *const cJSON = required_field(
            data,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut failures: *const cJSON = required_field(
            data,
            b"consecutive_failures\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        if reason.is_null() || failures.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        if *(*reason).valuestring.offset(0isize) == 0
            || (*failures).valueint < 1 as ::core::ffi::c_int
            || (*failures).valuedouble != (*failures).valueint as ::core::ffi::c_double
        {
            set_error(
                error,
                error_size,
                b"camera offline data is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.camera.recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut previous: *const cJSON = required_field(
            data,
            b"previous_state\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut downtime: *const cJSON = required_field(
            data,
            b"downtime_ms\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        if previous.is_null() || downtime.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        if strcmp(
            (*previous).valuestring,
            b"offline\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            || (*downtime).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            set_error(
                error,
                error_size,
                b"camera recovered data is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.stream.degraded.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut reason_0: *const cJSON = required_field(
            data,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut observed: *const cJSON = required_field(
            data,
            b"observed_fps\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        let mut expected: *const cJSON = required_field(
            data,
            b"expected_fps\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        if reason_0.is_null() || observed.is_null() || expected.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        let mut valid_reason: bool = strcmp(
            (*reason_0).valuestring,
            b"low_fps\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*reason_0).valuestring,
                b"stale_frames\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if !valid_reason
            || (*observed).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*expected).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            set_error(
                error,
                error_size,
                b"stream degraded data is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.stream.recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut previous_0: *const cJSON = required_field(
            data,
            b"previous_state\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut observed_0: *const cJSON = required_field(
            data,
            b"observed_fps\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        let mut expected_0: *const cJSON = required_field(
            data,
            b"expected_fps\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        if previous_0.is_null() || observed_0.is_null() || expected_0.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        if strcmp(
            (*previous_0).valuestring,
            b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            || (*observed_0).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*expected_0).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            set_error(
                error,
                error_size,
                b"stream recovered data is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.stream.recording_gap.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut started: *const cJSON = required_field(
            data,
            b"started_at\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut duration: *const cJSON = required_field(
            data,
            b"duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        if started.is_null() || duration.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        if !valid_event_time((*started).valuestring)
            || (*duration).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            set_error(
                error,
                error_size,
                b"recording gap data is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.storage.pressure.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut level: *const cJSON = required_field(
            data,
            b"level\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut used: *const cJSON = required_field(
            data,
            b"used_percent\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        if level.is_null() || used.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        let mut valid_level: bool = strcmp(
            (*level).valuestring,
            b"warning\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*level).valuestring,
                b"critical\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*level).valuestring,
                b"emergency\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if !valid_level
            || (*used).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*used).valuedouble > 100 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            set_error(
                error,
                error_size,
                b"storage pressure data is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.storage.recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut previous_1: *const cJSON = required_field(
            data,
            b"previous_level\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut used_0: *const cJSON = required_field(
            data,
            b"used_percent\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        if previous_1.is_null() || used_0.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        let mut valid_previous: bool = strcmp(
            (*previous_1).valuestring,
            b"warning\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*previous_1).valuestring,
                b"critical\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*previous_1).valuestring,
                b"emergency\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if !valid_previous
            || (*used_0).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*used_0).valuedouble > 100 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            set_error(
                error,
                error_size,
                b"storage recovered data is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.storage.target_unavailable.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut target_uuid: *const cJSON = required_field(
            data,
            b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut previous_2: *const cJSON = required_field(
            data,
            b"previous_state\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut reason_1: *const cJSON = required_field(
            data,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut is_default: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            data,
            b"is_default\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if target_uuid.is_null()
            || previous_2.is_null()
            || reason_1.is_null()
            || cJSON_IsBool(is_default) == 0
        {
            if is_default.is_null() || cJSON_IsBool(is_default) == 0 {
                set_error(
                    error,
                    error_size,
                    b"event data is missing required field 'is_default'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        let mut valid_previous_0: bool = strcmp(
            (*previous_2).valuestring,
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*previous_2).valuestring,
                b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*previous_2).valuestring,
                b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        let mut valid_reason_0: bool = strcmp(
            (*reason_1).valuestring,
            b"mount_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*reason_1).valuestring,
                b"directory_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*reason_1).valuestring,
                b"capacity_probe_failed\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*reason_1).valuestring,
                b"not_writable\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*reason_1).valuestring,
                b"write_probe_failed\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*reason_1).valuestring,
                b"probe_cleanup_failed\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*reason_1).valuestring,
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if !lightnvr_uuid_is_valid((*target_uuid).valuestring)
            || !valid_previous_0
            || !valid_reason_0
        {
            set_error(
                error,
                error_size,
                b"storage target unavailable data is invalid\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.storage.target_recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut target_uuid_0: *const cJSON = required_field(
            data,
            b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut previous_3: *const cJSON = required_field(
            data,
            b"previous_state\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut current: *const cJSON = required_field(
            data,
            b"current_state\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut downtime_0: *const cJSON = required_field(
            data,
            b"downtime_ms\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        let mut is_default_0: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            data,
            b"is_default\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if target_uuid_0.is_null()
            || previous_3.is_null()
            || current.is_null()
            || downtime_0.is_null()
            || cJSON_IsBool(is_default_0) == 0
        {
            if is_default_0.is_null() || cJSON_IsBool(is_default_0) == 0 {
                set_error(
                    error,
                    error_size,
                    b"event data is missing required field 'is_default'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        let mut valid_current: bool = strcmp(
            (*current).valuestring,
            b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*current).valuestring,
                b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if !lightnvr_uuid_is_valid((*target_uuid_0).valuestring)
            || strcmp(
                (*previous_3).valuestring,
                b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            || !valid_current
            || (*downtime_0).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            set_error(
                error,
                error_size,
                b"storage target recovered data is invalid\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.system.health_alert.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut incident_id: *const cJSON = required_field(
            data,
            b"incident_id\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut code: *const cJSON = required_field(
            data,
            b"code\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut scope: *const cJSON = required_field(
            data,
            b"scope\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut resource: *const cJSON = required_field(
            data,
            b"resource\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut state: *const cJSON = required_field(
            data,
            b"state\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut severity: *const cJSON = required_field(
            data,
            b"severity\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut observed_1: *const cJSON = required_field(
            data,
            b"observed\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Object,
            error,
            error_size,
        );
        let mut threshold: *const cJSON = required_field(
            data,
            b"threshold\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Object,
            error,
            error_size,
        );
        if incident_id.is_null()
            || code.is_null()
            || scope.is_null()
            || resource.is_null()
            || state.is_null()
            || severity.is_null()
            || observed_1.is_null()
            || threshold.is_null()
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut valid_state: bool = strcmp(
            (*state).valuestring,
            b"open\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*state).valuestring,
                b"escalated\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*state).valuestring,
                b"updated\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if !lightnvr_uuid_is_valid((*incident_id).valuestring)
            || !valid_health_token((*code).valuestring)
            || !valid_health_token((*resource).valuestring)
            || !valid_state
            || !system_subject_matches_scope(
                &raw const (*event).subject as *const ::core::ffi::c_char,
                (*scope).valuestring,
            )
            || strcmp(
                (*severity).valuestring,
                event_severity_name((*event).severity),
            ) != 0 as ::core::ffi::c_int
        {
            set_error(
                error,
                error_size,
                b"system health alert data or severity is invalid\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if strcmp(
        r#type,
        b"io.lightnvr.system.health_recovered.v1\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut incident_id_0: *const cJSON = required_field(
            data,
            b"incident_id\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut code_0: *const cJSON = required_field(
            data,
            b"code\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut scope_0: *const cJSON = required_field(
            data,
            b"scope\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut resource_0: *const cJSON = required_field(
            data,
            b"resource\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut state_0: *const cJSON = required_field(
            data,
            b"state\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut previous_4: *const cJSON = required_field(
            data,
            b"previous_severity\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_String,
            error,
            error_size,
        );
        let mut duration_0: *const cJSON = required_field(
            data,
            b"duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Number,
            error,
            error_size,
        );
        let mut safe: *const cJSON = required_field(
            data,
            b"safe_observation\0".as_ptr() as *const ::core::ffi::c_char,
            cJSON_Object,
            error,
            error_size,
        );
        if incident_id_0.is_null()
            || code_0.is_null()
            || scope_0.is_null()
            || resource_0.is_null()
            || state_0.is_null()
            || previous_4.is_null()
            || duration_0.is_null()
            || safe.is_null()
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut valid_previous_1: bool = strcmp(
            (*previous_4).valuestring,
            b"warning\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*previous_4).valuestring,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                (*previous_4).valuestring,
                b"critical\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if !lightnvr_uuid_is_valid((*incident_id_0).valuestring)
            || !valid_health_token((*code_0).valuestring)
            || !valid_health_token((*resource_0).valuestring)
            || strcmp(
                (*state_0).valuestring,
                b"recovered\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            || !valid_previous_1
            || (*duration_0).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || !system_subject_matches_scope(
                &raw const (*event).subject as *const ::core::ffi::c_char,
                (*scope_0).valuestring,
            )
        {
            set_error(
                error,
                error_size,
                b"system health recovery data is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_registry_all(
    mut count: *mut ::core::ffi::c_int,
) -> *const event_type_definition_t {
    if !count.is_null() {
        *count = ::core::mem::size_of::<[event_type_definition_t; 14]>()
            .wrapping_div(::core::mem::size_of::<event_type_definition_t>())
            as ::core::ffi::c_int;
    }
    return &raw const EVENT_TYPES as *const event_type_definition_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_registry_find(
    mut r#type: *const ::core::ffi::c_char,
) -> *const event_type_definition_t {
    if r#type.is_null() {
        return ::core::ptr::null::<event_type_definition_t>();
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut definitions: *const event_type_definition_t = event_registry_all(&raw mut count);
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        if strcmp((*definitions.offset(index as isize)).r#type, r#type) == 0 as ::core::ffi::c_int {
            return definitions.offset(index as isize);
        }
        index += 1;
    }
    return ::core::ptr::null::<event_type_definition_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_severity_name(
    mut severity: event_severity_t,
) -> *const ::core::ffi::c_char {
    match severity {
        event_severity_t::EVENT_SEVERITY_INFO => {
            return b"info\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_severity_t::EVENT_SEVERITY_WARNING => {
            return b"warning\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_severity_t::EVENT_SEVERITY_ERROR => {
            return b"error\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_severity_t::EVENT_SEVERITY_CRITICAL => {
            return b"critical\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_sensitivity_name(
    mut sensitivity: event_sensitivity_t,
) -> *const ::core::ffi::c_char {
    match sensitivity {
        event_sensitivity_t::EVENT_SENSITIVITY_OPERATIONAL => {
            return b"operational\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_sensitivity_t::EVENT_SENSITIVITY_INTERNAL => {
            return b"internal\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_sensitivity_t::EVENT_SENSITIVITY_RESTRICTED => {
            return b"restricted\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_media_policy_name(
    mut policy: event_media_policy_t,
) -> *const ::core::ffi::c_char {
    match policy {
        event_media_policy_t::EVENT_MEDIA_FORBIDDEN => {
            return b"forbidden\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_media_policy_t::EVENT_MEDIA_REFERENCE_ALLOWED => {
            return b"reference_allowed\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_media_policy_t::EVENT_MEDIA_BINARY_OPT_IN => {
            return b"binary_opt_in\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_expected_rate_name(
    mut rate: event_expected_rate_t,
) -> *const ::core::ffi::c_char {
    match rate {
        event_expected_rate_t::EVENT_RATE_LOW => {
            return b"low\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_expected_rate_t::EVENT_RATE_MEDIUM => {
            return b"medium\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_expected_rate_t::EVENT_RATE_HIGH => {
            return b"high\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_envelope_validate(
    mut event: *const event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if event.is_null() {
        set_error(
            error,
            error_size,
            b"event is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !valid_text(
        &raw const (*event).r#type as *const ::core::ffi::c_char,
        EVENT_TYPE_MAX as size_t,
        r#false != 0,
    ) {
        set_error(
            error,
            error_size,
            b"event type is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut definition: *const event_type_definition_t =
        event_registry_find(&raw const (*event).r#type as *const ::core::ffi::c_char);
    if definition.is_null() {
        set_error(
            error,
            error_size,
            b"event type is not registered\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !valid_text(
        &raw const (*event).specversion as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4]>(),
        r#false != 0,
    ) || !valid_text(
        &raw const (*event).datacontenttype as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        r#false != 0,
    ) || strcmp(
        &raw const (*event).specversion as *const ::core::ffi::c_char,
        b"1.0\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*event).datacontenttype as *const ::core::ffi::c_char,
            b"application/json\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"event envelope version or content type is invalid\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !lightnvr_uuid_is_valid(&raw const (*event).id as *const ::core::ffi::c_char) {
        set_error(
            error,
            error_size,
            b"event id must be a UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !valid_text(
        &raw const (*event).source as *const ::core::ffi::c_char,
        EVENT_SOURCE_MAX as size_t,
        r#false != 0,
    ) || strncmp(
        &raw const (*event).source as *const ::core::ffi::c_char,
        b"urn:lightnvr:\0".as_ptr() as *const ::core::ffi::c_char,
        13 as size_t,
    ) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(
            (&raw const (*event).source as *const ::core::ffi::c_char)
                .offset(13 as ::core::ffi::c_int as isize),
        )
    {
        set_error(
            error,
            error_size,
            b"event source must use urn:lightnvr:<installation-uuid>\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !valid_text(
        &raw const (*event).subject as *const ::core::ffi::c_char,
        EVENT_SUBJECT_MAX as size_t,
        r#false != 0,
    ) {
        set_error(
            error,
            error_size,
            b"event subject is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*definition).subject_kind.0 == event_subject_kind_t::EVENT_SUBJECT_CAMERA.0 {
        if strncmp(
            &raw const (*event).subject as *const ::core::ffi::c_char,
            b"camera/\0".as_ptr() as *const ::core::ffi::c_char,
            7 as size_t,
        ) != 0 as ::core::ffi::c_int
            || !lightnvr_uuid_is_valid(
                (&raw const (*event).subject as *const ::core::ffi::c_char)
                    .offset(7 as ::core::ffi::c_int as isize),
            )
        {
            set_error(
                error,
                error_size,
                b"camera event subject must use camera/<uuid>\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    } else if (*definition).subject_kind.0 == event_subject_kind_t::EVENT_SUBJECT_STORAGE.0
        && strcmp(
            &raw const (*event).subject as *const ::core::ffi::c_char,
            b"system/storage\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"storage event subject must use system/storage\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    } else if (*definition).subject_kind.0 == event_subject_kind_t::EVENT_SUBJECT_SYSTEM.0
        && strcmp(
            &raw const (*event).subject as *const ::core::ffi::c_char,
            b"system/host\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*event).subject as *const ::core::ffi::c_char,
            b"system/container\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*event).subject as *const ::core::ffi::c_char,
            b"system/process\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*event).subject as *const ::core::ffi::c_char,
            b"system/storage\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"system event subject is not a supported logical scope\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut minimum: event_severity_t = event_severity_t(
        if (*definition).dynamic_severity as ::core::ffi::c_int != 0 {
            (*definition).minimum_severity.0
        } else {
            (*definition).severity.0
        },
    );
    let mut maximum: event_severity_t = event_severity_t(
        if (*definition).dynamic_severity as ::core::ffi::c_int != 0 {
            (*definition).maximum_severity.0
        } else {
            (*definition).severity.0
        },
    );
    if (*event).severity.0 < minimum.0 || (*event).severity.0 > maximum.0 {
        set_error(
            error,
            error_size,
            b"event severity is outside the registered range\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !valid_event_time(&raw const (*event).time as *const ::core::ffi::c_char)
        || (*event).occurred_at <= 0 as time_t
        || (*event).expires_at
            != (*event).occurred_at + (*definition).default_expiry_seconds as time_t
    {
        set_error(
            error,
            error_size,
            b"event timestamps are invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*event).data.is_null() || cJSON_IsObject((*event).data) == 0 {
        set_error(
            error,
            error_size,
            b"event data must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if validate_data_node((*event).data, 0 as ::core::ffi::c_int, error, error_size)
        != 0 as ::core::ffi::c_int
        || validate_type_data(event, error, error_size) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn check_data_size(
    mut data: *const cJSON,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    let mut data_json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(data);
    if data_json.is_null() {
        set_error(
            error,
            error_size,
            b"event data could not be serialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data_size: size_t = strlen(data_json);
    free(data_json as *mut ::core::ffi::c_void);
    if data_size > EVENT_DATA_MAX_BYTES as size_t {
        set_error(
            error,
            error_size,
            b"event data exceeds maximum size\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_envelope_create(
    mut event: *mut event_envelope_t,
    mut r#type: *const ::core::ffi::c_char,
    mut source: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut occurred_at: time_t,
    mut data: *const cJSON,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    let mut definition: *const event_type_definition_t = event_registry_find(r#type);
    if definition.is_null() {
        if !error.is_null() && error_size > 0 as size_t {
            *error.offset(0isize) = '\0' as ::core::ffi::c_char;
        }
        set_error(
            error,
            error_size,
            b"event type is not registered\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return event_envelope_create_with_severity_and_id(
        event,
        r#type,
        source,
        subject,
        occurred_at,
        data,
        (*definition).severity,
        ::core::ptr::null::<::core::ffi::c_char>(),
        error,
        error_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_envelope_create_with_severity_and_id(
    mut event: *mut event_envelope_t,
    mut r#type: *const ::core::ffi::c_char,
    mut source: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut occurred_at: time_t,
    mut data: *const cJSON,
    mut severity: event_severity_t,
    mut event_id: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if event.is_null() {
        set_error(
            error,
            error_size,
            b"event output is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_envelope_t>(),
    );
    if !valid_text(r#type, EVENT_TYPE_MAX as size_t, r#false != 0)
        || !valid_text(source, EVENT_SOURCE_MAX as size_t, r#false != 0)
        || !valid_text(subject, EVENT_SUBJECT_MAX as size_t, r#false != 0)
        || data.is_null()
        || cJSON_IsObject(data) == 0
    {
        set_error(
            error,
            error_size,
            b"event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if event_registry_find(r#type).is_null() {
        set_error(
            error,
            error_size,
            b"event type is not registered\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        &raw mut (*event).specversion as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4]>(),
        b"1.0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*event).r#type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        r#type,
    );
    snprintf(
        &raw mut (*event).source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        source,
    );
    snprintf(
        &raw mut (*event).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        subject,
    );
    snprintf(
        &raw mut (*event).datacontenttype as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"application/json\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*event).occurred_at = if occurred_at > 0 as time_t {
        occurred_at
    } else {
        time(::core::ptr::null_mut::<time_t>())
    };
    let mut definition: *const event_type_definition_t = event_registry_find(r#type);
    (*event).expires_at = (*event).occurred_at + (*definition).default_expiry_seconds as time_t;
    (*event).severity = severity;
    let mut identity_result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !event_id.is_null() {
        if !lightnvr_uuid_is_valid(event_id) {
            identity_result = -1 as ::core::ffi::c_int;
        } else {
            snprintf(
                &raw mut (*event).id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                event_id,
            );
        }
    } else {
        identity_result =
            lightnvr_uuid_generate_v4(&raw mut (*event).id as *mut ::core::ffi::c_char);
    }
    if identity_result != 0 as ::core::ffi::c_int
        || format_event_time(
            (*event).occurred_at,
            &raw mut (*event).time as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"event identity or timestamp generation failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        event_envelope_clear(event);
        return -1 as ::core::ffi::c_int;
    }
    (*event).data = cJSON_Duplicate(data, r#true);
    if (*event).data.is_null() {
        set_error(
            error,
            error_size,
            b"event data allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        event_envelope_clear(event);
        return -1 as ::core::ffi::c_int;
    }
    if event_envelope_validate(event, error, error_size) != 0 as ::core::ffi::c_int
        || check_data_size((*event).data, error, error_size) != 0 as ::core::ffi::c_int
    {
        event_envelope_clear(event);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_envelope_serialize(
    mut event: *const event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> *mut ::core::ffi::c_char {
    if event_envelope_validate(event, error, error_size) != 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut definition: *const event_type_definition_t =
        event_registry_find(&raw const (*event).r#type as *const ::core::ffi::c_char);
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        set_error(
            error,
            error_size,
            b"event envelope allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut valid: bool = !cJSON_AddStringToObject(
        root,
        b"specversion\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).specversion as *const ::core::ffi::c_char,
    )
    .is_null()
        && !cJSON_AddStringToObject(
            root,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).id as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            root,
            b"type\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).r#type as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            root,
            b"source\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).source as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            root,
            b"subject\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).subject as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            root,
            b"time\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).time as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            root,
            b"datacontenttype\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).datacontenttype as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            root,
            b"severity\0".as_ptr() as *const ::core::ffi::c_char,
            event_severity_name((*event).severity),
        )
        .is_null()
        && !cJSON_AddStringToObject(
            root,
            b"sensitivity\0".as_ptr() as *const ::core::ffi::c_char,
            event_sensitivity_name((*definition).sensitivity),
        )
        .is_null();
    if !valid
        || cJSON_AddItemReferenceToObject(
            root,
            b"data\0".as_ptr() as *const ::core::ffi::c_char,
            (*event).data,
        ) == 0
    {
        cJSON_Delete(root);
        set_error(
            error,
            error_size,
            b"event envelope allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut serialized: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if serialized.is_null() {
        set_error(
            error,
            error_size,
            b"event envelope serialization failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if strlen(serialized) > EVENT_ENVELOPE_MAX_BYTES as size_t {
        free(serialized as *mut ::core::ffi::c_void);
        set_error(
            error,
            error_size,
            b"event envelope exceeds maximum size\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return serialized;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_envelope_clone(
    mut destination: *mut event_envelope_t,
    mut source: *const event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if destination.is_null() {
        set_error(
            error,
            error_size,
            b"event clone destination is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        destination as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_envelope_t>(),
    );
    if event_envelope_validate(source, error, error_size) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    *destination = *source;
    (*destination).data = cJSON_Duplicate((*source).data, r#true);
    if (*destination).data.is_null() {
        memset(
            destination as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<event_envelope_t>(),
        );
        set_error(
            error,
            error_size,
            b"event clone allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_envelope_clear(mut event: *mut event_envelope_t) {
    if event.is_null() {
        return;
    }
    cJSON_Delete((*event).data);
    memset(
        event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_envelope_t>(),
    );
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
