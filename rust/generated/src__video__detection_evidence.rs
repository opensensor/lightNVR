unsafe extern "C" {
    unsafe fn observation_evidence_submit(
        batch: *const observation_batch_input_t,
        rgb24: *mut uint8_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        transform: *const observation_evidence_transform_t,
        asset_uuid_out: *mut ::core::ffi::c_char,
    ) -> observation_evidence_result_t;
    unsafe fn observation_evidence_submit_jpeg(
        batch: *const observation_batch_input_t,
        jpeg: *mut uint8_t,
        jpeg_size: size_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        transform: *const observation_evidence_transform_t,
        asset_uuid_out: *mut ::core::ffi::c_char,
    ) -> observation_evidence_result_t;
    unsafe fn observation_evidence_submit_unavailable(
        batch: *const observation_batch_input_t,
        reason: *const ::core::ffi::c_char,
        asset_uuid_out: *mut ::core::ffi::c_char,
    ) -> observation_evidence_result_t;
    unsafe fn lroundf(__x: ::core::ffi::c_float) -> ::core::ffi::c_long;
    unsafe fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
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
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn sod_model_input_size(
        model: detection_model_t,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_once_t = ::core::ffi::c_int;
pub type uint8_t = u8;
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
pub struct detection_t {
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub width: ::core::ffi::c_float,
    pub height: ::core::ffi::c_float,
    pub track_id: ::core::ffi::c_int,
    pub zone_id: [::core::ffi::c_char; 32],
}
pub type detection_model_t = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_evidence_engine_t {
    pub engine_key: *const ::core::ffi::c_char,
    pub engine_type: *const ::core::ffi::c_char,
    pub model_path: *const ::core::ffi::c_char,
    pub model_version: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_evidence_camera_t {
    pub camera_uuid: *const ::core::ffi::c_char,
    pub stream_name: *const ::core::ffi::c_char,
    pub evidence_enabled: bool,
    pub labels_csv: *const ::core::ffi::c_char,
    pub recording_id: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct detection_evidence_status_t(pub ::core::ffi::c_uint);
impl detection_evidence_status_t {
    pub const DETECTION_EVIDENCE_SUBMITTED: Self = Self(0);
    pub const DETECTION_EVIDENCE_NOT_ENABLED: Self = Self(1);
    pub const DETECTION_EVIDENCE_NOT_ELIGIBLE: Self = Self(2);
    pub const DETECTION_EVIDENCE_NOT_OBJECT_ENGINE: Self = Self(3);
    pub const DETECTION_EVIDENCE_ERROR: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct model_t {
    pub r#type: [::core::ffi::c_char; 16],
    pub c2rust_unnamed: C2Rust_Unnamed,
    pub threshold: ::core::ffi::c_float,
    pub path: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub sod: *mut ::core::ffi::c_void,
    pub sod_realnet: *mut ::core::ffi::c_void,
    pub tflite: tflite_model_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tflite_model_t {
    pub engine: *mut ::core::ffi::c_void,
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
pub const OBS_MAX_BATCH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const OBS_LABEL_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MODEL_TYPE_SOD: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"sod\0") };
pub const PTHREAD_ONCE_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DEFAULT_ENGINE_KEY: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"legacy-primary\0") };
static mut boot_once: pthread_once_t = PTHREAD_ONCE_INIT;
static mut boot_session: [::core::ffi::c_char; 37] = [0; 37];
unsafe extern "C" fn boot_session_init() {
    if lightnvr_uuid_generate_v4(&raw mut boot_session as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        boot_session[0usize] = '\0' as ::core::ffi::c_char;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_boot_session() -> *const ::core::ffi::c_char {
    pthread_once(
        &raw mut boot_once,
        Some(boot_session_init as unsafe extern "C" fn() -> ()),
    );
    return &raw mut boot_session as *mut ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_label_eligible(
    mut labels_csv: *const ::core::ffi::c_char,
    mut label: *const ::core::ffi::c_char,
) -> bool {
    if label.is_null() || *label.offset(0isize) == 0 {
        return r#false != 0;
    }
    if labels_csv.is_null() {
        return r#true != 0;
    }
    let mut scan: *const ::core::ffi::c_char = labels_csv;
    while *scan as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *scan as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        scan = scan.offset(1);
    }
    if *scan as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#true != 0;
    }
    let mut label_len: size_t = strlen(label);
    let mut cursor: *const ::core::ffi::c_char = labels_csv;
    loop {
        let mut end: *const ::core::ffi::c_char = strchr(cursor, ',' as ::core::ffi::c_int);
        let mut stop: *const ::core::ffi::c_char = if !end.is_null() {
            end
        } else {
            cursor.offset(strlen(cursor) as isize)
        };
        let mut first: *const ::core::ffi::c_char = cursor;
        let mut last: *const ::core::ffi::c_char = stop;
        while first < last
            && (*first as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *first as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
        {
            first = first.offset(1);
        }
        while last > first
            && (*last.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *last.offset(-1isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
        {
            last = last.offset(-1);
        }
        if last.offset_from(first) as size_t == label_len
            && strncasecmp(first, label, label_len) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        if end.is_null() {
            return r#false != 0;
        }
        cursor = end.offset(1 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn clamp01(mut v: ::core::ffi::c_double) -> ::core::ffi::c_double {
    if !(v > 0.0f64) {
        return 0.0f64;
    }
    return if v > 1.0f64 { 1.0f64 } else { v };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_clamp_box(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
    mut ox: *mut ::core::ffi::c_double,
    mut oy: *mut ::core::ffi::c_double,
    mut ow: *mut ::core::ffi::c_double,
    mut oh: *mut ::core::ffi::c_double,
) -> bool {
    if x.is_finite() as ::core::ffi::c_int == 0
        || y.is_finite() as ::core::ffi::c_int == 0
        || w.is_finite() as ::core::ffi::c_int == 0
        || h.is_finite() as ::core::ffi::c_int == 0
    {
        return r#false != 0;
    }
    let mut x0: ::core::ffi::c_double = clamp01(x);
    let mut y0: ::core::ffi::c_double = clamp01(y);
    let mut x1: ::core::ffi::c_double = clamp01(x + w);
    let mut y1: ::core::ffi::c_double = clamp01(y + h);
    if x1 - x0 < 1e-6f64 || y1 - y0 < 1e-6f64 {
        return r#false != 0;
    }
    if !ox.is_null() {
        *ox = x0;
    }
    if !oy.is_null() {
        *oy = y0;
    }
    if !ow.is_null() {
        *ow = x1 - x0;
    }
    if !oh.is_null() {
        *oh = y1 - y0;
    }
    return r#true != 0;
}
unsafe extern "C" fn basename_of(
    mut path: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if path.is_null() {
        return b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let mut slash: *const ::core::ffi::c_char = strrchr(path, '/' as ::core::ffi::c_int);
    return if !slash.is_null() {
        slash.offset(1 as ::core::ffi::c_int as isize)
    } else {
        path
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_build_batch(
    mut camera: *const detection_evidence_camera_t,
    mut engine: *const detection_evidence_engine_t,
    mut detections: *const detection_t,
    mut count: ::core::ffi::c_int,
    mut observed_at_ms: int64_t,
    mut out: *mut observation_batch_input_t,
) -> ::core::ffi::c_int {
    if camera.is_null()
        || engine.is_null()
        || out.is_null()
        || count < 0 as ::core::ffi::c_int
        || count > 0 as ::core::ffi::c_int && detections.is_null()
        || (*camera).camera_uuid.is_null()
        || *(*camera).camera_uuid.offset(0isize) == 0
        || observed_at_ms <= 0 as int64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_batch_input_t>(),
    );
    safe_strcpy(
        &raw mut (*out).camera_uuid as *mut ::core::ffi::c_char,
        (*camera).camera_uuid,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    if !(*camera).stream_name.is_null() {
        safe_strcpy(
            &raw mut (*out).stream_name as *mut ::core::ffi::c_char,
            (*camera).stream_name,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    (*out).observed_at_ms = observed_at_ms;
    (*out).recording_id = if (*camera).recording_id > 0 as int64_t {
        (*camera).recording_id
    } else {
        0 as int64_t
    };
    let mut key: *const ::core::ffi::c_char = if !(*engine).engine_key.is_null()
        && *(*engine).engine_key.offset(0isize) as ::core::ffi::c_int != 0
    {
        (*engine).engine_key
    } else {
        DEFAULT_ENGINE_KEY.as_ptr()
    };
    safe_strcpy(
        &raw mut (*out).engine_key as *mut ::core::ffi::c_char,
        key,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*out).engine_type as *mut ::core::ffi::c_char,
        if !(*engine).engine_type.is_null()
            && *(*engine).engine_type.offset(0isize) as ::core::ffi::c_int != 0
        {
            (*engine).engine_type
        } else {
            b"object\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut model: *const ::core::ffi::c_char = basename_of((*engine).model_path);
    if strlen(model) < ::core::mem::size_of::<[::core::ffi::c_char; 129]>() {
        safe_strcpy(
            &raw mut (*out).model as *mut ::core::ffi::c_char,
            model,
            ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
            0 as size_t,
        );
    }
    if !(*engine).model_version.is_null()
        && strlen((*engine).model_version) < ::core::mem::size_of::<[::core::ffi::c_char; 65]>()
    {
        safe_strcpy(
            &raw mut (*out).model_version as *mut ::core::ffi::c_char,
            (*engine).model_version,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
            0 as size_t,
        );
    }
    let mut session: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count && n < OBS_MAX_BATCH {
        let mut d: *const detection_t = detections.offset(i as isize);
        let mut label_len: size_t = strnlen(
            &raw const (*d).label as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        );
        if !(label_len == 0 as size_t || label_len >= OBS_LABEL_SIZE as size_t) {
            if detection_evidence_label_eligible(
                (*camera).labels_csv,
                &raw const (*d).label as *const ::core::ffi::c_char,
            ) {
                if (*d).confidence.is_finite() as ::core::ffi::c_int != 0 {
                    let mut o: *mut observation_object_input_t = (&raw mut (*out).objects
                        as *mut observation_object_input_t)
                        .offset(n as isize);
                    (*o).object_index = n;
                    memcpy(
                        &raw mut (*o).label as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        &raw const (*d).label as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        label_len,
                    );
                    (*o).label[label_len] = '\0' as ::core::ffi::c_char;
                    (*o).confidence = clamp01((*d).confidence as ::core::ffi::c_double);
                    (*o).has_box = detection_evidence_clamp_box(
                        (*d).x as ::core::ffi::c_double,
                        (*d).y as ::core::ffi::c_double,
                        (*d).width as ::core::ffi::c_double,
                        (*d).height as ::core::ffi::c_double,
                        &raw mut (*o).box_x,
                        &raw mut (*o).box_y,
                        &raw mut (*o).box_width,
                        &raw mut (*o).box_height,
                    );
                    if (*d).track_id >= 0 as ::core::ffi::c_int {
                        if session.is_null() {
                            session = detection_evidence_boot_session();
                        }
                        if *session.offset(0isize) != 0 {
                            (*o).has_track = r#true != 0;
                            snprintf(
                                &raw mut (*o).track_id as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
                                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                                (*d).track_id,
                            );
                            safe_strcpy(
                                &raw mut (*o).track_namespace as *mut ::core::ffi::c_char,
                                &raw mut (*out).engine_key as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
                                0 as size_t,
                            );
                            safe_strcpy(
                                &raw mut (*o).track_session as *mut ::core::ffi::c_char,
                                session,
                                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                                0 as size_t,
                            );
                        }
                    }
                    n += 1;
                }
            }
        }
        i += 1;
    }
    (*out).object_count = n;
    if n == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if lightnvr_uuid_generate_v4(&raw mut (*out).batch_id as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return n;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_transform_compute(
    mut frame_w: ::core::ffi::c_int,
    mut frame_h: ::core::ffi::c_int,
    mut net_w: ::core::ffi::c_int,
    mut net_h: ::core::ffi::c_int,
    mut letterbox: bool,
    mut out: *mut observation_evidence_transform_t,
) {
    if out.is_null() {
        return;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_evidence_transform_t>(),
    );
    (*out).mode = observation_transform_mode_t::OBS_TRANSFORM_NONE;
    (*out).inference_width = frame_w;
    (*out).source_width = (*out).inference_width;
    (*out).inference_height = frame_h;
    (*out).source_height = (*out).inference_height;
    (*out).scale_y = 1.0f64;
    (*out).scale_x = (*out).scale_y;
    if frame_w <= 0 as ::core::ffi::c_int
        || frame_h <= 0 as ::core::ffi::c_int
        || net_w <= 0 as ::core::ffi::c_int
        || net_h <= 0 as ::core::ffi::c_int
        || net_w == frame_w && net_h == frame_h
    {
        return;
    }
    (*out).inference_width = net_w;
    (*out).inference_height = net_h;
    if !letterbox {
        (*out).mode = observation_transform_mode_t::OBS_TRANSFORM_RESIZE;
        (*out).scale_x = net_w as ::core::ffi::c_double / frame_w as ::core::ffi::c_double;
        (*out).scale_y = net_h as ::core::ffi::c_double / frame_h as ::core::ffi::c_double;
        return;
    }
    let mut sx: ::core::ffi::c_float =
        net_w as ::core::ffi::c_float / frame_w as ::core::ffi::c_float;
    let mut sy: ::core::ffi::c_float =
        net_h as ::core::ffi::c_float / frame_h as ::core::ffi::c_float;
    let mut scale: ::core::ffi::c_float = if sx < sy { sx } else { sy };
    let mut new_w: ::core::ffi::c_long = lroundf(frame_w as ::core::ffi::c_float * scale);
    let mut new_h: ::core::ffi::c_long = lroundf(frame_h as ::core::ffi::c_float * scale);
    if new_w < 1 as ::core::ffi::c_long {
        new_w = 1 as ::core::ffi::c_long;
    }
    if new_h < 1 as ::core::ffi::c_long {
        new_h = 1 as ::core::ffi::c_long;
    }
    (*out).mode = observation_transform_mode_t::OBS_TRANSFORM_LETTERBOX;
    (*out).scale_y = scale as ::core::ffi::c_double;
    (*out).scale_x = (*out).scale_y;
    (*out).pad_x =
        ((net_w - new_w as ::core::ffi::c_int) / 2 as ::core::ffi::c_int) as ::core::ffi::c_double;
    (*out).pad_y =
        ((net_h - new_h as ::core::ffi::c_int) / 2 as ::core::ffi::c_int) as ::core::ffi::c_double;
    if (*out).pad_x < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        (*out).pad_x = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if (*out).pad_y < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        (*out).pad_y = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_model_geometry(
    mut model: detection_model_t,
    mut net_w: *mut ::core::ffi::c_int,
    mut net_h: *mut ::core::ffi::c_int,
    mut letterbox: *mut bool,
) -> ::core::ffi::c_int {
    if model.is_null() || net_w.is_null() || net_h.is_null() || letterbox.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut m: *const model_t = model as *const model_t;
    if strcmp(
        &raw const (*m).r#type as *const ::core::ffi::c_char,
        MODEL_TYPE_SOD.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        if sod_model_input_size(model, net_w, net_h) != 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        *letterbox = r#false != 0;
        return 0 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_observed_at_ms(mut frame_ts: time_t) -> int64_t {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&raw mut tv, NULL);
    if frame_ts <= 0 as time_t {
        return tv.tv_sec as int64_t * 1000 as int64_t + tv.tv_usec as int64_t / 1000 as int64_t;
    }
    if tv.tv_sec == frame_ts {
        return tv.tv_sec as int64_t * 1000 as int64_t + tv.tv_usec as int64_t / 1000 as int64_t;
    }
    return frame_ts as int64_t * 1000 as int64_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_capture(
    mut camera: *const detection_evidence_camera_t,
    mut engine: *const detection_evidence_engine_t,
    mut detections: *const detection_t,
    mut count: ::core::ffi::c_int,
    mut observed_at_ms: int64_t,
    mut rgb24: *mut uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut transform: *const observation_evidence_transform_t,
    mut submit_result: *mut observation_evidence_result_t,
    mut asset_uuid_out: *mut ::core::ffi::c_char,
) -> detection_evidence_status_t {
    if !asset_uuid_out.is_null() {
        *asset_uuid_out.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if camera.is_null() || engine.is_null() {
        free(rgb24 as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_ERROR;
    }
    if !(*camera).evidence_enabled {
        free(rgb24 as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_NOT_ENABLED;
    }
    if (*engine).engine_type.is_null()
        || strcmp(
            (*engine).engine_type,
            b"object\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        free(rgb24 as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_NOT_OBJECT_ENGINE;
    }
    if rgb24.is_null() || width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
        free(rgb24 as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_ERROR;
    }
    let mut batch: *mut observation_batch_input_t = malloc(::core::mem::size_of::<
        observation_batch_input_t,
    >()) as *mut observation_batch_input_t;
    if batch.is_null() {
        free(rgb24 as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_ERROR;
    }
    let mut n: ::core::ffi::c_int =
        detection_evidence_build_batch(camera, engine, detections, count, observed_at_ms, batch);
    if n <= 0 as ::core::ffi::c_int {
        free(batch as *mut ::core::ffi::c_void);
        free(rgb24 as *mut ::core::ffi::c_void);
        return detection_evidence_status_t(
            (if n == 0 as ::core::ffi::c_int {
                detection_evidence_status_t::DETECTION_EVIDENCE_NOT_ELIGIBLE.0 as ::core::ffi::c_int
            } else {
                detection_evidence_status_t::DETECTION_EVIDENCE_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
    }
    let mut r: observation_evidence_result_t =
        observation_evidence_submit(batch, rgb24, width, height, transform, asset_uuid_out);
    if !submit_result.is_null() {
        *submit_result = r;
    }
    if r.0 != observation_evidence_result_t::OBS_EVIDENCE_ACCEPTED.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Observation evidence not accepted (result %d, %d object(s))\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !(*camera).stream_name.is_null() {
                (*camera).stream_name
            } else {
                b"?\0".as_ptr() as *const ::core::ffi::c_char
            },
            r.0,
            n,
        );
    }
    free(batch as *mut ::core::ffi::c_void);
    return detection_evidence_status_t::DETECTION_EVIDENCE_SUBMITTED;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_capture_jpeg(
    mut camera: *const detection_evidence_camera_t,
    mut engine: *const detection_evidence_engine_t,
    mut detections: *const detection_t,
    mut count: ::core::ffi::c_int,
    mut observed_at_ms: int64_t,
    mut jpeg: *mut uint8_t,
    mut jpeg_size: size_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut transform: *const observation_evidence_transform_t,
    mut submit_result: *mut observation_evidence_result_t,
    mut asset_uuid_out: *mut ::core::ffi::c_char,
) -> detection_evidence_status_t {
    if !asset_uuid_out.is_null() {
        *asset_uuid_out.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if camera.is_null() || engine.is_null() {
        free(jpeg as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_ERROR;
    }
    if !(*camera).evidence_enabled {
        free(jpeg as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_NOT_ENABLED;
    }
    if (*engine).engine_type.is_null()
        || strcmp(
            (*engine).engine_type,
            b"api\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        free(jpeg as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_NOT_OBJECT_ENGINE;
    }
    if jpeg.is_null()
        || jpeg_size == 0 as size_t
        || width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
    {
        free(jpeg as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_ERROR;
    }
    let mut batch: *mut observation_batch_input_t = malloc(::core::mem::size_of::<
        observation_batch_input_t,
    >()) as *mut observation_batch_input_t;
    if batch.is_null() {
        free(jpeg as *mut ::core::ffi::c_void);
        return detection_evidence_status_t::DETECTION_EVIDENCE_ERROR;
    }
    let mut n: ::core::ffi::c_int =
        detection_evidence_build_batch(camera, engine, detections, count, observed_at_ms, batch);
    if n <= 0 as ::core::ffi::c_int {
        free(batch as *mut ::core::ffi::c_void);
        free(jpeg as *mut ::core::ffi::c_void);
        return detection_evidence_status_t(
            (if n == 0 as ::core::ffi::c_int {
                detection_evidence_status_t::DETECTION_EVIDENCE_NOT_ELIGIBLE.0 as ::core::ffi::c_int
            } else {
                detection_evidence_status_t::DETECTION_EVIDENCE_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
    }
    let mut r: observation_evidence_result_t = observation_evidence_submit_jpeg(
        batch,
        jpeg,
        jpeg_size,
        width,
        height,
        transform,
        asset_uuid_out,
    );
    if !submit_result.is_null() {
        *submit_result = r;
    }
    if r.0 != observation_evidence_result_t::OBS_EVIDENCE_ACCEPTED.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] API observation evidence not accepted (result %d, %d object(s))\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !(*camera).stream_name.is_null() {
                (*camera).stream_name
            } else {
                b"?\0".as_ptr() as *const ::core::ffi::c_char
            },
            r.0,
            n,
        );
    }
    free(batch as *mut ::core::ffi::c_void);
    return detection_evidence_status_t::DETECTION_EVIDENCE_SUBMITTED;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_is_motion_label(
    mut label: *const ::core::ffi::c_char,
) -> bool {
    return !label.is_null()
        && strcasecmp(label, b"motion\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_evidence_record_no_media(
    mut camera: *const detection_evidence_camera_t,
    mut engine: *const detection_evidence_engine_t,
    mut detections: *const detection_t,
    mut count: ::core::ffi::c_int,
    mut observed_at_ms: int64_t,
    mut keep_boxes: bool,
    mut submit_result: *mut observation_evidence_result_t,
    mut asset_uuid_out: *mut ::core::ffi::c_char,
) -> detection_evidence_status_t {
    if !asset_uuid_out.is_null() {
        *asset_uuid_out.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if camera.is_null()
        || engine.is_null()
        || count < 0 as ::core::ffi::c_int
        || count > 0 as ::core::ffi::c_int && detections.is_null()
    {
        return detection_evidence_status_t::DETECTION_EVIDENCE_ERROR;
    }
    if !(*camera).evidence_enabled {
        return detection_evidence_status_t::DETECTION_EVIDENCE_NOT_ENABLED;
    }
    if (*engine).engine_type.is_null()
        || strcmp(
            (*engine).engine_type,
            b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*engine).engine_type,
                b"external\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
    {
        return detection_evidence_status_t::DETECTION_EVIDENCE_NOT_OBJECT_ENGINE;
    }
    let mut objects: [detection_t; 20] = [detection_t {
        label: [0; 32],
        confidence: 0.,
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
        track_id: 0,
        zone_id: [0; 32],
    }; 20];
    let mut kept: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count && kept < MAX_DETECTIONS {
        if !detection_evidence_is_motion_label(
            &raw const (*detections.offset(i as isize)).label as *const ::core::ffi::c_char,
        ) {
            let c2rust_fresh0 = kept;
            kept += 1;
            objects[c2rust_fresh0 as usize] = *detections.offset(i as isize);
        }
        i += 1;
    }
    if kept == 0 as ::core::ffi::c_int {
        return detection_evidence_status_t::DETECTION_EVIDENCE_NOT_ELIGIBLE;
    }
    let mut batch: *mut observation_batch_input_t = malloc(::core::mem::size_of::<
        observation_batch_input_t,
    >()) as *mut observation_batch_input_t;
    if batch.is_null() {
        return detection_evidence_status_t::DETECTION_EVIDENCE_ERROR;
    }
    let mut n: ::core::ffi::c_int = detection_evidence_build_batch(
        camera,
        engine,
        &raw mut objects as *mut detection_t,
        kept,
        observed_at_ms,
        batch,
    );
    if n <= 0 as ::core::ffi::c_int {
        free(batch as *mut ::core::ffi::c_void);
        return detection_evidence_status_t(
            (if n == 0 as ::core::ffi::c_int {
                detection_evidence_status_t::DETECTION_EVIDENCE_NOT_ELIGIBLE.0 as ::core::ffi::c_int
            } else {
                detection_evidence_status_t::DETECTION_EVIDENCE_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
    }
    if !keep_boxes {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < n {
            let mut o: *mut observation_object_input_t =
                (&raw mut (*batch).objects as *mut observation_object_input_t).offset(i_0 as isize);
            (*o).has_box = r#false != 0;
            (*o).box_height = 0.0f64;
            (*o).box_width = (*o).box_height;
            (*o).box_y = (*o).box_width;
            (*o).box_x = (*o).box_y;
            i_0 += 1;
        }
    }
    let mut r: observation_evidence_result_t = observation_evidence_submit_unavailable(
        batch,
        b"no_matching_media\0".as_ptr() as *const ::core::ffi::c_char,
        asset_uuid_out,
    );
    if !submit_result.is_null() {
        *submit_result = r;
    }
    if r.0 != observation_evidence_result_t::OBS_EVIDENCE_ACCEPTED.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] %s observation not recorded (result %d, %d object(s))\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !(*camera).stream_name.is_null() {
                (*camera).stream_name
            } else {
                b"?\0".as_ptr() as *const ::core::ffi::c_char
            },
            (*engine).engine_type,
            r.0,
            n,
        );
    }
    free(batch as *mut ::core::ffi::c_void);
    return detection_evidence_status_t::DETECTION_EVIDENCE_SUBMITTED;
}
