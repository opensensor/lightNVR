use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_device_info_t {
    pub inventory_uuid: [::core::ffi::c_char; 37],
    pub endpoint: [::core::ffi::c_char; 512],
    pub device_service: [::core::ffi::c_char; 512],
    pub media_service: [::core::ffi::c_char; 512],
    pub ptz_service: [::core::ffi::c_char; 512],
    pub imaging_service: [::core::ffi::c_char; 512],
    pub manufacturer: [::core::ffi::c_char; 64],
    pub model: [::core::ffi::c_char; 64],
    pub firmware_version: [::core::ffi::c_char; 32],
    pub serial_number: [::core::ffi::c_char; 64],
    pub hardware_id: [::core::ffi::c_char; 64],
    pub ip_address: [::core::ffi::c_char; 64],
    pub mac_address: [::core::ffi::c_char; 32],
    pub discovery_time: time_t,
    pub first_seen_at: time_t,
    pub last_seen_at: time_t,
    pub last_scan_network: [::core::ffi::c_char; 64],
    pub claim_state: [::core::ffi::c_char; 16],
    pub claimed_camera_uuid: [::core::ffi::c_char; 37],
    pub duplicate_suspected: bool,
    pub online: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_onvif_inventory_result_t(pub ::core::ffi::c_int);
impl db_onvif_inventory_result_t {
    pub const DB_ONVIF_INVENTORY_OK: Self = Self(0);
    pub const DB_ONVIF_INVENTORY_NOT_FOUND: Self = Self(-1);
    pub const DB_ONVIF_INVENTORY_INVALID: Self = Self(-2);
    pub const DB_ONVIF_INVENTORY_CONFLICT: Self = Self(-3);
    pub const DB_ONVIF_INVENTORY_ERROR: Self = Self(-4);
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const MAX_URL_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const ONVIF_DISCOVERY_INVENTORY_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const ONVIF_DISCOVERY_ADDRESS_MAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
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
        size,
        0 as size_t,
    );
}
unsafe extern "C" fn populate(
    mut statement: *mut sqlite3_stmt,
    mut device: *mut onvif_device_info_t,
) {
    memset(
        device as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<onvif_device_info_t>(),
    );
    copy_column(
        &raw mut (*device).inventory_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).endpoint as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).device_service as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).media_service as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        3 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).ptz_service as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        4 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).imaging_service as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        5 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).manufacturer as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        6 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).model as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        7 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).firmware_version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        statement,
        8 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).serial_number as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        9 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).hardware_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        10 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).ip_address as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        11 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).mac_address as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        statement,
        12 as ::core::ffi::c_int,
    );
    (*device).first_seen_at = sqlite3_column_int64(statement, 13 as ::core::ffi::c_int) as time_t;
    (*device).last_seen_at = sqlite3_column_int64(statement, 14 as ::core::ffi::c_int) as time_t;
    (*device).discovery_time = (*device).last_seen_at;
    copy_column(
        &raw mut (*device).last_scan_network as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        15 as ::core::ffi::c_int,
    );
    (*device).online =
        sqlite3_column_int(statement, 16 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    copy_column(
        &raw mut (*device).claim_state as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        statement,
        17 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*device).claimed_camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        18 as ::core::ffi::c_int,
    );
    (*device).duplicate_suspected =
        sqlite3_column_int(statement, 19 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn device_endpoint(
    mut device: *const onvif_device_info_t,
) -> *const ::core::ffi::c_char {
    if (*device).endpoint[0usize] != 0 {
        return &raw const (*device).endpoint as *const ::core::ffi::c_char;
    }
    return if (*device).device_service[0usize] as ::core::ffi::c_int != 0 {
        &raw const (*device).device_service as *const ::core::ffi::c_char
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
unsafe extern "C" fn find_uuid_locked(
    mut db: *mut sqlite3,
    mut column: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
    mut uuid: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if value.is_null() || *value.offset(0isize) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut sql: [::core::ffi::c_char; 192] = [0; 192];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
        b"SELECT uuid FROM onvif_discovery_inventory WHERE %s=? ORDER BY last_seen_at DESC LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        column,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 192]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            value,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_ROW {
        copy_column(
            uuid,
            CAMERA_UUID_STRING_SIZE as size_t,
            statement,
            0 as ::core::ffi::c_int,
        );
        found = 1 as ::core::ffi::c_int;
    } else if result != SQLITE_DONE {
        found = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return found;
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut device: *mut onvif_device_info_t,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"SELECT i.uuid,i.endpoint,i.device_service,i.media_service,i.ptz_service,i.imaging_service,i.manufacturer,i.model,i.firmware_version,i.serial_number,i.hardware_id,i.ip_address,i.mac_address,i.first_seen_at,i.last_seen_at,i.last_scan_network,i.online,i.claim_state,COALESCE(i.claimed_camera_uuid,''),i.duplicate_suspected FROM onvif_discovery_inventory i WHERE i.uuid=? LIMIT 1;\0"
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
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_ROW {
        populate(statement, device);
        found = 1 as ::core::ffi::c_int;
    } else if result != SQLITE_DONE {
        found = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return found;
}
unsafe extern "C" fn load_stable_identity_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut serial: *mut ::core::ffi::c_char,
    mut mac: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT serial_number,mac_address FROM onvif_discovery_inventory WHERE uuid=?;\0".as_ptr()
            as *const ::core::ffi::c_char;
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
    if result == SQLITE_ROW {
        copy_column(serial, 64 as size_t, statement, 0 as ::core::ffi::c_int);
        copy_column(mac, 32 as size_t, statement, 1 as ::core::ffi::c_int);
        result = SQLITE_OK;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn mark_duplicate_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if uuid.is_null() || *uuid.offset(0isize) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE onvif_discovery_inventory SET duplicate_suspected=1,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=?;\0"
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
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn store_address_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut r#type: *const ::core::ffi::c_char,
    mut address: *const ::core::ffi::c_char,
    mut observed_at: int64_t,
) -> ::core::ffi::c_int {
    if address.is_null() || *address.offset(0isize) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO onvif_discovery_addresses(inventory_uuid,address_type,address,first_seen_at,last_seen_at) VALUES(?,?,?,?,?) ON CONFLICT(inventory_uuid,address_type,address) DO UPDATE SET last_seen_at=excluded.last_seen_at;\0"
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
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            r#type,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            address,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            4 as ::core::ffi::c_int,
            observed_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            5 as ::core::ffi::c_int,
            observed_at as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn update_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut device: *const onvif_device_info_t,
    mut endpoint: *const ::core::ffi::c_char,
    mut network: *const ::core::ffi::c_char,
    mut observed_at: int64_t,
    mut duplicate: bool,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"UPDATE onvif_discovery_inventory SET endpoint=?,device_service=?,media_service=COALESCE(NULLIF(?,''),media_service),ptz_service=COALESCE(NULLIF(?,''),ptz_service),imaging_service=COALESCE(NULLIF(?,''),imaging_service),manufacturer=COALESCE(NULLIF(?,''),manufacturer),model=COALESCE(NULLIF(?,''),model),firmware_version=COALESCE(NULLIF(?,''),firmware_version),serial_number=COALESCE(NULLIF(?,''),serial_number),hardware_id=COALESCE(NULLIF(?,''),hardware_id),ip_address=COALESCE(NULLIF(?,''),ip_address),mac_address=COALESCE(NULLIF(?,''),mac_address),last_seen_at=?,last_scan_network=?,online=1,duplicate_suspected=MAX(duplicate_suspected,?),revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=?;\0"
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
            endpoint,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            if (*device).device_service[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*device).device_service as *const ::core::ffi::c_char
            } else {
                endpoint
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw const (*device).media_service as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            &raw const (*device).ptz_service as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            &raw const (*device).imaging_service as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw const (*device).manufacturer as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            7 as ::core::ffi::c_int,
            &raw const (*device).model as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw const (*device).firmware_version as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw const (*device).serial_number as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            &raw const (*device).hardware_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            11 as ::core::ffi::c_int,
            &raw const (*device).ip_address as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            12 as ::core::ffi::c_int,
            &raw const (*device).mac_address as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            13 as ::core::ffi::c_int,
            observed_at as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            14 as ::core::ffi::c_int,
            network,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            15 as ::core::ffi::c_int,
            if duplicate as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            16 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE && sqlite3_changes(db) == 1 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn insert_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut device: *const onvif_device_info_t,
    mut endpoint: *const ::core::ffi::c_char,
    mut network: *const ::core::ffi::c_char,
    mut observed_at: int64_t,
    mut duplicate: bool,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO onvif_discovery_inventory(uuid,endpoint,device_service,media_service,ptz_service,imaging_service,manufacturer,model,firmware_version,serial_number,hardware_id,ip_address,mac_address,first_seen_at,last_seen_at,last_scan_network,online,duplicate_suspected) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,1,?);\0"
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
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            endpoint,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            if (*device).device_service[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*device).device_service as *const ::core::ffi::c_char
            } else {
                endpoint
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            &raw const (*device).media_service as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            &raw const (*device).ptz_service as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw const (*device).imaging_service as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            7 as ::core::ffi::c_int,
            &raw const (*device).manufacturer as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw const (*device).model as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw const (*device).firmware_version as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            &raw const (*device).serial_number as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            11 as ::core::ffi::c_int,
            &raw const (*device).hardware_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            12 as ::core::ffi::c_int,
            &raw const (*device).ip_address as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            13 as ::core::ffi::c_int,
            &raw const (*device).mac_address as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            14 as ::core::ffi::c_int,
            observed_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            15 as ::core::ffi::c_int,
            observed_at as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            16 as ::core::ffi::c_int,
            network,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            17 as ::core::ffi::c_int,
            if duplicate as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn record_device_locked(
    mut db: *mut sqlite3,
    mut network: *const ::core::ffi::c_char,
    mut device: *mut onvif_device_info_t,
    mut observed_at: int64_t,
) -> ::core::ffi::c_int {
    let mut endpoint: *const ::core::ffi::c_char = device_endpoint(device);
    if endpoint.is_null()
        || !valid_text(endpoint, MAX_URL_LENGTH as size_t, r#true != 0)
        || !valid_text(
            &raw mut (*device).device_service as *mut ::core::ffi::c_char,
            MAX_URL_LENGTH as size_t,
            r#false != 0,
        )
        || !valid_text(
            &raw mut (*device).serial_number as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            r#false != 0,
        )
        || !valid_text(
            &raw mut (*device).mac_address as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            r#false != 0,
        )
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut serial_uuid: [::core::ffi::c_char; 37] = [
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
    ];
    let mut mac_uuid: [::core::ffi::c_char; 37] = [
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
    ];
    let mut endpoint_uuid: [::core::ffi::c_char; 37] = [
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
    ];
    let mut serial_found: ::core::ffi::c_int = find_uuid_locked(
        db,
        b"serial_number\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).serial_number as *mut ::core::ffi::c_char,
        &raw mut serial_uuid as *mut ::core::ffi::c_char,
    );
    let mut mac_found: ::core::ffi::c_int = find_uuid_locked(
        db,
        b"mac_address\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).mac_address as *mut ::core::ffi::c_char,
        &raw mut mac_uuid as *mut ::core::ffi::c_char,
    );
    let mut endpoint_found: ::core::ffi::c_int = find_uuid_locked(
        db,
        b"endpoint\0".as_ptr() as *const ::core::ffi::c_char,
        endpoint,
        &raw mut endpoint_uuid as *mut ::core::ffi::c_char,
    );
    if serial_found < 0 as ::core::ffi::c_int
        || mac_found < 0 as ::core::ffi::c_int
        || endpoint_found < 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut duplicate: bool = r#false != 0;
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    ];
    if serial_found != 0 {
        safe_strcpy(
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut serial_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    } else if mac_found != 0 {
        safe_strcpy(
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut mac_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    if serial_found != 0
        && mac_found != 0
        && strcmp(
            &raw mut serial_uuid as *mut ::core::ffi::c_char,
            &raw mut mac_uuid as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        duplicate = r#true != 0;
        if mark_duplicate_locked(db, &raw mut serial_uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            || mark_duplicate_locked(db, &raw mut mac_uuid as *mut ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
    }
    if uuid[0usize] == 0 && endpoint_found != 0 {
        let mut existing_serial: [::core::ffi::c_char; 64] = [
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
        ];
        let mut existing_mac: [::core::ffi::c_char; 32] = [
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
        ];
        if load_stable_identity_locked(
            db,
            &raw mut endpoint_uuid as *mut ::core::ffi::c_char,
            &raw mut existing_serial as *mut ::core::ffi::c_char,
            &raw mut existing_mac as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut serial_conflict: bool = (*device).serial_number[0usize] as ::core::ffi::c_int != 0
            && existing_serial[0usize] as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*device).serial_number as *mut ::core::ffi::c_char,
                &raw mut existing_serial as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int;
        let mut mac_conflict: bool = (*device).mac_address[0usize] as ::core::ffi::c_int != 0
            && existing_mac[0usize] as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*device).mac_address as *mut ::core::ffi::c_char,
                &raw mut existing_mac as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int;
        if serial_conflict as ::core::ffi::c_int != 0 || mac_conflict as ::core::ffi::c_int != 0 {
            duplicate = r#true != 0;
            if mark_duplicate_locked(db, &raw mut endpoint_uuid as *mut ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
        } else {
            safe_strcpy(
                &raw mut uuid as *mut ::core::ffi::c_char,
                &raw mut endpoint_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
    }
    if uuid[0usize] as ::core::ffi::c_int != 0
        && endpoint_found != 0
        && strcmp(
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut endpoint_uuid as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        duplicate = r#true != 0;
        if mark_duplicate_locked(db, &raw mut uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            || mark_duplicate_locked(db, &raw mut endpoint_uuid as *mut ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
    }
    if uuid[0usize] == 0
        && lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut c2rust_lvalue: onvif_device_info_t = onvif_device_info_t {
        inventory_uuid: [
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
        ],
        endpoint: [0; 512],
        device_service: [0; 512],
        media_service: [0; 512],
        ptz_service: [0; 512],
        imaging_service: [0; 512],
        manufacturer: [0; 64],
        model: [0; 64],
        firmware_version: [0; 32],
        serial_number: [0; 64],
        hardware_id: [0; 64],
        ip_address: [0; 64],
        mac_address: [0; 32],
        discovery_time: 0,
        first_seen_at: 0,
        last_seen_at: 0,
        last_scan_network: [0; 64],
        claim_state: [0; 16],
        claimed_camera_uuid: [0; 37],
        duplicate_suspected: false,
        online: false,
    };
    let mut result: ::core::ffi::c_int = get_locked(
        db,
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut c2rust_lvalue,
    );
    if result < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if result == 1 as ::core::ffi::c_int {
        if update_locked(
            db,
            &raw mut uuid as *mut ::core::ffi::c_char,
            device,
            endpoint,
            network,
            observed_at,
            duplicate,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
    } else if insert_locked(
        db,
        &raw mut uuid as *mut ::core::ffi::c_char,
        device,
        endpoint,
        network,
        observed_at,
        duplicate,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if store_address_locked(
        db,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"endpoint\0".as_ptr() as *const ::core::ffi::c_char,
        endpoint,
        observed_at,
    ) != 0 as ::core::ffi::c_int
        || store_address_locked(
            db,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*device).device_service as *mut ::core::ffi::c_char,
            observed_at,
        ) != 0 as ::core::ffi::c_int
        || store_address_locked(
            db,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"ip\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*device).ip_address as *mut ::core::ffi::c_char,
            observed_at,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return if get_locked(db, &raw mut uuid as *mut ::core::ffi::c_char, device)
        == 1 as ::core::ffi::c_int
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_onvif_inventory_record_scan(
    mut scan_network: *const ::core::ffi::c_char,
    mut devices: *mut onvif_device_info_t,
    mut count: ::core::ffi::c_int,
    mut observed_at: int64_t,
) -> ::core::ffi::c_int {
    let mut network: *const ::core::ffi::c_char =
        if !scan_network.is_null() && *scan_network.offset(0isize) as ::core::ffi::c_int != 0 {
            scan_network
        } else {
            b"auto\0".as_ptr() as *const ::core::ffi::c_char
        };
    if count < 0 as ::core::ffi::c_int
        || count > ONVIF_DISCOVERY_INVENTORY_MAX
        || count > 0 as ::core::ffi::c_int && devices.is_null()
        || observed_at <= 0 as int64_t
        || !valid_text(network, 64 as size_t, r#true != 0)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int = sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if result == SQLITE_OK {
        let mut offline_sql: *const ::core::ffi::c_char = b"UPDATE onvif_discovery_inventory SET online=0,revision=revision+1,updated_at=strftime('%s','now') WHERE last_scan_network=? AND online=1;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        result = sqlite3_prepare_v2(
            db,
            offline_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                network,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            result = sqlite3_step(statement);
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while result == SQLITE_DONE && index < count {
        result = if record_device_locked(db, network, devices.offset(index as isize), observed_at)
            == 0 as ::core::ffi::c_int
        {
            SQLITE_DONE
        } else {
            SQLITE_ERROR
        };
        index += 1;
    }
    let mut final_result: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        )
    } else {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        )
    };
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE && final_result == SQLITE_OK {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_onvif_inventory_list(
    mut devices: *mut onvif_device_info_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if devices.is_null()
        || max_count < 1 as ::core::ffi::c_int
        || max_count > ONVIF_DISCOVERY_INVENTORY_MAX
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT i.uuid,i.endpoint,i.device_service,i.media_service,i.ptz_service,i.imaging_service,i.manufacturer,i.model,i.firmware_version,i.serial_number,i.hardware_id,i.ip_address,i.mac_address,i.first_seen_at,i.last_seen_at,i.last_scan_network,i.online,i.claim_state,COALESCE(i.claimed_camera_uuid,''),i.duplicate_suspected FROM onvif_discovery_inventory i ORDER BY i.online DESC,CASE i.claim_state WHEN 'unclaimed' THEN 0 WHEN 'claimed' THEN 1 ELSE 2 END,i.last_seen_at DESC,i.uuid LIMIT ?;\0"
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
    if result == SQLITE_OK {
        sqlite3_bind_int(statement, 1 as ::core::ffi::c_int, max_count);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_OK {
        while count < max_count && {
            result = sqlite3_step(statement);
            result == SQLITE_ROW
        } {
            let c2rust_fresh0 = count;
            count += 1;
            populate(statement, devices.offset(c2rust_fresh0 as isize));
        }
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
pub unsafe extern "C" fn db_onvif_inventory_list_addresses(
    mut inventory_uuid: *const ::core::ffi::c_char,
    mut addresses: *mut [::core::ffi::c_char; 512],
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !lightnvr_uuid_is_valid(inventory_uuid)
        || addresses.is_null()
        || max_count < 1 as ::core::ffi::c_int
        || max_count > ONVIF_DISCOVERY_ADDRESS_MAX
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT address FROM onvif_discovery_addresses WHERE inventory_uuid=? ORDER BY last_seen_at DESC,address LIMIT ?;\0"
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
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            inventory_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(statement, 2 as ::core::ffi::c_int, max_count);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_OK {
        while count < max_count && {
            result = sqlite3_step(statement);
            result == SQLITE_ROW
        } {
            let c2rust_fresh1 = count;
            count += 1;
            copy_column(
                &raw mut *addresses.offset(c2rust_fresh1 as isize) as *mut ::core::ffi::c_char,
                MAX_URL_LENGTH as size_t,
                statement,
                0 as ::core::ffi::c_int,
            );
        }
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
pub unsafe extern "C" fn db_onvif_inventory_claim_stream(
    mut inventory_uuid: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
) -> db_onvif_inventory_result_t {
    if !lightnvr_uuid_is_valid(inventory_uuid)
        || !valid_text(stream_name, MAX_STREAM_NAME as size_t, r#true != 0)
    {
        return db_onvif_inventory_result_t::DB_ONVIF_INVENTORY_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_onvif_inventory_result_t::DB_ONVIF_INVENTORY_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE onvif_discovery_inventory SET claim_state='claimed',claimed_camera_uuid=(SELECT camera_uuid FROM streams WHERE name=?),revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND EXISTS(SELECT 1 FROM streams WHERE name=?);\0"
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
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            inventory_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
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
    if result != SQLITE_DONE {
        return db_onvif_inventory_result_t::DB_ONVIF_INVENTORY_ERROR;
    }
    return db_onvif_inventory_result_t(if changed == 1 as ::core::ffi::c_int {
        db_onvif_inventory_result_t::DB_ONVIF_INVENTORY_OK.0
    } else {
        db_onvif_inventory_result_t::DB_ONVIF_INVENTORY_NOT_FOUND.0
    });
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
