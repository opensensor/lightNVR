unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn go2rtc_api_get_stream_activity(
        streams: *mut go2rtc_stream_activity_t,
        stream_count: size_t,
    ) -> bool;
    unsafe fn go2rtc_integration_is_initialized() -> bool;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
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
pub struct go2rtc_stream_activity_t {
    pub stream_name: *const ::core::ffi::c_char,
    pub alternate_stream_name: *const ::core::ffi::c_char,
    pub video_active: bool,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
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
unsafe extern "C" fn activity_matches_camera(
    mut entry: *const go2rtc_stream_activity_t,
    mut camera: *const fleet_camera_t,
) -> bool {
    return !entry.is_null()
        && (*entry).video_active as ::core::ffi::c_int != 0
        && !(*entry).stream_name.is_null()
        && !camera.is_null()
        && strcmp(
            (*entry).stream_name,
            &raw const (*camera).name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_health_apply_go2rtc_activity(
    mut cameras: *mut fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut activity: *const go2rtc_stream_activity_t,
    mut activity_count: size_t,
    mut observed_at: time_t,
) {
    if cameras.is_null() || camera_count <= 0 as ::core::ffi::c_int || activity.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        let mut camera: *mut fleet_camera_t = cameras.offset(i as isize);
        if !(!(*camera).enabled
            || !(*camera).streaming_enabled
            || (*camera).health.0 != fleet_health_state_t::FLEET_HEALTH_UNKNOWN.0)
        {
            let mut active: bool = r#false != 0;
            let mut j: size_t = 0 as size_t;
            while j < activity_count {
                if activity_matches_camera(activity.offset(j as isize), camera) {
                    active = r#true != 0;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
            if active {
                (*camera).health = fleet_health_state_t::FLEET_HEALTH_UP;
                (*camera).availability = fleet_availability_state_t::FLEET_AVAILABILITY_LIVE;
                (*camera).health_changed_at = observed_at as int64_t;
                (*camera).last_frame_ts = observed_at as int64_t;
                if (*camera).first_video_at == 0 as int64_t {
                    (*camera).first_video_at = observed_at as int64_t;
                }
            }
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_health_capture_go2rtc_activity(
    mut cameras: *mut fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if camera_count < 0 as ::core::ffi::c_int
        || cameras.is_null() && camera_count > 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if camera_count == 0 as ::core::ffi::c_int || !go2rtc_integration_is_initialized() {
        return 0 as ::core::ffi::c_int;
    }
    let mut activity: *mut go2rtc_stream_activity_t = calloc(
        camera_count as size_t,
        ::core::mem::size_of::<go2rtc_stream_activity_t>(),
    ) as *mut go2rtc_stream_activity_t;
    let mut substream_names: *mut [::core::ffi::c_char; 261] = calloc(
        camera_count as size_t,
        ::core::mem::size_of::<[::core::ffi::c_char; 261]>(),
    )
        as *mut [::core::ffi::c_char; 261];
    if activity.is_null() || substream_names.is_null() {
        free(activity as *mut ::core::ffi::c_void);
        free(substream_names as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut activity_count: size_t = 0 as size_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        if !(!(*cameras.offset(i as isize)).enabled
            || !(*cameras.offset(i as isize)).streaming_enabled
            || (*cameras.offset(i as isize)).health.0
                != fleet_health_state_t::FLEET_HEALTH_UNKNOWN.0)
        {
            let mut written: ::core::ffi::c_int = snprintf(
                &raw mut *substream_names.offset(activity_count as isize)
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 261]>(),
                b"%s_sub\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*cameras.offset(i as isize)).name as *mut ::core::ffi::c_char,
            );
            (*activity.offset(activity_count as isize)).stream_name =
                &raw mut (*cameras.offset(i as isize)).name as *mut ::core::ffi::c_char;
            if written > 0 as ::core::ffi::c_int
                && (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 261]>()
            {
                (*activity.offset(activity_count as isize)).alternate_stream_name =
                    &raw mut *substream_names.offset(activity_count as isize)
                        as *mut ::core::ffi::c_char;
            }
            activity_count = activity_count.wrapping_add(1);
        }
        i += 1;
    }
    if activity_count > 0 as size_t
        && go2rtc_api_get_stream_activity(activity, activity_count) as ::core::ffi::c_int != 0
    {
        fleet_health_apply_go2rtc_activity(
            cameras,
            camera_count,
            activity,
            activity_count,
            time(::core::ptr::null_mut::<time_t>()),
        );
    }
    free(substream_names as *mut ::core::ffi::c_void);
    free(activity as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_health_enrich_go2rtc_activity(
    mut cameras: *mut fleet_camera_t,
    mut count: ::core::ffi::c_int,
) {
    fleet_health_capture_go2rtc_activity(cameras, count);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
