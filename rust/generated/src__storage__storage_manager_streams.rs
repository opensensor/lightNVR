unsafe extern "C" {
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
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
    unsafe fn get_recording_count(
        start_time: time_t,
        end_time: time_t,
        stream_name: *const ::core::ffi::c_char,
        has_detection: ::core::ffi::c_int,
        detection_label: *const ::core::ffi::c_char,
        protected_filter: ::core::ffi::c_int,
        allowed_streams: *const *const ::core::ffi::c_char,
        allowed_streams_count: ::core::ffi::c_int,
        tag_filter: *const ::core::ffi::c_char,
        capture_method_filter: *const ::core::ffi::c_char,
        include_in_progress: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_storage_bytes(stream_name: *const ::core::ffi::c_char) -> int64_t;
    unsafe fn count_stream_configs() -> ::core::ffi::c_int;
    unsafe fn get_all_stream_names(
        names: *mut [::core::ffi::c_char; 256],
        max_count: ::core::ffi::c_int,
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_storage_info_t {
    pub name: [::core::ffi::c_char; 64],
    pub size_bytes: ::core::ffi::c_ulong,
    pub recording_count: ::core::ffi::c_int,
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
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_storage_usage(
    mut storage_path: *const ::core::ffi::c_char,
    mut stream_info: *mut stream_storage_info_t,
    mut max_streams: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_info.is_null() || max_streams <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_stream_storage_usage\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut name_limit: ::core::ffi::c_int = if MAX_STREAMS < max_streams {
        MAX_STREAMS
    } else {
        max_streams
    };
    let mut stream_names: *mut [::core::ffi::c_char; 256] = calloc(
        name_limit as size_t,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    )
        as *mut [::core::ffi::c_char; 256];
    if stream_names.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate stream-name buffer for storage usage\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut name_count: ::core::ffi::c_int = get_all_stream_names(stream_names, name_limit);
    if name_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No streams found in database for storage usage\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(stream_names as *mut ::core::ffi::c_void);
        return name_count;
    }
    let mut stream_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < name_count && stream_count < max_streams {
        let mut total_bytes: int64_t = get_stream_storage_bytes(
            &raw mut *stream_names.offset(i as isize) as *mut ::core::ffi::c_char,
        );
        if total_bytes < 0 as int64_t {
            total_bytes = 0 as int64_t;
        }
        let mut rec_count: ::core::ffi::c_int = get_recording_count(
            0 as time_t,
            0 as time_t,
            &raw mut *stream_names.offset(i as isize) as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            -1 as ::core::ffi::c_int,
            ::core::ptr::null::<*const ::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            r#false != 0,
        );
        if rec_count < 0 as ::core::ffi::c_int {
            rec_count = 0 as ::core::ffi::c_int;
        }
        safe_strcpy(
            &raw mut (*stream_info.offset(stream_count as isize)).name as *mut ::core::ffi::c_char,
            &raw mut *stream_names.offset(i as isize) as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        (*stream_info.offset(stream_count as isize)).size_bytes =
            total_bytes as ::core::ffi::c_ulong;
        (*stream_info.offset(stream_count as isize)).recording_count = rec_count;
        stream_count += 1;
        i += 1;
    }
    free(stream_names as *mut ::core::ffi::c_void);
    return stream_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_all_stream_storage_usage(
    mut stream_info: *mut *mut stream_storage_info_t,
) -> ::core::ffi::c_int {
    if stream_info.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for get_all_stream_storage_usage\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream_count: ::core::ffi::c_int = count_stream_configs();
    if stream_count <= 0 as ::core::ffi::c_int {
        *stream_info = ::core::ptr::null_mut::<stream_storage_info_t>();
        return if stream_count == 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    *stream_info = malloc(
        (stream_count as size_t).wrapping_mul(::core::mem::size_of::<stream_storage_info_t>()),
    ) as *mut stream_storage_info_t;
    if (*stream_info).is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for stream storage info\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut actual_count: ::core::ffi::c_int = get_stream_storage_usage(
        ::core::ptr::null::<::core::ffi::c_char>(),
        *stream_info,
        stream_count,
    );
    if actual_count <= 0 as ::core::ffi::c_int {
        free(*stream_info as *mut ::core::ffi::c_void);
        *stream_info = ::core::ptr::null_mut::<stream_storage_info_t>();
    }
    return actual_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_stream_storage_usage_to_json(
    mut json_obj: *mut cJSON,
) -> ::core::ffi::c_int {
    if json_obj.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid JSON object for add_stream_storage_usage_to_json\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream_storage_array: *mut cJSON = cJSON_CreateArray();
    if stream_storage_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create stream storage JSON array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream_info: *mut stream_storage_info_t =
        ::core::ptr::null_mut::<stream_storage_info_t>();
    let mut stream_count: ::core::ffi::c_int = get_all_stream_storage_usage(&raw mut stream_info);
    if stream_count <= 0 as ::core::ffi::c_int || stream_info.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"No stream storage usage information available\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_AddItemToObject(
            json_obj,
            b"streamStorage\0".as_ptr() as *const ::core::ffi::c_char,
            stream_storage_array,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < stream_count {
        let mut stream_obj: *mut cJSON = cJSON_CreateObject();
        if !stream_obj.is_null() {
            cJSON_AddStringToObject(
                stream_obj,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*stream_info.offset(i as isize)).name as *mut ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"size\0".as_ptr() as *const ::core::ffi::c_char,
                (*stream_info.offset(i as isize)).size_bytes as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"count\0".as_ptr() as *const ::core::ffi::c_char,
                (*stream_info.offset(i as isize)).recording_count as ::core::ffi::c_double,
            );
            cJSON_AddItemToArray(stream_storage_array, stream_obj);
        }
        i += 1;
    }
    cJSON_AddItemToObject(
        json_obj,
        b"streamStorage\0".as_ptr() as *const ::core::ffi::c_char,
        stream_storage_array,
    );
    free(stream_info as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
