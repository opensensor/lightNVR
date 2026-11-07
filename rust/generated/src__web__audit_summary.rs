unsafe extern "C" {
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_summary_key_t {
    pub principal_user_id: int64_t,
    pub principal_username: [::core::ffi::c_char; 64],
    pub action: ::core::ffi::c_int,
    pub target_type: [::core::ffi::c_char; 48],
    pub target_uuid: [::core::ffi::c_char; 128],
    pub remote_address: [::core::ffi::c_char; 64],
    pub reserved_before_window_start: [::core::ffi::c_char; 4],
    pub window_start: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_summary_entry_t {
    pub key: audit_summary_key_t,
    pub count: uint64_t,
    pub first_at: int64_t,
    pub last_at: int64_t,
    pub auth_method: [::core::ffi::c_char; 24],
    pub api_token_uuid: [::core::ffi::c_char; 37],
    pub request_id: [::core::ffi::c_char; 65],
    pub method: [::core::ffi::c_char; 16],
    pub path: [::core::ffi::c_char; 512],
    pub decision_source: [::core::ffi::c_char; 32],
    pub explanation: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_summary_sample_t {
    pub auth_method: *const ::core::ffi::c_char,
    pub api_token_uuid: *const ::core::ffi::c_char,
    pub request_id: *const ::core::ffi::c_char,
    pub method: *const ::core::ffi::c_char,
    pub path: *const ::core::ffi::c_char,
    pub decision_source: *const ::core::ffi::c_char,
    pub explanation: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct audit_summary_add_result_t(pub ::core::ffi::c_uint);
impl audit_summary_add_result_t {
    pub const AUDIT_SUMMARY_ADDED: Self = Self(0);
    pub const AUDIT_SUMMARY_FULL: Self = Self(1);
    pub const AUDIT_SUMMARY_UNAVAILABLE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct summary_slot_t {
    pub used: bool,
    pub entry: audit_summary_entry_t,
}
pub const AUDIT_SUMMARY_WINDOW_SECONDS: ::core::ffi::c_int = 900 as ::core::ffi::c_int;
static mut summary_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut slots: *mut summary_slot_t = ::core::ptr::null_mut::<summary_slot_t>();
static mut scratch: *mut summary_slot_t = ::core::ptr::null_mut::<summary_slot_t>();
static mut slot_capacity: size_t = 0 as size_t;
static mut used_count: size_t = 0 as size_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_summary_window_start(mut now: int64_t) -> int64_t {
    if now < 0 as int64_t {
        return 0 as int64_t;
    }
    return now - now % AUDIT_SUMMARY_WINDOW_SECONDS as int64_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_summary_key_init(mut key: *mut audit_summary_key_t) {
    if !key.is_null() {
        memset(
            key as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<audit_summary_key_t>(),
        );
    }
}
unsafe extern "C" fn key_hash(mut key: *const audit_summary_key_t) -> uint64_t {
    let mut bytes: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
    let mut hash: uint64_t = 1469598103934665603 as uint64_t;
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<audit_summary_key_t>() {
        hash ^= *bytes.offset(i as isize) as uint64_t;
        hash = (hash as ::core::ffi::c_ulonglong)
            .wrapping_mul(1099511628211 as ::core::ffi::c_ulonglong) as uint64_t;
        i = i.wrapping_add(1);
    }
    return hash;
}
unsafe extern "C" fn find_slot_locked(
    mut key: *const audit_summary_key_t,
    mut found: *mut bool,
) -> *mut summary_slot_t {
    *found = r#false != 0;
    let mut start: size_t = key_hash(key).wrapping_rem(slot_capacity as uint64_t) as size_t;
    let mut probe: size_t = 0 as size_t;
    while probe < slot_capacity {
        let mut slot: *mut summary_slot_t =
            slots.offset(start.wrapping_add(probe).wrapping_rem(slot_capacity) as isize);
        if !(*slot).used {
            return slot;
        }
        if memcmp(
            &raw mut (*slot).entry.key as *const ::core::ffi::c_void,
            key as *const ::core::ffi::c_void,
            ::core::mem::size_of::<audit_summary_key_t>(),
        ) == 0 as ::core::ffi::c_int
        {
            *found = r#true != 0;
            return slot;
        }
        probe = probe.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<summary_slot_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_summary_init(mut capacity: size_t) -> ::core::ffi::c_int {
    if capacity == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut new_slots: *mut summary_slot_t =
        calloc(capacity, ::core::mem::size_of::<summary_slot_t>()) as *mut summary_slot_t;
    let mut new_scratch: *mut summary_slot_t =
        calloc(capacity, ::core::mem::size_of::<summary_slot_t>()) as *mut summary_slot_t;
    if new_slots.is_null() || new_scratch.is_null() {
        free(new_slots as *mut ::core::ffi::c_void);
        free(new_scratch as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut summary_mutex);
    free(slots as *mut ::core::ffi::c_void);
    free(scratch as *mut ::core::ffi::c_void);
    slots = new_slots;
    scratch = new_scratch;
    slot_capacity = capacity;
    used_count = 0 as size_t;
    pthread_mutex_unlock(&raw mut summary_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_summary_shutdown() {
    pthread_mutex_lock(&raw mut summary_mutex);
    free(slots as *mut ::core::ffi::c_void);
    free(scratch as *mut ::core::ffi::c_void);
    slots = ::core::ptr::null_mut::<summary_slot_t>();
    scratch = ::core::ptr::null_mut::<summary_slot_t>();
    slot_capacity = 0 as size_t;
    used_count = 0 as size_t;
    pthread_mutex_unlock(&raw mut summary_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_summary_pending() -> size_t {
    pthread_mutex_lock(&raw mut summary_mutex);
    let mut pending: size_t = used_count;
    pthread_mutex_unlock(&raw mut summary_mutex);
    return pending;
}
unsafe extern "C" fn copy_sample_field(
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut src: *const ::core::ffi::c_char,
) {
    safe_strcpy(
        dst,
        if !src.is_null() {
            src
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        size,
        0 as size_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_summary_add(
    mut key: *const audit_summary_key_t,
    mut now: int64_t,
    mut sample: *const audit_summary_sample_t,
) -> audit_summary_add_result_t {
    if key.is_null() {
        return audit_summary_add_result_t::AUDIT_SUMMARY_UNAVAILABLE;
    }
    pthread_mutex_lock(&raw mut summary_mutex);
    if slots.is_null() {
        pthread_mutex_unlock(&raw mut summary_mutex);
        return audit_summary_add_result_t::AUDIT_SUMMARY_UNAVAILABLE;
    }
    let mut found: bool = r#false != 0;
    let mut slot: *mut summary_slot_t = find_slot_locked(key, &raw mut found);
    if found {
        (*slot).entry.count = (*slot).entry.count.wrapping_add(1);
        if now > (*slot).entry.last_at {
            (*slot).entry.last_at = now;
        }
        pthread_mutex_unlock(&raw mut summary_mutex);
        return audit_summary_add_result_t::AUDIT_SUMMARY_ADDED;
    }
    if slot.is_null() {
        pthread_mutex_unlock(&raw mut summary_mutex);
        return audit_summary_add_result_t::AUDIT_SUMMARY_FULL;
    }
    memset(
        slot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<summary_slot_t>(),
    );
    (*slot).used = r#true != 0;
    (*slot).entry.key = *key;
    (*slot).entry.count = 1 as uint64_t;
    (*slot).entry.first_at = now;
    (*slot).entry.last_at = now;
    if !sample.is_null() {
        copy_sample_field(
            &raw mut (*slot).entry.auth_method as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            (*sample).auth_method,
        );
        copy_sample_field(
            &raw mut (*slot).entry.api_token_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            (*sample).api_token_uuid,
        );
        copy_sample_field(
            &raw mut (*slot).entry.request_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
            (*sample).request_id,
        );
        copy_sample_field(
            &raw mut (*slot).entry.method as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            (*sample).method,
        );
        copy_sample_field(
            &raw mut (*slot).entry.path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            (*sample).path,
        );
        copy_sample_field(
            &raw mut (*slot).entry.decision_source as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            (*sample).decision_source,
        );
        copy_sample_field(
            &raw mut (*slot).entry.explanation as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            (*sample).explanation,
        );
    }
    used_count = used_count.wrapping_add(1);
    pthread_mutex_unlock(&raw mut summary_mutex);
    return audit_summary_add_result_t::AUDIT_SUMMARY_ADDED;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_summary_drain(
    mut all: bool,
    mut current_window_start: int64_t,
    mut out: *mut audit_summary_entry_t,
    mut out_capacity: size_t,
) -> size_t {
    pthread_mutex_lock(&raw mut summary_mutex);
    if slots.is_null() || out.is_null() || out_capacity == 0 as size_t {
        pthread_mutex_unlock(&raw mut summary_mutex);
        return 0 as size_t;
    }
    if used_count == 0 as size_t {
        pthread_mutex_unlock(&raw mut summary_mutex);
        return 0 as size_t;
    }
    let mut drained: size_t = 0 as size_t;
    let mut kept: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < slot_capacity {
        if (*slots.offset(i as isize)).used {
            let mut take: bool = all as ::core::ffi::c_int != 0
                || (*slots.offset(i as isize)).entry.key.window_start != current_window_start;
            if take as ::core::ffi::c_int != 0 && drained < out_capacity {
                let c2rust_fresh0 = drained;
                drained = drained.wrapping_add(1);
                *out.offset(c2rust_fresh0 as isize) = (*slots.offset(i as isize)).entry;
            } else {
                let c2rust_fresh1 = kept;
                kept = kept.wrapping_add(1);
                *scratch.offset(c2rust_fresh1 as isize) = *slots.offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
    }
    memset(
        slots as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        slot_capacity.wrapping_mul(::core::mem::size_of::<summary_slot_t>()),
    );
    used_count = 0 as size_t;
    let mut i_0: size_t = 0 as size_t;
    while i_0 < kept {
        let mut found: bool = r#false != 0;
        let mut slot: *mut summary_slot_t = find_slot_locked(
            &raw mut (*scratch.offset(i_0 as isize)).entry.key,
            &raw mut found,
        );
        *slot = *scratch.offset(i_0 as isize);
        used_count = used_count.wrapping_add(1);
        i_0 = i_0.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut summary_mutex);
    return drained;
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
