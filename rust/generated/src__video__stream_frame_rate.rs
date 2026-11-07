pub type int64_t = i64;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_frame_rate_measurement_t {
    pub started: bool,
    pub warmup_until_us: int64_t,
    pub window_start_us: int64_t,
    pub packets: uint64_t,
    pub fps: ::core::ffi::c_int,
}
#[inline]
unsafe extern "C" fn av_q2d(mut a: AVRational) -> ::core::ffi::c_double {
    return a.num as ::core::ffi::c_double / a.den as ::core::ffi::c_double;
}
pub const STREAM_FRAME_RATE_MAX: ::core::ffi::c_double = 240.0f64;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn valid_rate(mut fps: ::core::ffi::c_double) -> bool {
    return fps.is_finite() as ::core::ffi::c_int != 0
        && fps > 0.0f64
        && fps <= STREAM_FRAME_RATE_MAX;
}
unsafe extern "C" fn rational_rate(mut rate: AVRational) -> ::core::ffi::c_double {
    if rate.num <= 0 as ::core::ffi::c_int || rate.den <= 0 as ::core::ffi::c_int {
        return 0.0f64;
    }
    let mut fps: ::core::ffi::c_double = av_q2d(rate);
    return if valid_rate(fps) as ::core::ffi::c_int != 0 {
        fps
    } else {
        0.0f64
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_frame_rate_select(
    mut average: AVRational,
    mut nominal: AVRational,
    mut provisional: *mut bool,
) -> ::core::ffi::c_double {
    let mut fps: ::core::ffi::c_double = rational_rate(average);
    if !provisional.is_null() {
        *provisional = fps == 0.0f64;
    }
    return if fps > 0.0f64 {
        fps
    } else {
        rational_rate(nominal)
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_frame_rate_round(
    mut fps: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if !valid_rate(fps) {
        return 0 as ::core::ffi::c_int;
    }
    let mut rounded: ::core::ffi::c_int = (fps + 0.5f64) as ::core::ffi::c_int;
    return if rounded > 0 as ::core::ffi::c_int {
        rounded
    } else {
        1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stream_frame_rate_observe(
    mut m: *mut stream_frame_rate_measurement_t,
    mut now_us: int64_t,
) -> ::core::ffi::c_int {
    if (*m).fps > 0 as ::core::ffi::c_int {
        return (*m).fps;
    }
    if !(*m).started {
        (*m).started = r#true != 0;
        (*m).warmup_until_us = now_us + 2000000 as int64_t;
    }
    if now_us < (*m).warmup_until_us {
        return 0 as ::core::ffi::c_int;
    }
    let c2rust_fresh0 = (*m).packets;
    (*m).packets = (*m).packets.wrapping_add(1);
    if c2rust_fresh0 == 0 as uint64_t {
        (*m).window_start_us = now_us;
    }
    let mut elapsed_us: int64_t = now_us - (*m).window_start_us;
    if elapsed_us < 5000000 as int64_t {
        return 0 as ::core::ffi::c_int;
    }
    (*m).fps = stream_frame_rate_round(
        (*m).packets.wrapping_sub(1 as uint64_t) as ::core::ffi::c_double * 1000000.0f64
            / elapsed_us as ::core::ffi::c_double,
    );
    if (*m).fps == 0 as ::core::ffi::c_int {
        (*m).packets = 0 as uint64_t;
    }
    return (*m).fps;
}
