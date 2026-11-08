unsafe extern "C" {
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type int64_t = i64;
pub type uint8_t = u8;
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
pub const OBSERVATION_DEFAULT_MAX_AGE_DAYS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OBSERVATION_DEFAULT_MAX_BYTES: ::core::ffi::c_longlong =
    2147483648 as ::core::ffi::c_longlong;
pub const OBSERVATION_LABELS_MAX_COUNT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OBSERVATION_LABEL_MAX_BYTES: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const OBSERVATION_MAX_AGE_DAYS_MIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OBSERVATION_MAX_AGE_DAYS_MAX: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const OBSERVATION_MAX_BYTES_MIN: ::core::ffi::c_longlong = 64 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong;
pub const OBSERVATION_MAX_BYTES_MAX: ::core::ffi::c_longlong = 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn fail(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut message: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size != 0 {
        snprintf(
            error,
            error_size,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            message,
        );
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_integer_in_range(
    mut value: *const cJSON,
    mut minimum: ::core::ffi::c_double,
    mut maximum: ::core::ffi::c_double,
) -> bool {
    return cJSON_IsNumber(value) != 0
        && (*value).valuedouble.is_finite() as ::core::ffi::c_int != 0
        && (*value).valuedouble == floor((*value).valuedouble)
        && (*value).valuedouble >= minimum
        && (*value).valuedouble <= maximum;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_observation_labels_normalize(
    mut input: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if input.is_null() || out.is_null() || out_size == 0 as size_t {
        return fail(
            error,
            error_size,
            b"observation_labels is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut result: [::core::ffi::c_char; 512] = ::core::mem::transmute::<
        [u8; 512],
        [::core::ffi::c_char; 512],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut used: size_t = 0 as size_t;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut scan: *const ::core::ffi::c_char = input;
    while *scan as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *scan as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        scan = scan.offset(1);
    }
    if *scan as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        *out.offset(0isize) = '\0' as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    }
    let mut cursor: *const ::core::ffi::c_char = input;
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
        let mut length: size_t = last.offset_from(first) as size_t;
        if length == 0 as size_t {
            return fail(
                error,
                error_size,
                b"observation_labels must not contain empty labels\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if length > OBSERVATION_LABEL_MAX_BYTES as size_t {
            return fail(
                error,
                error_size,
                b"observation_labels entries must be 1..31 bytes\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        let mut c: *const ::core::ffi::c_char = first;
        while c < last {
            let mut ch: ::core::ffi::c_uchar = *c as ::core::ffi::c_uchar;
            if (ch as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
                || ch as ::core::ffi::c_int > 0x7e as ::core::ffi::c_int
            {
                return fail(
                    error,
                    error_size,
                    b"observation_labels entries must be printable ASCII\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            c = c.offset(1);
        }
        count += 1;
        if count > OBSERVATION_LABELS_MAX_COUNT {
            return fail(
                error,
                error_size,
                b"observation_labels allows at most 16 labels\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if used.wrapping_add(length).wrapping_add(
            (if count > 1 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as size_t,
        ) >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            return fail(
                error,
                error_size,
                b"observation_labels is too long\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if count > 1 as ::core::ffi::c_int {
            let c2rust_fresh0 = used;
            used = used.wrapping_add(1);
            result[c2rust_fresh0] = ',' as ::core::ffi::c_char;
        }
        memcpy(
            (&raw mut result as *mut ::core::ffi::c_char).offset(used as isize)
                as *mut ::core::ffi::c_void,
            first as *const ::core::ffi::c_void,
            length,
        );
        used = used.wrapping_add(length);
        result[used] = '\0' as ::core::ffi::c_char;
        if end.is_null() {
            break;
        }
        cursor = end.offset(1 as ::core::ffi::c_int as isize);
    }
    if used >= out_size {
        return fail(
            error,
            error_size,
            b"observation_labels is too long\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    memcpy(
        out as *mut ::core::ffi::c_void,
        &raw mut result as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        used.wrapping_add(1 as size_t),
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_observation_settings_to_json(
    mut object: *mut cJSON,
    mut config: *const stream_config_t,
) {
    if object.is_null() || config.is_null() {
        return;
    }
    cJSON_AddBoolToObject(
        object,
        b"observation_evidence_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).observation_evidence_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        object,
        b"observation_labels\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).observation_labels as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"observation_max_age_days\0".as_ptr() as *const ::core::ffi::c_char,
        (if (*config).observation_max_age_days > 0 as ::core::ffi::c_int {
            (*config).observation_max_age_days
        } else {
            OBSERVATION_DEFAULT_MAX_AGE_DAYS
        }) as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"observation_max_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (if (*config).observation_max_bytes > 0 as int64_t {
            (*config).observation_max_bytes as ::core::ffi::c_longlong
        } else {
            OBSERVATION_DEFAULT_MAX_BYTES
        }) as ::core::ffi::c_double,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_observation_settings_apply_json(
    mut body: *const cJSON,
    mut config: *mut stream_config_t,
    mut changed: *mut bool,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !changed.is_null() {
        *changed = r#false != 0;
    }
    if body.is_null() || config.is_null() {
        return fail(
            error,
            error_size,
            b"observation settings are invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut enabled: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"observation_evidence_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut labels: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"observation_labels\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut max_age: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"observation_max_age_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut max_bytes: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"observation_max_bytes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !enabled.is_null() && cJSON_IsBool(enabled) == 0 {
        return fail(
            error,
            error_size,
            b"observation_evidence_enabled must be a boolean\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut normalized: [::core::ffi::c_char; 512] = ::core::mem::transmute::<
        [u8; 512],
        [::core::ffi::c_char; 512],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    if !labels.is_null() {
        if cJSON_IsString(labels) == 0 || (*labels).valuestring.is_null() {
            return fail(
                error,
                error_size,
                b"observation_labels must be a string\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if stream_observation_labels_normalize(
            (*labels).valuestring,
            &raw mut normalized as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            error,
            error_size,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
    }
    if !max_age.is_null()
        && !is_integer_in_range(
            max_age,
            OBSERVATION_MAX_AGE_DAYS_MIN as ::core::ffi::c_double,
            OBSERVATION_MAX_AGE_DAYS_MAX as ::core::ffi::c_double,
        )
    {
        return fail(
            error,
            error_size,
            b"observation_max_age_days must be an integer between 1 and 90\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !max_bytes.is_null()
        && !is_integer_in_range(
            max_bytes,
            OBSERVATION_MAX_BYTES_MIN as ::core::ffi::c_double,
            OBSERVATION_MAX_BYTES_MAX as ::core::ffi::c_double,
        )
    {
        return fail(
            error,
            error_size,
            b"observation_max_bytes must be an integer between 67108864 (64 MiB) and 1099511627776 (1 TiB)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut any_change: bool = r#false != 0;
    if !enabled.is_null() {
        let mut value: bool = cJSON_IsTrue(enabled) != 0;
        any_change = any_change as ::core::ffi::c_int
            | (value as ::core::ffi::c_int
                != (*config).observation_evidence_enabled as ::core::ffi::c_int)
                as ::core::ffi::c_int
            != 0;
        (*config).observation_evidence_enabled = value;
    }
    if !labels.is_null() {
        any_change = any_change as ::core::ffi::c_int
            | (strcmp(
                &raw mut normalized as *mut ::core::ffi::c_char,
                &raw mut (*config).observation_labels as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            != 0;
        safe_strcpy(
            &raw mut (*config).observation_labels as *mut ::core::ffi::c_char,
            &raw mut normalized as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    if !max_age.is_null() {
        let mut value_0: ::core::ffi::c_int = (*max_age).valuedouble as ::core::ffi::c_int;
        any_change = any_change as ::core::ffi::c_int
            | (value_0 != (*config).observation_max_age_days) as ::core::ffi::c_int
            != 0;
        (*config).observation_max_age_days = value_0;
    }
    if !max_bytes.is_null() {
        let mut value_1: int64_t = (*max_bytes).valuedouble as int64_t;
        any_change = any_change as ::core::ffi::c_int
            | (value_1 != (*config).observation_max_bytes) as ::core::ffi::c_int
            != 0;
        (*config).observation_max_bytes = value_1;
    }
    if !changed.is_null() {
        *changed = any_change;
    }
    return 0 as ::core::ffi::c_int;
}
