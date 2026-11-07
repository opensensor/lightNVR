use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
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
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operator_floor_plan_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub parent_plan_uuid: [::core::ffi::c_char; 37],
    pub canvas_width: ::core::ffi::c_int,
    pub canvas_height: ::core::ffi::c_int,
    pub background_mime: [::core::ffi::c_char; 16],
    pub sketch_bytes: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operator_floor_plan_camera_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub rotation: ::core::ffi::c_double,
    pub fov: ::core::ffi::c_double,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_operator_floor_plan_result_t(pub ::core::ffi::c_int);
impl db_operator_floor_plan_result_t {
    pub const DB_OPERATOR_FLOOR_PLAN_OK: Self = Self(0);
    pub const DB_OPERATOR_FLOOR_PLAN_NOT_FOUND: Self = Self(-1);
    pub const DB_OPERATOR_FLOOR_PLAN_INVALID: Self = Self(-2);
    pub const DB_OPERATOR_FLOOR_PLAN_CONFLICT: Self = Self(-3);
    pub const DB_OPERATOR_FLOOR_PLAN_STALE: Self = Self(-4);
    pub const DB_OPERATOR_FLOOR_PLAN_LIMIT: Self = Self(-5);
    pub const DB_OPERATOR_FLOOR_PLAN_ERROR: Self = Self(-6);
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
pub const OPERATOR_FLOOR_PLAN_NAME_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OPERATOR_FLOOR_PLAN_MAX_VISIBLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const OPERATOR_FLOOR_PLAN_MAX_CAMERAS: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const OPERATOR_FLOOR_PLAN_SKETCH_MAX: ::core::ffi::c_int =
    256 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn populate_plan(
    mut statement: *mut sqlite3_stmt,
    mut plan: *mut operator_floor_plan_t,
) {
    memset(
        plan as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<operator_floor_plan_t>(),
    );
    copy_column(
        &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*plan).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*plan).location_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*plan).parent_plan_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        3 as ::core::ffi::c_int,
    );
    (*plan).canvas_width = sqlite3_column_int(statement, 4 as ::core::ffi::c_int);
    (*plan).canvas_height = sqlite3_column_int(statement, 5 as ::core::ffi::c_int);
    copy_column(
        &raw mut (*plan).background_mime as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        statement,
        6 as ::core::ffi::c_int,
    );
    (*plan).revision = sqlite3_column_int64(statement, 7 as ::core::ffi::c_int) as int64_t;
    (*plan).created_at = sqlite3_column_int64(statement, 8 as ::core::ffi::c_int) as int64_t;
    (*plan).updated_at = sqlite3_column_int64(statement, 9 as ::core::ffi::c_int) as int64_t;
    (*plan).sketch_bytes = sqlite3_column_int(statement, 10 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_name(mut name: *const ::core::ffi::c_char) -> bool {
    let mut length: size_t = if !name.is_null() {
        strnlen(name, OPERATOR_FLOOR_PLAN_NAME_MAX as size_t)
    } else {
        0 as size_t
    };
    if length == 0 as size_t || length == OPERATOR_FLOOR_PLAN_NAME_MAX as size_t {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = name as *const ::core::ffi::c_uchar;
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
unsafe extern "C" fn valid_background_mime(
    mut background_mime: *const ::core::ffi::c_char,
) -> bool {
    return *background_mime.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strcmp(
            background_mime,
            b"image/png\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            background_mime,
            b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn valid_sketch(mut sketch_json: *const ::core::ffi::c_char) -> bool {
    return sketch_json.is_null()
        || strnlen(
            sketch_json,
            (OPERATOR_FLOOR_PLAN_SKETCH_MAX + 1 as ::core::ffi::c_int) as size_t,
        ) <= OPERATOR_FLOOR_PLAN_SKETCH_MAX as size_t;
}
unsafe extern "C" fn bind_sketch(
    mut statement: *mut sqlite3_stmt,
    mut index: ::core::ffi::c_int,
    mut sketch_json: *const ::core::ffi::c_char,
) {
    if !sketch_json.is_null() && *sketch_json.offset(0isize) as ::core::ffi::c_int != 0 {
        sqlite3_bind_text(
            statement,
            index,
            sketch_json,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    } else {
        sqlite3_bind_null(statement, index);
    };
}
unsafe extern "C" fn valid_plan(
    mut plan: *const operator_floor_plan_t,
    mut require_uuid: bool,
) -> bool {
    return !plan.is_null()
        && (!require_uuid
            || lightnvr_uuid_is_valid(&raw const (*plan).uuid as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0)
        && valid_name(&raw const (*plan).name as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
        && ((*plan).location_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || lightnvr_uuid_is_valid(
                &raw const (*plan).location_uuid as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0)
        && ((*plan).parent_plan_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || lightnvr_uuid_is_valid(
                &raw const (*plan).parent_plan_uuid as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0)
        && ((*plan).parent_plan_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || (*plan).uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || strcmp(
                &raw const (*plan).uuid as *const ::core::ffi::c_char,
                &raw const (*plan).parent_plan_uuid as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int)
        && (*plan).canvas_width >= 400 as ::core::ffi::c_int
        && (*plan).canvas_width <= 4000 as ::core::ffi::c_int
        && (*plan).canvas_height >= 300 as ::core::ffi::c_int
        && (*plan).canvas_height <= 4000 as ::core::ffi::c_int
        && valid_background_mime(&raw const (*plan).background_mime as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn valid_cameras(
    mut cameras: *const operator_floor_plan_camera_t,
    mut count: ::core::ffi::c_int,
) -> bool {
    if count < 0 as ::core::ffi::c_int
        || count > OPERATOR_FLOOR_PLAN_MAX_CAMERAS
        || count > 0 as ::core::ffi::c_int && cameras.is_null()
    {
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut camera: *const operator_floor_plan_camera_t = cameras.offset(index as isize);
        if !lightnvr_uuid_is_valid(&raw const (*camera).camera_uuid as *const ::core::ffi::c_char)
            || (*camera).x.is_finite() as ::core::ffi::c_int == 0
            || (*camera).x < 0.0f64
            || (*camera).x > 1.0f64
            || (*camera).y.is_finite() as ::core::ffi::c_int == 0
            || (*camera).y < 0.0f64
            || (*camera).y > 1.0f64
            || (*camera).rotation.is_finite() as ::core::ffi::c_int == 0
            || (*camera).rotation < -180.0f64
            || (*camera).rotation > 180.0f64
            || (*camera).fov.is_finite() as ::core::ffi::c_int == 0
            || (*camera).fov < 1.0f64
            || (*camera).fov > 180.0f64
        {
            return r#false != 0;
        }
        let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while previous < index {
            if strcmp(
                &raw const (*cameras.offset(previous as isize)).camera_uuid
                    as *const ::core::ffi::c_char,
                &raw const (*camera).camera_uuid as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            previous += 1;
        }
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn get_locked(
    mut database: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut plan: *mut operator_floor_plan_t,
) -> db_operator_floor_plan_result_t {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"SELECT uuid,name,COALESCE(location_uuid,''),COALESCE(parent_plan_uuid,''),canvas_width,canvas_height,COALESCE(background_mime,''),revision,created_at,updated_at,COALESCE(length(CAST(sketch_json AS BLOB)),0) FROM operator_floor_plans WHERE uuid=? LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
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
    let mut outcome: db_operator_floor_plan_result_t =
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_NOT_FOUND;
    if result == SQLITE_ROW {
        populate_plan(statement, plan);
        outcome = db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK;
    } else if result != SQLITE_DONE {
        outcome = db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return outcome;
}
unsafe extern "C" fn insert_cameras_locked(
    mut database: *mut sqlite3,
    mut plan_uuid: *const ::core::ffi::c_char,
    mut cameras: *const operator_floor_plan_camera_t,
    mut camera_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"INSERT INTO operator_floor_plan_cameras(plan_uuid,camera_uuid,x,y,rotation,fov) VALUES(?,?,?,?,?,?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while result == SQLITE_OK && index < camera_count {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            plan_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw const (*cameras.offset(index as isize)).camera_uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_double(
            statement,
            3 as ::core::ffi::c_int,
            (*cameras.offset(index as isize)).x,
        );
        sqlite3_bind_double(
            statement,
            4 as ::core::ffi::c_int,
            (*cameras.offset(index as isize)).y,
        );
        sqlite3_bind_double(
            statement,
            5 as ::core::ffi::c_int,
            (*cameras.offset(index as isize)).rotation,
        );
        sqlite3_bind_double(
            statement,
            6 as ::core::ffi::c_int,
            (*cameras.offset(index as isize)).fov,
        );
        result = sqlite3_step(statement);
        if result == SQLITE_DONE && (index + 1 as ::core::ffi::c_int) < camera_count {
            result = sqlite3_reset(statement);
            sqlite3_clear_bindings(statement);
        }
        index += 1;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if camera_count == 0 as ::core::ffi::c_int || result == SQLITE_DONE {
        SQLITE_OK
    } else {
        result
    };
}
unsafe extern "C" fn parent_would_cycle_locked(
    mut database: *mut sqlite3,
    mut plan_uuid: *const ::core::ffi::c_char,
    mut parent_uuid: *const ::core::ffi::c_char,
) -> bool {
    if parent_uuid.is_null()
        || *parent_uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"WITH RECURSIVE ancestors(uuid) AS (SELECT ? UNION ALL SELECT p.parent_plan_uuid FROM operator_floor_plans p JOIN ancestors a ON p.uuid=a.uuid WHERE p.parent_plan_uuid IS NOT NULL) SELECT 1 FROM ancestors WHERE uuid=? LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            parent_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            plan_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut cycle: bool = result == SQLITE_ROW || result != SQLITE_DONE;
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return cycle;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_operator_floor_plan_list(
    mut plans: *mut operator_floor_plan_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null()
        || mutex.is_null()
        || plans.is_null()
        || max_count < 1 as ::core::ffi::c_int
        || max_count > OPERATOR_FLOOR_PLAN_MAX_VISIBLE
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"SELECT uuid,name,COALESCE(location_uuid,''),COALESCE(parent_plan_uuid,''),canvas_width,canvas_height,COALESCE(background_mime,''),revision,created_at,updated_at,COALESCE(length(CAST(sketch_json AS BLOB)),0) FROM operator_floor_plans ORDER BY name COLLATE NOCASE,uuid LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
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
            populate_plan(statement, plans.offset(c2rust_fresh0 as isize));
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
pub unsafe extern "C" fn db_operator_floor_plan_get(
    mut uuid: *const ::core::ffi::c_char,
    mut plan: *mut operator_floor_plan_t,
) -> db_operator_floor_plan_result_t {
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() || plan.is_null() || !lightnvr_uuid_is_valid(uuid) {
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_operator_floor_plan_result_t = get_locked(database, uuid, plan);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_operator_floor_plan_camera_list(
    mut plan_uuid: *const ::core::ffi::c_char,
    mut cameras: *mut operator_floor_plan_camera_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null()
        || mutex.is_null()
        || !lightnvr_uuid_is_valid(plan_uuid)
        || cameras.is_null()
        || max_count < 1 as ::core::ffi::c_int
        || max_count > OPERATOR_FLOOR_PLAN_MAX_CAMERAS
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"SELECT camera_uuid,x,y,rotation,fov FROM operator_floor_plan_cameras WHERE plan_uuid=? ORDER BY camera_uuid LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            plan_uuid,
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
            let mut camera: *mut operator_floor_plan_camera_t =
                cameras.offset(c2rust_fresh1 as isize);
            memset(
                camera as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<operator_floor_plan_camera_t>(),
            );
            copy_column(
                &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                statement,
                0 as ::core::ffi::c_int,
            );
            (*camera).x = sqlite3_column_double(statement, 1 as ::core::ffi::c_int);
            (*camera).y = sqlite3_column_double(statement, 2 as ::core::ffi::c_int);
            (*camera).rotation = sqlite3_column_double(statement, 3 as ::core::ffi::c_int);
            (*camera).fov = sqlite3_column_double(statement, 4 as ::core::ffi::c_int);
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
pub unsafe extern "C" fn db_operator_floor_plan_create(
    mut plan: *mut operator_floor_plan_t,
    mut cameras: *const operator_floor_plan_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut sketch_json: *const ::core::ffi::c_char,
) -> db_operator_floor_plan_result_t {
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut name: [::core::ffi::c_char; 128] = [0; 128];
    if database.is_null()
        || mutex.is_null()
        || !valid_plan(plan, r#false != 0)
        || !valid_cameras(cameras, camera_count)
        || !valid_sketch(sketch_json)
        || copy_trimmed_value(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            &raw mut (*plan).name as *mut ::core::ffi::c_char,
            0 as size_t,
        ) == 0 as size_t
        || lightnvr_uuid_generate_v4(&raw mut (*plan).uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"SELECT COUNT(*) FROM operator_floor_plans;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut plan_count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW {
        plan_count = sqlite3_column_int(statement, 0 as ::core::ffi::c_int);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if plan_count < 0 as ::core::ffi::c_int || plan_count >= OPERATOR_FLOOR_PLAN_MAX_VISIBLE {
        pthread_mutex_unlock(mutex);
        return db_operator_floor_plan_result_t(if plan_count < 0 as ::core::ffi::c_int {
            db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_ERROR.0
        } else {
            db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_LIMIT.0
        });
    }
    sqlite3_exec(
        database,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    result = sqlite3_prepare_v2(
        database,
        b"INSERT INTO operator_floor_plans(uuid,name,location_uuid,parent_plan_uuid,canvas_width,canvas_height,sketch_json) VALUES(?,?,?,?,?,?,?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*plan).location_uuid[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                3 as ::core::ffi::c_int,
                &raw mut (*plan).location_uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 3 as ::core::ffi::c_int);
        }
        if (*plan).parent_plan_uuid[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                4 as ::core::ffi::c_int,
                &raw mut (*plan).parent_plan_uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 4 as ::core::ffi::c_int);
        }
        sqlite3_bind_int(statement, 5 as ::core::ffi::c_int, (*plan).canvas_width);
        sqlite3_bind_int(statement, 6 as ::core::ffi::c_int, (*plan).canvas_height);
        bind_sketch(statement, 7 as ::core::ffi::c_int, sketch_json);
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result == SQLITE_DONE {
        result = insert_cameras_locked(
            database,
            &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
            cameras,
            camera_count,
        );
    }
    if result != SQLITE_OK {
        sqlite3_exec(
            database,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_operator_floor_plan_result_t(if result == SQLITE_CONSTRAINT {
            db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_CONFLICT.0
        } else {
            db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_ERROR.0
        });
    }
    sqlite3_exec(
        database,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    safe_strcpy(
        &raw mut (*plan).name as *mut ::core::ffi::c_char,
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    let mut outcome: db_operator_floor_plan_result_t = get_locked(
        database,
        &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
        plan,
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_operator_floor_plan_update(
    mut plan: *mut operator_floor_plan_t,
    mut cameras: *const operator_floor_plan_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut expected_revision: int64_t,
    mut sketch_json: *const ::core::ffi::c_char,
) -> db_operator_floor_plan_result_t {
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut name: [::core::ffi::c_char; 128] = [0; 128];
    if database.is_null()
        || mutex.is_null()
        || expected_revision < 1 as int64_t
        || !valid_plan(plan, r#true != 0)
        || !valid_cameras(cameras, camera_count)
        || !valid_sketch(sketch_json)
        || copy_trimmed_value(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            &raw mut (*plan).name as *mut ::core::ffi::c_char,
            0 as size_t,
        ) == 0 as size_t
    {
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut existing: operator_floor_plan_t = operator_floor_plan_t {
        uuid: [0; 37],
        name: [0; 128],
        location_uuid: [0; 37],
        parent_plan_uuid: [0; 37],
        canvas_width: 0,
        canvas_height: 0,
        background_mime: [0; 16],
        sketch_bytes: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut outcome: db_operator_floor_plan_result_t = get_locked(
        database,
        &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if outcome.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0
        || existing.revision != expected_revision
    {
        pthread_mutex_unlock(mutex);
        return db_operator_floor_plan_result_t(
            if outcome.0 == db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
                db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_STALE.0
            } else {
                outcome.0
            },
        );
    }
    if parent_would_cycle_locked(
        database,
        &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
        &raw mut (*plan).parent_plan_uuid as *mut ::core::ffi::c_char,
    ) {
        pthread_mutex_unlock(mutex);
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_CONFLICT;
    }
    sqlite3_exec(
        database,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        if !sketch_json.is_null() {
            b"UPDATE operator_floor_plans SET name=?,location_uuid=?,parent_plan_uuid=?,canvas_width=?,canvas_height=?,revision=revision+1,updated_at=strftime('%s','now'),sketch_json=? WHERE uuid=? AND revision=?;\0"
                .as_ptr() as *const ::core::ffi::c_char
        } else {
            b"UPDATE operator_floor_plans SET name=?,location_uuid=?,parent_plan_uuid=?,canvas_width=?,canvas_height=?,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND revision=?;\0"
                .as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        let mut next: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*plan).location_uuid[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                &raw mut (*plan).location_uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 2 as ::core::ffi::c_int);
        }
        if (*plan).parent_plan_uuid[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                3 as ::core::ffi::c_int,
                &raw mut (*plan).parent_plan_uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 3 as ::core::ffi::c_int);
        }
        sqlite3_bind_int(statement, 4 as ::core::ffi::c_int, (*plan).canvas_width);
        sqlite3_bind_int(statement, 5 as ::core::ffi::c_int, (*plan).canvas_height);
        if !sketch_json.is_null() {
            let c2rust_fresh2 = next;
            next += 1;
            bind_sketch(statement, c2rust_fresh2, sketch_json);
        }
        let c2rust_fresh3 = next;
        next += 1;
        sqlite3_bind_text(
            statement,
            c2rust_fresh3,
            &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(statement, next, expected_revision as sqlite3_int64);
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(database)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result == SQLITE_DONE && changed != 1 as ::core::ffi::c_int {
        sqlite3_exec(
            database,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_STALE;
    }
    if result == SQLITE_DONE {
        result = sqlite3_prepare_v2(
            database,
            b"DELETE FROM operator_floor_plan_cameras WHERE plan_uuid=?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
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
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if result == SQLITE_DONE {
        result = insert_cameras_locked(
            database,
            &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
            cameras,
            camera_count,
        );
    }
    if result != SQLITE_OK {
        sqlite3_exec(
            database,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_operator_floor_plan_result_t(if result == SQLITE_CONSTRAINT {
            db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_CONFLICT.0
        } else {
            db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_ERROR.0
        });
    }
    sqlite3_exec(
        database,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    safe_strcpy(
        &raw mut (*plan).name as *mut ::core::ffi::c_char,
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    outcome = get_locked(
        database,
        &raw mut (*plan).uuid as *mut ::core::ffi::c_char,
        plan,
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_operator_floor_plan_delete(
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
) -> db_operator_floor_plan_result_t {
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null()
        || mutex.is_null()
        || !lightnvr_uuid_is_valid(uuid)
        || expected_revision < 1 as int64_t
    {
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"DELETE FROM operator_floor_plans WHERE uuid=? AND revision=?;\0".as_ptr()
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
        sqlite3_changes(database)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if result != SQLITE_DONE {
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_ERROR;
    }
    return db_operator_floor_plan_result_t(if changed == 1 as ::core::ffi::c_int {
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0
    } else {
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_STALE.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_operator_floor_plan_set_background(
    mut uuid: *const ::core::ffi::c_char,
    mut background_mime: *const ::core::ffi::c_char,
) -> db_operator_floor_plan_result_t {
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null()
        || mutex.is_null()
        || !lightnvr_uuid_is_valid(uuid)
        || !valid_background_mime(if !background_mime.is_null() {
            background_mime
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        })
    {
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    if !background_mime.is_null()
        && *background_mime.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        background_mime = ::core::ptr::null::<::core::ffi::c_char>();
    }
    pthread_mutex_lock(mutex);
    let mut existing: operator_floor_plan_t = operator_floor_plan_t {
        uuid: [0; 37],
        name: [0; 128],
        location_uuid: [0; 37],
        parent_plan_uuid: [0; 37],
        canvas_width: 0,
        canvas_height: 0,
        background_mime: [0; 16],
        sketch_bytes: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut outcome: db_operator_floor_plan_result_t =
        get_locked(database, uuid, &raw mut existing);
    if outcome.0 == db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
            database,
            b"UPDATE operator_floor_plans SET background_mime=?,updated_at=strftime('%s','now') WHERE uuid=?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            if !background_mime.is_null() {
                sqlite3_bind_text(
                    statement,
                    1 as ::core::ffi::c_int,
                    background_mime,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            } else {
                sqlite3_bind_null(statement, 1 as ::core::ffi::c_int);
            }
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
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
        outcome = db_operator_floor_plan_result_t(if result == SQLITE_DONE {
            db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0
        } else {
            db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_ERROR.0
        });
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_operator_floor_plan_sketch_load(
    mut uuid: *const ::core::ffi::c_char,
    mut sketch_json: *mut *mut ::core::ffi::c_char,
) -> db_operator_floor_plan_result_t {
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if sketch_json.is_null() {
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    *sketch_json = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if database.is_null() || mutex.is_null() || !lightnvr_uuid_is_valid(uuid) {
        return db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"SELECT sketch_json FROM operator_floor_plans WHERE uuid=? LIMIT 1;\0".as_ptr()
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
    let mut outcome: db_operator_floor_plan_result_t =
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_NOT_FOUND;
    if result == SQLITE_ROW {
        outcome = db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK;
        let mut text: *const ::core::ffi::c_uchar =
            sqlite3_column_text(statement, 0 as ::core::ffi::c_int);
        let mut bytes: ::core::ffi::c_int =
            sqlite3_column_bytes(statement, 0 as ::core::ffi::c_int);
        if !text.is_null()
            && bytes > 0 as ::core::ffi::c_int
            && bytes <= OPERATOR_FLOOR_PLAN_SKETCH_MAX
        {
            *sketch_json =
                malloc((bytes as size_t).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
            if !(*sketch_json).is_null() {
                memcpy(
                    *sketch_json as *mut ::core::ffi::c_void,
                    text as *const ::core::ffi::c_void,
                    bytes as size_t,
                );
                *(*sketch_json).offset(bytes as isize) = '\0' as ::core::ffi::c_char;
            } else {
                outcome = db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_ERROR;
            }
        }
    } else if result != SQLITE_DONE {
        outcome = db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
