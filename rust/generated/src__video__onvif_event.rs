unsafe extern "C" {
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strptime(
        __s: *const ::core::ffi::c_char,
        __fmt: *const ::core::ffi::c_char,
        __tp: *mut tm,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn timegm(__tp: *mut tm) -> time_t;
    unsafe fn ezxml_parse_str(s: *mut ::core::ffi::c_char, len: size_t) -> ezxml_t;
    unsafe fn ezxml_attr(
        xml: ezxml_t,
        attr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    unsafe fn ezxml_free(xml: ezxml_t);
    unsafe fn ezxml_error(xml: ezxml_t) -> *const ::core::ffi::c_char;
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
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    unsafe fn strtof(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_float;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct onvif_lpr_source_t(pub ::core::ffi::c_uint);
impl onvif_lpr_source_t {
    pub const ONVIF_LPR_SOURCE_PROFILE_M: Self = Self(0);
    pub const ONVIF_LPR_SOURCE_VENDOR_TOPIC: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_lpr_event_t {
    pub source: onvif_lpr_source_t,
    pub asserted: bool,
    pub observed_at_ms: int64_t,
    pub topic: [::core::ffi::c_char; 256],
    pub plate: [::core::ffi::c_char; 64],
    pub has_confidence: bool,
    pub confidence: ::core::ffi::c_float,
    pub country: [::core::ffi::c_char; 64],
    pub region: [::core::ffi::c_char; 64],
    pub plate_type: [::core::ffi::c_char; 64],
    pub direction: [::core::ffi::c_char; 64],
    pub lane: [::core::ffi::c_char; 64],
    pub vehicle_type: [::core::ffi::c_char; 64],
    pub vehicle_color: [::core::ffi::c_char; 64],
    pub object_id: [::core::ffi::c_char; 96],
    pub correlation_id: [::core::ffi::c_char; 96],
    pub has_bounding_box: bool,
    pub bbox_left: ::core::ffi::c_float,
    pub bbox_top: ::core::ffi::c_float,
    pub bbox_right: ::core::ffi::c_float,
    pub bbox_bottom: ::core::ffi::c_float,
}
pub type ezxml_t = *mut ezxml;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml {
    pub name: *mut ::core::ffi::c_char,
    pub attr: *mut *mut ::core::ffi::c_char,
    pub txt: *mut ::core::ffi::c_char,
    pub off: size_t,
    pub next: ezxml_t,
    pub sibling: ezxml_t,
    pub ordered: ezxml_t,
    pub child: ezxml_t,
    pub parent: ezxml_t,
    pub flags: ::core::ffi::c_short,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ONVIF_EVENT_XML_MAX_BYTES: ::core::ffi::c_uint = (4 as ::core::ffi::c_uint)
    .wrapping_mul(1024 as ::core::ffi::c_uint)
    .wrapping_mul(1024 as ::core::ffi::c_uint);
unsafe extern "C" fn local_name(mut node: ezxml_t) -> *const ::core::ffi::c_char {
    let mut name: *const ::core::ffi::c_char = if !node.is_null() && !(*node).name.is_null() {
        (*node).name
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut colon: *const ::core::ffi::c_char = strrchr(name, ':' as ::core::ffi::c_int);
    return if !colon.is_null() {
        colon.offset(1 as ::core::ffi::c_int as isize)
    } else {
        name
    };
}
unsafe extern "C" fn ci_contains(
    mut haystack: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    if haystack.is_null() || needle.is_null() || *needle.offset(0isize) == 0 {
        return r#false != 0;
    }
    let mut needle_len: size_t = strlen(needle);
    let mut p: *const ::core::ffi::c_char = haystack;
    while *p != 0 {
        if strncasecmp(p, needle, needle_len) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        p = p.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn copy_text(
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut src: *const ::core::ffi::c_char,
) {
    if dst.is_null()
        || size == 0 as size_t
        || src.is_null()
        || *dst.offset(0isize) as ::core::ffi::c_int != 0
    {
        return;
    }
    while *src as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*src as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        src = src.offset(1);
    }
    let mut length: size_t = strlen(src);
    while length > 0 as size_t
        && *(*__ctype_b_loc()).offset(*src.offset(length.wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_uchar as ::core::ffi::c_int
            as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        length = length.wrapping_sub(1);
    }
    if length >= size {
        length = size.wrapping_sub(1 as size_t);
    }
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        length,
    );
    *dst.offset(length as isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn copy_plate_text(
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut src: *const ::core::ffi::c_char,
) {
    if dst.is_null()
        || size == 0 as size_t
        || src.is_null()
        || *dst.offset(0isize) as ::core::ffi::c_int != 0
    {
        return;
    }
    while *src as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*src as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        src = src.offset(1);
    }
    let mut length: size_t = strlen(src);
    while length > 0 as size_t
        && *(*__ctype_b_loc()).offset(*src.offset(length.wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_uchar as ::core::ffi::c_int
            as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        length = length.wrapping_sub(1);
    }
    if length == 0 as size_t || length >= size {
        return;
    }
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        length,
    );
    *dst.offset(length as isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn name_is(
    mut actual: *const ::core::ffi::c_char,
    mut names: *const *const ::core::ffi::c_char,
    mut count: size_t,
) -> bool {
    if actual.is_null() {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < count {
        if strcasecmp(actual, *names.offset(i as isize)) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn topic_is_standard_lpr(mut topic: *const ::core::ffi::c_char) -> bool {
    return ci_contains(
        topic,
        b"RuleEngine/Recognition/LicensePlate\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn topic_is_vendor_lpr(mut topic: *const ::core::ffi::c_char) -> bool {
    if topic.is_null() {
        return r#false != 0;
    }
    if ci_contains(
        topic,
        b"LicensePlate\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || ci_contains(topic, b"ANPR\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || ci_contains(topic, b"ALPR\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
    {
        return r#true != 0;
    }
    let mut p: *const ::core::ffi::c_char = topic;
    while *p != 0 {
        if strncasecmp(
            p,
            b"LPR\0".as_ptr() as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            let mut left: bool = p == topic
                || *(*__ctype_b_loc()).offset(
                    *p.offset(-1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
                ) as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    == 0;
            let mut right: bool = *(*__ctype_b_loc())
                .offset(*p.offset(3isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0;
            if left as ::core::ffi::c_int != 0 && right as ::core::ffi::c_int != 0 {
                return r#true != 0;
            }
        }
        p = p.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn first_descendant(
    mut node: ezxml_t,
    mut wanted: *const ::core::ffi::c_char,
) -> ezxml_t {
    let mut child: ezxml_t = if !node.is_null() {
        (*node).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !child.is_null() {
        if strcasecmp(local_name(child), wanted) == 0 as ::core::ffi::c_int {
            return child;
        }
        let mut found: ezxml_t = first_descendant(child, wanted);
        if !found.is_null() {
            return found;
        }
        child = (*child).ordered;
    }
    return ::core::ptr::null_mut::<ezxml>();
}
unsafe extern "C" fn first_descendant_with_attr(
    mut node: ezxml_t,
    mut wanted: *const ::core::ffi::c_char,
    mut attribute: *const ::core::ffi::c_char,
) -> ezxml_t {
    let mut child: ezxml_t = if !node.is_null() {
        (*node).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !child.is_null() {
        if strcasecmp(local_name(child), wanted) == 0 as ::core::ffi::c_int
            && !ezxml_attr(child, attribute).is_null()
        {
            return child;
        }
        let mut found: ezxml_t = first_descendant_with_attr(child, wanted, attribute);
        if !found.is_null() {
            return found;
        }
        child = (*child).ordered;
    }
    return ::core::ptr::null_mut::<ezxml>();
}
unsafe extern "C" fn normalized_confidence(
    mut value: *const ::core::ffi::c_char,
    mut valid: *mut bool,
) -> ::core::ffi::c_float {
    if !valid.is_null() {
        *valid = r#false != 0;
    }
    if value.is_null() || *value.offset(0isize) == 0 {
        return 0.0f32;
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_float = strtof(value, &raw mut end);
    if *__errno_location() != 0 || end == value as *mut ::core::ffi::c_char {
        return 0.0f32;
    }
    while *end as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        end = end.offset(1);
    }
    if *end as ::core::ffi::c_int != 0 || result < 0.0f32 {
        return 0.0f32;
    }
    if result > 1.0f32 && result <= 100.0f32 {
        result /= 100.0f32;
    }
    if result > 1.0f32 {
        return 0.0f32;
    }
    if !valid.is_null() {
        *valid = r#true != 0;
    }
    return result;
}
unsafe extern "C" fn parse_utc_ms(mut value: *const ::core::ffi::c_char) -> int64_t {
    if value.is_null() || strlen(value) < 20 as size_t {
        return 0 as int64_t;
    }
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
    memset(
        &raw mut utc as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tm>(),
    );
    let mut end: *mut ::core::ffi::c_char = strptime(
        value,
        b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut utc,
    );
    if end.is_null() {
        return 0 as int64_t;
    }
    let mut milliseconds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *end as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
        end = end.offset(1);
        let mut digits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while *(*__ctype_b_loc())
            .offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            if digits < 3 as ::core::ffi::c_int {
                milliseconds = milliseconds * 10 as ::core::ffi::c_int
                    + (*end as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
            }
            digits += 1;
            end = end.offset(1);
        }
        loop {
            let c2rust_fresh2 = digits;
            digits += 1;
            if c2rust_fresh2 >= 3 as ::core::ffi::c_int {
                break;
            }
            milliseconds *= 10 as ::core::ffi::c_int;
        }
    }
    if *end as ::core::ffi::c_int != 'Z' as ::core::ffi::c_int
        && *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        return 0 as int64_t;
    }
    let mut seconds: time_t = timegm(&raw mut utc);
    return if seconds < 0 as time_t {
        0 as int64_t
    } else {
        seconds as int64_t * 1000 as int64_t + milliseconds as int64_t
    };
}
unsafe extern "C" fn set_named_value(
    mut event: *mut onvif_lpr_event_t,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    static mut plate_names: [*const ::core::ffi::c_char; 5] = [
        b"PlateNumber\0".as_ptr() as *const ::core::ffi::c_char,
        b"LicensePlate\0".as_ptr() as *const ::core::ffi::c_char,
        b"Plate\0".as_ptr() as *const ::core::ffi::c_char,
        b"PlateNo\0".as_ptr() as *const ::core::ffi::c_char,
        b"PlateText\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut confidence_names: [*const ::core::ffi::c_char; 4] = [
        b"Likelihood\0".as_ptr() as *const ::core::ffi::c_char,
        b"Confidence\0".as_ptr() as *const ::core::ffi::c_char,
        b"PlateConfidence\0".as_ptr() as *const ::core::ffi::c_char,
        b"RecognitionConfidence\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut country_names: [*const ::core::ffi::c_char; 2] = [
        b"CountryCode\0".as_ptr() as *const ::core::ffi::c_char,
        b"Country\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut region_names: [*const ::core::ffi::c_char; 3] = [
        b"IssuingEntity\0".as_ptr() as *const ::core::ffi::c_char,
        b"Region\0".as_ptr() as *const ::core::ffi::c_char,
        b"State\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut plate_type_names: [*const ::core::ffi::c_char; 2] = [
        b"PlateType\0".as_ptr() as *const ::core::ffi::c_char,
        b"LicensePlateType\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut direction_names: [*const ::core::ffi::c_char; 2] = [
        b"Direction\0".as_ptr() as *const ::core::ffi::c_char,
        b"MotionDirection\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut lane_names: [*const ::core::ffi::c_char; 3] = [
        b"Lane\0".as_ptr() as *const ::core::ffi::c_char,
        b"LaneId\0".as_ptr() as *const ::core::ffi::c_char,
        b"LaneID\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut vehicle_type_names: [*const ::core::ffi::c_char; 2] = [
        b"VehicleType\0".as_ptr() as *const ::core::ffi::c_char,
        b"VehicleClass\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut vehicle_color_names: [*const ::core::ffi::c_char; 2] = [
        b"VehicleColor\0".as_ptr() as *const ::core::ffi::c_char,
        b"Color\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut object_names: [*const ::core::ffi::c_char; 3] = [
        b"ObjectId\0".as_ptr() as *const ::core::ffi::c_char,
        b"ObjectID\0".as_ptr() as *const ::core::ffi::c_char,
        b"ObjectToken\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut correlation_names: [*const ::core::ffi::c_char; 4] = [
        b"CorrelationId\0".as_ptr() as *const ::core::ffi::c_char,
        b"CorrelationID\0".as_ptr() as *const ::core::ffi::c_char,
        b"FrameId\0".as_ptr() as *const ::core::ffi::c_char,
        b"FrameID\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if name.is_null() || value.is_null() {
        return;
    }
    if name_is(
        name,
        &raw mut plate_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_plate_text(
            &raw mut (*event).plate as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut confidence_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        let mut valid: bool = r#false != 0;
        let mut confidence: ::core::ffi::c_float = normalized_confidence(value, &raw mut valid);
        if valid as ::core::ffi::c_int != 0 && !(*event).has_confidence {
            (*event).confidence = confidence;
            (*event).has_confidence = r#true != 0;
        }
    } else if name_is(
        name,
        &raw mut country_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).country as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut region_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).region as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut plate_type_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).plate_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut direction_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).direction as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut lane_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).lane as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut vehicle_type_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).vehicle_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut vehicle_color_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).vehicle_color as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut object_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).object_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            value,
        );
    } else if name_is(
        name,
        &raw mut correlation_names as *mut *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) {
        copy_text(
            &raw mut (*event).correlation_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            value,
        );
    }
}
unsafe extern "C" fn read_bbox(mut event: *mut onvif_lpr_event_t, mut node: ezxml_t) {
    let mut left: *const ::core::ffi::c_char =
        ezxml_attr(node, b"left\0".as_ptr() as *const ::core::ffi::c_char);
    let mut top: *const ::core::ffi::c_char =
        ezxml_attr(node, b"top\0".as_ptr() as *const ::core::ffi::c_char);
    let mut right: *const ::core::ffi::c_char =
        ezxml_attr(node, b"right\0".as_ptr() as *const ::core::ffi::c_char);
    let mut bottom: *const ::core::ffi::c_char =
        ezxml_attr(node, b"bottom\0".as_ptr() as *const ::core::ffi::c_char);
    if left.is_null() {
        left = ezxml_attr(node, b"Left\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if top.is_null() {
        top = ezxml_attr(node, b"Top\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if right.is_null() {
        right = ezxml_attr(node, b"Right\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if bottom.is_null() {
        bottom = ezxml_attr(node, b"Bottom\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if left.is_null() || top.is_null() || right.is_null() || bottom.is_null() {
        return;
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    (*event).bbox_left = strtof(left, &raw mut end);
    if *__errno_location() != 0
        || end == left as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != 0
        || (*event).bbox_left.is_finite() as ::core::ffi::c_int == 0
    {
        return;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    (*event).bbox_top = strtof(top, &raw mut end);
    if *__errno_location() != 0
        || end == top as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != 0
        || (*event).bbox_top.is_finite() as ::core::ffi::c_int == 0
    {
        return;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    (*event).bbox_right = strtof(right, &raw mut end);
    if *__errno_location() != 0
        || end == right as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != 0
        || (*event).bbox_right.is_finite() as ::core::ffi::c_int == 0
    {
        return;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    (*event).bbox_bottom = strtof(bottom, &raw mut end);
    if *__errno_location() != 0
        || end == bottom as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != 0
        || (*event).bbox_bottom.is_finite() as ::core::ffi::c_int == 0
    {
        return;
    }
    if (*event).bbox_left < -1.0f32
        || (*event).bbox_left > 1.0f32
        || (*event).bbox_top < -1.0f32
        || (*event).bbox_top > 1.0f32
        || (*event).bbox_right < -1.0f32
        || (*event).bbox_right > 1.0f32
        || (*event).bbox_bottom < -1.0f32
        || (*event).bbox_bottom > 1.0f32
        || (*event).bbox_right < (*event).bbox_left
        || (*event).bbox_bottom < (*event).bbox_top
    {
        return;
    }
    (*event).has_bounding_box = r#true != 0;
}
unsafe extern "C" fn collect_values(mut node: ezxml_t, mut event: *mut onvif_lpr_event_t) {
    let mut child: ezxml_t = if !node.is_null() {
        (*node).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !child.is_null() {
        let mut name: *const ::core::ffi::c_char = local_name(child);
        if strcasecmp(name, b"SimpleItem\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            set_named_value(
                event,
                ezxml_attr(child, b"Name\0".as_ptr() as *const ::core::ffi::c_char),
                ezxml_attr(child, b"Value\0".as_ptr() as *const ::core::ffi::c_char),
            );
        } else {
            let mut text: *const ::core::ffi::c_char = if !child.is_null() {
                (*child).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
            if !text.is_null() && *text.offset(0isize) as ::core::ffi::c_int != 0 {
                set_named_value(event, name, text);
            }
            let mut likelihood: *const ::core::ffi::c_char = ezxml_attr(
                child,
                b"Likelihood\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !likelihood.is_null() {
                set_named_value(
                    event,
                    b"Likelihood\0".as_ptr() as *const ::core::ffi::c_char,
                    likelihood,
                );
            }
            if strcasecmp(
                name,
                b"BoundingBox\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcasecmp(name, b"Rectangle\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                read_bbox(event, child);
            }
        }
        collect_values(child, event);
        child = (*child).ordered;
    }
}
unsafe extern "C" fn has_explicit_false_state(mut node: ezxml_t) -> bool {
    let mut child: ezxml_t = if !node.is_null() {
        (*node).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !child.is_null() {
        let mut operation: *const ::core::ffi::c_char = ezxml_attr(
            child,
            b"PropertyOperation\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !operation.is_null()
            && (strcasecmp(
                operation,
                b"Deleted\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcasecmp(
                    operation,
                    b"Removed\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
        {
            return r#true != 0;
        }
        let mut text: *const ::core::ffi::c_char = if !child.is_null() {
            (*child).txt
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        };
        if (strcasecmp(
            local_name(child),
            b"State\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                local_name(child),
                b"IsRecognized\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                local_name(child),
                b"IsLicensePlate\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
            && !text.is_null()
            && (strcasecmp(text, b"false\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(text, b"0\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int)
        {
            return r#true != 0;
        }
        if strcasecmp(
            local_name(child),
            b"SimpleItem\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut name: *const ::core::ffi::c_char =
                ezxml_attr(child, b"Name\0".as_ptr() as *const ::core::ffi::c_char);
            let mut value: *const ::core::ffi::c_char =
                ezxml_attr(child, b"Value\0".as_ptr() as *const ::core::ffi::c_char);
            if !name.is_null()
                && !value.is_null()
                && (strcasecmp(name, b"State\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        name,
                        b"IsRecognized\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        name,
                        b"IsLicensePlate\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int)
                && (strcasecmp(value, b"false\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"0\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int)
            {
                return r#true != 0;
            }
        }
        if has_explicit_false_state(child) {
            return r#true != 0;
        }
        child = (*child).ordered;
    }
    return r#false != 0;
}
unsafe extern "C" fn parse_notification(
    mut notification: ezxml_t,
    mut event: *mut onvif_lpr_event_t,
) -> ::core::ffi::c_int {
    let mut topic_node: ezxml_t = first_descendant(
        notification,
        b"Topic\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut topic: *const ::core::ffi::c_char = if !topic_node.is_null() {
        if !topic_node.is_null() {
            (*topic_node).txt
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        }
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
    let mut standard: bool = topic_is_standard_lpr(topic);
    if !standard && !topic_is_vendor_lpr(topic) {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<onvif_lpr_event_t>(),
    );
    (*event).source = onvif_lpr_source_t(
        (if standard as ::core::ffi::c_int != 0 {
            onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M.0 as ::core::ffi::c_int
        } else {
            onvif_lpr_source_t::ONVIF_LPR_SOURCE_VENDOR_TOPIC.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*event).asserted = !has_explicit_false_state(notification);
    copy_text(
        &raw mut (*event).topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        topic,
    );
    let mut message: ezxml_t = first_descendant_with_attr(
        notification,
        b"Message\0".as_ptr() as *const ::core::ffi::c_char,
        b"UtcTime\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !message.is_null() {
        (*event).observed_at_ms = parse_utc_ms(ezxml_attr(
            message,
            b"UtcTime\0".as_ptr() as *const ::core::ffi::c_char,
        ));
    }
    collect_values(notification, event);
    return if (*event).asserted as ::core::ffi::c_int != 0
        && (*event).plate[0usize] as ::core::ffi::c_int != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn walk_notifications(
    mut node: ezxml_t,
    mut events: *mut onvif_lpr_event_t,
    mut capacity: size_t,
    mut count: *mut size_t,
) -> ::core::ffi::c_int {
    let mut child: ezxml_t = if !node.is_null() {
        (*node).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !child.is_null() {
        if strcasecmp(
            local_name(child),
            b"NotificationMessage\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut parsed: onvif_lpr_event_t = onvif_lpr_event_t {
                source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
                asserted: false,
                observed_at_ms: 0,
                topic: [0; 256],
                plate: [0; 64],
                has_confidence: false,
                confidence: 0.,
                country: [0; 64],
                region: [0; 64],
                plate_type: [0; 64],
                direction: [0; 64],
                lane: [0; 64],
                vehicle_type: [0; 64],
                vehicle_color: [0; 64],
                object_id: [0; 96],
                correlation_id: [0; 96],
                has_bounding_box: false,
                bbox_left: 0.,
                bbox_top: 0.,
                bbox_right: 0.,
                bbox_bottom: 0.,
            };
            if parse_notification(child, &raw mut parsed) == 1 as ::core::ffi::c_int
                && *count < capacity
            {
                let c2rust_fresh1 = *count;
                *count = (*count).wrapping_add(1);
                *events.offset(c2rust_fresh1 as isize) = parsed;
            }
        } else {
            walk_notifications(child, events, capacity, count);
        }
        child = (*child).ordered;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onvif_parse_lpr_events(
    mut xml_text: *const ::core::ffi::c_char,
    mut events: *mut onvif_lpr_event_t,
    mut capacity: size_t,
) -> ::core::ffi::c_int {
    if xml_text.is_null()
        || events.is_null()
        || capacity == 0 as size_t
        || strnlen(
            xml_text,
            ONVIF_EVENT_XML_MAX_BYTES.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        ) > ONVIF_EVENT_XML_MAX_BYTES as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut copy: *mut ::core::ffi::c_char = strdup(xml_text);
    if copy.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut root: ezxml_t = ezxml_parse_str(copy, strlen(copy));
    if root.is_null() {
        free(copy as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if *ezxml_error(root).offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        ezxml_free(root);
        free(copy as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut count: size_t = 0 as size_t;
    if strcasecmp(
        local_name(root),
        b"NotificationMessage\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut parsed: onvif_lpr_event_t = onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        };
        if parse_notification(root, &raw mut parsed) == 1 as ::core::ffi::c_int {
            let c2rust_fresh0 = count;
            count = count.wrapping_add(1);
            *events.offset(c2rust_fresh0 as isize) = parsed;
        }
    } else {
        walk_notifications(root, events, capacity, &raw mut count);
    }
    ezxml_free(root);
    free(copy as *mut ::core::ffi::c_void);
    return count as ::core::ffi::c_int;
}
