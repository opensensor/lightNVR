unsafe extern "C" {
    unsafe fn time(__timer: *mut time_t) -> time_t;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub struct timestamp_tracker_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub pts_discontinuity_count: int64_t,
    pub expected_next_pts: int64_t,
    pub is_udp_stream: bool,
    pub initialized: bool,
    pub last_keyframe_time: time_t,
    pub last_detection_time: time_t,
}
pub const AV_NOPTS_VALUE: int64_t = 0x8000000000000000 as ::core::ffi::c_ulong as int64_t;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const MAX_TIMESTAMP_TRACKERS: ::core::ffi::c_int = MAX_STREAMS;
static mut timestamp_trackers: [timestamp_tracker_t; 1024] = [timestamp_tracker_t {
    stream_name: [0; 256],
    last_pts: 0,
    last_dts: 0,
    pts_discontinuity_count: 0,
    expected_next_pts: 0,
    is_udp_stream: false,
    initialized: false,
    last_keyframe_time: 0,
    last_detection_time: 0,
}; 1024];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_timestamp_tracker(
    mut stream_name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"get_timestamp_tracker: NULL stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    let mut local_stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut local_stream_name as *mut ::core::ffi::c_char,
        stream_name,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i as usize].initialized as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                &raw mut local_stream_name as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            return (&raw mut timestamp_trackers as *mut timestamp_tracker_t).offset(i as isize)
                as *mut ::core::ffi::c_void;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_TIMESTAMP_TRACKERS {
        if !timestamp_trackers[i_0 as usize].initialized {
            safe_strcpy(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i_0 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                &raw mut local_stream_name as *mut ::core::ffi::c_char,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            timestamp_trackers[i_0 as usize].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].pts_discontinuity_count = 0 as int64_t;
            timestamp_trackers[i_0 as usize].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].is_udp_stream = r#false != 0;
            timestamp_trackers[i_0 as usize].last_keyframe_time = 0 as time_t;
            timestamp_trackers[i_0 as usize].last_detection_time = 0 as time_t;
            timestamp_trackers[i_0 as usize].initialized = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Created new timestamp tracker for stream %s at index %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut local_stream_name as *mut ::core::ffi::c_char,
                i_0,
            );
            return (&raw mut timestamp_trackers as *mut timestamp_tracker_t).offset(i_0 as isize)
                as *mut ::core::ffi::c_void;
        }
        i_0 += 1;
    }
    let mut current_time: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i_1 as usize].initialized as ::core::ffi::c_int != 0
            && timestamp_trackers[i_1 as usize].last_keyframe_time > 0 as time_t
            && current_time - timestamp_trackers[i_1 as usize].last_keyframe_time > 300 as time_t
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Reusing stale timestamp tracker for stream %s (previously used by %s, last keyframe %ld seconds ago)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut local_stream_name as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i_1 as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                current_time - timestamp_trackers[i_1 as usize].last_keyframe_time,
            );
            safe_strcpy(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i_1 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                &raw mut local_stream_name as *mut ::core::ffi::c_char,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            timestamp_trackers[i_1 as usize].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i_1 as usize].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i_1 as usize].pts_discontinuity_count = 0 as int64_t;
            timestamp_trackers[i_1 as usize].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i_1 as usize].is_udp_stream = r#false != 0;
            timestamp_trackers[i_1 as usize].last_keyframe_time = 0 as time_t;
            timestamp_trackers[i_1 as usize].last_detection_time = 0 as time_t;
            return (&raw mut timestamp_trackers as *mut timestamp_tracker_t).offset(i_1 as isize)
                as *mut ::core::ffi::c_void;
        }
        i_1 += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"No available slots for timestamp tracker for stream %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut local_stream_name as *mut ::core::ffi::c_char,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_timestamp_trackers() {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        timestamp_trackers[i as usize].initialized = r#false != 0;
        timestamp_trackers[i as usize].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i as usize].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[i as usize].pts_discontinuity_count = 0 as int64_t;
        timestamp_trackers[i as usize].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i as usize].is_udp_stream = r#false != 0;
        timestamp_trackers[i as usize].last_keyframe_time = 0 as time_t;
        timestamp_trackers[i as usize].last_detection_time = 0 as time_t;
        timestamp_trackers[i as usize].stream_name[0usize] = '\0' as ::core::ffi::c_char;
        i += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Timestamp trackers initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_timestamp_tracker_udp_flag(
    mut stream_name: *const ::core::ffi::c_char,
    mut is_udp: bool,
) {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"set_timestamp_tracker_udp_flag: NULL stream name\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut local_stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut local_stream_name as *mut ::core::ffi::c_char,
        stream_name,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i as usize].initialized as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                &raw mut local_stream_name as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            timestamp_trackers[i as usize].is_udp_stream = is_udp;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Set UDP flag to %s for stream %s timestamp tracker\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if is_udp as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut local_stream_name as *mut ::core::ffi::c_char,
            );
            found = 1 as ::core::ffi::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if found == 0 {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < MAX_TIMESTAMP_TRACKERS {
            if !timestamp_trackers[i_0 as usize].initialized {
                safe_strcpy(
                    &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                        .offset(i_0 as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                    &raw mut local_stream_name as *mut ::core::ffi::c_char,
                    MAX_STREAM_NAME as size_t,
                    0 as size_t,
                );
                timestamp_trackers[i_0 as usize].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].pts_discontinuity_count = 0 as int64_t;
                timestamp_trackers[i_0 as usize].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].is_udp_stream = is_udp;
                timestamp_trackers[i_0 as usize].last_keyframe_time = 0 as time_t;
                timestamp_trackers[i_0 as usize].last_detection_time = 0 as time_t;
                timestamp_trackers[i_0 as usize].initialized = r#true != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Created new timestamp tracker for stream %s at index %d with UDP flag %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut local_stream_name as *mut ::core::ffi::c_char,
                    i_0,
                    if is_udp as ::core::ffi::c_int != 0 {
                        b"true\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"false\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                found = 1 as ::core::ffi::c_int;
                break;
            } else {
                i_0 += 1;
            }
        }
        if found == 0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"No available slots for timestamp tracker for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut local_stream_name as *mut ::core::ffi::c_char,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_timestamp_tracker(mut stream_name: *const ::core::ffi::c_char) {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"reset_timestamp_tracker: NULL stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut found: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i as usize].initialized as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            timestamp_trackers[i as usize].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i as usize].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i as usize].pts_discontinuity_count = 0 as int64_t;
            timestamp_trackers[i as usize].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i as usize].last_keyframe_time = 0 as time_t;
            timestamp_trackers[i as usize].last_detection_time = 0 as time_t;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Reset timestamp tracker for stream %s (UDP flag: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                if timestamp_trackers[i as usize].is_udp_stream as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            found = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    if !found {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No timestamp tracker found for stream %s during reset\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_timestamp_tracker(mut stream_name: *const ::core::ffi::c_char) {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"remove_timestamp_tracker: NULL stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut found: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i as usize].initialized as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            timestamp_trackers[i as usize].initialized = r#false != 0;
            timestamp_trackers[i as usize].stream_name[0usize] = '\0' as ::core::ffi::c_char;
            timestamp_trackers[i as usize].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i as usize].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i as usize].pts_discontinuity_count = 0 as int64_t;
            timestamp_trackers[i as usize].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i as usize].is_udp_stream = r#false != 0;
            timestamp_trackers[i as usize].last_keyframe_time = 0 as time_t;
            timestamp_trackers[i as usize].last_detection_time = 0 as time_t;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Removed timestamp tracker for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
            found = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    if !found {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No timestamp tracker found for stream %s during removal\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_timestamp_trackers() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaning up timestamp trackers...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        timestamp_trackers[i as usize].initialized = r#false != 0;
        timestamp_trackers[i as usize].stream_name[0usize] = '\0' as ::core::ffi::c_char;
        timestamp_trackers[i as usize].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i as usize].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[i as usize].pts_discontinuity_count = 0 as int64_t;
        timestamp_trackers[i as usize].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i as usize].is_udp_stream = r#false != 0;
        timestamp_trackers[i as usize].last_keyframe_time = 0 as time_t;
        timestamp_trackers[i as usize].last_detection_time = 0 as time_t;
        i += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"All timestamp trackers cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_keyframe_time(mut stream_name: *const ::core::ffi::c_char) {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"update_keyframe_time: NULL stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut found: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i as usize].initialized as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            let mut prev_keyframe_time: time_t = timestamp_trackers[i as usize].last_keyframe_time;
            timestamp_trackers[i as usize].last_keyframe_time =
                time(::core::ptr::null_mut::<time_t>());
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Updated keyframe time for stream %s: previous=%ld, new=%ld, delta=%ld seconds\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                prev_keyframe_time,
                timestamp_trackers[i as usize].last_keyframe_time,
                if prev_keyframe_time > 0 as time_t {
                    timestamp_trackers[i as usize].last_keyframe_time - prev_keyframe_time
                } else {
                    0 as ::core::ffi::c_long
                },
            );
            found = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    if !found {
        let mut created: bool = r#false != 0;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < MAX_TIMESTAMP_TRACKERS {
            if !timestamp_trackers[i_0 as usize].initialized {
                safe_strcpy(
                    &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                        .offset(i_0 as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                    stream_name,
                    MAX_STREAM_NAME as size_t,
                    0 as size_t,
                );
                timestamp_trackers[i_0 as usize].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].pts_discontinuity_count = 0 as int64_t;
                timestamp_trackers[i_0 as usize].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].is_udp_stream = r#false != 0;
                timestamp_trackers[i_0 as usize].last_keyframe_time =
                    time(::core::ptr::null_mut::<time_t>());
                timestamp_trackers[i_0 as usize].last_detection_time = 0 as time_t;
                timestamp_trackers[i_0 as usize].initialized = r#true != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Created new timestamp tracker for stream %s with keyframe time %ld\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                    timestamp_trackers[i_0 as usize].last_keyframe_time,
                );
                created = r#true != 0;
                break;
            } else {
                i_0 += 1;
            }
        }
        if !created {
            let mut oldest_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut oldest_time: time_t = time(::core::ptr::null_mut::<time_t>());
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < MAX_TIMESTAMP_TRACKERS {
                if timestamp_trackers[i_1 as usize].initialized as ::core::ffi::c_int != 0
                    && timestamp_trackers[i_1 as usize].last_keyframe_time < oldest_time
                {
                    oldest_time = timestamp_trackers[i_1 as usize].last_keyframe_time;
                    oldest_idx = i_1;
                }
                i_1 += 1;
            }
            if oldest_idx >= 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Reusing oldest timestamp tracker (from stream %s, last keyframe %ld seconds ago) for stream %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                        .offset(oldest_idx as isize))
                        .stream_name as *mut ::core::ffi::c_char,
                    time(::core::ptr::null_mut::<time_t>())
                        - timestamp_trackers[oldest_idx as usize].last_keyframe_time,
                    stream_name,
                );
                safe_strcpy(
                    &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                        .offset(oldest_idx as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                    stream_name,
                    MAX_STREAM_NAME as size_t,
                    0 as size_t,
                );
                timestamp_trackers[oldest_idx as usize].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx as usize].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx as usize].pts_discontinuity_count = 0 as int64_t;
                timestamp_trackers[oldest_idx as usize].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx as usize].is_udp_stream = r#false != 0;
                timestamp_trackers[oldest_idx as usize].last_keyframe_time =
                    time(::core::ptr::null_mut::<time_t>());
                timestamp_trackers[oldest_idx as usize].last_detection_time = 0 as time_t;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to find or create timestamp tracker for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn last_keyframe_received(
    mut stream_name: *const ::core::ffi::c_char,
    mut keyframe_time: *mut time_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"last_keyframe_received: NULL stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i as usize].initialized as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            let mut check_time: time_t = 0 as time_t;
            if !keyframe_time.is_null() && *keyframe_time > 0 as time_t {
                check_time = *keyframe_time;
            }
            let mut last_kf_time: time_t = timestamp_trackers[i as usize].last_keyframe_time;
            if !keyframe_time.is_null() {
                *keyframe_time = last_kf_time;
            }
            let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if last_kf_time > 0 as time_t {
                if check_time == 0 as time_t || last_kf_time > check_time {
                    result = 1 as ::core::ffi::c_int;
                }
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Keyframe check for stream %s: last_keyframe_time=%ld, check_time=%ld, result=%d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                last_kf_time,
                check_time,
                result,
            );
            return result;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_TIMESTAMP_TRACKERS {
        if !timestamp_trackers[i_0 as usize].initialized {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Creating new timestamp tracker for stream %s during keyframe check\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            safe_strcpy(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i_0 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            timestamp_trackers[i_0 as usize].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].pts_discontinuity_count = 0 as int64_t;
            timestamp_trackers[i_0 as usize].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].is_udp_stream = r#false != 0;
            timestamp_trackers[i_0 as usize].last_keyframe_time = 0 as time_t;
            timestamp_trackers[i_0 as usize].last_detection_time = 0 as time_t;
            timestamp_trackers[i_0 as usize].initialized = r#true != 0;
            if !keyframe_time.is_null() {
                *keyframe_time = 0 as time_t;
            }
            return 0 as ::core::ffi::c_int;
        }
        i_0 += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"No timestamp tracker found for stream %s during last_keyframe_received and couldn't create one\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    if !keyframe_time.is_null() {
        *keyframe_time = 0 as time_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_last_detection_time(
    mut stream_name: *const ::core::ffi::c_char,
) -> time_t {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"get_last_detection_time: NULL stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as time_t;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i as usize].initialized as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            let mut last_detection_time: time_t =
                timestamp_trackers[i as usize].last_detection_time;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Last detection time for stream %s: %ld\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                last_detection_time,
            );
            return last_detection_time;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_TIMESTAMP_TRACKERS {
        if !timestamp_trackers[i_0 as usize].initialized {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Creating new timestamp tracker for stream %s during get_last_detection_time\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
            safe_strcpy(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i_0 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            timestamp_trackers[i_0 as usize].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].pts_discontinuity_count = 0 as int64_t;
            timestamp_trackers[i_0 as usize].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i_0 as usize].is_udp_stream = r#false != 0;
            timestamp_trackers[i_0 as usize].last_keyframe_time = 0 as time_t;
            timestamp_trackers[i_0 as usize].last_detection_time = 0 as time_t;
            timestamp_trackers[i_0 as usize].initialized = r#true != 0;
            return 0 as time_t;
        }
        i_0 += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"No timestamp tracker found for stream %s during get_last_detection_time and couldn't create one\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    return 0 as time_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_last_detection_time(
    mut stream_name: *const ::core::ffi::c_char,
    mut detection_time: time_t,
) {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"update_last_detection_time: NULL stream name\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut found: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TIMESTAMP_TRACKERS {
        if timestamp_trackers[i as usize].initialized as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            let mut prev_detection_time: time_t =
                timestamp_trackers[i as usize].last_detection_time;
            timestamp_trackers[i as usize].last_detection_time = detection_time;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Updated detection time for stream %s: previous=%ld, new=%ld, delta=%ld seconds\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                prev_detection_time,
                timestamp_trackers[i as usize].last_detection_time,
                if prev_detection_time > 0 as time_t {
                    timestamp_trackers[i as usize].last_detection_time - prev_detection_time
                } else {
                    0 as ::core::ffi::c_long
                },
            );
            found = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    if !found {
        let mut created: bool = r#false != 0;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < MAX_TIMESTAMP_TRACKERS {
            if !timestamp_trackers[i_0 as usize].initialized {
                safe_strcpy(
                    &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                        .offset(i_0 as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                    stream_name,
                    MAX_STREAM_NAME as size_t,
                    0 as size_t,
                );
                timestamp_trackers[i_0 as usize].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].pts_discontinuity_count = 0 as int64_t;
                timestamp_trackers[i_0 as usize].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i_0 as usize].is_udp_stream = r#false != 0;
                timestamp_trackers[i_0 as usize].last_keyframe_time = 0 as time_t;
                timestamp_trackers[i_0 as usize].last_detection_time = detection_time;
                timestamp_trackers[i_0 as usize].initialized = r#true != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Created new timestamp tracker for stream %s with detection time %ld\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_name,
                    timestamp_trackers[i_0 as usize].last_detection_time,
                );
                created = r#true != 0;
                break;
            } else {
                i_0 += 1;
            }
        }
        if !created {
            let mut oldest_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut oldest_time: time_t = time(::core::ptr::null_mut::<time_t>());
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < MAX_TIMESTAMP_TRACKERS {
                if timestamp_trackers[i_1 as usize].initialized as ::core::ffi::c_int != 0
                    && timestamp_trackers[i_1 as usize].last_keyframe_time < oldest_time
                {
                    oldest_time = timestamp_trackers[i_1 as usize].last_keyframe_time;
                    oldest_idx = i_1;
                }
                i_1 += 1;
            }
            if oldest_idx >= 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Reusing oldest timestamp tracker (from stream %s, last keyframe %ld seconds ago) for stream %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                        .offset(oldest_idx as isize))
                        .stream_name as *mut ::core::ffi::c_char,
                    time(::core::ptr::null_mut::<time_t>())
                        - timestamp_trackers[oldest_idx as usize].last_keyframe_time,
                    stream_name,
                );
                safe_strcpy(
                    &raw mut (*(&raw mut timestamp_trackers as *mut timestamp_tracker_t)
                        .offset(oldest_idx as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                    stream_name,
                    MAX_STREAM_NAME as size_t,
                    0 as size_t,
                );
                timestamp_trackers[oldest_idx as usize].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx as usize].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx as usize].pts_discontinuity_count = 0 as int64_t;
                timestamp_trackers[oldest_idx as usize].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx as usize].is_udp_stream = r#false != 0;
                timestamp_trackers[oldest_idx as usize].last_keyframe_time = 0 as time_t;
                timestamp_trackers[oldest_idx as usize].last_detection_time = detection_time;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to find or create timestamp tracker for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
            }
        }
    }
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
