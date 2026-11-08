use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_busy_timeout(_: *mut sqlite3, ms: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_get_autocommit(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn playback_transport_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn db_camera_tags_sync_legacy_by_name_locked(
        db: *mut sqlite3,
        stream_name: *const ::core::ffi::c_char,
        legacy_tags: *const ::core::ffi::c_char,
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
    unsafe fn string_has_non_whitespace(value: *const ::core::ffi::c_char) -> bool;
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
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}
pub type size_t = usize;
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_retention_config_t {
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: uint64_t,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const COL_NAME: Self = Self(0);
    pub const COL_URL: Self = Self(1);
    pub const COL_ENABLED: Self = Self(2);
    pub const COL_STREAMING_ENABLED: Self = Self(3);
    pub const COL_WIDTH: Self = Self(4);
    pub const COL_HEIGHT: Self = Self(5);
    pub const COL_FPS: Self = Self(6);
    pub const COL_CODEC: Self = Self(7);
    pub const COL_PRIORITY: Self = Self(8);
    pub const COL_RECORD: Self = Self(9);
    pub const COL_SEGMENT_DURATION: Self = Self(10);
    pub const COL_DETECTION_BASED_RECORDING: Self = Self(11);
    pub const COL_DETECTION_MODEL: Self = Self(12);
    pub const COL_DETECTION_THRESHOLD: Self = Self(13);
    pub const COL_DETECTION_INTERVAL: Self = Self(14);
    pub const COL_PRE_DETECTION_BUFFER: Self = Self(15);
    pub const COL_POST_DETECTION_BUFFER: Self = Self(16);
    pub const COL_DETECTION_API_URL: Self = Self(17);
    pub const COL_DETECTION_OBJECT_FILTER: Self = Self(18);
    pub const COL_DETECTION_OBJECT_FILTER_LIST: Self = Self(19);
    pub const COL_PROTOCOL: Self = Self(20);
    pub const COL_IS_ONVIF: Self = Self(21);
    pub const COL_RECORD_AUDIO: Self = Self(22);
    pub const COL_BACKCHANNEL_ENABLED: Self = Self(23);
    pub const COL_RETENTION_DAYS: Self = Self(24);
    pub const COL_DETECTION_RETENTION_DAYS: Self = Self(25);
    pub const COL_MAX_STORAGE_MB: Self = Self(26);
    pub const COL_TIER_CRITICAL_MULTIPLIER: Self = Self(27);
    pub const COL_TIER_IMPORTANT_MULTIPLIER: Self = Self(28);
    pub const COL_TIER_EPHEMERAL_MULTIPLIER: Self = Self(29);
    pub const COL_STORAGE_PRIORITY: Self = Self(30);
    pub const COL_PTZ_ENABLED: Self = Self(31);
    pub const COL_PTZ_MAX_X: Self = Self(32);
    pub const COL_PTZ_MAX_Y: Self = Self(33);
    pub const COL_PTZ_MAX_Z: Self = Self(34);
    pub const COL_PTZ_HAS_HOME: Self = Self(35);
    pub const COL_ONVIF_USERNAME: Self = Self(36);
    pub const COL_ONVIF_PASSWORD: Self = Self(37);
    pub const COL_ONVIF_PROFILE: Self = Self(38);
    pub const COL_ONVIF_PORT: Self = Self(39);
    pub const COL_RECORD_ON_SCHEDULE: Self = Self(40);
    pub const COL_RECORDING_SCHEDULE: Self = Self(41);
    pub const COL_TAGS: Self = Self(42);
    pub const COL_ADMIN_URL: Self = Self(43);
    pub const COL_PRIVACY_MODE: Self = Self(44);
    pub const COL_MOTION_TRIGGER_SOURCE: Self = Self(45);
    pub const COL_GO2RTC_SOURCE_OVERRIDE: Self = Self(46);
    pub const COL_SUB_STREAM_URL: Self = Self(47);
    pub const COL_AUDIO_VOICE_ENHANCEMENT: Self = Self(48);
    pub const COL_DETECTION_URL: Self = Self(49);
    pub const COL_PUBLISH_URL: Self = Self(50);
    pub const COL_DETECTION_RECORD_ON_SCHEDULE: Self = Self(51);
    pub const COL_DETECTION_RECORDING_SCHEDULE: Self = Self(52);
    pub const COL_CAMERA_UUID: Self = Self(53);
    pub const COL_LOCATION_UUID: Self = Self(54);
    pub const COL_PLAYBACK_TRANSPORT: Self = Self(55);
    pub const COL_EPTZ_CONFIG: Self = Self(56);
    pub const COL_OBSERVATION_EVIDENCE_ENABLED: Self = Self(57);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const COL_NAME: Self = Self(0);
    pub const COL_URL: Self = Self(1);
    pub const COL_ENABLED: Self = Self(2);
    pub const COL_STREAMING_ENABLED: Self = Self(3);
    pub const COL_WIDTH: Self = Self(4);
    pub const COL_HEIGHT: Self = Self(5);
    pub const COL_FPS: Self = Self(6);
    pub const COL_CODEC: Self = Self(7);
    pub const COL_PRIORITY: Self = Self(8);
    pub const COL_RECORD: Self = Self(9);
    pub const COL_SEGMENT_DURATION: Self = Self(10);
    pub const COL_DETECTION_BASED_RECORDING: Self = Self(11);
    pub const COL_DETECTION_MODEL: Self = Self(12);
    pub const COL_DETECTION_THRESHOLD: Self = Self(13);
    pub const COL_DETECTION_INTERVAL: Self = Self(14);
    pub const COL_PRE_DETECTION_BUFFER: Self = Self(15);
    pub const COL_POST_DETECTION_BUFFER: Self = Self(16);
    pub const COL_DETECTION_API_URL: Self = Self(17);
    pub const COL_DETECTION_OBJECT_FILTER: Self = Self(18);
    pub const COL_DETECTION_OBJECT_FILTER_LIST: Self = Self(19);
    pub const COL_PROTOCOL: Self = Self(20);
    pub const COL_IS_ONVIF: Self = Self(21);
    pub const COL_RECORD_AUDIO: Self = Self(22);
    pub const COL_BACKCHANNEL_ENABLED: Self = Self(23);
    pub const COL_RETENTION_DAYS: Self = Self(24);
    pub const COL_DETECTION_RETENTION_DAYS: Self = Self(25);
    pub const COL_MAX_STORAGE_MB: Self = Self(26);
    pub const COL_TIER_CRITICAL_MULTIPLIER: Self = Self(27);
    pub const COL_TIER_IMPORTANT_MULTIPLIER: Self = Self(28);
    pub const COL_TIER_EPHEMERAL_MULTIPLIER: Self = Self(29);
    pub const COL_STORAGE_PRIORITY: Self = Self(30);
    pub const COL_PTZ_ENABLED: Self = Self(31);
    pub const COL_PTZ_MAX_X: Self = Self(32);
    pub const COL_PTZ_MAX_Y: Self = Self(33);
    pub const COL_PTZ_MAX_Z: Self = Self(34);
    pub const COL_PTZ_HAS_HOME: Self = Self(35);
    pub const COL_ONVIF_USERNAME: Self = Self(36);
    pub const COL_ONVIF_PASSWORD: Self = Self(37);
    pub const COL_ONVIF_PROFILE: Self = Self(38);
    pub const COL_ONVIF_PORT: Self = Self(39);
    pub const COL_RECORD_ON_SCHEDULE: Self = Self(40);
    pub const COL_RECORDING_SCHEDULE: Self = Self(41);
    pub const COL_TAGS: Self = Self(42);
    pub const COL_ADMIN_URL: Self = Self(43);
    pub const COL_PRIVACY_MODE: Self = Self(44);
    pub const COL_MOTION_TRIGGER_SOURCE: Self = Self(45);
    pub const COL_GO2RTC_SOURCE_OVERRIDE: Self = Self(46);
    pub const COL_SUB_STREAM_URL: Self = Self(47);
    pub const COL_AUDIO_VOICE_ENHANCEMENT: Self = Self(48);
    pub const COL_DETECTION_URL: Self = Self(49);
    pub const COL_PUBLISH_URL: Self = Self(50);
    pub const COL_DETECTION_RECORD_ON_SCHEDULE: Self = Self(51);
    pub const COL_DETECTION_RECORDING_SCHEDULE: Self = Self(52);
    pub const COL_CAMERA_UUID: Self = Self(53);
    pub const COL_LOCATION_UUID: Self = Self(54);
    pub const COL_PLAYBACK_TRANSPORT: Self = Self(55);
    pub const COL_EPTZ_CONFIG: Self = Self(56);
    pub const COL_OBSERVATION_EVIDENCE_ENABLED: Self = Self(57);
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const MAX_URL_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const OBSERVATION_DEFAULT_MAX_AGE_DAYS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OBSERVATION_DEFAULT_MAX_BYTES: ::core::ffi::c_longlong =
    2147483648 as ::core::ffi::c_longlong;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn serialize_recording_schedule(
    mut schedule: *const uint8_t,
    mut buf: *mut ::core::ffi::c_char,
    mut buf_size: size_t,
) {
    if schedule.is_null() || buf.is_null() || buf_size < 4 as size_t {
        if !buf.is_null() && buf_size > 0 as size_t {
            *buf.offset(0isize) = '\0' as ::core::ffi::c_char;
        }
        return;
    }
    let mut pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 168 as ::core::ffi::c_int {
        let mut written: ::core::ffi::c_int = snprintf(
            buf.offset(pos as isize),
            buf_size.wrapping_sub(pos as size_t),
            b"%s%d\0".as_ptr() as *const ::core::ffi::c_char,
            if i > 0 as ::core::ffi::c_int {
                b",\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            if *schedule.offset(i as isize) as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        if written < 0 as ::core::ffi::c_int
            || (pos as size_t).wrapping_add(written as size_t) >= buf_size
        {
            break;
        }
        pos += written;
        i += 1;
    }
}
unsafe extern "C" fn deserialize_recording_schedule(
    mut text: *const ::core::ffi::c_char,
    mut schedule: *mut uint8_t,
) {
    if schedule.is_null() {
        return;
    }
    if text.is_null() || *text.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        memset(
            schedule as *mut ::core::ffi::c_void,
            1 as ::core::ffi::c_int,
            168 as size_t,
        );
        return;
    }
    let mut buf: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        text,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *const ::core::ffi::c_char = strtok_r(
        &raw mut buf as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() && idx < 168 as ::core::ffi::c_int {
        let c2rust_fresh0 = idx;
        idx += 1;
        *schedule.offset(c2rust_fresh0 as isize) = (if strtol(
            token,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    while idx < 168 as ::core::ffi::c_int {
        let c2rust_fresh1 = idx;
        idx += 1;
        *schedule.offset(c2rust_fresh1 as isize) = 1 as uint8_t;
    }
}
unsafe extern "C" fn normalized_playback_transport(
    mut value: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    return if playback_transport_is_valid(value) as ::core::ffi::c_int != 0 {
        value
    } else {
        b"auto\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn normalized_observation_max_age_days(
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if value > 0 as ::core::ffi::c_int {
        value
    } else {
        OBSERVATION_DEFAULT_MAX_AGE_DAYS
    };
}
unsafe extern "C" fn normalized_observation_max_bytes(mut value: int64_t) -> sqlite3_int64 {
    return if value > 0 as int64_t {
        value as sqlite3_int64
    } else {
        OBSERVATION_DEFAULT_MAX_BYTES
    };
}
unsafe extern "C" fn bind_observation_settings(
    mut stmt: *mut sqlite3_stmt,
    mut first: ::core::ffi::c_int,
    mut stream: *const stream_config_t,
) {
    sqlite3_bind_int(
        stmt,
        first,
        if (*stream).observation_evidence_enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        stmt,
        first + 1 as ::core::ffi::c_int,
        &raw const (*stream).observation_labels as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        first + 2 as ::core::ffi::c_int,
        normalized_observation_max_age_days((*stream).observation_max_age_days),
    );
    sqlite3_bind_int64(
        stmt,
        first + 3 as ::core::ffi::c_int,
        normalized_observation_max_bytes((*stream).observation_max_bytes),
    );
}
unsafe extern "C" fn load_observation_settings(
    mut stmt: *mut sqlite3_stmt,
    mut first: ::core::ffi::c_int,
    mut stream: *mut stream_config_t,
) {
    (*stream).observation_evidence_enabled =
        sqlite3_column_int(stmt, first) != 0 as ::core::ffi::c_int;
    let mut labels: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, first + 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    safe_strcpy(
        &raw mut (*stream).observation_labels as *mut ::core::ffi::c_char,
        if !labels.is_null() {
            labels
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    (*stream).observation_max_age_days = normalized_observation_max_age_days(sqlite3_column_int(
        stmt,
        first + 2 as ::core::ffi::c_int,
    ));
    (*stream).observation_max_bytes = normalized_observation_max_bytes(sqlite3_column_int64(
        stmt,
        first + 3 as ::core::ffi::c_int,
    ) as int64_t) as int64_t;
}
unsafe extern "C" fn normalized_go2rtc_source_override(
    mut value: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    return if string_has_non_whitespace(value) as ::core::ffi::c_int != 0 {
        value
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn stream_transaction_begin(
    mut db: *mut sqlite3,
    mut owns_transaction: *mut bool,
) -> bool {
    *owns_transaction = sqlite3_get_autocommit(db) != 0 as ::core::ffi::c_int;
    if !*owns_transaction {
        return r#true != 0;
    }
    return sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
}
unsafe extern "C" fn stream_transaction_finish(
    mut db: *mut sqlite3,
    mut owns_transaction: bool,
    mut success: bool,
) -> bool {
    if !owns_transaction {
        return success;
    }
    if !success {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        return r#false != 0;
    }
    if sqlite3_exec(
        db,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        return r#true != 0;
    }
    sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_stream_config(mut stream: *const stream_config_t) -> uint64_t {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream configuration is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    let mut p: *const ::core::ffi::c_char = &raw const (*stream).name as *const ::core::ffi::c_char;
    while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name is empty or whitespace-only, rejecting\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    pthread_mutex_lock(db_mutex);
    let mut check_sql: *const ::core::ffi::c_char =
        b"SELECT id FROM streams WHERE name = ? AND enabled = 0;\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut check_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        check_sql,
        -1 as ::core::ffi::c_int,
        &raw mut check_stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement to check for disabled stream: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return 0 as uint64_t;
    }
    sqlite3_bind_text(
        check_stmt,
        1 as ::core::ffi::c_int,
        &raw const (*stream).name as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(check_stmt) == SQLITE_ROW {
        let mut existing_id: uint64_t =
            sqlite3_column_int64(check_stmt, 0 as ::core::ffi::c_int) as uint64_t;
        if !check_stmt.is_null() {
            sqlite3_finalize(check_stmt);
            check_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        let mut update_sql: *const ::core::ffi::c_char = b"UPDATE streams SET url = ?, enabled = ?, streaming_enabled = ?, width = ?, height = ?, fps = ?, codec = ?, priority = ?, record = ?, segment_duration = ?, detection_based_recording = ?, detection_model = ?, detection_threshold = ?, detection_interval = ?, pre_detection_buffer = ?, post_detection_buffer = ?, detection_api_url = ?, detection_object_filter = ?, detection_object_filter_list = ?, protocol = ?, is_onvif = ?, record_audio = ?, backchannel_enabled = ?, retention_days = ?, detection_retention_days = ?, max_storage_mb = ?, tier_critical_multiplier = ?, tier_important_multiplier = ?, tier_ephemeral_multiplier = ?, storage_priority = ?, ptz_enabled = ?, ptz_max_x = ?, ptz_max_y = ?, ptz_max_z = ?, ptz_has_home = ?, onvif_username = ?, onvif_password = ?, onvif_profile = ?, onvif_port = ?, record_on_schedule = ?, recording_schedule = ?, tags = ?, admin_url = ?, privacy_mode = ?, motion_trigger_source = ?, go2rtc_source_override = ?, sub_stream_url = ?, audio_voice_enhancement = ?, detection_url = ?, publish_url = ?, detection_record_on_schedule = ?, detection_recording_schedule = ?, playback_transport = ?, eptz_config = ?, observation_evidence_enabled = ?, observation_labels = ?, observation_max_age_days = ?, observation_max_bytes = ? WHERE id = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        rc = sqlite3_prepare_v2(
            db,
            update_sql,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to prepare statement to update disabled stream: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            pthread_mutex_unlock(db_mutex);
            return 0 as uint64_t;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw const (*stream).url as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(
            stmt,
            2 as ::core::ffi::c_int,
            if (*stream).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(
            stmt,
            3 as ::core::ffi::c_int,
            if (*stream).streaming_enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(stmt, 4 as ::core::ffi::c_int, (*stream).width);
        sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, (*stream).height);
        sqlite3_bind_int(stmt, 6 as ::core::ffi::c_int, (*stream).fps);
        sqlite3_bind_text(
            stmt,
            7 as ::core::ffi::c_int,
            &raw const (*stream).codec as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(stmt, 8 as ::core::ffi::c_int, (*stream).priority);
        sqlite3_bind_int(
            stmt,
            9 as ::core::ffi::c_int,
            if (*stream).record as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(stmt, 10 as ::core::ffi::c_int, (*stream).segment_duration);
        sqlite3_bind_int(
            stmt,
            11 as ::core::ffi::c_int,
            if (*stream).detection_based_recording as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            stmt,
            12 as ::core::ffi::c_int,
            &raw const (*stream).detection_model as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_double(
            stmt,
            13 as ::core::ffi::c_int,
            (*stream).detection_threshold as ::core::ffi::c_double,
        );
        sqlite3_bind_int(stmt, 14 as ::core::ffi::c_int, (*stream).detection_interval);
        sqlite3_bind_int(
            stmt,
            15 as ::core::ffi::c_int,
            (*stream).pre_detection_buffer,
        );
        sqlite3_bind_int(
            stmt,
            16 as ::core::ffi::c_int,
            (*stream).post_detection_buffer,
        );
        sqlite3_bind_text(
            stmt,
            17 as ::core::ffi::c_int,
            &raw const (*stream).detection_api_url as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            18 as ::core::ffi::c_int,
            &raw const (*stream).detection_object_filter as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            19 as ::core::ffi::c_int,
            &raw const (*stream).detection_object_filter_list as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(
            stmt,
            20 as ::core::ffi::c_int,
            (*stream).protocol.0 as ::core::ffi::c_int,
        );
        sqlite3_bind_int(
            stmt,
            21 as ::core::ffi::c_int,
            if (*stream).is_onvif as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(
            stmt,
            22 as ::core::ffi::c_int,
            if (*stream).record_audio as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(
            stmt,
            23 as ::core::ffi::c_int,
            if (*stream).backchannel_enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(stmt, 24 as ::core::ffi::c_int, (*stream).retention_days);
        sqlite3_bind_int(
            stmt,
            25 as ::core::ffi::c_int,
            (*stream).detection_retention_days,
        );
        sqlite3_bind_int(stmt, 26 as ::core::ffi::c_int, (*stream).max_storage_mb);
        sqlite3_bind_double(
            stmt,
            27 as ::core::ffi::c_int,
            (*stream).tier_critical_multiplier,
        );
        sqlite3_bind_double(
            stmt,
            28 as ::core::ffi::c_int,
            (*stream).tier_important_multiplier,
        );
        sqlite3_bind_double(
            stmt,
            29 as ::core::ffi::c_int,
            (*stream).tier_ephemeral_multiplier,
        );
        sqlite3_bind_int(stmt, 30 as ::core::ffi::c_int, (*stream).storage_priority);
        sqlite3_bind_int(
            stmt,
            31 as ::core::ffi::c_int,
            if (*stream).ptz_enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(stmt, 32 as ::core::ffi::c_int, (*stream).ptz_max_x);
        sqlite3_bind_int(stmt, 33 as ::core::ffi::c_int, (*stream).ptz_max_y);
        sqlite3_bind_int(stmt, 34 as ::core::ffi::c_int, (*stream).ptz_max_z);
        sqlite3_bind_int(
            stmt,
            35 as ::core::ffi::c_int,
            if (*stream).ptz_has_home as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            stmt,
            36 as ::core::ffi::c_int,
            &raw const (*stream).onvif_username as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            37 as ::core::ffi::c_int,
            &raw const (*stream).onvif_password as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            38 as ::core::ffi::c_int,
            &raw const (*stream).onvif_profile as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(stmt, 39 as ::core::ffi::c_int, (*stream).onvif_port);
        sqlite3_bind_int(
            stmt,
            40 as ::core::ffi::c_int,
            if (*stream).record_on_schedule as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        let mut schedule_buf: [::core::ffi::c_char; 512] = [0; 512];
        serialize_recording_schedule(
            &raw const (*stream).recording_schedule as *const uint8_t,
            &raw mut schedule_buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
        sqlite3_bind_text(
            stmt,
            41 as ::core::ffi::c_int,
            &raw mut schedule_buf as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            42 as ::core::ffi::c_int,
            &raw const (*stream).tags as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            43 as ::core::ffi::c_int,
            &raw const (*stream).admin_url as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(
            stmt,
            44 as ::core::ffi::c_int,
            if (*stream).privacy_mode as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            stmt,
            45 as ::core::ffi::c_int,
            &raw const (*stream).motion_trigger_source as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            46 as ::core::ffi::c_int,
            normalized_go2rtc_source_override(
                &raw const (*stream).go2rtc_source_override as *const ::core::ffi::c_char,
            ),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            47 as ::core::ffi::c_int,
            &raw const (*stream).sub_stream_url as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(
            stmt,
            48 as ::core::ffi::c_int,
            if (*stream).audio_voice_enhancement as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            stmt,
            49 as ::core::ffi::c_int,
            &raw const (*stream).detection_url as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            50 as ::core::ffi::c_int,
            &raw const (*stream).publish_url as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(
            stmt,
            51 as ::core::ffi::c_int,
            if (*stream).detection_record_on_schedule as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        let mut detection_schedule_buf: [::core::ffi::c_char; 512] = [0; 512];
        serialize_recording_schedule(
            &raw const (*stream).detection_recording_schedule as *const uint8_t,
            &raw mut detection_schedule_buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
        sqlite3_bind_text(
            stmt,
            52 as ::core::ffi::c_int,
            &raw mut detection_schedule_buf as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            53 as ::core::ffi::c_int,
            normalized_playback_transport(
                &raw const (*stream).playback_transport as *const ::core::ffi::c_char,
            ),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            54 as ::core::ffi::c_int,
            &raw const (*stream).eptz_config as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        bind_observation_settings(stmt, 55 as ::core::ffi::c_int, stream);
        sqlite3_bind_int64(stmt, 59 as ::core::ffi::c_int, existing_id as sqlite3_int64);
        let mut owns_transaction: bool = r#false != 0;
        if !stream_transaction_begin(db, &raw mut owns_transaction) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to begin reactivation transaction: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            sqlite3_finalize(stmt);
            pthread_mutex_unlock(db_mutex);
            return 0 as uint64_t;
        }
        rc = sqlite3_step(stmt);
        if rc != SQLITE_DONE {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to update disabled stream configuration: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            if !stmt.is_null() {
                sqlite3_finalize(stmt);
                stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            }
            stream_transaction_finish(db, owns_transaction, r#false != 0);
            pthread_mutex_unlock(db_mutex);
            return 0 as uint64_t;
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        if db_camera_tags_sync_legacy_by_name_locked(
            db,
            &raw const (*stream).name as *const ::core::ffi::c_char,
            &raw const (*stream).tags as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to sync normalized tags for reactivated stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw const (*stream).name as *const ::core::ffi::c_char,
            );
            stream_transaction_finish(db, owns_transaction, r#false != 0);
            pthread_mutex_unlock(db_mutex);
            return 0 as uint64_t;
        }
        if !stream_transaction_finish(db, owns_transaction, r#true != 0) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to commit reactivated stream %s: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw const (*stream).name as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            pthread_mutex_unlock(db_mutex);
            return 0 as uint64_t;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated disabled stream configuration: name=%s, enabled=%s, detection=%s, model=%s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw const (*stream).name as *const ::core::ffi::c_char,
            if (*stream).enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            if (*stream).detection_based_recording as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw const (*stream).detection_model as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(db_mutex);
        return existing_id;
    }
    if !check_stmt.is_null() {
        sqlite3_finalize(check_stmt);
        check_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO streams (name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, detection_based_recording, detection_model, detection_threshold, detection_interval, pre_detection_buffer, post_detection_buffer, detection_api_url, detection_object_filter, detection_object_filter_list, protocol, is_onvif, record_audio, backchannel_enabled, retention_days, detection_retention_days, max_storage_mb, tier_critical_multiplier, tier_important_multiplier, tier_ephemeral_multiplier, storage_priority, ptz_enabled, ptz_max_x, ptz_max_y, ptz_max_z, ptz_has_home, onvif_username, onvif_password, onvif_profile, onvif_port, record_on_schedule, recording_schedule, tags, admin_url, privacy_mode, motion_trigger_source, go2rtc_source_override, sub_stream_url, audio_voice_enhancement, detection_url, publish_url, detection_record_on_schedule, detection_recording_schedule, playback_transport, eptz_config, observation_evidence_enabled, observation_labels, observation_max_age_days, observation_max_bytes, camera_uuid, location_uuid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)), 2) || '-' || substr('89ab', (abs(random()) % 4) + 1, 1) || substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), (SELECT uuid FROM camera_locations WHERE is_system = 1 LIMIT 1));\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return 0 as uint64_t;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        &raw const (*stream).name as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw const (*stream).url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        3 as ::core::ffi::c_int,
        if (*stream).enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(
        stmt,
        4 as ::core::ffi::c_int,
        if (*stream).streaming_enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, (*stream).width);
    sqlite3_bind_int(stmt, 6 as ::core::ffi::c_int, (*stream).height);
    sqlite3_bind_int(stmt, 7 as ::core::ffi::c_int, (*stream).fps);
    sqlite3_bind_text(
        stmt,
        8 as ::core::ffi::c_int,
        &raw const (*stream).codec as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 9 as ::core::ffi::c_int, (*stream).priority);
    sqlite3_bind_int(
        stmt,
        10 as ::core::ffi::c_int,
        if (*stream).record as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(stmt, 11 as ::core::ffi::c_int, (*stream).segment_duration);
    sqlite3_bind_int(
        stmt,
        12 as ::core::ffi::c_int,
        if (*stream).detection_based_recording as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        stmt,
        13 as ::core::ffi::c_int,
        &raw const (*stream).detection_model as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_double(
        stmt,
        14 as ::core::ffi::c_int,
        (*stream).detection_threshold as ::core::ffi::c_double,
    );
    sqlite3_bind_int(stmt, 15 as ::core::ffi::c_int, (*stream).detection_interval);
    sqlite3_bind_int(
        stmt,
        16 as ::core::ffi::c_int,
        (*stream).pre_detection_buffer,
    );
    sqlite3_bind_int(
        stmt,
        17 as ::core::ffi::c_int,
        (*stream).post_detection_buffer,
    );
    sqlite3_bind_text(
        stmt,
        18 as ::core::ffi::c_int,
        &raw const (*stream).detection_api_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        19 as ::core::ffi::c_int,
        &raw const (*stream).detection_object_filter as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        20 as ::core::ffi::c_int,
        &raw const (*stream).detection_object_filter_list as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        21 as ::core::ffi::c_int,
        (*stream).protocol.0 as ::core::ffi::c_int,
    );
    sqlite3_bind_int(
        stmt,
        22 as ::core::ffi::c_int,
        if (*stream).is_onvif as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(
        stmt,
        23 as ::core::ffi::c_int,
        if (*stream).record_audio as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(
        stmt,
        24 as ::core::ffi::c_int,
        if (*stream).backchannel_enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(stmt, 25 as ::core::ffi::c_int, (*stream).retention_days);
    sqlite3_bind_int(
        stmt,
        26 as ::core::ffi::c_int,
        (*stream).detection_retention_days,
    );
    sqlite3_bind_int(stmt, 27 as ::core::ffi::c_int, (*stream).max_storage_mb);
    sqlite3_bind_double(
        stmt,
        28 as ::core::ffi::c_int,
        (*stream).tier_critical_multiplier,
    );
    sqlite3_bind_double(
        stmt,
        29 as ::core::ffi::c_int,
        (*stream).tier_important_multiplier,
    );
    sqlite3_bind_double(
        stmt,
        30 as ::core::ffi::c_int,
        (*stream).tier_ephemeral_multiplier,
    );
    sqlite3_bind_int(stmt, 31 as ::core::ffi::c_int, (*stream).storage_priority);
    sqlite3_bind_int(
        stmt,
        32 as ::core::ffi::c_int,
        if (*stream).ptz_enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(stmt, 33 as ::core::ffi::c_int, (*stream).ptz_max_x);
    sqlite3_bind_int(stmt, 34 as ::core::ffi::c_int, (*stream).ptz_max_y);
    sqlite3_bind_int(stmt, 35 as ::core::ffi::c_int, (*stream).ptz_max_z);
    sqlite3_bind_int(
        stmt,
        36 as ::core::ffi::c_int,
        if (*stream).ptz_has_home as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        stmt,
        37 as ::core::ffi::c_int,
        &raw const (*stream).onvif_username as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        38 as ::core::ffi::c_int,
        &raw const (*stream).onvif_password as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        39 as ::core::ffi::c_int,
        &raw const (*stream).onvif_profile as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 40 as ::core::ffi::c_int, (*stream).onvif_port);
    sqlite3_bind_int(
        stmt,
        41 as ::core::ffi::c_int,
        if (*stream).record_on_schedule as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    let mut insert_schedule_buf: [::core::ffi::c_char; 512] = [0; 512];
    serialize_recording_schedule(
        &raw const (*stream).recording_schedule as *const uint8_t,
        &raw mut insert_schedule_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    sqlite3_bind_text(
        stmt,
        42 as ::core::ffi::c_int,
        &raw mut insert_schedule_buf as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        stmt,
        43 as ::core::ffi::c_int,
        &raw const (*stream).tags as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        44 as ::core::ffi::c_int,
        &raw const (*stream).admin_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        45 as ::core::ffi::c_int,
        if (*stream).privacy_mode as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        stmt,
        46 as ::core::ffi::c_int,
        &raw const (*stream).motion_trigger_source as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        47 as ::core::ffi::c_int,
        normalized_go2rtc_source_override(
            &raw const (*stream).go2rtc_source_override as *const ::core::ffi::c_char,
        ),
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        48 as ::core::ffi::c_int,
        &raw const (*stream).sub_stream_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        49 as ::core::ffi::c_int,
        if (*stream).audio_voice_enhancement as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        stmt,
        50 as ::core::ffi::c_int,
        &raw const (*stream).detection_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        51 as ::core::ffi::c_int,
        &raw const (*stream).publish_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        52 as ::core::ffi::c_int,
        if (*stream).detection_record_on_schedule as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    let mut insert_detection_schedule_buf: [::core::ffi::c_char; 512] = [0; 512];
    serialize_recording_schedule(
        &raw const (*stream).detection_recording_schedule as *const uint8_t,
        &raw mut insert_detection_schedule_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    sqlite3_bind_text(
        stmt,
        53 as ::core::ffi::c_int,
        &raw mut insert_detection_schedule_buf as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        stmt,
        54 as ::core::ffi::c_int,
        normalized_playback_transport(
            &raw const (*stream).playback_transport as *const ::core::ffi::c_char,
        ),
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        55 as ::core::ffi::c_int,
        &raw const (*stream).eptz_config as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    bind_observation_settings(stmt, 56 as ::core::ffi::c_int, stream);
    let mut owns_transaction_0: bool = r#false != 0;
    if !stream_transaction_begin(db, &raw mut owns_transaction_0) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin stream insert transaction: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return 0 as uint64_t;
    }
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to add stream configuration: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    } else {
        stream_id = sqlite3_last_insert_rowid(db) as uint64_t;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    let mut success: bool = stream_id != 0 as uint64_t;
    if success as ::core::ffi::c_int != 0
        && db_camera_tags_sync_legacy_by_name_locked(
            db,
            &raw const (*stream).name as *const ::core::ffi::c_char,
            &raw const (*stream).tags as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to sync normalized tags for new stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*stream).name as *const ::core::ffi::c_char,
        );
        success = r#false != 0;
    }
    if !stream_transaction_finish(db, owns_transaction_0, success) {
        stream_id = 0 as uint64_t;
    }
    if stream_id != 0 as uint64_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Added stream configuration with ID %llu\0".as_ptr() as *const ::core::ffi::c_char,
            stream_id as ::core::ffi::c_ulonglong,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Added stream configuration: name=%s, enabled=%s, detection=%s, model=%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*stream).name as *const ::core::ffi::c_char,
            if (*stream).enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            if (*stream).detection_based_recording as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw const (*stream).detection_model as *const ::core::ffi::c_char,
        );
    }
    pthread_mutex_unlock(db_mutex);
    return stream_id;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_privacy_mode(
    mut name: *const ::core::ffi::c_char,
    mut privacy_mode: bool,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"UPDATE streams SET privacy_mode = ? WHERE name = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int(
            stmt,
            1 as ::core::ffi::c_int,
            if privacy_mode as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if sqlite3_step(stmt) == SQLITE_DONE {
            result = if sqlite3_changes(db) > 0 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to set privacy mode for stream %s: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                name,
                sqlite3_errmsg(db),
            );
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare privacy mode update: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_stream_config(
    mut name: *const ::core::ffi::c_char,
    mut stream: *const stream_config_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if name.is_null() || stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name and configuration are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"UPDATE streams SET name = ?, url = ?, enabled = ?, streaming_enabled = ?, width = ?, height = ?, fps = ?, codec = ?, priority = ?, record = ?, segment_duration = ?, detection_based_recording = ?, detection_model = ?, detection_threshold = ?, detection_interval = ?, pre_detection_buffer = ?, post_detection_buffer = ?, detection_api_url = ?, detection_object_filter = ?, detection_object_filter_list = ?, protocol = ?, is_onvif = ?, record_audio = ?, backchannel_enabled = ?, retention_days = ?, detection_retention_days = ?, max_storage_mb = ?, tier_critical_multiplier = ?, tier_important_multiplier = ?, tier_ephemeral_multiplier = ?, storage_priority = ?, ptz_enabled = ?, ptz_max_x = ?, ptz_max_y = ?, ptz_max_z = ?, ptz_has_home = ?, onvif_username = ?, onvif_password = ?, onvif_profile = ?, onvif_port = ?, record_on_schedule = ?, recording_schedule = ?, tags = ?, admin_url = ?, privacy_mode = COALESCE(?, privacy_mode), motion_trigger_source = ?, go2rtc_source_override = ?, sub_stream_url = ?, audio_voice_enhancement = ?, detection_url = ?, publish_url = ?, detection_record_on_schedule = ?, detection_recording_schedule = ?, playback_transport = ?, eptz_config = ?, observation_evidence_enabled = ?, observation_labels = ?, observation_max_age_days = ?, observation_max_bytes = ? WHERE name = ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        &raw const (*stream).name as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw const (*stream).url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        3 as ::core::ffi::c_int,
        if (*stream).enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(
        stmt,
        4 as ::core::ffi::c_int,
        if (*stream).streaming_enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, (*stream).width);
    sqlite3_bind_int(stmt, 6 as ::core::ffi::c_int, (*stream).height);
    sqlite3_bind_int(stmt, 7 as ::core::ffi::c_int, (*stream).fps);
    sqlite3_bind_text(
        stmt,
        8 as ::core::ffi::c_int,
        &raw const (*stream).codec as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 9 as ::core::ffi::c_int, (*stream).priority);
    sqlite3_bind_int(
        stmt,
        10 as ::core::ffi::c_int,
        if (*stream).record as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(stmt, 11 as ::core::ffi::c_int, (*stream).segment_duration);
    sqlite3_bind_int(
        stmt,
        12 as ::core::ffi::c_int,
        if (*stream).detection_based_recording as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        stmt,
        13 as ::core::ffi::c_int,
        &raw const (*stream).detection_model as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_double(
        stmt,
        14 as ::core::ffi::c_int,
        (*stream).detection_threshold as ::core::ffi::c_double,
    );
    sqlite3_bind_int(stmt, 15 as ::core::ffi::c_int, (*stream).detection_interval);
    sqlite3_bind_int(
        stmt,
        16 as ::core::ffi::c_int,
        (*stream).pre_detection_buffer,
    );
    sqlite3_bind_int(
        stmt,
        17 as ::core::ffi::c_int,
        (*stream).post_detection_buffer,
    );
    sqlite3_bind_text(
        stmt,
        18 as ::core::ffi::c_int,
        &raw const (*stream).detection_api_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        19 as ::core::ffi::c_int,
        &raw const (*stream).detection_object_filter as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        20 as ::core::ffi::c_int,
        &raw const (*stream).detection_object_filter_list as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        21 as ::core::ffi::c_int,
        (*stream).protocol.0 as ::core::ffi::c_int,
    );
    sqlite3_bind_int(
        stmt,
        22 as ::core::ffi::c_int,
        if (*stream).is_onvif as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(
        stmt,
        23 as ::core::ffi::c_int,
        if (*stream).record_audio as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(
        stmt,
        24 as ::core::ffi::c_int,
        if (*stream).backchannel_enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(stmt, 25 as ::core::ffi::c_int, (*stream).retention_days);
    sqlite3_bind_int(
        stmt,
        26 as ::core::ffi::c_int,
        (*stream).detection_retention_days,
    );
    sqlite3_bind_int(stmt, 27 as ::core::ffi::c_int, (*stream).max_storage_mb);
    sqlite3_bind_double(
        stmt,
        28 as ::core::ffi::c_int,
        (*stream).tier_critical_multiplier,
    );
    sqlite3_bind_double(
        stmt,
        29 as ::core::ffi::c_int,
        (*stream).tier_important_multiplier,
    );
    sqlite3_bind_double(
        stmt,
        30 as ::core::ffi::c_int,
        (*stream).tier_ephemeral_multiplier,
    );
    sqlite3_bind_int(stmt, 31 as ::core::ffi::c_int, (*stream).storage_priority);
    sqlite3_bind_int(
        stmt,
        32 as ::core::ffi::c_int,
        if (*stream).ptz_enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int(stmt, 33 as ::core::ffi::c_int, (*stream).ptz_max_x);
    sqlite3_bind_int(stmt, 34 as ::core::ffi::c_int, (*stream).ptz_max_y);
    sqlite3_bind_int(stmt, 35 as ::core::ffi::c_int, (*stream).ptz_max_z);
    sqlite3_bind_int(
        stmt,
        36 as ::core::ffi::c_int,
        if (*stream).ptz_has_home as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        stmt,
        37 as ::core::ffi::c_int,
        &raw const (*stream).onvif_username as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        38 as ::core::ffi::c_int,
        &raw const (*stream).onvif_password as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        39 as ::core::ffi::c_int,
        &raw const (*stream).onvif_profile as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 40 as ::core::ffi::c_int, (*stream).onvif_port);
    sqlite3_bind_int(
        stmt,
        41 as ::core::ffi::c_int,
        if (*stream).record_on_schedule as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    let mut update_schedule_buf: [::core::ffi::c_char; 512] = [0; 512];
    serialize_recording_schedule(
        &raw const (*stream).recording_schedule as *const uint8_t,
        &raw mut update_schedule_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    sqlite3_bind_text(
        stmt,
        42 as ::core::ffi::c_int,
        &raw mut update_schedule_buf as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        stmt,
        43 as ::core::ffi::c_int,
        &raw const (*stream).tags as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        44 as ::core::ffi::c_int,
        &raw const (*stream).admin_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_null(stmt, 45 as ::core::ffi::c_int);
    sqlite3_bind_text(
        stmt,
        46 as ::core::ffi::c_int,
        &raw const (*stream).motion_trigger_source as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        47 as ::core::ffi::c_int,
        normalized_go2rtc_source_override(
            &raw const (*stream).go2rtc_source_override as *const ::core::ffi::c_char,
        ),
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        48 as ::core::ffi::c_int,
        &raw const (*stream).sub_stream_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        49 as ::core::ffi::c_int,
        if (*stream).audio_voice_enhancement as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        stmt,
        50 as ::core::ffi::c_int,
        &raw const (*stream).detection_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        51 as ::core::ffi::c_int,
        &raw const (*stream).publish_url as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        52 as ::core::ffi::c_int,
        if (*stream).detection_record_on_schedule as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    let mut update_detection_schedule_buf: [::core::ffi::c_char; 512] = [0; 512];
    serialize_recording_schedule(
        &raw const (*stream).detection_recording_schedule as *const uint8_t,
        &raw mut update_detection_schedule_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    sqlite3_bind_text(
        stmt,
        53 as ::core::ffi::c_int,
        &raw mut update_detection_schedule_buf as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        stmt,
        54 as ::core::ffi::c_int,
        normalized_playback_transport(
            &raw const (*stream).playback_transport as *const ::core::ffi::c_char,
        ),
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        55 as ::core::ffi::c_int,
        &raw const (*stream).eptz_config as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    bind_observation_settings(stmt, 56 as ::core::ffi::c_int, stream);
    sqlite3_bind_text(
        stmt,
        60 as ::core::ffi::c_int,
        name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut owns_transaction: bool = r#false != 0;
    if !stream_transaction_begin(db, &raw mut owns_transaction) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin stream update transaction: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update stream configuration: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        stream_transaction_finish(db, owns_transaction, r#false != 0);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if db_camera_tags_sync_legacy_by_name_locked(
        db,
        &raw const (*stream).name as *const ::core::ffi::c_char,
        &raw const (*stream).tags as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to sync normalized tags for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
        stream_transaction_finish(db, owns_transaction, r#false != 0);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if !stream_transaction_finish(db, owns_transaction, r#true != 0) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to commit stream update for %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            name,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Updated stream configuration for %s: enabled=%s, detection=%s, model=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*stream).name as *const ::core::ffi::c_char,
        if (*stream).enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if (*stream).detection_based_recording as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw const (*stream).detection_model as *const ::core::ffi::c_char,
    );
    pthread_mutex_unlock(db_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_stream_video_params(
    mut stream_name: *const ::core::ffi::c_char,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut fps: ::core::ffi::c_int,
    mut codec: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name is required for video param update\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut old_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut old_height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut old_fps: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut old_codec: [::core::ffi::c_char; 16] = [
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
    ];
    let mut select_sql: *const ::core::ffi::c_char =
        b"SELECT width, height, fps, codec FROM streams WHERE name = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        select_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if sqlite3_step(stmt) == SQLITE_ROW {
            old_width = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
            old_height = sqlite3_column_int(stmt, 1 as ::core::ffi::c_int);
            old_fps = sqlite3_column_int(stmt, 2 as ::core::ffi::c_int);
            let mut c: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 3 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if !c.is_null() {
                safe_strcpy(
                    &raw mut old_codec as *mut ::core::ffi::c_char,
                    c,
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                    0 as size_t,
                );
            }
        }
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    let mut same: bool = old_width == width && old_height == height && old_fps == fps;
    if same as ::core::ffi::c_int != 0 && !codec.is_null() {
        same = strcmp(&raw mut old_codec as *mut ::core::ffi::c_char, codec)
            == 0 as ::core::ffi::c_int;
    }
    if same {
        pthread_mutex_unlock(db_mutex);
        return 0 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char =
        b"UPDATE streams SET width  = ?, height = ?, fps    = ?, codec  = ? WHERE name = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare video params update: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, width);
    sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, height);
    sqlite3_bind_int(stmt, 3 as ::core::ffi::c_int, fps);
    sqlite3_bind_text(
        stmt,
        4 as ::core::ffi::c_int,
        if !codec.is_null() {
            codec
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        5 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update video params for stream %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut changes: ::core::ffi::c_int = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if changes > 0 as ::core::ffi::c_int {
        if old_width != width || old_height != height {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s resolution changed: %dx%d -> %dx%d (fps %d->%d, codec %s->%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                old_width,
                old_height,
                width,
                height,
                old_fps,
                fps,
                if old_codec[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut old_codec as *mut ::core::ffi::c_char
                } else {
                    b"?\0".as_ptr() as *const ::core::ffi::c_char
                },
                if !codec.is_null() {
                    codec
                } else {
                    b"?\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Auto-detected video params for stream %s: %dx%d @ %d fps, codec=%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                width,
                height,
                fps,
                if !codec.is_null() {
                    codec
                } else {
                    b"unknown\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_stream_config(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return delete_stream_config_internal(name, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_stream_config_internal(
    mut name: *const ::core::ffi::c_char,
    mut permanent: bool,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if permanent {
        sql = b"DELETE FROM streams WHERE name = ?;\0".as_ptr() as *const ::core::ffi::c_char;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Preparing to permanently delete stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
    } else {
        sql = b"UPDATE streams SET enabled = 0 WHERE name = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Preparing to disable stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
    }
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to %s stream configuration: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if permanent as ::core::ffi::c_int != 0 {
                b"permanently delete\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disable\0".as_ptr() as *const ::core::ffi::c_char
            },
            sqlite3_errmsg(db),
        );
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if permanent {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Permanently deleted stream configuration: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Disabled stream configuration: %s\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
    }
    pthread_mutex_unlock(db_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_config_by_name(
    mut name: *const ::core::ffi::c_char,
    mut stream: *mut stream_config_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if name.is_null() || stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name and configuration pointer are required\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, detection_based_recording, detection_model, detection_threshold, detection_interval, pre_detection_buffer, post_detection_buffer, detection_api_url, detection_object_filter, detection_object_filter_list, protocol, is_onvif, record_audio, backchannel_enabled, retention_days, detection_retention_days, max_storage_mb, tier_critical_multiplier, tier_important_multiplier, tier_ephemeral_multiplier, storage_priority, ptz_enabled, ptz_max_x, ptz_max_y, ptz_max_z, ptz_has_home, onvif_username, onvif_password, onvif_profile, onvif_port, record_on_schedule, recording_schedule, tags, admin_url, privacy_mode, motion_trigger_source, go2rtc_source_override, sub_stream_url, audio_voice_enhancement, detection_url, publish_url, detection_record_on_schedule, detection_recording_schedule, camera_uuid, location_uuid, playback_transport, eptz_config, observation_evidence_enabled, observation_labels, observation_max_age_days, observation_max_bytes FROM streams WHERE name = ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) == SQLITE_ROW {
        memset(
            stream as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stream_config_t>(),
        );
        let mut camera_uuid: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_CAMERA_UUID.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !camera_uuid.is_null() {
            safe_strcpy(
                &raw mut (*stream).camera_uuid as *mut ::core::ffi::c_char,
                camera_uuid,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
        let mut location_uuid: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_LOCATION_UUID.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !location_uuid.is_null() {
            safe_strcpy(
                &raw mut (*stream).location_uuid as *mut ::core::ffi::c_char,
                location_uuid,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
        let mut stream_name: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed::COL_NAME.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !stream_name.is_null() {
            safe_strcpy(
                &raw mut (*stream).name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
        }
        let mut url: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed::COL_URL.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !url.is_null() {
            safe_strcpy(
                &raw mut (*stream).url as *mut ::core::ffi::c_char,
                url,
                MAX_URL_LENGTH as size_t,
                0 as size_t,
            );
        }
        (*stream).enabled =
            sqlite3_column_int(stmt, C2Rust_Unnamed::COL_ENABLED.0 as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int;
        (*stream).streaming_enabled = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_STREAMING_ENABLED.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        (*stream).width =
            sqlite3_column_int(stmt, C2Rust_Unnamed::COL_WIDTH.0 as ::core::ffi::c_int);
        (*stream).height =
            sqlite3_column_int(stmt, C2Rust_Unnamed::COL_HEIGHT.0 as ::core::ffi::c_int);
        (*stream).fps = sqlite3_column_int(stmt, C2Rust_Unnamed::COL_FPS.0 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed::COL_CODEC.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*stream).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*stream).priority =
            sqlite3_column_int(stmt, C2Rust_Unnamed::COL_PRIORITY.0 as ::core::ffi::c_int);
        (*stream).record =
            sqlite3_column_int(stmt, C2Rust_Unnamed::COL_RECORD.0 as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int;
        (*stream).segment_duration = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_SEGMENT_DURATION.0 as ::core::ffi::c_int,
        );
        (*stream).detection_based_recording = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_BASED_RECORDING.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut detection_model: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_MODEL.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_model.is_null() {
            safe_strcpy(
                &raw mut (*stream).detection_model as *mut ::core::ffi::c_char,
                detection_model,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        }
        (*stream).detection_threshold = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_THRESHOLD.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_double(
                stmt,
                C2Rust_Unnamed::COL_DETECTION_THRESHOLD.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_float
        } else {
            0.5f32
        };
        (*stream).detection_interval = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_INTERVAL.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed::COL_DETECTION_INTERVAL.0 as ::core::ffi::c_int,
            )
        } else {
            10 as ::core::ffi::c_int
        };
        (*stream).pre_detection_buffer = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_PRE_DETECTION_BUFFER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed::COL_PRE_DETECTION_BUFFER.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*stream).post_detection_buffer = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_POST_DETECTION_BUFFER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed::COL_POST_DETECTION_BUFFER.0 as ::core::ffi::c_int,
            )
        } else {
            3 as ::core::ffi::c_int
        };
        let mut detection_api_url: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_API_URL.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_api_url.is_null() {
            safe_strcpy(
                &raw mut (*stream).detection_api_url as *mut ::core::ffi::c_char,
                detection_api_url,
                MAX_URL_LENGTH as size_t,
                0 as size_t,
            );
        }
        let mut detection_object_filter: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_OBJECT_FILTER.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_object_filter.is_null() {
            safe_strcpy(
                &raw mut (*stream).detection_object_filter as *mut ::core::ffi::c_char,
                detection_object_filter,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*stream).detection_object_filter as *mut ::core::ffi::c_char,
                b"none\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        let mut detection_object_filter_list: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_OBJECT_FILTER_LIST.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_object_filter_list.is_null() {
            safe_strcpy(
                &raw mut (*stream).detection_object_filter_list as *mut ::core::ffi::c_char,
                detection_object_filter_list,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
        (*stream).protocol = stream_protocol_t(
            if sqlite3_column_type(stmt, C2Rust_Unnamed::COL_PROTOCOL.0 as ::core::ffi::c_int)
                != SQLITE_NULL
            {
                stream_protocol_t(sqlite3_column_int(
                    stmt,
                    C2Rust_Unnamed::COL_PROTOCOL.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint)
                .0
            } else {
                stream_protocol_t::STREAM_PROTOCOL_TCP.0
            },
        );
        (*stream).is_onvif =
            sqlite3_column_int(stmt, C2Rust_Unnamed::COL_IS_ONVIF.0 as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int;
        (*stream).record_audio = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_RECORD_AUDIO.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        (*stream).backchannel_enabled = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_BACKCHANNEL_ENABLED.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        (*stream).retention_days = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_RETENTION_DAYS.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed::COL_RETENTION_DAYS.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*stream).detection_retention_days = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_RETENTION_DAYS.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed::COL_DETECTION_RETENTION_DAYS.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*stream).max_storage_mb = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_MAX_STORAGE_MB.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed::COL_MAX_STORAGE_MB.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*stream).tier_critical_multiplier = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_TIER_CRITICAL_MULTIPLIER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_double(
                stmt,
                C2Rust_Unnamed::COL_TIER_CRITICAL_MULTIPLIER.0 as ::core::ffi::c_int,
            )
        } else {
            3.0f64
        };
        (*stream).tier_important_multiplier = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_TIER_IMPORTANT_MULTIPLIER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_double(
                stmt,
                C2Rust_Unnamed::COL_TIER_IMPORTANT_MULTIPLIER.0 as ::core::ffi::c_int,
            )
        } else {
            2.0f64
        };
        (*stream).tier_ephemeral_multiplier = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_TIER_EPHEMERAL_MULTIPLIER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_double(
                stmt,
                C2Rust_Unnamed::COL_TIER_EPHEMERAL_MULTIPLIER.0 as ::core::ffi::c_int,
            )
        } else {
            0.25f64
        };
        (*stream).storage_priority = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed::COL_STORAGE_PRIORITY.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed::COL_STORAGE_PRIORITY.0 as ::core::ffi::c_int,
            )
        } else {
            5 as ::core::ffi::c_int
        };
        (*stream).ptz_enabled = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_PTZ_ENABLED.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        (*stream).ptz_max_x =
            if sqlite3_column_type(stmt, C2Rust_Unnamed::COL_PTZ_MAX_X.0 as ::core::ffi::c_int)
                != SQLITE_NULL
            {
                sqlite3_column_int(stmt, C2Rust_Unnamed::COL_PTZ_MAX_X.0 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            };
        (*stream).ptz_max_y =
            if sqlite3_column_type(stmt, C2Rust_Unnamed::COL_PTZ_MAX_Y.0 as ::core::ffi::c_int)
                != SQLITE_NULL
            {
                sqlite3_column_int(stmt, C2Rust_Unnamed::COL_PTZ_MAX_Y.0 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            };
        (*stream).ptz_max_z =
            if sqlite3_column_type(stmt, C2Rust_Unnamed::COL_PTZ_MAX_Z.0 as ::core::ffi::c_int)
                != SQLITE_NULL
            {
                sqlite3_column_int(stmt, C2Rust_Unnamed::COL_PTZ_MAX_Z.0 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            };
        (*stream).ptz_has_home = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_PTZ_HAS_HOME.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut onvif_username: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_ONVIF_USERNAME.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !onvif_username.is_null() {
            safe_strcpy(
                &raw mut (*stream).onvif_username as *mut ::core::ffi::c_char,
                onvif_username,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        let mut onvif_password: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_ONVIF_PASSWORD.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !onvif_password.is_null() {
            safe_strcpy(
                &raw mut (*stream).onvif_password as *mut ::core::ffi::c_char,
                onvif_password,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        let mut onvif_profile: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_ONVIF_PROFILE.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !onvif_profile.is_null() {
            safe_strcpy(
                &raw mut (*stream).onvif_profile as *mut ::core::ffi::c_char,
                onvif_profile,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        (*stream).onvif_port =
            if sqlite3_column_type(stmt, C2Rust_Unnamed::COL_ONVIF_PORT.0 as ::core::ffi::c_int)
                != SQLITE_NULL
            {
                sqlite3_column_int(stmt, C2Rust_Unnamed::COL_ONVIF_PORT.0 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            };
        (*stream).record_on_schedule = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_RECORD_ON_SCHEDULE.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut recording_schedule_text: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_RECORDING_SCHEDULE.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        deserialize_recording_schedule(
            recording_schedule_text,
            &raw mut (*stream).recording_schedule as *mut uint8_t,
        );
        let mut tags_val: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed::COL_TAGS.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !tags_val.is_null() {
            safe_strcpy(
                &raw mut (*stream).tags as *mut ::core::ffi::c_char,
                tags_val,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else {
            (*stream).tags[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut admin_url: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed::COL_ADMIN_URL.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !admin_url.is_null() {
            safe_strcpy(
                &raw mut (*stream).admin_url as *mut ::core::ffi::c_char,
                admin_url,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*stream).admin_url[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*stream).privacy_mode = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_PRIVACY_MODE.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut motion_trigger_source: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_MOTION_TRIGGER_SOURCE.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !motion_trigger_source.is_null() {
            safe_strcpy(
                &raw mut (*stream).motion_trigger_source as *mut ::core::ffi::c_char,
                motion_trigger_source,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else {
            (*stream).motion_trigger_source[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut go2rtc_source_override: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_GO2RTC_SOURCE_OVERRIDE.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if string_has_non_whitespace(go2rtc_source_override) {
            safe_strcpy(
                &raw mut (*stream).go2rtc_source_override as *mut ::core::ffi::c_char,
                go2rtc_source_override,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                0 as size_t,
            );
        } else {
            (*stream).go2rtc_source_override[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut sub_stream_url_val: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_SUB_STREAM_URL.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !sub_stream_url_val.is_null() {
            safe_strcpy(
                &raw mut (*stream).sub_stream_url as *mut ::core::ffi::c_char,
                sub_stream_url_val,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*stream).sub_stream_url[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*stream).audio_voice_enhancement = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_AUDIO_VOICE_ENHANCEMENT.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut detection_url_val: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_URL.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_url_val.is_null() {
            safe_strcpy(
                &raw mut (*stream).detection_url as *mut ::core::ffi::c_char,
                detection_url_val,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*stream).detection_url[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut publish_url_val: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_PUBLISH_URL.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !publish_url_val.is_null() {
            safe_strcpy(
                &raw mut (*stream).publish_url as *mut ::core::ffi::c_char,
                publish_url_val,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*stream).publish_url[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*stream).detection_record_on_schedule = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_RECORD_ON_SCHEDULE.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut detection_schedule_text: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_DETECTION_RECORDING_SCHEDULE.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        deserialize_recording_schedule(
            detection_schedule_text,
            &raw mut (*stream).detection_recording_schedule as *mut uint8_t,
        );
        let mut playback_transport: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_PLAYBACK_TRANSPORT.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*stream).playback_transport as *mut ::core::ffi::c_char,
            normalized_playback_transport(playback_transport),
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
        let mut eptz_config: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed::COL_EPTZ_CONFIG.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*stream).eptz_config as *mut ::core::ffi::c_char,
            if !eptz_config.is_null() {
                eptz_config
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            0 as size_t,
        );
        load_observation_settings(
            stmt,
            C2Rust_Unnamed::COL_OBSERVATION_EVIDENCE_ENABLED.0 as ::core::ffi::c_int,
            stream,
        );
        result = 0 as ::core::ffi::c_int;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    pthread_mutex_unlock(db_mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_config_by_uuid(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut stream: *mut stream_config_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut stream_name: [::core::ffi::c_char; 256] = [
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
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if camera_uuid.is_null()
        || strlen(camera_uuid) != (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
        || stream.is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Valid camera UUID and stream configuration pointer are required\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT name FROM streams WHERE camera_uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            camera_uuid,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if sqlite3_step(stmt) == SQLITE_ROW {
            let mut name: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if !name.is_null() {
                safe_strcpy(
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    name,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
            }
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare camera UUID lookup: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(db_mutex);
    if stream_name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    return get_stream_config_by_name(&raw mut stream_name as *mut ::core::ffi::c_char, stream);
}
unsafe extern "C" fn get_all_stream_configs_on_connection(
    mut db: *mut sqlite3,
    mut streams: *mut stream_config_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if streams.is_null() || max_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_all_stream_configs\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, detection_based_recording, detection_model, detection_threshold, detection_interval, pre_detection_buffer, post_detection_buffer, detection_api_url, detection_object_filter, detection_object_filter_list, protocol, is_onvif, record_audio, backchannel_enabled, retention_days, detection_retention_days, max_storage_mb, tier_critical_multiplier, tier_important_multiplier, tier_ephemeral_multiplier, storage_priority, ptz_enabled, ptz_max_x, ptz_max_y, ptz_max_z, ptz_has_home, onvif_username, onvif_password, onvif_profile, onvif_port, record_on_schedule, recording_schedule, tags, admin_url, privacy_mode, motion_trigger_source, go2rtc_source_override, sub_stream_url, audio_voice_enhancement, detection_url, publish_url, detection_record_on_schedule, detection_recording_schedule, camera_uuid, location_uuid, playback_transport, eptz_config, observation_evidence_enabled, observation_labels, observation_max_age_days, observation_max_bytes FROM streams ORDER BY name;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    while count < max_count && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        let mut s: *mut stream_config_t = streams.offset(count as isize);
        memset(
            s as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stream_config_t>(),
        );
        let mut camera_uuid: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_CAMERA_UUID.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !camera_uuid.is_null() {
            safe_strcpy(
                &raw mut (*s).camera_uuid as *mut ::core::ffi::c_char,
                camera_uuid,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
        let mut location_uuid: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_LOCATION_UUID.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !location_uuid.is_null() {
            safe_strcpy(
                &raw mut (*s).location_uuid as *mut ::core::ffi::c_char,
                location_uuid,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
        let mut name: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed_0::COL_NAME.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !name.is_null() {
            safe_strcpy(
                &raw mut (*s).name as *mut ::core::ffi::c_char,
                name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
        }
        let mut url: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed_0::COL_URL.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !url.is_null() {
            safe_strcpy(
                &raw mut (*s).url as *mut ::core::ffi::c_char,
                url,
                MAX_URL_LENGTH as size_t,
                0 as size_t,
            );
        }
        (*s).enabled =
            sqlite3_column_int(stmt, C2Rust_Unnamed_0::COL_ENABLED.0 as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int;
        (*s).streaming_enabled = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_STREAMING_ENABLED.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        (*s).width = sqlite3_column_int(stmt, C2Rust_Unnamed_0::COL_WIDTH.0 as ::core::ffi::c_int);
        (*s).height =
            sqlite3_column_int(stmt, C2Rust_Unnamed_0::COL_HEIGHT.0 as ::core::ffi::c_int);
        (*s).fps = sqlite3_column_int(stmt, C2Rust_Unnamed_0::COL_FPS.0 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed_0::COL_CODEC.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*s).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*s).priority =
            sqlite3_column_int(stmt, C2Rust_Unnamed_0::COL_PRIORITY.0 as ::core::ffi::c_int);
        (*s).record =
            sqlite3_column_int(stmt, C2Rust_Unnamed_0::COL_RECORD.0 as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int;
        (*s).segment_duration = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_SEGMENT_DURATION.0 as ::core::ffi::c_int,
        );
        (*s).detection_based_recording = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_BASED_RECORDING.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut detection_model: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_MODEL.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_model.is_null() {
            safe_strcpy(
                &raw mut (*s).detection_model as *mut ::core::ffi::c_char,
                detection_model,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        }
        (*s).detection_threshold = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_THRESHOLD.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_double(
                stmt,
                C2Rust_Unnamed_0::COL_DETECTION_THRESHOLD.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_float
        } else {
            0.5f32
        };
        (*s).detection_interval = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_INTERVAL.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_DETECTION_INTERVAL.0 as ::core::ffi::c_int,
            )
        } else {
            10 as ::core::ffi::c_int
        };
        (*s).pre_detection_buffer = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_PRE_DETECTION_BUFFER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_PRE_DETECTION_BUFFER.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*s).post_detection_buffer = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_POST_DETECTION_BUFFER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_POST_DETECTION_BUFFER.0 as ::core::ffi::c_int,
            )
        } else {
            3 as ::core::ffi::c_int
        };
        let mut detection_api_url: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_API_URL.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_api_url.is_null() {
            safe_strcpy(
                &raw mut (*s).detection_api_url as *mut ::core::ffi::c_char,
                detection_api_url,
                MAX_URL_LENGTH as size_t,
                0 as size_t,
            );
        }
        let mut detection_object_filter: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_OBJECT_FILTER.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_object_filter.is_null() {
            safe_strcpy(
                &raw mut (*s).detection_object_filter as *mut ::core::ffi::c_char,
                detection_object_filter,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*s).detection_object_filter as *mut ::core::ffi::c_char,
                b"none\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        let mut detection_object_filter_list: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_OBJECT_FILTER_LIST.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !detection_object_filter_list.is_null() {
            safe_strcpy(
                &raw mut (*s).detection_object_filter_list as *mut ::core::ffi::c_char,
                detection_object_filter_list,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
        (*s).protocol = stream_protocol_t(
            if sqlite3_column_type(stmt, C2Rust_Unnamed_0::COL_PROTOCOL.0 as ::core::ffi::c_int)
                != SQLITE_NULL
            {
                stream_protocol_t(sqlite3_column_int(
                    stmt,
                    C2Rust_Unnamed_0::COL_PROTOCOL.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint)
                .0
            } else {
                stream_protocol_t::STREAM_PROTOCOL_TCP.0
            },
        );
        (*s).is_onvif =
            sqlite3_column_int(stmt, C2Rust_Unnamed_0::COL_IS_ONVIF.0 as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int;
        (*s).record_audio = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_RECORD_AUDIO.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        (*s).backchannel_enabled = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_BACKCHANNEL_ENABLED.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        (*s).retention_days = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_RETENTION_DAYS.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_RETENTION_DAYS.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*s).detection_retention_days = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_RETENTION_DAYS.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_DETECTION_RETENTION_DAYS.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*s).max_storage_mb = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_MAX_STORAGE_MB.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_MAX_STORAGE_MB.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*s).tier_critical_multiplier = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_TIER_CRITICAL_MULTIPLIER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_double(
                stmt,
                C2Rust_Unnamed_0::COL_TIER_CRITICAL_MULTIPLIER.0 as ::core::ffi::c_int,
            )
        } else {
            3.0f64
        };
        (*s).tier_important_multiplier = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_TIER_IMPORTANT_MULTIPLIER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_double(
                stmt,
                C2Rust_Unnamed_0::COL_TIER_IMPORTANT_MULTIPLIER.0 as ::core::ffi::c_int,
            )
        } else {
            2.0f64
        };
        (*s).tier_ephemeral_multiplier = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_TIER_EPHEMERAL_MULTIPLIER.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_double(
                stmt,
                C2Rust_Unnamed_0::COL_TIER_EPHEMERAL_MULTIPLIER.0 as ::core::ffi::c_int,
            )
        } else {
            0.25f64
        };
        (*s).storage_priority = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_STORAGE_PRIORITY.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_STORAGE_PRIORITY.0 as ::core::ffi::c_int,
            )
        } else {
            5 as ::core::ffi::c_int
        };
        (*s).ptz_enabled = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_PTZ_ENABLED.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        (*s).ptz_max_x = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_PTZ_MAX_X.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_PTZ_MAX_X.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*s).ptz_max_y = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_PTZ_MAX_Y.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_PTZ_MAX_Y.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*s).ptz_max_z = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_PTZ_MAX_Z.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_PTZ_MAX_Z.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*s).ptz_has_home = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_PTZ_HAS_HOME.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut onvif_username: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_ONVIF_USERNAME.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !onvif_username.is_null() {
            safe_strcpy(
                &raw mut (*s).onvif_username as *mut ::core::ffi::c_char,
                onvif_username,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        let mut onvif_password: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_ONVIF_PASSWORD.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !onvif_password.is_null() {
            safe_strcpy(
                &raw mut (*s).onvif_password as *mut ::core::ffi::c_char,
                onvif_password,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        let mut onvif_profile: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_ONVIF_PROFILE.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !onvif_profile.is_null() {
            safe_strcpy(
                &raw mut (*s).onvif_profile as *mut ::core::ffi::c_char,
                onvif_profile,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        (*s).onvif_port = if sqlite3_column_type(
            stmt,
            C2Rust_Unnamed_0::COL_ONVIF_PORT.0 as ::core::ffi::c_int,
        ) != SQLITE_NULL
        {
            sqlite3_column_int(
                stmt,
                C2Rust_Unnamed_0::COL_ONVIF_PORT.0 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        (*s).record_on_schedule = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_RECORD_ON_SCHEDULE.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut recording_schedule_text: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_RECORDING_SCHEDULE.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        deserialize_recording_schedule(
            recording_schedule_text,
            &raw mut (*s).recording_schedule as *mut uint8_t,
        );
        let mut tags_val: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, C2Rust_Unnamed_0::COL_TAGS.0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !tags_val.is_null() {
            safe_strcpy(
                &raw mut (*s).tags as *mut ::core::ffi::c_char,
                tags_val,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else {
            (*s).tags[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut admin_url: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_ADMIN_URL.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !admin_url.is_null() {
            safe_strcpy(
                &raw mut (*s).admin_url as *mut ::core::ffi::c_char,
                admin_url,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*s).admin_url[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*s).privacy_mode = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_PRIVACY_MODE.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut motion_trigger_src: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_MOTION_TRIGGER_SOURCE.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if !motion_trigger_src.is_null() {
            safe_strcpy(
                &raw mut (*s).motion_trigger_source as *mut ::core::ffi::c_char,
                motion_trigger_src,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else {
            (*s).motion_trigger_source[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut go2rtc_src_override: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_GO2RTC_SOURCE_OVERRIDE.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        if string_has_non_whitespace(go2rtc_src_override) {
            safe_strcpy(
                &raw mut (*s).go2rtc_source_override as *mut ::core::ffi::c_char,
                go2rtc_src_override,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                0 as size_t,
            );
        } else {
            (*s).go2rtc_source_override[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut sub_url: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_SUB_STREAM_URL.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !sub_url.is_null() {
            safe_strcpy(
                &raw mut (*s).sub_stream_url as *mut ::core::ffi::c_char,
                sub_url,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*s).sub_stream_url[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*s).audio_voice_enhancement = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_AUDIO_VOICE_ENHANCEMENT.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut det_url: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_URL.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !det_url.is_null() {
            safe_strcpy(
                &raw mut (*s).detection_url as *mut ::core::ffi::c_char,
                det_url,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*s).detection_url[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut pub_url: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_PUBLISH_URL.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        if !pub_url.is_null() {
            safe_strcpy(
                &raw mut (*s).publish_url as *mut ::core::ffi::c_char,
                pub_url,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*s).publish_url[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*s).detection_record_on_schedule = sqlite3_column_int(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_RECORD_ON_SCHEDULE.0 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int;
        let mut detection_schedule_text: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_DETECTION_RECORDING_SCHEDULE.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        deserialize_recording_schedule(
            detection_schedule_text,
            &raw mut (*s).detection_recording_schedule as *mut uint8_t,
        );
        let mut playback_transport: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_PLAYBACK_TRANSPORT.0 as ::core::ffi::c_int,
        )
            as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*s).playback_transport as *mut ::core::ffi::c_char,
            normalized_playback_transport(playback_transport),
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
        let mut eptz_config: *const ::core::ffi::c_char = sqlite3_column_text(
            stmt,
            C2Rust_Unnamed_0::COL_EPTZ_CONFIG.0 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*s).eptz_config as *mut ::core::ffi::c_char,
            if !eptz_config.is_null() {
                eptz_config
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            0 as size_t,
        );
        load_observation_settings(
            stmt,
            C2Rust_Unnamed_0::COL_OBSERVATION_EVIDENCE_ENABLED.0 as ::core::ffi::c_int,
            s,
        );
        count += 1;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if rc != SQLITE_DONE && count < max_count {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read stream configurations: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_all_stream_configs(
    mut streams: *mut stream_config_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    pthread_mutex_lock(mutex);
    let mut count: ::core::ffi::c_int =
        get_all_stream_configs_on_connection(get_db_handle(), streams, max_count);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_all_stream_configs_readonly(
    mut streams: *mut stream_config_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if streams.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut connection: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut connection) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_busy_timeout(connection, 1000 as ::core::ffi::c_int);
    let mut count: ::core::ffi::c_int =
        get_all_stream_configs_on_connection(connection, streams, max_count);
    db_close_readonly_connection(connection);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_stream_eligible_for_live_streaming(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT enabled, streaming_enabled, privacy_mode FROM streams WHERE name = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) == SQLITE_ROW {
        let mut enabled: bool =
            sqlite3_column_int(stmt, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut streaming_enabled: bool =
            sqlite3_column_int(stmt, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut privacy_mode: bool =
            sqlite3_column_int(stmt, 2 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        result = if enabled as ::core::ffi::c_int != 0
            && streaming_enabled as ::core::ffi::c_int != 0
            && !privacy_mode
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        if !enabled {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s is not eligible for live streaming: not enabled\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        } else if !streaming_enabled {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s is not eligible for live streaming: streaming not enabled\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        } else if privacy_mode {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s is not eligible for live streaming: privacy mode active\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        result = 0 as ::core::ffi::c_int;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    pthread_mutex_unlock(db_mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_enabled_stream_count() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT COUNT(*) FROM streams WHERE enabled = 1;\0".as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_stream_configs() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT COUNT(*) FROM streams;\0".as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_retention_config(
    mut stream_name: *const ::core::ffi::c_char,
    mut config: *mut stream_retention_config_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if stream_name.is_null() || config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name and config pointer are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    (*config).retention_days = -1 as ::core::ffi::c_int;
    (*config).detection_retention_days = -1 as ::core::ffi::c_int;
    (*config).max_storage_mb = 0 as uint64_t;
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT retention_days, detection_retention_days, max_storage_mb FROM streams WHERE name = ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) == SQLITE_ROW {
        if sqlite3_column_type(stmt, 0 as ::core::ffi::c_int) != SQLITE_NULL {
            (*config).retention_days = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
        }
        if sqlite3_column_type(stmt, 1 as ::core::ffi::c_int) != SQLITE_NULL {
            (*config).detection_retention_days = sqlite3_column_int(stmt, 1 as ::core::ffi::c_int);
        }
        if sqlite3_column_type(stmt, 2 as ::core::ffi::c_int) != SQLITE_NULL {
            (*config).max_storage_mb =
                sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as uint64_t;
        }
        result = 0 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_stream_retention_config(
    mut stream_name: *const ::core::ffi::c_char,
    mut config: *const stream_retention_config_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if stream_name.is_null() || config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name and config are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"UPDATE streams SET retention_days = ?, detection_retention_days = ?, max_storage_mb = ? WHERE name = ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, (*config).retention_days);
    sqlite3_bind_int(
        stmt,
        2 as ::core::ffi::c_int,
        (*config).detection_retention_days,
    );
    sqlite3_bind_int64(
        stmt,
        3 as ::core::ffi::c_int,
        (*config).max_storage_mb as sqlite3_int64,
    );
    sqlite3_bind_text(
        stmt,
        4 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update stream retention config: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Updated retention config for stream %s: retention_days=%d, detection_retention_days=%d, max_storage_mb=%lu\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        (*config).retention_days,
        (*config).detection_retention_days,
        (*config).max_storage_mb as ::core::ffi::c_ulong,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn collect_stream_names(
    mut sql: *const ::core::ffi::c_char,
    mut caller: *const ::core::ffi::c_char,
    mut names: *mut [::core::ffi::c_char; 256],
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if names.is_null() || max_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for %s\0".as_ptr() as *const ::core::ffi::c_char,
            caller,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement for %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            caller,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_count {
        let mut name: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !name.is_null() {
            safe_strcpy(
                &raw mut *names.offset(count as isize) as *mut ::core::ffi::c_char,
                name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            count += 1;
        }
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_all_stream_names(
    mut names: *mut [::core::ffi::c_char; 256],
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return collect_stream_names(
        b"SELECT name FROM streams WHERE enabled = 1 ORDER BY name;\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"get_all_stream_names\0".as_ptr() as *const ::core::ffi::c_char,
        names,
        max_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_all_stream_names_including_disabled(
    mut names: *mut [::core::ffi::c_char; 256],
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return collect_stream_names(
        b"SELECT name FROM streams ORDER BY enabled DESC, name;\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"get_all_stream_names_including_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        names,
        max_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_orphaned_recording_stream_names(
    mut names: *mut [::core::ffi::c_char; 256],
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return collect_stream_names(
        b"SELECT stream_name FROM (SELECT DISTINCT stream_name FROM recordings) r WHERE stream_name IS NOT NULL AND stream_name != '' AND NOT EXISTS (SELECT 1 FROM streams s WHERE s.name = r.stream_name) ORDER BY stream_name;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"get_orphaned_recording_stream_names\0".as_ptr() as *const ::core::ffi::c_char,
        names,
        max_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_storage_usage_db(
    mut stream_name: *const ::core::ffi::c_char,
) -> uint64_t {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut size_bytes: uint64_t = 0 as uint64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT COALESCE(SUM(size_bytes), 0) FROM recordings WHERE stream_name = ? AND is_complete = 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return 0 as uint64_t;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) == SQLITE_ROW {
        size_bytes = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return size_bytes;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
