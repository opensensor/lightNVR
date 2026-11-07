use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn fdopendir(__fd: ::core::ffi::c_int) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn rewinddir(__dirp: *mut DIR);
    unsafe fn dirfd(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn openat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn unlinkat(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
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
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hls_cache_cleanup_result_t {
    pub removed: ::core::ffi::c_uint,
    pub errors: ::core::ffi::c_uint,
    pub skipped_directories: ::core::ffi::c_uint,
    pub bytes: uint64_t,
}
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const HLS_CACHE_MIN_AGE_SECONDS: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
pub const MAX_PLAYLIST_REFERENCES: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn suffix(
    mut name: *const ::core::ffi::c_char,
    mut ending: *const ::core::ffi::c_char,
) -> bool {
    let mut n: size_t = strlen(name);
    let mut e: size_t = strlen(ending);
    return n >= e
        && strcmp(name.offset(n as isize).offset(-(e as isize)), ending)
            == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn candidate(mut name: *const ::core::ffi::c_char) -> bool {
    return suffix(name, b".ts\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
        != 0
        || suffix(name, b".m4s\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
        || suffix(name, b".ts.tmp\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
        || suffix(name, b".m4s.tmp\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || suffix(name, b".m3u8.tmp\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn compare_names(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        *(a as *const *const ::core::ffi::c_char),
        *(b as *const *const ::core::ffi::c_char),
    );
}
unsafe extern "C" fn read_references(
    mut fd: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut refs: *mut *mut ::core::ffi::c_char,
    mut count: *mut size_t,
) -> ::core::ffi::c_int {
    let mut file: ::core::ffi::c_int = openat(fd, name, O_RDONLY | O_NOFOLLOW | O_NONBLOCK);
    if file < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
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
    if fstat(file, &raw mut st) != 0 || !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
    {
        close(file);
        return -1 as ::core::ffi::c_int;
    }
    let mut playlist: *mut FILE = fdopen(file, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if playlist.is_null() {
        close(file);
        return -1 as ::core::ffi::c_int;
    }
    let mut line: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int,
        playlist,
    )
    .is_null()
    {
        let mut length: size_t = strlen(&raw mut line as *mut ::core::ffi::c_char);
        if length == ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1usize)
            && line[length.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
                != '\n' as ::core::ffi::c_int
        {
            status = -1 as ::core::ffi::c_int;
            break;
        } else {
            line[strcspn(
                &raw mut line as *mut ::core::ffi::c_char,
                b"\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            ) as usize] = '\0' as ::core::ffi::c_char;
            if line[0usize] == 0 {
                continue;
            }
            let mut uri: *mut ::core::ffi::c_char = &raw mut line as *mut ::core::ffi::c_char;
            if line[0usize] as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
                uri = strstr(
                    &raw mut line as *mut ::core::ffi::c_char,
                    b"URI=\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if uri.is_null() {
                    continue;
                }
                uri = uri.offset(strlen(b"URI=\0".as_ptr() as *const ::core::ffi::c_char) as isize);
                let c2rust_fresh0 = uri;
                uri = uri.offset(1);
                if *c2rust_fresh0 as ::core::ffi::c_int != '"' as ::core::ffi::c_int {
                    status = -1 as ::core::ffi::c_int;
                    break;
                } else {
                    let mut end: *mut ::core::ffi::c_char = strchr(uri, '"' as ::core::ffi::c_int);
                    if end.is_null() {
                        status = -1 as ::core::ffi::c_int;
                        break;
                    } else {
                        *end = '\0' as ::core::ffi::c_char;
                    }
                }
            }
            *uri.offset(strcspn(uri, b"?\0".as_ptr() as *const ::core::ffi::c_char) as isize) =
                '\0' as ::core::ffi::c_char;
            if *uri.offset(0isize) == 0
                || !strchr(uri, '/' as ::core::ffi::c_int).is_null()
                || !strchr(uri, '%' as ::core::ffi::c_int).is_null()
                || strcmp(uri, b".\0".as_ptr() as *const ::core::ffi::c_char) == 0
                || strcmp(uri, b"..\0".as_ptr() as *const ::core::ffi::c_char) == 0
                || *count == MAX_PLAYLIST_REFERENCES as size_t
            {
                status = -1 as ::core::ffi::c_int;
                break;
            } else {
                *refs.offset(*count as isize) = strdup(uri);
                if (*refs.offset(*count as isize)).is_null() {
                    status = -1 as ::core::ffi::c_int;
                    break;
                } else {
                    *count = (*count).wrapping_add(1);
                }
            }
        }
    }
    if ferror(playlist) != 0 {
        status = -1 as ::core::ffi::c_int;
    }
    fclose(playlist);
    return status;
}
unsafe extern "C" fn prune_stream(
    mut stream: *mut DIR,
    mut now: time_t,
    mut age: ::core::ffi::c_uint,
    mut limit: ::core::ffi::c_uint,
    mut dry_run: bool,
    mut result: *mut hls_cache_cleanup_result_t,
) {
    let mut refs: *mut *mut ::core::ffi::c_char = calloc(
        MAX_PLAYLIST_REFERENCES as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
    ) as *mut *mut ::core::ffi::c_char;
    if refs.is_null() {
        (*result).errors = (*result).errors.wrapping_add(1);
        return;
    }
    let mut count: size_t = 0 as size_t;
    let mut safe: bool = r#true != 0;
    let mut fd: ::core::ffi::c_int = dirfd(stream);
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(stream);
        if entry.is_null() {
            break;
        }
        if !(suffix(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            b".m3u8\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            && read_references(
                fd,
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                refs,
                &raw mut count,
            ) != 0)
        {
            continue;
        }
        safe = r#false != 0;
        break;
    }
    if !safe {
        (*result).skipped_directories = (*result).skipped_directories.wrapping_add(1);
    } else {
        qsort(
            refs as *mut ::core::ffi::c_void,
            count,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
            Some(
                compare_names
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        rewinddir(stream);
        while (limit == 0 || (*result).removed < limit) && {
            entry = readdir(stream);
            !entry.is_null()
        } {
            if !candidate(&raw mut (*entry).d_name as *mut ::core::ffi::c_char) {
                continue;
            }
            let mut name: *const ::core::ffi::c_char =
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char;
            if count != 0
                && !bsearch(
                    &raw mut name as *const ::core::ffi::c_void,
                    refs as *const ::core::ffi::c_void,
                    count,
                    ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
                    Some(
                        compare_names
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *const ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                )
                .is_null()
            {
                continue;
            }
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
            if fstatat(fd, name, &raw mut st, AT_SYMLINK_NOFOLLOW) != 0 {
                if *__errno_location() != ENOENT {
                    (*result).errors = (*result).errors.wrapping_add(1);
                }
            } else {
                if !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
                    || st.st_mtim.tv_sec > now
                    || difftime(now, st.st_mtim.tv_sec) <= age as ::core::ffi::c_double
                {
                    continue;
                }
                if !dry_run && unlinkat(fd, name, 0 as ::core::ffi::c_int) != 0 {
                    if *__errno_location() != ENOENT {
                        (*result).errors = (*result).errors.wrapping_add(1);
                    }
                } else {
                    (*result).removed = (*result).removed.wrapping_add(1);
                    (*result).bytes = (*result).bytes.wrapping_add(st.st_size as uint64_t);
                }
            }
        }
    }
    let mut i: size_t = 0 as size_t;
    while i < count {
        free(*refs.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free(refs as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_prune_cache(
    mut root: *const ::core::ffi::c_char,
    mut now: time_t,
    mut minimum_age: ::core::ffi::c_uint,
    mut max_removals: ::core::ffi::c_uint,
    mut dry_run: bool,
    mut result: *mut hls_cache_cleanup_result_t,
) -> ::core::ffi::c_int {
    if root.is_null()
        || *root.offset(0isize) == 0
        || result.is_null()
        || minimum_age < HLS_CACHE_MIN_AGE_SECONDS as ::core::ffi::c_uint
    {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<hls_cache_cleanup_result_t>(),
    );
    let mut fd: ::core::ffi::c_int = open(root, O_RDONLY | O_DIRECTORY | O_NOFOLLOW);
    if fd < 0 as ::core::ffi::c_int {
        return if *__errno_location() == ENOENT {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    let mut directory: *mut DIR = fdopendir(fd);
    if directory.is_null() {
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    while (max_removals == 0 || (*result).removed < max_removals) && {
        entry = readdir(directory);
        !entry.is_null()
    } {
        if strcmp(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            b".\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
            || strcmp(
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                b"..\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            continue;
        }
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
        if fstatat(
            fd,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            &raw mut st,
            AT_SYMLINK_NOFOLLOW,
        ) != 0
            || !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
        {
            continue;
        }
        let mut child: ::core::ffi::c_int = openat(
            fd,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            O_RDONLY | O_DIRECTORY | O_NOFOLLOW,
        );
        if child < 0 as ::core::ffi::c_int {
            (*result).errors = (*result).errors.wrapping_add(1);
        } else {
            let mut stream: *mut DIR = fdopendir(child);
            if stream.is_null() {
                close(child);
                (*result).errors = (*result).errors.wrapping_add(1);
            } else {
                prune_stream(stream, now, minimum_age, max_removals, dry_run, result);
                closedir(stream);
            }
        }
    }
    closedir(directory);
    return if (*result).errors != 0 {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const ::core::ffi::c_void,
    mut __base: *const ::core::ffi::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut ::core::ffi::c_void {
    let mut __p: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut __comparison: ::core::ffi::c_int = 0;
    while __nmemb != 0 {
        __p = (__base as *const ::core::ffi::c_char)
            .offset((__nmemb >> 1 as ::core::ffi::c_int).wrapping_mul(__size) as isize)
            as *const ::core::ffi::c_void;
        __comparison = Some(__compar.expect("non-null function pointer"))
            .expect("non-null function pointer")(__key, __p);
        if __comparison == 0 as ::core::ffi::c_int {
            return __p as *mut ::core::ffi::c_void;
        }
        if __comparison > 0 as ::core::ffi::c_int {
            __base = (__p as *const ::core::ffi::c_char).offset(__size as isize)
                as *const ::core::ffi::c_void;
            __nmemb = __nmemb.wrapping_sub(1);
        }
        __nmemb >>= 1 as ::core::ffi::c_int;
    }
    return NULL;
}
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
