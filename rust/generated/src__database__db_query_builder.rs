unsafe extern "C" {
    pub type sqlite3_stmt;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn cached_column_exists(
        table_name: *const ::core::ffi::c_char,
        column_name: *const ::core::ffi::c_char,
    ) -> bool;
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
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct column_info_t {
    pub name: [::core::ffi::c_char; 64],
    pub index: ::core::ffi::c_int,
    pub present: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct query_builder_t {
    pub table_name: *const ::core::ffi::c_char,
    pub columns: [column_info_t; 64],
    pub column_count: ::core::ffi::c_int,
    pub query: [::core::ffi::c_char; 4096],
    pub result_column_count: ::core::ffi::c_int,
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
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MAX_TRACKED_COLUMNS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_init(
    mut qb: *mut query_builder_t,
    mut table_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if qb.is_null() || table_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        qb as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<query_builder_t>(),
    );
    (*qb).table_name = table_name;
    (*qb).column_count = 0 as ::core::ffi::c_int;
    (*qb).result_column_count = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_add_column(
    mut qb: *mut query_builder_t,
    mut column_name: *const ::core::ffi::c_char,
    mut is_required: bool,
) -> ::core::ffi::c_int {
    if qb.is_null() || column_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*qb).column_count >= MAX_TRACKED_COLUMNS {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Too many columns tracked (max %d)\0".as_ptr() as *const ::core::ffi::c_char,
            64 as ::core::ffi::c_int,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut col: *mut column_info_t =
        (&raw mut (*qb).columns as *mut column_info_t).offset((*qb).column_count as isize);
    safe_strcpy(
        &raw mut (*col).name as *mut ::core::ffi::c_char,
        column_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    (*col).present = cached_column_exists((*qb).table_name, column_name);
    (*col).index = -1 as ::core::ffi::c_int;
    if is_required as ::core::ffi::c_int != 0 && !(*col).present {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Required column '%s.%s' does not exist\0".as_ptr() as *const ::core::ffi::c_char,
            (*qb).table_name,
            column_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    (*qb).column_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_build_select(
    mut qb: *mut query_builder_t,
    mut where_clause: *const ::core::ffi::c_char,
    mut order_by: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if qb.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut ptr: *mut ::core::ffi::c_char = &raw mut (*qb).query as *mut ::core::ffi::c_char;
    let mut remaining: size_t = ::core::mem::size_of::<[::core::ffi::c_char; 4096]>();
    let mut written: ::core::ffi::c_int = 0;
    written = snprintf(
        ptr,
        remaining,
        b"SELECT \0".as_ptr() as *const ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    ptr = ptr.offset(written as isize);
    remaining = remaining.wrapping_sub(written as size_t);
    let mut result_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut first: bool = r#true != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*qb).column_count {
        let mut col: *mut column_info_t =
            (&raw mut (*qb).columns as *mut column_info_t).offset(i as isize);
        if (*col).present {
            if !first {
                written = snprintf(
                    ptr,
                    remaining,
                    b", \0".as_ptr() as *const ::core::ffi::c_char,
                );
                if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
                    break;
                }
                ptr = ptr.offset(written as isize);
                remaining = remaining.wrapping_sub(written as size_t);
            }
            written = snprintf(
                ptr,
                remaining,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*col).name as *mut ::core::ffi::c_char,
            );
            if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
                break;
            }
            ptr = ptr.offset(written as isize);
            remaining = remaining.wrapping_sub(written as size_t);
            let c2rust_fresh0 = result_idx;
            result_idx += 1;
            (*col).index = c2rust_fresh0;
            first = r#false != 0;
        }
        i += 1;
    }
    (*qb).result_column_count = result_idx;
    if result_idx == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No columns available for query on table %s\0".as_ptr() as *const ::core::ffi::c_char,
            (*qb).table_name,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    written = snprintf(
        ptr,
        remaining,
        b" FROM %s\0".as_ptr() as *const ::core::ffi::c_char,
        (*qb).table_name,
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    ptr = ptr.offset(written as isize);
    remaining = remaining.wrapping_sub(written as size_t);
    if !where_clause.is_null() && strlen(where_clause) > 0 as size_t {
        written = snprintf(
            ptr,
            remaining,
            b" WHERE %s\0".as_ptr() as *const ::core::ffi::c_char,
            where_clause,
        );
        if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        ptr = ptr.offset(written as isize);
        remaining = remaining.wrapping_sub(written as size_t);
    }
    if !order_by.is_null() && strlen(order_by) > 0 as size_t {
        written = snprintf(
            ptr,
            remaining,
            b" ORDER BY %s\0".as_ptr() as *const ::core::ffi::c_char,
            order_by,
        );
        if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        ptr = ptr.offset(written as isize);
        remaining = remaining.wrapping_sub(written as size_t);
    }
    written = snprintf(
        ptr,
        remaining,
        b";\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return &raw mut (*qb).query as *mut ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_has_column(
    mut qb: *const query_builder_t,
    mut column_name: *const ::core::ffi::c_char,
) -> bool {
    if qb.is_null() || column_name.is_null() {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*qb).column_count {
        if strcmp(
            &raw const (*(&raw const (*qb).columns as *const column_info_t).offset(i as isize)).name
                as *const ::core::ffi::c_char,
            column_name,
        ) == 0 as ::core::ffi::c_int
        {
            return (*qb).columns[i as usize].present as ::core::ffi::c_int != 0
                && (*qb).columns[i as usize].index >= 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_get_column_index(
    mut qb: *const query_builder_t,
    mut column_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if qb.is_null() || column_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*qb).column_count {
        if strcmp(
            &raw const (*(&raw const (*qb).columns as *const column_info_t).offset(i as isize)).name
                as *const ::core::ffi::c_char,
            column_name,
        ) == 0 as ::core::ffi::c_int
        {
            return (*qb).columns[i as usize].index;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_get_int(
    mut stmt: *mut sqlite3_stmt,
    mut qb: *const query_builder_t,
    mut column_name: *const ::core::ffi::c_char,
    mut default_value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = qb_get_column_index(qb, column_name);
    if idx < 0 as ::core::ffi::c_int {
        return default_value;
    }
    if sqlite3_column_type(stmt, idx) == SQLITE_NULL {
        return default_value;
    }
    return sqlite3_column_int(stmt, idx);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_get_text(
    mut stmt: *mut sqlite3_stmt,
    mut qb: *const query_builder_t,
    mut column_name: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_len: size_t,
    mut default_value: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut idx: ::core::ffi::c_int = qb_get_column_index(qb, column_name);
    if idx < 0 as ::core::ffi::c_int {
        if !default_value.is_null() && !buffer.is_null() && buffer_len > 0 as size_t {
            safe_strcpy(buffer, default_value, buffer_len, 0 as size_t);
        }
        return default_value;
    }
    let mut text: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, idx) as *const ::core::ffi::c_char;
    if text.is_null() {
        if !default_value.is_null() && !buffer.is_null() && buffer_len > 0 as size_t {
            safe_strcpy(buffer, default_value, buffer_len, 0 as size_t);
        }
        return default_value;
    }
    if !buffer.is_null() && buffer_len > 0 as size_t {
        safe_strcpy(buffer, text, buffer_len, 0 as size_t);
    }
    return text;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_get_double(
    mut stmt: *mut sqlite3_stmt,
    mut qb: *const query_builder_t,
    mut column_name: *const ::core::ffi::c_char,
    mut default_value: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut idx: ::core::ffi::c_int = qb_get_column_index(qb, column_name);
    if idx < 0 as ::core::ffi::c_int {
        return default_value;
    }
    if sqlite3_column_type(stmt, idx) == SQLITE_NULL {
        return default_value;
    }
    return sqlite3_column_double(stmt, idx);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qb_get_bool(
    mut stmt: *mut sqlite3_stmt,
    mut qb: *const query_builder_t,
    mut column_name: *const ::core::ffi::c_char,
    mut default_value: bool,
) -> bool {
    let mut idx: ::core::ffi::c_int = qb_get_column_index(qb, column_name);
    if idx < 0 as ::core::ffi::c_int {
        return default_value;
    }
    if sqlite3_column_type(stmt, idx) == SQLITE_NULL {
        return default_value;
    }
    return sqlite3_column_int(stmt, idx) != 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
