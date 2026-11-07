use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn getrandom(
        __buffer: *mut ::core::ffi::c_void,
        __length: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> ssize_t;
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
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
    pub const PTHREAD_MUTEX_TIMED_NP: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE_NP: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK_NP: Self = Self(2);
    pub const PTHREAD_MUTEX_ADAPTIVE_NP: Self = Self(3);
    pub const PTHREAD_MUTEX_NORMAL: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK: Self = Self(2);
    pub const PTHREAD_MUTEX_DEFAULT: Self = Self(0);
    pub const PTHREAD_MUTEX_FAST_NP: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct batch_delete_status_t(pub ::core::ffi::c_uint);
impl batch_delete_status_t {
    pub const BATCH_DELETE_STATUS_PENDING: Self = Self(0);
    pub const BATCH_DELETE_STATUS_RUNNING: Self = Self(1);
    pub const BATCH_DELETE_STATUS_COMPLETE: Self = Self(2);
    pub const BATCH_DELETE_STATUS_ERROR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct batch_delete_progress_t {
    pub job_id: [::core::ffi::c_char; 64],
    pub status: batch_delete_status_t,
    pub total: ::core::ffi::c_int,
    pub current: ::core::ffi::c_int,
    pub succeeded: ::core::ffi::c_int,
    pub pending_deletions: ::core::ffi::c_int,
    pub failed: ::core::ffi::c_int,
    pub status_message: [::core::ffi::c_char; 256],
    pub error_message: [::core::ffi::c_char; 256],
    pub created_at: time_t,
    pub updated_at: time_t,
    pub owner_user_id: int64_t,
    pub owner_api_token_uuid: [::core::ffi::c_char; 37],
    pub is_active: bool,
}
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RecordingsAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn generate_uuid_v4(mut uuid_str: *mut ::core::ffi::c_char) {
    let mut uuid: [::core::ffi::c_uchar; 16] = [0; 16];
    if getrandom(
        &raw mut uuid as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        16 as size_t,
        0 as ::core::ffi::c_uint,
    ) < 0 as ssize_t
    {
        let mut f: *mut FILE = fopen(
            b"/dev/urandom\0".as_ptr() as *const ::core::ffi::c_char,
            b"rb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !f.is_null() {
            fread(
                &raw mut uuid as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                1 as size_t,
                16 as size_t,
                f,
            );
            fclose(f);
        }
    }
    uuid[6usize] = (uuid[6usize] as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
        | 0x40 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    uuid[8usize] = (uuid[8usize] as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
        | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    snprintf(
        uuid_str,
        37 as size_t,
        b"%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x\0".as_ptr()
            as *const ::core::ffi::c_char,
        uuid[0usize] as ::core::ffi::c_int,
        uuid[1usize] as ::core::ffi::c_int,
        uuid[2usize] as ::core::ffi::c_int,
        uuid[3usize] as ::core::ffi::c_int,
        uuid[4usize] as ::core::ffi::c_int,
        uuid[5usize] as ::core::ffi::c_int,
        uuid[6usize] as ::core::ffi::c_int,
        uuid[7usize] as ::core::ffi::c_int,
        uuid[8usize] as ::core::ffi::c_int,
        uuid[9usize] as ::core::ffi::c_int,
        uuid[10usize] as ::core::ffi::c_int,
        uuid[11usize] as ::core::ffi::c_int,
        uuid[12usize] as ::core::ffi::c_int,
        uuid[13usize] as ::core::ffi::c_int,
        uuid[14usize] as ::core::ffi::c_int,
        uuid[15usize] as ::core::ffi::c_int,
    );
}
pub const MAX_BATCH_DELETE_JOBS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const JOB_RETENTION_TIME: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
static mut g_jobs: [batch_delete_progress_t; 10] = [batch_delete_progress_t {
    job_id: [0; 64],
    status: batch_delete_status_t::BATCH_DELETE_STATUS_PENDING,
    total: 0,
    current: 0,
    succeeded: 0,
    pending_deletions: 0,
    failed: 0,
    status_message: [0; 256],
    error_message: [0; 256],
    created_at: 0,
    updated_at: 0,
    owner_user_id: 0,
    owner_api_token_uuid: [0; 37],
    is_active: false,
}; 10];
static mut g_jobs_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut g_initialized: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_init() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    if g_initialized {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        return 0 as ::core::ffi::c_int;
    }
    memset(
        &raw mut g_jobs as *mut batch_delete_progress_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[batch_delete_progress_t; 10]>(),
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_BATCH_DELETE_JOBS {
        g_jobs[i as usize].is_active = r#false != 0;
        i += 1;
    }
    g_initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch delete progress tracking initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_cleanup() {
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    if !g_initialized {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        return;
    }
    memset(
        &raw mut g_jobs as *mut batch_delete_progress_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[batch_delete_progress_t; 10]>(),
    );
    g_initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch delete progress tracking cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn find_available_slot() -> ::core::ffi::c_int {
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_BATCH_DELETE_JOBS {
        if !g_jobs[i as usize].is_active {
            return i;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_BATCH_DELETE_JOBS {
        if (g_jobs[i_0 as usize].status.0 == batch_delete_status_t::BATCH_DELETE_STATUS_COMPLETE.0
            || g_jobs[i_0 as usize].status.0 == batch_delete_status_t::BATCH_DELETE_STATUS_ERROR.0)
            && now - g_jobs[i_0 as usize].updated_at > JOB_RETENTION_TIME as time_t
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Reusing slot %d for new job (old job: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i_0,
                &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(i_0 as isize))
                    .job_id as *mut ::core::ffi::c_char,
            );
            return i_0;
        }
        i_0 += 1;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn find_job_by_id(mut job_id: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if job_id.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_BATCH_DELETE_JOBS {
        if g_jobs[i as usize].is_active as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(i as isize))
                    .job_id as *mut ::core::ffi::c_char,
                job_id,
            ) == 0 as ::core::ffi::c_int
        {
            return i;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_create_job_for_principal(
    mut total: ::core::ffi::c_int,
    mut owner_user_id: int64_t,
    mut owner_api_token_uuid: *const ::core::ffi::c_char,
    mut job_id_out: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete progress tracking not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if job_id_out.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid job_id_out parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    let mut slot: ::core::ffi::c_int = find_available_slot();
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"No available slots for new batch delete job\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    generate_uuid_v4(
        &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize)).job_id
            as *mut ::core::ffi::c_char,
    );
    g_jobs[slot as usize].status = batch_delete_status_t::BATCH_DELETE_STATUS_PENDING;
    g_jobs[slot as usize].total = total;
    g_jobs[slot as usize].current = 0 as ::core::ffi::c_int;
    g_jobs[slot as usize].succeeded = 0 as ::core::ffi::c_int;
    g_jobs[slot as usize].pending_deletions = 0 as ::core::ffi::c_int;
    g_jobs[slot as usize].failed = 0 as ::core::ffi::c_int;
    snprintf(
        &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize))
            .status_message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"Preparing to delete %d recordings...\0".as_ptr() as *const ::core::ffi::c_char,
        total,
    );
    g_jobs[slot as usize].error_message[0usize] = '\0' as ::core::ffi::c_char;
    g_jobs[slot as usize].created_at = time(::core::ptr::null_mut::<time_t>());
    g_jobs[slot as usize].updated_at = g_jobs[slot as usize].created_at;
    g_jobs[slot as usize].owner_user_id = owner_user_id;
    safe_strcpy(
        &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize))
            .owner_api_token_uuid as *mut ::core::ffi::c_char,
        if !owner_api_token_uuid.is_null() {
            owner_api_token_uuid
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    g_jobs[slot as usize].is_active = r#true != 0;
    safe_strcpy(
        job_id_out,
        &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize)).job_id
            as *mut ::core::ffi::c_char,
        64 as size_t,
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Created batch delete job: %s (total: %d)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize)).job_id
            as *mut ::core::ffi::c_char,
        total,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_create_job(
    mut total: ::core::ffi::c_int,
    mut job_id_out: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return batch_delete_progress_create_job_for_principal(
        total,
        0 as int64_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
        job_id_out,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_set_total(
    mut job_id: *const ::core::ffi::c_char,
    mut total: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete progress tracking not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if job_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid job_id parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    let mut slot: ::core::ffi::c_int = find_job_by_id(job_id);
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Job not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            job_id,
        );
        return -1 as ::core::ffi::c_int;
    }
    g_jobs[slot as usize].total = total;
    g_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Updated total for job %s: %d\0".as_ptr() as *const ::core::ffi::c_char,
        job_id,
        total,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_update(
    mut job_id: *const ::core::ffi::c_char,
    mut current: ::core::ffi::c_int,
    mut succeeded: ::core::ffi::c_int,
    mut failed: ::core::ffi::c_int,
    mut status_message: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete progress tracking not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if job_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid job_id parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    let mut slot: ::core::ffi::c_int = find_job_by_id(job_id);
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Job not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            job_id,
        );
        return -1 as ::core::ffi::c_int;
    }
    g_jobs[slot as usize].status = batch_delete_status_t::BATCH_DELETE_STATUS_RUNNING;
    g_jobs[slot as usize].current = current;
    g_jobs[slot as usize].succeeded = succeeded;
    g_jobs[slot as usize].failed = failed;
    if !status_message.is_null() {
        safe_strcpy(
            &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize))
                .status_message as *mut ::core::ffi::c_char,
            status_message,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    g_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_complete(
    mut job_id: *const ::core::ffi::c_char,
    mut succeeded: ::core::ffi::c_int,
    mut failed: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return batch_delete_progress_complete_with_pending(
        job_id,
        succeeded,
        failed,
        0 as ::core::ffi::c_int,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_complete_with_pending(
    mut job_id: *const ::core::ffi::c_char,
    mut succeeded: ::core::ffi::c_int,
    mut failed: ::core::ffi::c_int,
    mut pending: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete progress tracking not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if job_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid job_id parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    let mut slot: ::core::ffi::c_int = find_job_by_id(job_id);
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Job not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            job_id,
        );
        return -1 as ::core::ffi::c_int;
    }
    g_jobs[slot as usize].pending_deletions = pending;
    g_jobs[slot as usize].status = batch_delete_status_t::BATCH_DELETE_STATUS_COMPLETE;
    g_jobs[slot as usize].current = g_jobs[slot as usize].total;
    g_jobs[slot as usize].succeeded = succeeded;
    g_jobs[slot as usize].failed = failed;
    snprintf(
        &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize))
            .status_message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        if pending != 0 {
            b"Deletion requests accepted; storage cleanup continues in background\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"Batch delete operation complete\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    g_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch delete job completed: %s (succeeded: %d, failed: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        job_id,
        succeeded,
        failed,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_error(
    mut job_id: *const ::core::ffi::c_char,
    mut error_message: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete progress tracking not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if job_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid job_id parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    let mut slot: ::core::ffi::c_int = find_job_by_id(job_id);
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Job not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            job_id,
        );
        return -1 as ::core::ffi::c_int;
    }
    g_jobs[slot as usize].status = batch_delete_status_t::BATCH_DELETE_STATUS_ERROR;
    if !error_message.is_null() {
        safe_strcpy(
            &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize))
                .error_message as *mut ::core::ffi::c_char,
            error_message,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        snprintf(
            &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize))
                .status_message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            error_message,
        );
    } else {
        snprintf(
            &raw mut (*(&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize))
                .status_message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Error: Unknown error\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    g_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch delete job failed: %s (%s)\0".as_ptr() as *const ::core::ffi::c_char,
        job_id,
        if !error_message.is_null() {
            error_message
        } else {
            b"Unknown error\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_get(
    mut job_id: *const ::core::ffi::c_char,
    mut progress_out: *mut batch_delete_progress_t,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete progress tracking not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if job_id.is_null() || progress_out.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    let mut slot: ::core::ffi::c_int = find_job_by_id(job_id);
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Job not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            job_id,
        );
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        progress_out as *mut ::core::ffi::c_void,
        (&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize)
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<batch_delete_progress_t>(),
    );
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn batch_delete_progress_delete(
    mut job_id: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete progress tracking not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if job_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid job_id parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_jobs_mutex);
    let mut slot: ::core::ffi::c_int = find_job_by_id(job_id);
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut g_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Job not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            job_id,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        (&raw mut g_jobs as *mut batch_delete_progress_t).offset(slot as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<batch_delete_progress_t>(),
    );
    g_jobs[slot as usize].is_active = r#false != 0;
    pthread_mutex_unlock(&raw mut g_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Deleted batch delete job: %s\0".as_ptr() as *const ::core::ffi::c_char,
        job_id,
    );
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
