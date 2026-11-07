unsafe extern "C" {
    pub type AVOptionRanges;
    pub type AVOption;
    unsafe fn avio_alloc_context(
        buffer: *mut ::core::ffi::c_uchar,
        buffer_size: ::core::ffi::c_int,
        write_flag: ::core::ffi::c_int,
        opaque: *mut ::core::ffi::c_void,
        read_packet: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        write_packet: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        seek: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, int64_t, ::core::ffi::c_int) -> int64_t,
        >,
    ) -> *mut AVIOContext;
    unsafe fn avio_context_free(s: *mut *mut AVIOContext);
    unsafe fn avio_flush(s: *mut AVIOContext);
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off_t,
        __whence: ::core::ffi::c_int,
    ) -> __off_t;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn av_free(ptr: *mut ::core::ffi::c_void);
    unsafe fn av_freep(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = isize;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = u8;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVClassCategory(pub ::core::ffi::c_uint);
impl AVClassCategory {
    pub const AV_CLASS_CATEGORY_NA: Self = Self(0);
    pub const AV_CLASS_CATEGORY_INPUT: Self = Self(1);
    pub const AV_CLASS_CATEGORY_OUTPUT: Self = Self(2);
    pub const AV_CLASS_CATEGORY_MUXER: Self = Self(3);
    pub const AV_CLASS_CATEGORY_DEMUXER: Self = Self(4);
    pub const AV_CLASS_CATEGORY_ENCODER: Self = Self(5);
    pub const AV_CLASS_CATEGORY_DECODER: Self = Self(6);
    pub const AV_CLASS_CATEGORY_FILTER: Self = Self(7);
    pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: Self = Self(8);
    pub const AV_CLASS_CATEGORY_SWSCALER: Self = Self(9);
    pub const AV_CLASS_CATEGORY_SWRESAMPLER: Self = Self(10);
    pub const AV_CLASS_CATEGORY_HWDEVICE: Self = Self(11);
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: Self = Self(40);
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: Self = Self(41);
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: Self = Self(42);
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: Self = Self(43);
    pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: Self = Self(44);
    pub const AV_CLASS_CATEGORY_DEVICE_INPUT: Self = Self(45);
    pub const AV_CLASS_CATEGORY_NB: Self = Self(46);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const ::core::ffi::c_char,
    pub item_name:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char>,
    pub option: *const AVOption,
    pub version: ::core::ffi::c_int,
    pub log_level_offset_offset: ::core::ffi::c_int,
    pub parent_log_context_offset: ::core::ffi::c_int,
    pub category: AVClassCategory,
    pub get_category: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory>,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub child_next: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub child_class_iterate:
        Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass>,
    pub state_flags_offset: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVIODataMarkerType(pub ::core::ffi::c_uint);
impl AVIODataMarkerType {
    pub const AVIO_DATA_MARKER_HEADER: Self = Self(0);
    pub const AVIO_DATA_MARKER_SYNC_POINT: Self = Self(1);
    pub const AVIO_DATA_MARKER_BOUNDARY_POINT: Self = Self(2);
    pub const AVIO_DATA_MARKER_UNKNOWN: Self = Self(3);
    pub const AVIO_DATA_MARKER_TRAILER: Self = Self(4);
    pub const AVIO_DATA_MARKER_FLUSH_POINT: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut ::core::ffi::c_uchar,
    pub buffer_size: ::core::ffi::c_int,
    pub buf_ptr: *mut ::core::ffi::c_uchar,
    pub buf_end: *mut ::core::ffi::c_uchar,
    pub opaque: *mut ::core::ffi::c_void,
    pub read_packet: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut uint8_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub write_packet: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const uint8_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, int64_t, ::core::ffi::c_int) -> int64_t,
    >,
    pub pos: int64_t,
    pub eof_reached: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub write_flag: ::core::ffi::c_int,
    pub max_packet_size: ::core::ffi::c_int,
    pub min_packet_size: ::core::ffi::c_int,
    pub checksum: ::core::ffi::c_ulong,
    pub checksum_ptr: *mut ::core::ffi::c_uchar,
    pub update_checksum: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_ulong,
            *const uint8_t,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_ulong,
    >,
    pub read_pause: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub read_seek: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            int64_t,
            ::core::ffi::c_int,
        ) -> int64_t,
    >,
    pub seekable: ::core::ffi::c_int,
    pub direct: ::core::ffi::c_int,
    pub protocol_whitelist: *const ::core::ffi::c_char,
    pub protocol_blacklist: *const ::core::ffi::c_char,
    pub write_data_type: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const uint8_t,
            ::core::ffi::c_int,
            AVIODataMarkerType,
            int64_t,
        ) -> ::core::ffi::c_int,
    >,
    pub ignore_boundary_point: ::core::ffi::c_int,
    pub buf_ptr_max: *mut ::core::ffi::c_uchar,
    pub bytes_read: int64_t,
    pub bytes_written: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub const AVSEEK_SIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const AVSEEK_FORCE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
unsafe extern "C" fn write_file(
    mut opaque: *mut ::core::ffi::c_void,
    mut buffer: *const uint8_t,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = *(opaque as *mut ::core::ffi::c_int);
    let mut done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while done < size {
        let mut count: ssize_t = write(
            fd,
            buffer.offset(done as isize) as *const ::core::ffi::c_void,
            (size - done) as size_t,
        );
        if count < 0 as ssize_t && *__errno_location() == EINTR {
            continue;
        }
        if count <= 0 as ssize_t {
            return -if count < 0 as ssize_t {
                *__errno_location()
            } else {
                5 as ::core::ffi::c_int
            };
        }
        done += count as ::core::ffi::c_int;
    }
    return done;
}
unsafe extern "C" fn seek_file(
    mut opaque: *mut ::core::ffi::c_void,
    mut offset: int64_t,
    mut whence: ::core::ffi::c_int,
) -> int64_t {
    let mut fd: ::core::ffi::c_int = *(opaque as *mut ::core::ffi::c_int);
    if whence == AVSEEK_SIZE {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        return if fstat(fd, &raw mut st) == 0 as ::core::ffi::c_int {
            st.st_size as int64_t
        } else {
            -*__errno_location() as int64_t
        };
    }
    let mut result: off_t = lseek(fd, offset as __off_t, whence & !AVSEEK_FORCE);
    return if result < 0 as off_t {
        -*__errno_location() as int64_t
    } else {
        result as int64_t
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_file_open(
    mut context: *mut *mut AVIOContext,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if context.is_null() || !(*context).is_null() || path.is_null() || *path == 0 {
        return -22 as ::core::ffi::c_int;
    }
    let mut fd: *mut ::core::ffi::c_int =
        av_malloc(::core::mem::size_of::<::core::ffi::c_int>()) as *mut ::core::ffi::c_int;
    let mut buffer: *mut ::core::ffi::c_uchar =
        av_malloc(32768 as size_t) as *mut ::core::ffi::c_uchar;
    if fd.is_null() || buffer.is_null() {
        av_free(fd as *mut ::core::ffi::c_void);
        av_free(buffer as *mut ::core::ffi::c_void);
        return -12 as ::core::ffi::c_int;
    }
    let mut io: *mut AVIOContext = avio_alloc_context(
        buffer,
        32768 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fd as *mut ::core::ffi::c_void,
        None,
        Some(
            write_file
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            seek_file
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    int64_t,
                    ::core::ffi::c_int,
                ) -> int64_t,
        ),
    );
    if io.is_null() {
        av_free(fd as *mut ::core::ffi::c_void);
        av_free(buffer as *mut ::core::ffi::c_void);
        return -12 as ::core::ffi::c_int;
    }
    *fd = open(
        path,
        O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC,
        S_IRUSR | S_IWUSR,
    );
    if *fd < 0 as ::core::ffi::c_int {
        let mut error: ::core::ffi::c_int = -*__errno_location();
        av_freep(&raw mut (*io).buffer as *mut ::core::ffi::c_void);
        avio_context_free(&raw mut io);
        av_free(fd as *mut ::core::ffi::c_void);
        return error;
    }
    *context = io;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_file_close(
    mut context: *mut *mut AVIOContext,
) -> ::core::ffi::c_int {
    if context.is_null() || (*context).is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut io: *mut AVIOContext = *context;
    avio_flush(io);
    let mut error: ::core::ffi::c_int = (*io).error;
    let mut fd: *mut ::core::ffi::c_int = (*io).opaque as *mut ::core::ffi::c_int;
    if close(*fd) < 0 as ::core::ffi::c_int && error >= 0 as ::core::ffi::c_int {
        error = -*__errno_location();
    }
    av_free(fd as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*io).buffer as *mut ::core::ffi::c_void);
    avio_context_free(context);
    return if error < 0 as ::core::ffi::c_int {
        error
    } else {
        0 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
