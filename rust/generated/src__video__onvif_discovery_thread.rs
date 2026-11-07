unsafe extern "C" {
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn inet_ntoa(__in: in_addr) -> *mut ::core::ffi::c_char;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    static mut g_discovery_mutex: pthread_mutex_t;
    unsafe fn parse_network(
        network: *const ::core::ffi::c_char,
        base_addr: *mut uint32_t,
        subnet_mask: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn send_discovery_probe(ip_addr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn receive_discovery_responses(
        devices: *mut onvif_device_info_t,
        max_devices: ::core::ffi::c_int,
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
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type time_t = __time_t;
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct discovery_thread_t {
    pub thread: pthread_t,
    pub running: ::core::ffi::c_int,
    pub network: [::core::ffi::c_char; 64],
    pub interval: ::core::ffi::c_int,
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
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_DISCOVERED_DEVICES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut g_discovery_thread: discovery_thread_t = discovery_thread_t {
    thread: 0 as pthread_t,
    running: 0,
    network: [0; 64],
    interval: 0,
};
static mut g_discovered_devices: [onvif_device_info_t; 32] = [onvif_device_info_t {
    inventory_uuid: [0; 37],
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
}; 32];
static mut g_discovered_device_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn discovery_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut thread_data: *mut discovery_thread_t = arg as *mut discovery_thread_t;
    let mut base_addr: uint32_t = 0;
    let mut subnet_mask: uint32_t = 0;
    let mut ip_addr: [::core::ffi::c_char; 16] = [0; 16];
    let mut addr: in_addr = in_addr { s_addr: 0 };
    let mut devices: [onvif_device_info_t; 32] = [onvif_device_info_t {
        inventory_uuid: [0; 37],
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
    }; 32];
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF discovery thread started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if parse_network(
        &raw mut (*thread_data).network as *mut ::core::ffi::c_char,
        &raw mut base_addr,
        &raw mut subnet_mask,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse network: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*thread_data).network as *mut ::core::ffi::c_char,
        );
        return NULL;
    }
    while (*thread_data).running != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Starting ONVIF discovery scan on network %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*thread_data).network as *mut ::core::ffi::c_char,
        );
        let mut network: uint32_t = base_addr & subnet_mask;
        let mut broadcast: uint32_t = network | !subnet_mask;
        let mut ip: uint32_t = network.wrapping_add(1 as uint32_t);
        while ip < broadcast && (*thread_data).running != 0 {
            addr.s_addr = __bswap_32(ip) as in_addr_t;
            safe_strcpy(
                &raw mut ip_addr as *mut ::core::ffi::c_char,
                inet_ntoa(addr),
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            send_discovery_probe(&raw mut ip_addr as *mut ::core::ffi::c_char);
            usleep(10000 as __useconds_t);
            ip = ip.wrapping_add(1);
        }
        addr.s_addr = __bswap_32(broadcast) as in_addr_t;
        safe_strcpy(
            &raw mut ip_addr as *mut ::core::ffi::c_char,
            inet_ntoa(addr),
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sending multiple discovery probes to broadcast address: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut ip_addr as *mut ::core::ffi::c_char,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int {
            send_discovery_probe(&raw mut ip_addr as *mut ::core::ffi::c_char);
            usleep(100000 as ::core::ffi::c_int as __useconds_t);
            i += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sending discovery probe to ONVIF multicast address: 239.255.255.250\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        send_discovery_probe(b"239.255.255.250\0".as_ptr() as *const ::core::ffi::c_char);
        let mut count: ::core::ffi::c_int = receive_discovery_responses(
            &raw mut devices as *mut onvif_device_info_t,
            MAX_DISCOVERED_DEVICES,
        );
        pthread_mutex_lock(&raw mut g_discovery_mutex);
        memset(
            &raw mut g_discovered_devices as *mut onvif_device_info_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[onvif_device_info_t; 32]>(),
        );
        g_discovered_device_count = 0 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < count && i_0 < MAX_DISCOVERED_DEVICES {
            memcpy(
                (&raw mut g_discovered_devices as *mut onvif_device_info_t).offset(i_0 as isize)
                    as *mut ::core::ffi::c_void,
                (&raw mut devices as *mut onvif_device_info_t).offset(i_0 as isize)
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<onvif_device_info_t>(),
            );
            g_discovered_device_count += 1;
            i_0 += 1;
        }
        pthread_mutex_unlock(&raw mut g_discovery_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF discovery scan completed, found %d devices\0".as_ptr()
                as *const ::core::ffi::c_char,
            count,
        );
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*thread_data).interval && (*thread_data).running != 0 {
            sleep(1 as ::core::ffi::c_uint);
            i_1 += 1;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF discovery thread stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
