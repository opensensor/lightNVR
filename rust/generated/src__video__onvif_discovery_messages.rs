unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[unsafe(no_mangle)]
pub static mut ONVIF_DISCOVERY_MSG: *const ::core::ffi::c_char = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><e:Envelope xmlns:e=\"http://www.w3.org/2003/05/soap-envelope\" xmlns:w=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" xmlns:d=\"http://schemas.xmlsoap.org/ws/2005/04/discovery\" xmlns:dn=\"http://www.onvif.org/ver10/network/wsdl\"><e:Header><w:MessageID>uuid:%s</w:MessageID><w:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</w:To><w:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</w:Action></e:Header><e:Body><d:Probe><d:Types>dn:NetworkVideoTransmitter</d:Types></d:Probe></e:Body></e:Envelope>\0"
    .as_ptr() as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub static mut ONVIF_DISCOVERY_MSG_ALT: *const ::core::ffi::c_char = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" xmlns:a=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" xmlns:d=\"http://schemas.xmlsoap.org/ws/2005/04/discovery\"><s:Header><a:MessageID>uuid:%s</a:MessageID><a:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</a:To><a:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</a:Action></s:Header><s:Body><d:Probe><d:Types xmlns:tds=\"http://www.onvif.org/ver10/device/wsdl\">tds:Device</d:Types></d:Probe></s:Body></s:Envelope>\0"
    .as_ptr() as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub static mut ONVIF_DISCOVERY_MSG_WITH_SCOPE: *const ::core::ffi::c_char = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><e:Envelope xmlns:e=\"http://www.w3.org/2003/05/soap-envelope\" xmlns:w=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" xmlns:d=\"http://schemas.xmlsoap.org/ws/2005/04/discovery\" xmlns:dn=\"http://www.onvif.org/ver10/network/wsdl\"><e:Header><w:MessageID>uuid:%s</w:MessageID><w:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</w:To><w:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</w:Action></e:Header><e:Body><d:Probe><d:Types>dn:NetworkVideoTransmitter</d:Types><d:Scopes>onvif://www.onvif.org</d:Scopes></d:Probe></e:Body></e:Envelope>\0"
    .as_ptr() as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn generate_uuid(mut uuid: *mut ::core::ffi::c_char, mut size: size_t) {
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut seed: ::core::ffi::c_uint = now as ::core::ffi::c_uint;
    let mut r1: ::core::ffi::c_uint =
        seed ^ 0x12345678 as ::core::ffi::c_int as ::core::ffi::c_uint;
    let mut r2: ::core::ffi::c_uint =
        seed >> 8 as ::core::ffi::c_int ^ 0x87654321 as ::core::ffi::c_uint;
    let mut r3: ::core::ffi::c_uint =
        seed >> 16 as ::core::ffi::c_int ^ 0xabcdef01 as ::core::ffi::c_uint;
    let mut r4: ::core::ffi::c_uint =
        seed >> 24 as ::core::ffi::c_int ^ 0x10fedcba as ::core::ffi::c_int as ::core::ffi::c_uint;
    let mut p1: ::core::ffi::c_uint = r1
        .wrapping_mul(1103515245 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_add(12345 as ::core::ffi::c_uint)
        & 0xffff as ::core::ffi::c_uint;
    let mut p2: ::core::ffi::c_uint = r2
        .wrapping_mul(1103515245 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_add(12345 as ::core::ffi::c_uint)
        & 0xffff as ::core::ffi::c_uint;
    let mut p3: ::core::ffi::c_uint = r3
        .wrapping_mul(1103515245 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_add(12345 as ::core::ffi::c_uint)
        & 0xffff as ::core::ffi::c_uint;
    let mut p4: ::core::ffi::c_uint = r4
        .wrapping_mul(1103515245 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_add(12345 as ::core::ffi::c_uint)
        & 0xffff as ::core::ffi::c_uint;
    snprintf(
        uuid,
        size,
        b"%04x%04x-%04x-%04x-%04x-%04x%04x%04x\0".as_ptr() as *const ::core::ffi::c_char,
        p1,
        p2,
        p3,
        p4 & 0xfff as ::core::ffi::c_uint | 0x4000 as ::core::ffi::c_uint,
        r1 & 0x3fff as ::core::ffi::c_uint | 0x8000 as ::core::ffi::c_uint,
        r2 & 0xffff as ::core::ffi::c_uint,
        r3 & 0xffff as ::core::ffi::c_uint,
        r4 & 0xffff as ::core::ffi::c_uint,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
