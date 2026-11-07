use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
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
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn secure_zero_memory(ptr: *mut ::core::ffi::c_void, size: size_t);
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
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
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type uint64_t = u64;
pub type uint_fast64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_destination_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub destination_type: [::core::ffi::c_char; 16],
    pub broker_host: [::core::ffi::c_char; 256],
    pub broker_port: ::core::ffi::c_int,
    pub client_id: [::core::ffi::c_char; 128],
    pub topic_template: [::core::ffi::c_char; 512],
    pub status_topic_template: [::core::ffi::c_char; 512],
    pub username: [::core::ffi::c_char; 128],
    pub password_configured: bool,
    pub tls_mode: [::core::ffi::c_char; 16],
    pub ca_file: [::core::ffi::c_char; 512],
    pub cert_file: [::core::ffi::c_char; 512],
    pub key_file: [::core::ffi::c_char; 512],
    pub keepalive_seconds: ::core::ffi::c_int,
    pub qos: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_event_destination_result_t(pub ::core::ffi::c_int);
impl db_event_destination_result_t {
    pub const DB_EVENT_DESTINATION_OK: Self = Self(0);
    pub const DB_EVENT_DESTINATION_NOT_FOUND: Self = Self(-1);
    pub const DB_EVENT_DESTINATION_CONFLICT: Self = Self(-2);
    pub const DB_EVENT_DESTINATION_INVALID: Self = Self(-3);
    pub const DB_EVENT_DESTINATION_STALE: Self = Self(-4);
    pub const DB_EVENT_DESTINATION_LIMIT: Self = Self(-5);
    pub const DB_EVENT_DESTINATION_IN_USE: Self = Self(-6);
    pub const DB_EVENT_DESTINATION_ERROR: Self = Self(-7);
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type atomic_uint_fast64_t = uint_fast64_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct memory_order(pub ::core::ffi::c_uint);
impl memory_order {
    pub const memory_order_relaxed: Self = Self(0);
    pub const memory_order_consume: Self = Self(1);
    pub const memory_order_acquire: Self = Self(2);
    pub const memory_order_release: Self = Self(3);
    pub const memory_order_acq_rel: Self = Self(4);
    pub const memory_order_seq_cst: Self = Self(5);
}
pub const EVENT_DESTINATION_KEY_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_HOST_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_TOPIC_TEMPLATE_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_STATUS_TOPIC_TEMPLATE_MAX: ::core::ffi::c_int =
    512 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_PASSWORD_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_PATH_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_MAX_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_KEY_PREFIX: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"mqtt:\0") };
pub const EVENT_DESTINATION_STATUS_INSTALLATION_PLACEHOLDER: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"{installation_uuid}\0")
};
pub const EVENT_DESTINATION_STATUS_DESTINATION_PLACEHOLDER: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"{destination_uuid}\0")
};
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const EVENT_TYPE_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_SUBJECT_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_TOPIC_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
static mut DESTINATION_GENERATION: atomic_uint_fast64_t = 1 as ::core::ffi::c_ulong;
unsafe extern "C" fn set_error(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if error.is_null()
        || error_size == 0 as size_t
        || *error.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        return;
    }
    let mut arguments: ::core::ffi::VaList;
    arguments = c2rust_args.clone();
    vsnprintf(error, error_size, format, arguments.clone());
}
unsafe extern "C" fn valid_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
    mut required: bool,
) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strnlen(value, maximum);
    if length == maximum || required as ::core::ffi::c_int != 0 && length == 0 as size_t {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_host(mut host: *const ::core::ffi::c_char) -> bool {
    if !valid_text(host, EVENT_DESTINATION_HOST_MAX as size_t, r#true != 0)
        || !strstr(host, b"://\0".as_ptr() as *const ::core::ffi::c_char).is_null()
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = host as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || *cursor as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '?' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_path(mut path: *const ::core::ffi::c_char, mut required: bool) -> bool {
    if !valid_text(path, EVENT_DESTINATION_PATH_MAX as size_t, required) {
        return r#false != 0;
    }
    return *path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *path.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int;
}
unsafe extern "C" fn valid_topic_template(mut topic: *const ::core::ffi::c_char) -> bool {
    if !valid_text(
        topic,
        EVENT_DESTINATION_TOPIC_TEMPLATE_MAX as size_t,
        r#true != 0,
    ) {
        return r#false != 0;
    }
    let mut has_type: bool = r#false != 0;
    let mut has_subject_id: bool = r#false != 0;
    let mut maximum_expanded_length: size_t = 0 as size_t;
    let mut index: size_t = 0 as size_t;
    while *topic.offset(index as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if *topic.offset(index as isize) as ::core::ffi::c_int == '+' as ::core::ffi::c_int
            || *topic.offset(index as isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        if *topic.offset(index as isize) as ::core::ffi::c_int == '}' as ::core::ffi::c_int {
            return r#false != 0;
        }
        if *topic.offset(index as isize) as ::core::ffi::c_int != '{' as ::core::ffi::c_int {
            maximum_expanded_length = maximum_expanded_length.wrapping_add(1);
            index = index.wrapping_add(1);
        } else {
            if strncmp(
                topic.offset(index as isize),
                b"{type}\0".as_ptr() as *const ::core::ffi::c_char,
                6 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                has_type = r#true != 0;
                maximum_expanded_length = maximum_expanded_length
                    .wrapping_add((EVENT_TYPE_MAX - 1 as ::core::ffi::c_int) as size_t);
                index = index.wrapping_add(6 as size_t);
            } else if strncmp(
                topic.offset(index as isize),
                b"{subject_id}\0".as_ptr() as *const ::core::ffi::c_char,
                12 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                has_subject_id = r#true != 0;
                maximum_expanded_length = maximum_expanded_length
                    .wrapping_add((EVENT_SUBJECT_MAX - 1 as ::core::ffi::c_int) as size_t);
                index = index.wrapping_add(12 as size_t);
            } else {
                return r#false != 0;
            }
            if maximum_expanded_length >= EVENT_OUTBOX_TOPIC_MAX as size_t {
                return r#false != 0;
            }
        }
    }
    return has_type as ::core::ffi::c_int != 0
        && has_subject_id as ::core::ffi::c_int != 0
        && *topic.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        && *topic.offset(strlen(topic).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            != '/' as ::core::ffi::c_int
        && maximum_expanded_length < EVENT_OUTBOX_TOPIC_MAX as size_t;
}
unsafe extern "C" fn valid_status_topic_template(mut topic: *const ::core::ffi::c_char) -> bool {
    if !valid_text(
        topic,
        EVENT_DESTINATION_STATUS_TOPIC_TEMPLATE_MAX as size_t,
        r#false != 0,
    ) {
        return r#false != 0;
    }
    if *topic.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#true != 0;
    }
    let mut has_installation_uuid: bool = r#false != 0;
    let mut maximum_expanded_length: size_t = 0 as size_t;
    let mut index: size_t = 0 as size_t;
    while *topic.offset(index as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if *topic.offset(index as isize) as ::core::ffi::c_int == '+' as ::core::ffi::c_int
            || *topic.offset(index as isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
            || *topic.offset(index as isize) as ::core::ffi::c_int == '}' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        if *topic.offset(index as isize) as ::core::ffi::c_int != '{' as ::core::ffi::c_int {
            maximum_expanded_length = maximum_expanded_length.wrapping_add(1);
            index = index.wrapping_add(1);
        } else {
            if strncmp(
                topic.offset(index as isize),
                EVENT_DESTINATION_STATUS_INSTALLATION_PLACEHOLDER.as_ptr(),
                ::core::mem::size_of::<[::core::ffi::c_char; 20]>().wrapping_sub(1 as size_t),
            ) == 0 as ::core::ffi::c_int
            {
                has_installation_uuid = r#true != 0;
                maximum_expanded_length = maximum_expanded_length.wrapping_add(36 as size_t);
                index = (index as ::core::ffi::c_ulong).wrapping_add(
                    ::core::mem::size_of::<[::core::ffi::c_char; 20]>().wrapping_sub(1usize)
                        as ::core::ffi::c_ulong,
                ) as size_t;
            } else if strncmp(
                topic.offset(index as isize),
                EVENT_DESTINATION_STATUS_DESTINATION_PLACEHOLDER.as_ptr(),
                ::core::mem::size_of::<[::core::ffi::c_char; 19]>().wrapping_sub(1 as size_t),
            ) == 0 as ::core::ffi::c_int
            {
                maximum_expanded_length = maximum_expanded_length.wrapping_add(36 as size_t);
                index = (index as ::core::ffi::c_ulong).wrapping_add(
                    ::core::mem::size_of::<[::core::ffi::c_char; 19]>().wrapping_sub(1usize)
                        as ::core::ffi::c_ulong,
                ) as size_t;
            } else {
                return r#false != 0;
            }
            if maximum_expanded_length >= EVENT_OUTBOX_TOPIC_MAX as size_t {
                return r#false != 0;
            }
        }
    }
    return has_installation_uuid as ::core::ffi::c_int != 0
        && *topic.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        && *topic.offset(strlen(topic).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            != '/' as ::core::ffi::c_int
        && maximum_expanded_length < EVENT_OUTBOX_TOPIC_MAX as size_t;
}
unsafe extern "C" fn valid_password(mut password: *const ::core::ffi::c_char) -> bool {
    return valid_text(
        password,
        EVENT_DESTINATION_PASSWORD_MAX as size_t,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_validate(
    mut destination: *const event_destination_t,
    mut password: *const ::core::ffi::c_char,
    mut validate_password: bool,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> db_event_destination_result_t {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if destination.is_null()
        || !valid_text(
            &raw const (*destination).name as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            r#true != 0,
        )
        || !valid_text(
            &raw const (*destination).description as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            r#false != 0,
        )
    {
        set_error(
            error,
            error_size,
            b"destination name or description is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw const (*destination).name as *const ::core::ffi::c_char,
        0 as size_t,
    ) == 0 as size_t
    {
        set_error(
            error,
            error_size,
            b"destination name cannot be blank\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    if !valid_text(
        &raw const (*destination).destination_type as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        r#true != 0,
    ) || strcmp(
        &raw const (*destination).destination_type as *const ::core::ffi::c_char,
        b"mqtt\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"destination type must be mqtt\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    if !valid_host(&raw const (*destination).broker_host as *const ::core::ffi::c_char)
        || (*destination).broker_port < 1 as ::core::ffi::c_int
        || (*destination).broker_port > 65535 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"broker host or port is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    if !valid_text(
        &raw const (*destination).client_id as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        r#true != 0,
    ) || !valid_topic_template(
        &raw const (*destination).topic_template as *const ::core::ffi::c_char,
    ) || !valid_status_topic_template(
        &raw const (*destination).status_topic_template as *const ::core::ffi::c_char,
    ) {
        set_error(
            error,
            error_size,
            b"client ID, event topic template, or status topic template is invalid\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    if !valid_text(
        &raw const (*destination).username as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        r#false != 0,
    ) || validate_password as ::core::ffi::c_int != 0 && !valid_password(password)
    {
        set_error(
            error,
            error_size,
            b"MQTT credentials are invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    let mut password_configured: bool = if validate_password as ::core::ffi::c_int != 0 {
        (!password.is_null()
            && *password.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } else {
        (*destination).password_configured as ::core::ffi::c_int
    } != 0;
    if password_configured as ::core::ffi::c_int != 0
        && (*destination).username[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"a password requires an MQTT username\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    if !valid_text(
        &raw const (*destination).tls_mode as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        r#true != 0,
    ) {
        set_error(
            error,
            error_size,
            b"TLS mode is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    let mut disabled: bool = strcmp(
        &raw const (*destination).tls_mode as *const ::core::ffi::c_char,
        b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut system: bool = strcmp(
        &raw const (*destination).tls_mode as *const ::core::ffi::c_char,
        b"system\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut custom_ca: bool = strcmp(
        &raw const (*destination).tls_mode as *const ::core::ffi::c_char,
        b"custom_ca\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut mutual: bool = strcmp(
        &raw const (*destination).tls_mode as *const ::core::ffi::c_char,
        b"mutual\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    if !disabled && !system && !custom_ca && !mutual
        || !valid_path(
            &raw const (*destination).ca_file as *const ::core::ffi::c_char,
            custom_ca as ::core::ffi::c_int != 0 || mutual as ::core::ffi::c_int != 0,
        )
        || !valid_path(
            &raw const (*destination).cert_file as *const ::core::ffi::c_char,
            mutual,
        )
        || !valid_path(
            &raw const (*destination).key_file as *const ::core::ffi::c_char,
            mutual,
        )
        || (disabled as ::core::ffi::c_int != 0 || system as ::core::ffi::c_int != 0)
            && ((*destination).ca_file[0usize] as ::core::ffi::c_int != 0
                || (*destination).cert_file[0usize] as ::core::ffi::c_int != 0
                || (*destination).key_file[0usize] as ::core::ffi::c_int != 0)
        || custom_ca as ::core::ffi::c_int != 0
            && ((*destination).cert_file[0usize] as ::core::ffi::c_int != 0
                || (*destination).key_file[0usize] as ::core::ffi::c_int != 0)
    {
        set_error(
            error,
            error_size,
            b"TLS mode and certificate paths are inconsistent\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    if (*destination).keepalive_seconds < 5 as ::core::ffi::c_int
        || (*destination).keepalive_seconds > 3600 as ::core::ffi::c_int
        || (*destination).qos < 0 as ::core::ffi::c_int
        || (*destination).qos > 2 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"keepalive or QoS is out of range\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    return db_event_destination_result_t::DB_EVENT_DESTINATION_OK;
}
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut statement: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, column) as *const ::core::ffi::c_char;
    safe_strcpy(
        destination,
        if !value.is_null() {
            value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        destination_size,
        0 as size_t,
    );
}
unsafe extern "C" fn populate(
    mut statement: *mut sqlite3_stmt,
    mut destination: *mut event_destination_t,
) {
    memset(
        destination as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_destination_t>(),
    );
    copy_column(
        &raw mut (*destination).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).description as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    (*destination).enabled =
        sqlite3_column_int(statement, 3 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    copy_column(
        &raw mut (*destination).destination_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        statement,
        4 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).broker_host as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        statement,
        5 as ::core::ffi::c_int,
    );
    (*destination).broker_port = sqlite3_column_int(statement, 6 as ::core::ffi::c_int);
    copy_column(
        &raw mut (*destination).client_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        7 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).topic_template as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        8 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).status_topic_template as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        9 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        10 as ::core::ffi::c_int,
    );
    (*destination).password_configured =
        sqlite3_column_int(statement, 11 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    copy_column(
        &raw mut (*destination).tls_mode as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        statement,
        12 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).ca_file as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        13 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).cert_file as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        14 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*destination).key_file as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        15 as ::core::ffi::c_int,
    );
    (*destination).keepalive_seconds = sqlite3_column_int(statement, 16 as ::core::ffi::c_int);
    (*destination).qos = sqlite3_column_int(statement, 17 as ::core::ffi::c_int);
    (*destination).revision = sqlite3_column_int64(statement, 18 as ::core::ffi::c_int) as int64_t;
    (*destination).created_at =
        sqlite3_column_int64(statement, 19 as ::core::ffi::c_int) as int64_t;
    (*destination).updated_at =
        sqlite3_column_int64(statement, 20 as ::core::ffi::c_int) as int64_t;
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut destination: *mut event_destination_t,
) -> db_event_destination_result_t {
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,name,description,enabled,destination_type,broker_host,broker_port,client_id,topic_template,status_topic_template,username,(password<>''),tls_mode,ca_file,cert_file,key_file,keepalive_seconds,qos,revision,created_at,updated_at FROM event_destinations WHERE uuid=? LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut outcome: db_event_destination_result_t =
        db_event_destination_result_t::DB_EVENT_DESTINATION_NOT_FOUND;
    if result == SQLITE_ROW {
        populate(statement, destination);
        outcome = db_event_destination_result_t::DB_EVENT_DESTINATION_OK;
    } else if result != SQLITE_DONE {
        outcome = db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return outcome;
}
unsafe extern "C" fn count_locked(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM event_destinations;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(statement) == SQLITE_ROW
    {
        count = sqlite3_column_int(statement, 0 as ::core::ffi::c_int);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_count() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut count: ::core::ffi::c_int = count_locked(db);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_list(
    mut destinations: *mut event_destination_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if destinations.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,name,description,enabled,destination_type,broker_host,broker_port,client_id,topic_template,status_topic_template,username,(password<>''),tls_mode,ca_file,cert_file,key_file,keepalive_seconds,qos,revision,created_at,updated_at FROM event_destinations ORDER BY name COLLATE NOCASE,uuid;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while result == SQLITE_OK && count < max_count && {
        result = sqlite3_step(statement);
        result == SQLITE_ROW
    } {
        let c2rust_fresh0 = count;
        count += 1;
        populate(statement, destinations.offset(c2rust_fresh0 as isize));
    }
    if result != SQLITE_DONE && result != SQLITE_ROW {
        count = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_get(
    mut uuid: *const ::core::ffi::c_char,
    mut destination: *mut event_destination_t,
) -> db_event_destination_result_t {
    if !lightnvr_uuid_is_valid(uuid) || destination.is_null() {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_event_destination_result_t = get_locked(db, uuid, destination);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_make_key(
    mut uuid: *const ::core::ffi::c_char,
    mut key: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !lightnvr_uuid_is_valid(uuid) || key.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut written: ::core::ffi::c_int = snprintf(
        key,
        EVENT_DESTINATION_KEY_MAX as size_t,
        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        EVENT_DESTINATION_KEY_PREFIX.as_ptr(),
        uuid,
    );
    return if written > 0 as ::core::ffi::c_int && written < EVENT_DESTINATION_KEY_MAX {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn uuid_from_key(
    mut key: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut prefix_length: size_t = strlen(EVENT_DESTINATION_KEY_PREFIX.as_ptr());
    if key.is_null()
        || strncmp(key, EVENT_DESTINATION_KEY_PREFIX.as_ptr(), prefix_length)
            != 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut uuid: *const ::core::ffi::c_char = key.offset(prefix_length as isize);
    return if lightnvr_uuid_is_valid(uuid) as ::core::ffi::c_int != 0 {
        uuid
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_get_by_key(
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut event_destination_t,
) -> db_event_destination_result_t {
    let mut uuid: *const ::core::ffi::c_char = uuid_from_key(key);
    return db_event_destination_result_t(if !uuid.is_null() {
        db_event_destination_get(uuid, destination).0
    } else {
        db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_key_exists(
    mut key: *const ::core::ffi::c_char,
) -> bool {
    let mut destination: event_destination_t = event_destination_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_type: [0; 16],
        broker_host: [0; 256],
        broker_port: 0,
        client_id: [0; 128],
        topic_template: [0; 512],
        status_topic_template: [0; 512],
        username: [0; 128],
        password_configured: false,
        tls_mode: [0; 16],
        ca_file: [0; 512],
        cert_file: [0; 512],
        key_file: [0; 512],
        keepalive_seconds: 0,
        qos: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    return db_event_destination_get_by_key(key, &raw mut destination).0
        == db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0;
}
unsafe extern "C" fn bind_update_fields(
    mut statement: *mut sqlite3_stmt,
    mut destination: *const event_destination_t,
    mut normalized_name: *const ::core::ffi::c_char,
) {
    sqlite3_bind_text(
        statement,
        1 as ::core::ffi::c_int,
        normalized_name,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        2 as ::core::ffi::c_int,
        &raw const (*destination).description as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int(
        statement,
        3 as ::core::ffi::c_int,
        if (*destination).enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        statement,
        4 as ::core::ffi::c_int,
        &raw const (*destination).destination_type as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        5 as ::core::ffi::c_int,
        &raw const (*destination).broker_host as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int(
        statement,
        6 as ::core::ffi::c_int,
        (*destination).broker_port,
    );
    sqlite3_bind_text(
        statement,
        7 as ::core::ffi::c_int,
        &raw const (*destination).client_id as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        8 as ::core::ffi::c_int,
        &raw const (*destination).topic_template as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        9 as ::core::ffi::c_int,
        &raw const (*destination).status_topic_template as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        10 as ::core::ffi::c_int,
        &raw const (*destination).username as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        11 as ::core::ffi::c_int,
        &raw const (*destination).tls_mode as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        12 as ::core::ffi::c_int,
        &raw const (*destination).ca_file as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        13 as ::core::ffi::c_int,
        &raw const (*destination).cert_file as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        14 as ::core::ffi::c_int,
        &raw const (*destination).key_file as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int(
        statement,
        15 as ::core::ffi::c_int,
        (*destination).keepalive_seconds,
    );
    sqlite3_bind_int(statement, 16 as ::core::ffi::c_int, (*destination).qos);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_create(
    mut destination: *mut event_destination_t,
    mut password: *const ::core::ffi::c_char,
) -> db_event_destination_result_t {
    if password.is_null() {
        password = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let mut error: [::core::ffi::c_char; 256] = [
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
    if destination.is_null()
        || db_event_destination_validate(
            destination,
            password,
            r#true != 0,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
        .0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0
    {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    if lightnvr_uuid_generate_v4(&raw mut (*destination).uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    }
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    copy_trimmed_value(
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw mut (*destination).name as *mut ::core::ffi::c_char,
        0 as size_t,
    );
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO event_destinations(uuid,name,description,enabled,destination_type,broker_host,broker_port,client_id,topic_template,status_topic_template,username,password,tls_mode,ca_file,cert_file,key_file,keepalive_seconds,qos) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    if count_locked(db) >= EVENT_DESTINATION_MAX_COUNT {
        pthread_mutex_unlock(mutex);
        return db_event_destination_result_t::DB_EVENT_DESTINATION_LIMIT;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*destination).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut (*destination).description as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            4 as ::core::ffi::c_int,
            if (*destination).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            &raw mut (*destination).destination_type as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw mut (*destination).broker_host as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            7 as ::core::ffi::c_int,
            (*destination).broker_port,
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw mut (*destination).client_id as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw mut (*destination).topic_template as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            &raw mut (*destination).status_topic_template as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            11 as ::core::ffi::c_int,
            &raw mut (*destination).username as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            12 as ::core::ffi::c_int,
            password,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            13 as ::core::ffi::c_int,
            &raw mut (*destination).tls_mode as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            14 as ::core::ffi::c_int,
            &raw mut (*destination).ca_file as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            15 as ::core::ffi::c_int,
            &raw mut (*destination).cert_file as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            16 as ::core::ffi::c_int,
            &raw mut (*destination).key_file as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            17 as ::core::ffi::c_int,
            (*destination).keepalive_seconds,
        );
        sqlite3_bind_int(statement, 18 as ::core::ffi::c_int, (*destination).qos);
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE {
        pthread_mutex_unlock(mutex);
        return db_event_destination_result_t(
            if result & 0xff as ::core::ffi::c_int == SQLITE_CONSTRAINT {
                db_event_destination_result_t::DB_EVENT_DESTINATION_CONFLICT.0
            } else {
                db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR.0
            },
        );
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut DESTINATION_GENERATION,
        1 as uint_fast64_t,
    );
    let mut outcome: db_event_destination_result_t = get_locked(
        db,
        &raw mut (*destination).uuid as *mut ::core::ffi::c_char,
        destination,
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_update(
    mut destination: *mut event_destination_t,
    mut expected_revision: int64_t,
    mut password: *const ::core::ffi::c_char,
    mut replace_password: bool,
) -> db_event_destination_result_t {
    if replace_password as ::core::ffi::c_int != 0 && password.is_null() {
        password = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let mut error: [::core::ffi::c_char; 256] = [
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
    if destination.is_null()
        || !lightnvr_uuid_is_valid(&raw mut (*destination).uuid as *mut ::core::ffi::c_char)
        || expected_revision < 1 as int64_t
        || db_event_destination_validate(
            destination,
            password,
            replace_password,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
        .0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0
    {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    copy_trimmed_value(
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw mut (*destination).name as *mut ::core::ffi::c_char,
        0 as size_t,
    );
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = if replace_password as ::core::ffi::c_int != 0 {
        b"UPDATE event_destinations SET name=?,description=?,enabled=?,destination_type=?,broker_host=?,broker_port=?,client_id=?,topic_template=?,status_topic_template=?,username=?,tls_mode=?,ca_file=?,cert_file=?,key_file=?,keepalive_seconds=?,qos=?,password=?,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND revision=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"UPDATE event_destinations SET name=?,description=?,enabled=?,destination_type=?,broker_host=?,broker_port=?,client_id=?,topic_template=?,status_topic_template=?,username=?,tls_mode=?,ca_file=?,cert_file=?,key_file=?,keepalive_seconds=?,qos=?,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND revision=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    };
    pthread_mutex_lock(mutex);
    let mut existing: event_destination_t = event_destination_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_type: [0; 16],
        broker_host: [0; 256],
        broker_port: 0,
        client_id: [0; 128],
        topic_template: [0; 512],
        status_topic_template: [0; 512],
        username: [0; 128],
        password_configured: false,
        tls_mode: [0; 16],
        ca_file: [0; 512],
        cert_file: [0; 512],
        key_file: [0; 512],
        keepalive_seconds: 0,
        qos: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut outcome: db_event_destination_result_t = get_locked(
        db,
        &raw mut (*destination).uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if outcome.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if existing.revision != expected_revision {
        pthread_mutex_unlock(mutex);
        return db_event_destination_result_t::DB_EVENT_DESTINATION_STALE;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        bind_update_fields(
            statement,
            destination,
            &raw mut normalized_name as *mut ::core::ffi::c_char,
        );
        let mut index: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
        if replace_password {
            let c2rust_fresh1 = index;
            index += 1;
            sqlite3_bind_text(
                statement,
                c2rust_fresh1,
                password,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        let c2rust_fresh2 = index;
        index += 1;
        sqlite3_bind_text(
            statement,
            c2rust_fresh2,
            &raw mut (*destination).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(statement, index, expected_revision as sqlite3_int64);
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE || changed != 1 as ::core::ffi::c_int {
        pthread_mutex_unlock(mutex);
        return db_event_destination_result_t(
            if result & 0xff as ::core::ffi::c_int == SQLITE_CONSTRAINT {
                db_event_destination_result_t::DB_EVENT_DESTINATION_CONFLICT.0
            } else {
                db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR.0
            },
        );
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut DESTINATION_GENERATION,
        1 as uint_fast64_t,
    );
    outcome = get_locked(
        db,
        &raw mut (*destination).uuid as *mut ::core::ffi::c_char,
        destination,
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
unsafe extern "C" fn destination_in_use_locked(
    mut db: *mut sqlite3,
    mut key: *const ::core::ffi::c_char,
) -> bool {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT 1 FROM event_routes WHERE destination_key=? UNION ALL SELECT 1 FROM event_outbox WHERE destination=? AND state IN ('pending','delivering') LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            key,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            key,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut in_use: bool = result == SQLITE_ROW;
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return in_use;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_delete(
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
) -> db_event_destination_result_t {
    if !lightnvr_uuid_is_valid(uuid) || expected_revision < 1 as int64_t {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    let mut key: [::core::ffi::c_char; 64] = [0; 64];
    if db_event_destination_make_key(uuid, &raw mut key as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut existing: event_destination_t = event_destination_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_type: [0; 16],
        broker_host: [0; 256],
        broker_port: 0,
        client_id: [0; 128],
        topic_template: [0; 512],
        status_topic_template: [0; 512],
        username: [0; 128],
        password_configured: false,
        tls_mode: [0; 16],
        ca_file: [0; 512],
        cert_file: [0; 512],
        key_file: [0; 512],
        keepalive_seconds: 0,
        qos: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut outcome: db_event_destination_result_t = get_locked(db, uuid, &raw mut existing);
    if outcome.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if existing.revision != expected_revision {
        pthread_mutex_unlock(mutex);
        return db_event_destination_result_t::DB_EVENT_DESTINATION_STALE;
    }
    if destination_in_use_locked(db, &raw mut key as *mut ::core::ffi::c_char) {
        pthread_mutex_unlock(mutex);
        return db_event_destination_result_t::DB_EVENT_DESTINATION_IN_USE;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM event_destinations WHERE uuid=? AND revision=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if result != SQLITE_DONE || changed != 1 as ::core::ffi::c_int {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut DESTINATION_GENERATION,
        1 as uint_fast64_t,
    );
    return db_event_destination_result_t::DB_EVENT_DESTINATION_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_get_password(
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
    mut password: *mut ::core::ffi::c_char,
    mut password_size: size_t,
) -> db_event_destination_result_t {
    if !password.is_null() && password_size > 0 as size_t {
        secure_zero_memory(password as *mut ::core::ffi::c_void, password_size);
    }
    if !lightnvr_uuid_is_valid(uuid)
        || expected_revision < 1 as int64_t
        || password.is_null()
        || password_size == 0 as size_t
    {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT revision,password FROM event_destinations WHERE uuid=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut outcome: db_event_destination_result_t =
        db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR;
    if result == SQLITE_ROW {
        if sqlite3_column_int64(statement, 0 as ::core::ffi::c_int)
            != expected_revision as sqlite3_int64
        {
            outcome = db_event_destination_result_t::DB_EVENT_DESTINATION_STALE;
        } else {
            let mut value: *const ::core::ffi::c_char =
                sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            let mut length: size_t = if !value.is_null() {
                strlen(value)
            } else {
                0 as size_t
            };
            if length < password_size {
                safe_strcpy(
                    password,
                    if !value.is_null() {
                        value
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    password_size,
                    0 as size_t,
                );
                outcome = db_event_destination_result_t::DB_EVENT_DESTINATION_OK;
            }
        }
    } else if result == SQLITE_DONE {
        outcome = db_event_destination_result_t::DB_EVENT_DESTINATION_NOT_FOUND;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_destination_generation() -> uint64_t {
    return ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut DESTINATION_GENERATION,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
