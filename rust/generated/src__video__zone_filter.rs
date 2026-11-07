unsafe extern "C" {
    unsafe fn get_detection_zones(
        stream_name: *const ::core::ffi::c_char,
        zones: *mut detection_zone_t,
        max_zones: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
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
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_result_t {
    pub count: ::core::ffi::c_int,
    pub detections: [detection_t; 20],
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
pub struct detection_zone_t {
    pub id: [::core::ffi::c_char; 64],
    pub stream_name: [::core::ffi::c_char; 256],
    pub name: [::core::ffi::c_char; 64],
    pub enabled: bool,
    pub color: [::core::ffi::c_char; 8],
    pub polygon: [zone_point_t; 32],
    pub polygon_count: ::core::ffi::c_int,
    pub filter_classes: [::core::ffi::c_char; 256],
    pub min_confidence: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_point_t {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
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
pub type uint8_t = u8;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
}
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
unsafe extern "C" fn point_in_polygon(
    mut x: ::core::ffi::c_float,
    mut y: ::core::ffi::c_float,
    mut polygon: *const zone_point_t,
    mut polygon_count: ::core::ffi::c_int,
) -> bool {
    if polygon.is_null() || polygon_count < 3 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut inside: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = polygon_count - 1 as ::core::ffi::c_int;
    while i < polygon_count {
        let mut xi: ::core::ffi::c_float = (*polygon.offset(i as isize)).x;
        let mut yi: ::core::ffi::c_float = (*polygon.offset(i as isize)).y;
        let mut xj: ::core::ffi::c_float = (*polygon.offset(j as isize)).x;
        let mut yj: ::core::ffi::c_float = (*polygon.offset(j as isize)).y;
        let mut intersect: bool = (yi > y) as ::core::ffi::c_int != (yj > y) as ::core::ffi::c_int
            && x < (xj - xi) * (y - yi) / (yj - yi) + xi;
        if intersect {
            inside = !inside;
        }
        let c2rust_fresh0 = i;
        i += 1;
        j = c2rust_fresh0;
    }
    return inside;
}
unsafe extern "C" fn detection_in_zone(
    mut detection: *const detection_t,
    mut zone: *const detection_zone_t,
) -> bool {
    if detection.is_null() || zone.is_null() || !(*zone).enabled {
        return r#false != 0;
    }
    let mut center_x: ::core::ffi::c_float = (*detection).x + (*detection).width / 2.0f32;
    let mut center_y: ::core::ffi::c_float = (*detection).y + (*detection).height / 2.0f32;
    return point_in_polygon(
        center_x,
        center_y,
        &raw const (*zone).polygon as *const zone_point_t,
        (*zone).polygon_count,
    );
}
unsafe extern "C" fn detection_class_matches(
    mut detection: *const detection_t,
    mut zone: *const detection_zone_t,
) -> bool {
    if (&raw const (*zone).filter_classes as *const ::core::ffi::c_char).is_null()
        || (*zone).filter_classes[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    let mut filter_copy: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut filter_copy as *mut ::core::ffi::c_char,
        &raw const (*zone).filter_classes as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut filter_copy as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() {
        while *token as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            token = token.offset(1);
        }
        let mut end: *mut ::core::ffi::c_char = token
            .offset(strlen(token) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        while end > token && *end as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            *end = '\0' as ::core::ffi::c_char;
            end = end.offset(-1);
        }
        if strcmp(
            token,
            &raw const (*detection).label as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return r#false != 0;
}
unsafe extern "C" fn detection_meets_confidence(
    mut detection: *const detection_t,
    mut zone: *const detection_zone_t,
) -> bool {
    if (*zone).min_confidence <= 0.0f32 {
        return r#true != 0;
    }
    return (*detection).confidence >= (*zone).min_confidence;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filter_detections_by_zones(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || result.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for filter_detections_by_zones\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*result).count == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut zones: [detection_zone_t; 16] = [detection_zone_t {
        id: [0; 64],
        stream_name: [0; 256],
        name: [0; 64],
        enabled: false,
        color: [0; 8],
        polygon: [zone_point_t { x: 0., y: 0. }; 32],
        polygon_count: 0,
        filter_classes: [0; 256],
        min_confidence: 0.,
    }; 16];
    let mut zone_count: ::core::ffi::c_int = get_detection_zones(
        stream_name,
        &raw mut zones as *mut detection_zone_t,
        MAX_ZONES_PER_STREAM,
    );
    if zone_count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get detection zones for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    if zone_count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No zones configured for stream %s, allowing all detections\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut enabled_zone_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < zone_count {
        if zones[i as usize].enabled {
            enabled_zone_count += 1;
        }
        i += 1;
    }
    if enabled_zone_count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No enabled zones for stream %s, allowing all detections\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Filtering %d detections using %d enabled zones for stream %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*result).count,
        enabled_zone_count,
        stream_name,
    );
    let mut filtered: detection_result_t = detection_result_t {
        count: 0,
        detections: [detection_t {
            label: [0; 32],
            confidence: 0.,
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
            track_id: 0,
            zone_id: [0; 32],
        }; 20],
    };
    memset(
        &raw mut filtered as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*result).count {
        let mut det: *mut detection_t =
            (&raw mut (*result).detections as *mut detection_t).offset(i_0 as isize);
        let mut detection_accepted: bool = r#false != 0;
        let mut matched_zone_id: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < zone_count {
            let mut zone: *mut detection_zone_t =
                (&raw mut zones as *mut detection_zone_t).offset(j as isize);
            if (*zone).enabled {
                if detection_in_zone(det, zone) {
                    if !detection_class_matches(det, zone) {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_DEBUG,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Detection %s rejected by zone %s (class filter)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*det).label as *mut ::core::ffi::c_char,
                            &raw mut (*zone).name as *mut ::core::ffi::c_char,
                        );
                    } else if !detection_meets_confidence(det, zone) {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_DEBUG,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Detection %s rejected by zone %s (confidence %.2f < %.2f)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*det).label as *mut ::core::ffi::c_char,
                            &raw mut (*zone).name as *mut ::core::ffi::c_char,
                            (*det).confidence as ::core::ffi::c_double,
                            (*zone).min_confidence as ::core::ffi::c_double,
                        );
                    } else {
                        detection_accepted = r#true != 0;
                        matched_zone_id = &raw mut (*zone).id as *mut ::core::ffi::c_char;
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Detection %s (%.2f%%) accepted by zone %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*det).label as *mut ::core::ffi::c_char,
                            ((*det).confidence * 100.0f32) as ::core::ffi::c_double,
                            &raw mut (*zone).name as *mut ::core::ffi::c_char,
                        );
                        break;
                    }
                }
            }
            j += 1;
        }
        if detection_accepted {
            memcpy(
                (&raw mut filtered.detections as *mut detection_t).offset(filtered.count as isize)
                    as *mut ::core::ffi::c_void,
                det as *const ::core::ffi::c_void,
                ::core::mem::size_of::<detection_t>(),
            );
            if !matched_zone_id.is_null() {
                safe_strcpy(
                    &raw mut (*(&raw mut filtered.detections as *mut detection_t)
                        .offset(filtered.count as isize))
                    .zone_id as *mut ::core::ffi::c_char,
                    matched_zone_id,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                    0 as size_t,
                );
            }
            filtered.count += 1;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Detection %s (%.2f%%) at [%.2f, %.2f] rejected (not in any enabled zone)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*det).label as *mut ::core::ffi::c_char,
                ((*det).confidence * 100.0f32) as ::core::ffi::c_double,
                ((*det).x + (*det).width / 2.0f32) as ::core::ffi::c_double,
                ((*det).y + (*det).height / 2.0f32) as ::core::ffi::c_double,
            );
        }
        i_0 += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Zone filtering: %d detections -> %d detections (filtered out %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*result).count,
        filtered.count,
        (*result).count - filtered.count,
    );
    memcpy(
        result as *mut ::core::ffi::c_void,
        &raw mut filtered as *const ::core::ffi::c_void,
        ::core::mem::size_of::<detection_result_t>(),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn label_in_list(
    mut label: *const ::core::ffi::c_char,
    mut class_list: *const ::core::ffi::c_char,
) -> bool {
    if label.is_null()
        || class_list.is_null()
        || *class_list.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut list_copy: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut list_copy as *mut ::core::ffi::c_char,
        class_list,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut list_copy as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() {
        while *token as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            token = token.offset(1);
        }
        let mut end: *mut ::core::ffi::c_char = token
            .offset(strlen(token) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        while end > token && *end as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            *end = '\0' as ::core::ffi::c_char;
            end = end.offset(-1);
        }
        if strcmp(token, label) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filter_detections_by_stream_objects(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || result.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for filter_detections_by_stream_objects\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*result).count == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut config: stream_config_t = stream_config_t {
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
    memset(
        &raw mut config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_config_t>(),
    );
    if get_stream_config_by_name(stream_name, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not get stream config for '%s', skipping object filter\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    if config.detection_object_filter[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strcmp(
            &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
            b"none\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No object filter configured for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    if config.detection_object_filter_list[0usize] as ::core::ffi::c_int
        == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Object filter mode is '%s' but list is empty for stream '%s', skipping\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut is_include: bool = strcmp(
        &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
        b"include\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut is_exclude: bool = strcmp(
        &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
        b"exclude\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    if !is_include && !is_exclude {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unknown object filter mode '%s' for stream '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut filtered: detection_result_t = detection_result_t {
        count: 0,
        detections: [detection_t {
            label: [0; 32],
            confidence: 0.,
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
            track_id: 0,
            zone_id: [0; 32],
        }; 20],
    };
    memset(
        &raw mut filtered as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*result).count && filtered.count < MAX_DETECTIONS {
        let mut det: *const detection_t =
            (&raw mut (*result).detections as *mut detection_t).offset(i as isize);
        let mut in_list: bool = label_in_list(
            &raw const (*det).label as *const ::core::ffi::c_char,
            &raw mut config.detection_object_filter_list as *mut ::core::ffi::c_char,
        );
        let mut keep: bool = r#false != 0;
        if is_include {
            keep = in_list;
        } else {
            keep = !in_list;
        }
        if keep {
            memcpy(
                (&raw mut filtered.detections as *mut detection_t).offset(filtered.count as isize)
                    as *mut ::core::ffi::c_void,
                det as *const ::core::ffi::c_void,
                ::core::mem::size_of::<detection_t>(),
            );
            filtered.count += 1;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Object filter (%s): rejected detection '%s' for stream '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
                &raw const (*det).label as *const ::core::ffi::c_char,
                stream_name,
            );
        }
        i += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Object filtering (%s): %d detections -> %d detections for stream '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
        (*result).count,
        filtered.count,
        stream_name,
    );
    memcpy(
        result as *mut ::core::ffi::c_void,
        &raw mut filtered as *const ::core::ffi::c_void,
        ::core::mem::size_of::<detection_result_t>(),
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_motion_zone_mask(
    mut stream_name: *const ::core::ffi::c_char,
    mut grid_size: ::core::ffi::c_int,
    mut zone_mask: *mut bool,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || grid_size <= 0 as ::core::ffi::c_int || zone_mask.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut total_cells: ::core::ffi::c_int = grid_size * grid_size;
    let mut zones: [detection_zone_t; 16] = [detection_zone_t {
        id: [0; 64],
        stream_name: [0; 256],
        name: [0; 64],
        enabled: false,
        color: [0; 8],
        polygon: [zone_point_t { x: 0., y: 0. }; 32],
        polygon_count: 0,
        filter_classes: [0; 256],
        min_confidence: 0.,
    }; 16];
    let mut zone_count: ::core::ffi::c_int = get_detection_zones(
        stream_name,
        &raw mut zones as *mut detection_zone_t,
        MAX_ZONES_PER_STREAM,
    );
    if zone_count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get detection zones for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < total_cells {
            *zone_mask.offset(i as isize) = r#true != 0;
            i += 1;
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut enabled_zone_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < zone_count {
        if zones[i_0 as usize].enabled {
            enabled_zone_count += 1;
        }
        i_0 += 1;
    }
    if zone_count == 0 as ::core::ffi::c_int || enabled_zone_count == 0 as ::core::ffi::c_int {
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < total_cells {
            *zone_mask.offset(i_1 as isize) = r#true != 0;
            i_1 += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut gy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while gy < grid_size {
        let mut gx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while gx < grid_size {
            let mut center_x: ::core::ffi::c_float =
                (gx as ::core::ffi::c_float + 0.5f32) / grid_size as ::core::ffi::c_float;
            let mut center_y: ::core::ffi::c_float =
                (gy as ::core::ffi::c_float + 0.5f32) / grid_size as ::core::ffi::c_float;
            let mut in_any_zone: bool = r#false != 0;
            let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while z < zone_count {
                if zones[z as usize].enabled {
                    if point_in_polygon(
                        center_x,
                        center_y,
                        &raw mut (*(&raw mut zones as *mut detection_zone_t).offset(z as isize))
                            .polygon as *mut zone_point_t,
                        zones[z as usize].polygon_count,
                    ) {
                        in_any_zone = r#true != 0;
                        break;
                    }
                }
                z += 1;
            }
            *zone_mask.offset((gy * grid_size + gx) as isize) = in_any_zone;
            gx += 1;
        }
        gy += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Built zone mask for stream %s: %d/%d cells active (%d enabled zones)\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        enabled_zone_count,
        total_cells,
        enabled_zone_count,
    );
    return enabled_zone_count;
}
pub const MAX_ZONES_PER_STREAM: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
