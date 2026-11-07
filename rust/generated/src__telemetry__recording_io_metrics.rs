unsafe extern "C" {
    unsafe fn system_health_evaluator_note_immediate(
        condition: system_health_condition_t,
        resource: system_health_immediate_resource_t,
    );
}
pub type size_t = usize;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct recording_io_resource_t(pub ::core::ffi::c_uint);
impl recording_io_resource_t {
    pub const RECORDING_IO_RESOURCE_RECORDING: Self = Self(0);
    pub const RECORDING_IO_RESOURCE_HLS: Self = Self(1);
    pub const RECORDING_IO_RESOURCE_COUNT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct recording_io_operation_t(pub ::core::ffi::c_uint);
impl recording_io_operation_t {
    pub const RECORDING_IO_OPERATION_ALLOCATE: Self = Self(0);
    pub const RECORDING_IO_OPERATION_OPEN: Self = Self(1);
    pub const RECORDING_IO_OPERATION_HEADER: Self = Self(2);
    pub const RECORDING_IO_OPERATION_PACKET: Self = Self(3);
    pub const RECORDING_IO_OPERATION_TRAILER: Self = Self(4);
    pub const RECORDING_IO_OPERATION_CLOSE: Self = Self(5);
    pub const RECORDING_IO_OPERATION_FILESYSTEM: Self = Self(6);
    pub const RECORDING_IO_OPERATION_COUNT: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct recording_io_reason_t(pub ::core::ffi::c_uint);
impl recording_io_reason_t {
    pub const RECORDING_IO_REASON_NONE: Self = Self(0);
    pub const RECORDING_IO_REASON_NO_SPACE: Self = Self(1);
    pub const RECORDING_IO_REASON_QUOTA: Self = Self(2);
    pub const RECORDING_IO_REASON_READ_ONLY: Self = Self(3);
    pub const RECORDING_IO_REASON_IO: Self = Self(4);
    pub const RECORDING_IO_REASON_TIMEOUT: Self = Self(5);
    pub const RECORDING_IO_REASON_FD_LIMIT: Self = Self(6);
    pub const RECORDING_IO_REASON_ALLOCATION: Self = Self(7);
    pub const RECORDING_IO_REASON_OTHER: Self = Self(8);
    pub const RECORDING_IO_REASON_COUNT: Self = Self(9);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_io_last_error_t {
    pub valid: bool,
    pub resource: recording_io_resource_t,
    pub operation: recording_io_operation_t,
    pub reason: recording_io_reason_t,
    pub error_code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_io_metrics_snapshot_t {
    pub reason_totals: [[uint64_t; 9]; 2],
    pub last_error: recording_io_last_error_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_immediate_resource_t(pub ::core::ffi::c_uint);
impl system_health_immediate_resource_t {
    pub const SYSTEM_HEALTH_IMMEDIATE_PROCESS: Self = Self(0);
    pub const SYSTEM_HEALTH_IMMEDIATE_ROOT: Self = Self(1);
    pub const SYSTEM_HEALTH_IMMEDIATE_RECORDING: Self = Self(2);
    pub const SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_condition_t(pub ::core::ffi::c_uint);
impl system_health_condition_t {
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW: Self = Self(0);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL: Self = Self(1);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_SWAP_THRASH: Self = Self(2);
    pub const SYSTEM_HEALTH_CONDITION_CPU_SATURATION: Self = Self(3);
    pub const SYSTEM_HEALTH_CONDITION_CPU_THROTTLED: Self = Self(4);
    pub const SYSTEM_HEALTH_CONDITION_IO_PRESSURE: Self = Self(5);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW: Self = Self(6);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW: Self = Self(7);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY: Self = Self(8);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED: Self = Self(9);
    pub const SYSTEM_HEALTH_CONDITION_THERMAL_HIGH: Self = Self(10);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN: Self = Self(11);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE: Self = Self(12);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED: Self = Self(13);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_JUMP: Self = Self(14);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION: Self = Self(15);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION: Self = Self(16);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED: Self = Self(17);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL: Self = Self(18);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL: Self = Self(19);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED: Self = Self(20);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE: Self = Self(21);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED: Self = Self(22);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE: Self = Self(23);
    pub const SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE: Self = Self(24);
    pub const SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART: Self = Self(25);
    pub const SYSTEM_HEALTH_CONDITION_EVENT_DELIVERY_DEGRADED: Self = Self(26);
    pub const SYSTEM_HEALTH_CONDITION_COUNT: Self = Self(27);
}
pub type atomic_bool = bool;
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
pub const EIO: ::core::ffi::c_int = 5;
pub const ENOMEM: ::core::ffi::c_int = 12;
pub const ENFILE: ::core::ffi::c_int = 23;
pub const EMFILE: ::core::ffi::c_int = 24;
pub const ENOSPC: ::core::ffi::c_int = 28;
pub const EROFS: ::core::ffi::c_int = 30;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
static mut reason_totals: [[uint32_t; 9]; 2] = [[0; 9]; 2];
static mut last_error_packed: uint32_t = 0;
static mut device_refresh_requested: atomic_bool = false;
pub const LAST_VALID_SHIFT: ::core::ffi::c_uint = 31 as ::core::ffi::c_uint;
pub const LAST_RESOURCE_SHIFT: ::core::ffi::c_uint = 28 as ::core::ffi::c_uint;
pub const LAST_OPERATION_SHIFT: ::core::ffi::c_uint = 24 as ::core::ffi::c_uint;
pub const LAST_REASON_SHIFT: ::core::ffi::c_uint = 20 as ::core::ffi::c_uint;
pub const LAST_ERROR_MASK: ::core::ffi::c_uint = 0xfffff as ::core::ffi::c_uint;
unsafe extern "C" fn normalized_errno(mut error_code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if error_code >= 0 as ::core::ffi::c_int {
        return error_code;
    }
    if error_code == INT_MIN {
        return 0 as ::core::ffi::c_int;
    }
    return -error_code;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_io_reason_from_error(
    mut error_code: ::core::ffi::c_int,
) -> recording_io_reason_t {
    let mut value: ::core::ffi::c_int = normalized_errno(error_code);
    match value {
        ENOSPC => return recording_io_reason_t::RECORDING_IO_REASON_NO_SPACE,
        EDQUOT => return recording_io_reason_t::RECORDING_IO_REASON_QUOTA,
        EROFS => return recording_io_reason_t::RECORDING_IO_REASON_READ_ONLY,
        EIO => return recording_io_reason_t::RECORDING_IO_REASON_IO,
        ETIMEDOUT => return recording_io_reason_t::RECORDING_IO_REASON_TIMEOUT,
        EMFILE | ENFILE => return recording_io_reason_t::RECORDING_IO_REASON_FD_LIMIT,
        ENOMEM => return recording_io_reason_t::RECORDING_IO_REASON_ALLOCATION,
        _ => return recording_io_reason_t::RECORDING_IO_REASON_OTHER,
    };
}
unsafe extern "C" fn increment_saturating(mut counter: *mut uint32_t) {
    let mut current: uint32_t = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::Relaxed },
    >(counter);
    while current != UINT32_MAX as uint32_t && {
        let c2rust_result = ::core::intrinsics::atomic_cxchgweak::<
            _,
            { ::core::intrinsics::AtomicOrdering::Relaxed },
            { ::core::intrinsics::AtomicOrdering::Relaxed },
        >(
            counter,
            *&raw mut current,
            current.wrapping_add(1 as uint32_t),
        );
        *&raw mut current = c2rust_result.0;
        !c2rust_result.1
    } {}
}
unsafe extern "C" fn notify_evaluator(
    mut resource: recording_io_resource_t,
    mut reason: recording_io_reason_t,
) {
    let mut condition: system_health_condition_t =
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW;
    match reason {
        recording_io_reason_t::RECORDING_IO_REASON_NO_SPACE
        | recording_io_reason_t::RECORDING_IO_REASON_QUOTA => {
            condition = system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW;
        }
        recording_io_reason_t::RECORDING_IO_REASON_READ_ONLY => {
            condition = system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY;
        }
        recording_io_reason_t::RECORDING_IO_REASON_IO
        | recording_io_reason_t::RECORDING_IO_REASON_TIMEOUT => {
            condition = system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED;
        }
        recording_io_reason_t::RECORDING_IO_REASON_FD_LIMIT => {
            system_health_evaluator_note_immediate(
                system_health_condition_t::SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION,
                system_health_immediate_resource_t::SYSTEM_HEALTH_IMMEDIATE_PROCESS,
            );
            return;
        }
        recording_io_reason_t::RECORDING_IO_REASON_ALLOCATION => {
            system_health_evaluator_note_immediate(
                system_health_condition_t::SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED,
                system_health_immediate_resource_t::SYSTEM_HEALTH_IMMEDIATE_PROCESS,
            );
            return;
        }
        _ => return,
    }
    system_health_evaluator_note_immediate(
        condition,
        system_health_immediate_resource_t::SYSTEM_HEALTH_IMMEDIATE_RECORDING,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_io_report_failure(
    mut resource: recording_io_resource_t,
    mut operation: recording_io_operation_t,
    mut error_code: ::core::ffi::c_int,
) {
    if resource.0 < 0 as ::core::ffi::c_uint
        || resource.0 >= recording_io_resource_t::RECORDING_IO_RESOURCE_COUNT.0
        || operation.0 < 0 as ::core::ffi::c_uint
        || operation.0 >= recording_io_operation_t::RECORDING_IO_OPERATION_COUNT.0
    {
        return;
    }
    let mut reason: recording_io_reason_t = recording_io_reason_from_error(error_code);
    increment_saturating(
        (&raw mut *(&raw mut reason_totals as *mut [uint32_t; 9]).offset(resource.0 as isize)
            as *mut uint32_t)
            .offset(reason.0 as isize),
    );
    let mut positive_error: uint32_t = normalized_errno(error_code) as uint32_t;
    if positive_error > LAST_ERROR_MASK as uint32_t {
        positive_error = LAST_ERROR_MASK as uint32_t;
    }
    let mut packed: uint32_t = (1 as uint32_t) << LAST_VALID_SHIFT
        | (resource.0 as uint32_t) << LAST_RESOURCE_SHIFT
        | (operation.0 as uint32_t) << LAST_OPERATION_SHIFT
        | (reason.0 as uint32_t) << LAST_REASON_SHIFT
        | positive_error;
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::Release }>(
        &raw mut last_error_packed,
        packed,
    );
    if reason.0 == recording_io_reason_t::RECORDING_IO_REASON_IO.0
        || reason.0 == recording_io_reason_t::RECORDING_IO_REASON_TIMEOUT.0
    {
        ::core::sync::atomic::AtomicBool::from_ptr(&raw mut device_refresh_requested).store(r#true != 0, ::core::sync::atomic::Ordering::Release);
    }
    notify_evaluator(resource, reason);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_io_take_device_refresh_request() -> bool {
    return ::core::sync::atomic::AtomicBool::from_ptr(&raw mut device_refresh_requested).swap(r#false != 0, ::core::sync::atomic::Ordering::AcqRel);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_io_metrics_snapshot(
    mut snapshot: *mut recording_io_metrics_snapshot_t,
) {
    if snapshot.is_null() {
        return;
    }
    let mut packed: uint32_t = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::Acquire },
    >(&raw mut last_error_packed);
    let mut resource: size_t = 0 as size_t;
    while resource
        < recording_io_resource_t::RECORDING_IO_RESOURCE_COUNT.0 as ::core::ffi::c_int as size_t
    {
        let mut reason: size_t = 0 as size_t;
        while reason
            < recording_io_reason_t::RECORDING_IO_REASON_COUNT.0 as ::core::ffi::c_int as size_t
        {
            (*snapshot).reason_totals[resource][reason] = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::Relaxed },
            >(
                (&raw mut *(&raw mut reason_totals as *mut [uint32_t; 9]).offset(resource as isize)
                    as *mut uint32_t)
                    .offset(reason as isize),
            ) as uint64_t;
            reason = reason.wrapping_add(1);
        }
        resource = resource.wrapping_add(1);
    }
    (*snapshot).last_error.valid = packed >> LAST_VALID_SHIFT & 1 as uint32_t != 0 as uint32_t;
    (*snapshot).last_error.resource = recording_io_resource_t(
        (packed >> LAST_RESOURCE_SHIFT & 0x7 as uint32_t) as ::core::ffi::c_uint,
    );
    (*snapshot).last_error.operation = recording_io_operation_t(
        (packed >> LAST_OPERATION_SHIFT & 0xf as uint32_t) as ::core::ffi::c_uint,
    );
    (*snapshot).last_error.reason = recording_io_reason_t(
        (packed >> LAST_REASON_SHIFT & 0xf as uint32_t) as ::core::ffi::c_uint,
    );
    (*snapshot).last_error.error_code =
        (packed & LAST_ERROR_MASK as uint32_t) as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_io_reason_name(
    mut reason: recording_io_reason_t,
) -> *const ::core::ffi::c_char {
    static mut names: [*const ::core::ffi::c_char; 9] = [
        b"none\0".as_ptr() as *const ::core::ffi::c_char,
        b"no_space\0".as_ptr() as *const ::core::ffi::c_char,
        b"quota\0".as_ptr() as *const ::core::ffi::c_char,
        b"read_only\0".as_ptr() as *const ::core::ffi::c_char,
        b"io\0".as_ptr() as *const ::core::ffi::c_char,
        b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
        b"fd_limit\0".as_ptr() as *const ::core::ffi::c_char,
        b"allocation\0".as_ptr() as *const ::core::ffi::c_char,
        b"other\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if reason.0 < 0 as ::core::ffi::c_uint
        || reason.0 >= recording_io_reason_t::RECORDING_IO_REASON_COUNT.0
    {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return names[reason.0 as usize];
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110;
pub const EDQUOT: ::core::ffi::c_int = 122;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
