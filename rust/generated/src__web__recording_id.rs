unsafe extern "C" {
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
}
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
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_id_parse_json(
    mut item: *const cJSON,
    mut id: *mut uint64_t,
) -> bool {
    if cJSON_IsNumber(item) != 0 {
        let mut value: ::core::ffi::c_double = (*item).valuedouble;
        if !(value >= 1 as ::core::ffi::c_int as ::core::ffi::c_double
            && value <= 9007199254740991.0f64)
        {
            return r#false != 0;
        }
        *id = value as uint64_t;
        return *id as ::core::ffi::c_double == value;
    }
    return cJSON_IsString(item) != 0
        && recording_id_parse_decimal((*item).valuestring, id) as ::core::ffi::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_id_parse_decimal(
    mut text: *const ::core::ffi::c_char,
    mut id: *mut uint64_t,
) -> bool {
    if text.is_null() || *text.offset(0isize) == 0 {
        return r#false != 0;
    }
    let mut value: uint64_t = 0 as uint64_t;
    let mut p: *const ::core::ffi::c_char = text;
    while *p != 0 {
        if (*p as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int > '9' as ::core::ffi::c_int
            || value
                > (UINT64_MAX as uint64_t)
                    .wrapping_sub(
                        (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                            as ::core::ffi::c_uint as uint64_t,
                    )
                    .wrapping_div(10 as uint64_t)
        {
            return r#false != 0;
        }
        value = value.wrapping_mul(10 as uint64_t).wrapping_add(
            (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as ::core::ffi::c_uint
                as uint64_t,
        );
        p = p.offset(1);
    }
    *id = value;
    return value != 0 as uint64_t;
}
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
