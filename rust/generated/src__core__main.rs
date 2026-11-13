use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type stream_handle_s;
    unsafe fn lightnvr_sentry_init();
    unsafe fn lightnvr_sentry_shutdown();
    static mut stderr: *mut FILE;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigfillset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sigdelset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn execv(
        __path: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn _exit(__status: ::core::ffi::c_int) -> !;
    unsafe fn getpid() -> __pid_t;
    unsafe fn getppid() -> __pid_t;
    unsafe fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::core::ffi::c_int;
    unsafe fn fork() -> __pid_t;
    unsafe fn symlink(
        __from: *const ::core::ffi::c_char,
        __to: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn lockf(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        __len: __off_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    unsafe fn uname(__name: *mut utsname) -> ::core::ffi::c_int;
    unsafe fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn prctl(__option: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_policy_build(
        settings: *const system_health_policy_settings_t,
        override_json: *const ::core::ffi::c_char,
        policy: *mut system_health_policy_t,
        canonical_json: *mut ::core::ffi::c_char,
        canonical_json_size: size_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_policy_replace(
        candidate: *const system_health_policy_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn load_config(config_0: *mut config_t) -> ::core::ffi::c_int;
    unsafe fn config_copy(destination: *mut config_t, source: *const config_t);
    unsafe fn load_stream_configs(config_0: *mut config_t) -> ::core::ffi::c_int;
    unsafe fn set_custom_config_path(path: *const ::core::ffi::c_char);
    static mut g_config: config_t;
    unsafe fn init_logger() -> ::core::ffi::c_int;
    unsafe fn shutdown_logger();
    unsafe fn set_log_level(level: log_level_t);
    unsafe fn set_log_file(filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_log_level_string(level: log_level_t) -> *const ::core::ffi::c_char;
    unsafe fn enable_syslog(
        ident: *const ::core::ffi::c_char,
        facility: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn init_daemon(pid_file: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn write_pid_file(pid_file: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn remove_pid_file(
        fd: ::core::ffi::c_int,
        pid_file: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn init_shutdown_coordinator() -> ::core::ffi::c_int;
    unsafe fn shutdown_coordinator_cleanup();
    unsafe fn update_component_state(component_id: ::core::ffi::c_int, state: component_state_t);
    unsafe fn find_component_id_by_name(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn initiate_shutdown();
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn is_coordinator_destroyed() -> bool;
    unsafe fn wait_for_all_components_stopped(timeout_seconds: ::core::ffi::c_int) -> bool;
    unsafe fn request_background_abort();
    unsafe fn is_background_abort_requested() -> bool;
    unsafe fn curl_init_global() -> ::core::ffi::c_int;
    unsafe fn curl_cleanup_global();
    unsafe fn event_bus_init(max_events: size_t, max_bytes: size_t) -> ::core::ffi::c_int;
    unsafe fn event_bus_shutdown(drain: bool);
    unsafe fn event_identity_init() -> ::core::ffi::c_int;
    unsafe fn event_identity_shutdown();
    unsafe fn mqtt_event_adapter_register(config_0: *const config_t) -> ::core::ffi::c_int;
    unsafe fn mqtt_event_adapter_unregister() -> ::core::ffi::c_int;
    unsafe fn mqtt_init(config_0: *const config_t) -> ::core::ffi::c_int;
    unsafe fn mqtt_connect() -> ::core::ffi::c_int;
    unsafe fn mqtt_cleanup();
    unsafe fn mqtt_delivery_worker_start() -> ::core::ffi::c_int;
    unsafe fn mqtt_delivery_worker_shutdown();
    unsafe fn sqlite3_shutdown() -> ::core::ffi::c_int;
    unsafe fn sqlite3_release_memory(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn observation_evidence_init(
        config_0: *const observation_evidence_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_evidence_shutdown(drain_timeout_ms: ::core::ffi::c_int);
    unsafe fn observation_evidence_set_enabled_probe(
        r#fn: observation_evidence_enabled_fn,
        ctx: *mut ::core::ffi::c_void,
    );
    unsafe fn observation_evidence_stream_enabled_probe(
        camera_uuid: *const ::core::ffi::c_char,
        ctx: *mut ::core::ffi::c_void,
    ) -> bool;
    unsafe fn observation_relay_start(
        options: *const observation_relay_options_t,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_relay_stop();
    unsafe fn observation_retention_startup(
        storage_path: *const ::core::ffi::c_char,
        result: *mut observation_reconcile_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn ensure_path(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn init_stream_manager(max_streams: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn shutdown_stream_manager();
    unsafe fn stop_stream(handle: stream_handle_t) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn is_recording_scheduled(config_0: *const stream_config_t) -> bool;
    unsafe fn is_detection_recording_scheduled(config_0: *const stream_config_t) -> bool;
    unsafe fn init_stream_state_manager(max_streams: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn shutdown_stream_state_manager();
    unsafe fn init_stream_state_adapter() -> ::core::ffi::c_int;
    unsafe fn shutdown_stream_state_adapter();
    unsafe fn init_storage_manager(
        storage_path: *const ::core::ffi::c_char,
        max_size: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn shutdown_storage_manager();
    unsafe fn set_retention_days(days: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn storage_startup_reclaim_if_full(
        recordings_root: *const ::core::ffi::c_char,
        cfg: *const config_t,
    ) -> uint64_t;
    unsafe fn db_storage_target_bootstrap_default(
        legacy_root: *const ::core::ffi::c_char,
        uuid: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_target_refresh_health_and_publish() -> ::core::ffi::c_int;
    unsafe fn cleanup_all_hls_writers();
    unsafe fn init_hls_streaming_backend();
    unsafe fn cleanup_hls_streaming_backend();
    unsafe fn cleanup_hls_directories();
    unsafe fn init_mp4_recording_backend();
    unsafe fn cleanup_mp4_recording_backend();
    unsafe fn close_all_mp4_writers();
    unsafe fn init_timestamp_trackers();
    unsafe fn get_streaming_config() -> *mut config_t;
    unsafe fn stream_start_hls(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_recording_state(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn shutdown_recording_transcode();
    unsafe fn init_transcoding_backend();
    unsafe fn cleanup_transcoding_backend();
    unsafe fn init_detection_stream_system();
    unsafe fn shutdown_detection_stream_system();
    unsafe fn start_detection_stream_reader(
        stream_name: *const ::core::ffi::c_char,
        detection_interval: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn stop_detection_stream_reader(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn is_detection_stream_reader_running(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn print_detection_stream_status();
    unsafe fn init_detection_integration() -> ::core::ffi::c_int;
    unsafe fn cleanup_detection_resources();
    unsafe fn start_unified_detection_thread(
        stream_name: *const ::core::ffi::c_char,
        model_path: *const ::core::ffi::c_char,
        threshold: ::core::ffi::c_float,
        pre_buffer_seconds: ::core::ffi::c_int,
        post_buffer_seconds: ::core::ffi::c_int,
        annotation_only: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn init_onvif_discovery() -> ::core::ffi::c_int;
    unsafe fn shutdown_onvif_discovery();
    unsafe fn start_onvif_discovery(
        network: *const ::core::ffi::c_char,
        interval: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_init(max_streams: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn metrics_shutdown();
    unsafe fn player_telemetry_init();
    unsafe fn player_telemetry_shutdown();
    unsafe fn ffmpeg_get_allocation_count() -> ::core::ffi::c_int;
    unsafe fn ffmpeg_dump_allocations();
    unsafe fn system_health_options_from_policy(
        settings: *const system_health_policy_settings_t,
        recording_path: *const ::core::ffi::c_char,
        options: *mut system_health_options_t,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_init(options: *const system_health_options_t) -> ::core::ffi::c_int;
    unsafe fn system_health_start() -> ::core::ffi::c_int;
    unsafe fn system_health_shutdown();
    unsafe fn system_health_evaluator_service_start() -> ::core::ffi::c_int;
    unsafe fn system_health_evaluator_service_shutdown(clean_shutdown: bool);
    unsafe fn go2rtc_process_validate_existing_override_on_upgrade();
    unsafe fn go2rtc_process_generate_startup_config(
        binary_path: *const ::core::ffi::c_char,
        config_dir: *const ::core::ffi::c_char,
        api_port: ::core::ffi::c_int,
    ) -> bool;
    unsafe fn go2rtc_stream_cleanup();
    unsafe fn go2rtc_integration_start_recording(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_integration_full_start() -> bool;
    unsafe fn go2rtc_integration_cleanup();
    unsafe fn init_database_ex(
        db_path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    unsafe fn init_database(db_path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn shutdown_database();
    unsafe fn maybe_run_scheduled_database_backup() -> ::core::ffi::c_int;
    unsafe fn db_scheduled_backup_in_flight() -> bool;
    unsafe fn db_scheduled_backup_wait_idle(timeout_ms: ::core::ffi::c_int) -> bool;
    unsafe fn init_schema_cache();
    unsafe fn free_schema_cache();
    unsafe fn db_authorization_verify_action_catalog() -> ::core::ffi::c_int;
    unsafe fn start_recording_sync_thread(
        interval_seconds: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn stop_recording_sync_thread() -> ::core::ffi::c_int;
    unsafe fn db_get_system_setting(
        key: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_get_system_setting_alloc(
        key: *const ::core::ffi::c_char,
        out: *mut *mut ::core::ffi::c_char,
        out_len: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_server_start(server: http_server_handle_t) -> ::core::ffi::c_int;
    unsafe fn http_server_stop(server: http_server_handle_t);
    unsafe fn http_server_destroy(server: http_server_handle_t);
    unsafe fn libuv_server_init(config_0: *const http_server_config_t) -> http_server_handle_t;
    unsafe fn register_all_libuv_handlers(server: http_server_handle_t) -> ::core::ffi::c_int;
    unsafe fn register_static_file_handler(server: http_server_handle_t) -> ::core::ffi::c_int;
    unsafe fn init_auth_system() -> ::core::ffi::c_int;
    unsafe fn init_health_check_system();
    unsafe fn start_health_check_thread();
    unsafe fn cleanup_health_check_system();
    unsafe fn batch_delete_progress_init() -> ::core::ffi::c_int;
    unsafe fn batch_delete_progress_cleanup();
    unsafe fn audit_log_decision_modes_init() -> ::core::ffi::c_int;
    unsafe fn audit_log_flush_summaries(closed_windows_only: bool) -> size_t;
    unsafe fn audit_log_shutdown_summaries();
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn exit(__status: ::core::ffi::c_int) -> !;
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type __sig_atomic_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
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
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2Rust_Unnamed_8,
    pub _timer: C2Rust_Unnamed_7,
    pub _rt: C2Rust_Unnamed_6,
    pub _sigchld: C2Rust_Unnamed_5,
    pub _sigfault: C2Rust_Unnamed_2,
    pub _sigpoll: C2Rust_Unnamed_1,
    pub _sigsys: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub _addr_bnd: C2Rust_Unnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2Rust_Unnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
}
pub type socklen_t = __socklen_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [::core::ffi::c_char; 65],
    pub nodename: [::core::ffi::c_char; 65],
    pub release: [::core::ffi::c_char; 65],
    pub version: [::core::ffi::c_char; 65],
    pub machine: [::core::ffi::c_char; 65],
    pub domainname: [::core::ffi::c_char; 65],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __socket_type(pub ::core::ffi::c_uint);
impl __socket_type {
    pub const SOCK_STREAM: Self = Self(1);
    pub const SOCK_DGRAM: Self = Self(2);
    pub const SOCK_RAW: Self = Self(3);
    pub const SOCK_RDM: Self = Self(4);
    pub const SOCK_SEQPACKET: Self = Self(5);
    pub const SOCK_DCCP: Self = Self(6);
    pub const SOCK_PACKET: Self = Self(10);
    pub const SOCK_CLOEXEC: Self = Self(524288);
    pub const SOCK_NONBLOCK: Self = Self(2048);
}
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_unit_t(pub ::core::ffi::c_uint);
impl system_health_unit_t {
    pub const SYSTEM_HEALTH_UNIT_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_UNIT_RATIO: Self = Self(1);
    pub const SYSTEM_HEALTH_UNIT_BYTES: Self = Self(2);
    pub const SYSTEM_HEALTH_UNIT_COUNT: Self = Self(3);
    pub const SYSTEM_HEALTH_UNIT_SECONDS: Self = Self(4);
    pub const SYSTEM_HEALTH_UNIT_CELSIUS: Self = Self(5);
    pub const SYSTEM_HEALTH_UNIT_HERTZ: Self = Self(6);
    pub const SYSTEM_HEALTH_UNIT_BOOLEAN: Self = Self(7);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_profile_t(pub ::core::ffi::c_uint);
impl system_health_profile_t {
    pub const SYSTEM_HEALTH_PROFILE_BALANCED: Self = Self(0);
    pub const SYSTEM_HEALTH_PROFILE_CONSERVATIVE: Self = Self(1);
    pub const SYSTEM_HEALTH_PROFILE_DISABLED: Self = Self(2);
    pub const SYSTEM_HEALTH_PROFILE_CUSTOM: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_threshold_direction_t(pub ::core::ffi::c_uint);
impl system_health_threshold_direction_t {
    pub const SYSTEM_HEALTH_THRESHOLD_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE: Self = Self(1);
    pub const SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_settings_t {
    pub enabled: bool,
    pub profile: [::core::ffi::c_char; 16],
    pub fast_interval_seconds: uint32_t,
    pub normal_interval_seconds: uint32_t,
    pub slow_interval_seconds: uint32_t,
    pub device_interval_seconds: uint32_t,
    pub write_probe_enabled: bool,
    pub hardware_provider: [::core::ffi::c_char; 16],
    pub presence_interval_seconds: uint32_t,
    pub incident_retention_days: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_condition_policy_t {
    pub condition: system_health_condition_t,
    pub profile: system_health_profile_t,
    pub enabled: bool,
    pub overridden: bool,
    pub direction: system_health_threshold_direction_t,
    pub unit: system_health_unit_t,
    pub warning_threshold: ::core::ffi::c_double,
    pub critical_threshold: ::core::ffi::c_double,
    pub recovery_threshold: ::core::ffi::c_double,
    pub warning_for_seconds: uint32_t,
    pub critical_for_seconds: uint32_t,
    pub recovery_for_seconds: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_t {
    pub generation: uint64_t,
    pub settings: system_health_policy_settings_t,
    pub conditions: [system_health_condition_policy_t; 27],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_config_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub url: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub priority: ::core::ffi::c_int,
    pub record: bool,
    pub segment_duration: ::core::ffi::c_int,
    pub detection_based_recording: bool,
    pub detection_model: [::core::ffi::c_char; 512],
    pub detection_interval: ::core::ffi::c_int,
    pub detection_threshold: ::core::ffi::c_float,
    pub pre_detection_buffer: ::core::ffi::c_int,
    pub post_detection_buffer: ::core::ffi::c_int,
    pub detection_api_url: [::core::ffi::c_char; 512],
    pub detection_object_filter: [::core::ffi::c_char; 16],
    pub detection_object_filter_list: [::core::ffi::c_char; 256],
    pub streaming_enabled: bool,
    pub protocol: stream_protocol_t,
    pub record_audio: bool,
    pub audio_voice_enhancement: bool,
    pub onvif_username: [::core::ffi::c_char; 64],
    pub onvif_password: [::core::ffi::c_char; 64],
    pub onvif_profile: [::core::ffi::c_char; 64],
    pub onvif_port: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub is_onvif: bool,
    pub backchannel_enabled: bool,
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: ::core::ffi::c_int,
    pub tier_critical_multiplier: ::core::ffi::c_double,
    pub tier_important_multiplier: ::core::ffi::c_double,
    pub tier_ephemeral_multiplier: ::core::ffi::c_double,
    pub storage_priority: ::core::ffi::c_int,
    pub ptz_enabled: bool,
    pub ptz_max_x: ::core::ffi::c_int,
    pub ptz_max_y: ::core::ffi::c_int,
    pub ptz_max_z: ::core::ffi::c_int,
    pub ptz_has_home: bool,
    pub record_on_schedule: bool,
    pub recording_schedule: [uint8_t; 168],
    pub detection_record_on_schedule: bool,
    pub detection_recording_schedule: [uint8_t; 168],
    pub tags: [::core::ffi::c_char; 256],
    pub admin_url: [::core::ffi::c_char; 512],
    pub privacy_mode: bool,
    pub motion_trigger_source: [::core::ffi::c_char; 256],
    pub go2rtc_source_override: [::core::ffi::c_char; 8192],
    pub sub_stream_url: [::core::ffi::c_char; 512],
    pub detection_url: [::core::ffi::c_char; 512],
    pub publish_url: [::core::ffi::c_char; 512],
    pub playback_transport: [::core::ffi::c_char; 24],
    pub eptz_config: [::core::ffi::c_char; 1024],
    pub observation_evidence_enabled: bool,
    pub observation_labels: [::core::ffi::c_char; 512],
    pub observation_max_age_days: ::core::ffi::c_int,
    pub observation_max_bytes: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_engine_config_t {
    pub enabled: bool,
    pub num_threads: ::core::ffi::c_int,
    pub delegate: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub pid_file: [::core::ffi::c_char; 512],
    pub log_file: [::core::ffi::c_char; 512],
    pub log_level: ::core::ffi::c_int,
    pub syslog_enabled: bool,
    pub syslog_ident: [::core::ffi::c_char; 64],
    pub syslog_facility: ::core::ffi::c_int,
    pub storage_path: [::core::ffi::c_char; 512],
    pub storage_path_hls: [::core::ffi::c_char; 512],
    pub max_storage_size: uint64_t,
    pub retention_days: ::core::ffi::c_int,
    pub auto_delete_oldest: bool,
    pub mp4_faststart: bool,
    pub storage_min_free_pct: ::core::ffi::c_int,
    pub storage_pressure_warning_pct: ::core::ffi::c_double,
    pub storage_pressure_critical_pct: ::core::ffi::c_double,
    pub storage_pressure_emergency_pct: ::core::ffi::c_double,
    pub generate_thumbnails: bool,
    pub thumbnails_per_recording: ::core::ffi::c_int,
    pub record_mp4_directly: bool,
    pub mp4_storage_path: [::core::ffi::c_char; 512],
    pub mp4_directory_format: [::core::ffi::c_char; 32],
    pub mp4_segment_duration: ::core::ffi::c_int,
    pub mp4_retention_days: ::core::ffi::c_int,
    pub models_path: [::core::ffi::c_char; 512],
    pub api_detection_url: [::core::ffi::c_char; 512],
    pub api_detection_backend: [::core::ffi::c_char; 32],
    pub api_detection_format: [::core::ffi::c_char; 32],
    pub api_detection_detector_name: [::core::ffi::c_char; 64],
    pub default_detection_threshold: ::core::ffi::c_int,
    pub default_pre_detection_buffer: ::core::ffi::c_int,
    pub default_post_detection_buffer: ::core::ffi::c_int,
    pub default_playback_transport: [::core::ffi::c_char; 24],
    pub detection_grace_period: ::core::ffi::c_int,
    pub db_path: [::core::ffi::c_char; 512],
    pub db_backup_interval_minutes: ::core::ffi::c_int,
    pub db_backup_retention_count: ::core::ffi::c_int,
    pub db_post_backup_script: [::core::ffi::c_char; 512],
    pub db_startup_check: ::core::ffi::c_int,
    pub db_backup_verify: ::core::ffi::c_int,
    pub web_thread_pool_size: ::core::ffi::c_int,
    pub web_port: ::core::ffi::c_int,
    pub web_bind_ip: [::core::ffi::c_char; 32],
    pub web_root: [::core::ffi::c_char; 512],
    pub web_ssl_enabled: bool,
    pub web_ssl_cert_path: [::core::ffi::c_char; 512],
    pub web_ssl_key_path: [::core::ffi::c_char; 512],
    pub web_auth_enabled: bool,
    pub web_username: [::core::ffi::c_char; 32],
    pub web_password: [::core::ffi::c_char; 32],
    pub audio_disabled: bool,
    pub auto_disabled: bool,
    pub webrtc_disabled: bool,
    pub hls_disabled: bool,
    pub mse_disabled: bool,
    pub auth_timeout_hours: ::core::ffi::c_int,
    pub auth_absolute_timeout_hours: ::core::ffi::c_int,
    pub trusted_device_days: ::core::ffi::c_int,
    pub trusted_proxy_cidrs: [::core::ffi::c_char; 1024],
    pub demo_mode: bool,
    pub force_mfa_on_login: bool,
    pub login_rate_limit_enabled: bool,
    pub login_rate_limit_max_attempts: ::core::ffi::c_int,
    pub login_rate_limit_window_seconds: ::core::ffi::c_int,
    pub web_compression_enabled: bool,
    pub web_use_minified_assets: bool,
    pub web_cache_max_age_html: ::core::ffi::c_int,
    pub web_cache_max_age_css: ::core::ffi::c_int,
    pub web_cache_max_age_js: ::core::ffi::c_int,
    pub web_cache_max_age_images: ::core::ffi::c_int,
    pub web_cache_max_age_fonts: ::core::ffi::c_int,
    pub web_cache_max_age_default: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub onvif_discovery_interval: ::core::ffi::c_int,
    pub onvif_discovery_network: [::core::ffi::c_char; 64],
    pub max_streams: ::core::ffi::c_int,
    pub streams: *mut stream_config_t,
    pub buffer_size: ::core::ffi::c_int,
    pub use_swap: bool,
    pub swap_file: [::core::ffi::c_char; 512],
    pub swap_size: uint64_t,
    pub memory_constrained: bool,
    pub hw_accel_enabled: bool,
    pub hw_accel_device: [::core::ffi::c_char; 32],
    pub go2rtc_enabled: bool,
    pub go2rtc_binary_path: [::core::ffi::c_char; 512],
    pub go2rtc_config_dir: [::core::ffi::c_char; 512],
    pub go2rtc_api_port: ::core::ffi::c_int,
    pub go2rtc_rtsp_port: ::core::ffi::c_int,
    pub go2rtc_force_native_hls: bool,
    pub go2rtc_proxy_max_inflight: ::core::ffi::c_int,
    pub go2rtc_webrtc_enabled: bool,
    pub go2rtc_webrtc_listen_port: ::core::ffi::c_int,
    pub go2rtc_stun_enabled: bool,
    pub go2rtc_stun_server: [::core::ffi::c_char; 256],
    pub go2rtc_external_ip: [::core::ffi::c_char; 256],
    pub go2rtc_ice_servers: [::core::ffi::c_char; 512],
    pub webrtc_connection_timeout_ms: ::core::ffi::c_int,
    pub webrtc_ice_recovery_timeout_ms: ::core::ffi::c_int,
    pub turn_enabled: bool,
    pub turn_server_url: [::core::ffi::c_char; 256],
    pub turn_username: [::core::ffi::c_char; 64],
    pub turn_password: [::core::ffi::c_char; 64],
    pub mqtt_enabled: bool,
    pub mqtt_broker_host: [::core::ffi::c_char; 256],
    pub mqtt_broker_port: ::core::ffi::c_int,
    pub mqtt_username: [::core::ffi::c_char; 64],
    pub mqtt_password: [::core::ffi::c_char; 128],
    pub mqtt_client_id: [::core::ffi::c_char; 64],
    pub mqtt_topic_prefix: [::core::ffi::c_char; 128],
    pub mqtt_tls_enabled: bool,
    pub mqtt_keepalive: ::core::ffi::c_int,
    pub mqtt_qos: ::core::ffi::c_int,
    pub mqtt_retain: bool,
    pub mqtt_ha_discovery: bool,
    pub mqtt_ha_discovery_prefix: [::core::ffi::c_char; 128],
    pub mqtt_ha_snapshot_interval: ::core::ffi::c_int,
    pub health: system_health_policy_settings_t,
    pub detection_engine: detection_engine_config_t,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct component_state_t(pub ::core::ffi::c_uint);
impl component_state_t {
    pub const COMPONENT_RUNNING: Self = Self(0);
    pub const COMPONENT_STOPPING: Self = Self(1);
    pub const COMPONENT_STOPPED: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_evidence_config_t {
    pub storage_path: *const ::core::ffi::c_char,
    pub global_max_bytes: int64_t,
    pub global_max_items: ::core::ffi::c_int,
    pub per_camera_max_items: ::core::ffi::c_int,
    pub per_camera_max_bytes: int64_t,
}
pub type observation_evidence_enabled_fn =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_relay_options_t {
    pub batch_size: ::core::ffi::c_int,
    pub poll_interval_ms: ::core::ffi::c_int,
    pub outbox_cap: ::core::ffi::c_int,
    pub startup_drain_ms: ::core::ffi::c_int,
    pub storage_path: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_reconcile_result_t {
    pub tmp_files_deleted: ::core::ffi::c_int,
    pub stale_files_deleted: ::core::ffi::c_int,
    pub pending_interrupted: ::core::ffi::c_int,
    pub ready_missing: ::core::ffi::c_int,
    pub epoch_regenerated: bool,
}
pub type stream_handle_t = *mut stream_handle_s;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct http_method_t(pub ::core::ffi::c_uint);
impl http_method_t {
    pub const HTTP_METHOD_GET: Self = Self(0);
    pub const HTTP_METHOD_POST: Self = Self(1);
    pub const HTTP_METHOD_PUT: Self = Self(2);
    pub const HTTP_METHOD_DELETE: Self = Self(3);
    pub const HTTP_METHOD_OPTIONS: Self = Self(4);
    pub const HTTP_METHOD_HEAD: Self = Self(5);
    pub const HTTP_METHOD_PATCH: Self = Self(6);
    pub const HTTP_METHOD_UNKNOWN: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_t {
    pub name: [::core::ffi::c_char; 128],
    pub value: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_request_t {
    pub method: http_method_t,
    pub method_str: [::core::ffi::c_char; 16],
    pub path: [::core::ffi::c_char; 512],
    pub uri: [::core::ffi::c_char; 1536],
    pub query_string: [::core::ffi::c_char; 1024],
    pub content_type: [::core::ffi::c_char; 128],
    pub content_length: uint64_t,
    pub user_agent: [::core::ffi::c_char; 256],
    pub body: *mut ::core::ffi::c_void,
    pub body_len: size_t,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub client_ip: [::core::ffi::c_char; 64],
    pub request_id: [::core::ffi::c_char; 65],
    pub user_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response_t {
    pub status_code: ::core::ffi::c_int,
    pub content_type: [::core::ffi::c_char; 128],
    pub body_length: size_t,
    pub body: *mut ::core::ffi::c_void,
    pub body_allocated: bool,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub user_data: *mut ::core::ffi::c_void,
}
pub type request_handler_t =
    Option<unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_options_t {
    pub enabled: bool,
    pub register_builtin_collectors: bool,
    pub tier_interval_ms: [uint32_t; 4],
    pub collector_deadline_ms: [uint32_t; 4],
    pub proc_root: [::core::ffi::c_char; 1024],
    pub sys_root: [::core::ffi::c_char; 1024],
    pub cgroup_root: [::core::ffi::c_char; 1024],
    pub root_path: [::core::ffi::c_char; 1024],
    pub recording_path: [::core::ffi::c_char; 1024],
    pub hardware_provider: [::core::ffi::c_char; 16],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
    pub const DB_INIT_DEFAULT: Self = Self(0);
    pub const DB_INIT_NO_CHECK: Self = Self(1);
    pub const DB_INIT_NO_MIGRATE: Self = Self(2);
    pub const DB_INIT_NO_BACKUP: Self = Self(4);
    pub const DB_INIT_READ_ONLY: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_server_config_t {
    pub port: ::core::ffi::c_int,
    pub bind_ip: *const ::core::ffi::c_char,
    pub web_root: *const ::core::ffi::c_char,
    pub auth_enabled: bool,
    pub username: [::core::ffi::c_char; 32],
    pub password: [::core::ffi::c_char; 32],
    pub cors_enabled: bool,
    pub allowed_origins: [::core::ffi::c_char; 256],
    pub allowed_methods: [::core::ffi::c_char; 256],
    pub allowed_headers: [::core::ffi::c_char; 256],
    pub ssl_enabled: bool,
    pub cert_path: [::core::ffi::c_char; 512],
    pub key_path: [::core::ffi::c_char; 512],
    pub max_connections: ::core::ffi::c_int,
    pub connection_timeout: ::core::ffi::c_int,
    pub daemon_mode: bool,
    pub pid_file: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_server {
    pub config: http_server_config_t,
    pub running: bool,
    pub handlers: *mut C2Rust_Unnamed_13,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub path: [::core::ffi::c_char; 256],
    pub method: [::core::ffi::c_char; 16],
    pub handler: request_handler_t,
}
pub type http_server_t = http_server;
pub type http_server_handle_t = *mut http_server_t;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_TLOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const PR_SET_THP_DISABLE: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SYSTEM_HEALTH_OVERRIDES_SETTING_KEY: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"health_condition_overrides\0")
};
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn signal_safe_write(mut msg: *const ::core::ffi::c_char) {
    if msg.is_null() {
        return;
    }
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut remaining: size_t = strlen(msg);
    while remaining > 0 as size_t {
        let mut written: ssize_t =
            write(STDERR_FILENO, msg as *const ::core::ffi::c_void, remaining);
        if written > 0 as ssize_t {
            msg = msg.offset(written as isize);
            remaining = remaining.wrapping_sub(written as size_t);
        } else if !(written < 0 as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    *__errno_location() = saved_errno;
}
pub const OBS_EVIDENCE_DEFAULT_DRAIN_MS: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut running: bool = r#true != 0;
#[unsafe(no_mangle)]
pub static mut restart_requested: bool = r#false != 0;
static mut saved_argc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut saved_argv: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
#[unsafe(no_mangle)]
pub static mut daemon_mode: bool = r#false != 0;
static mut container_mode: bool = r#false != 0;
#[unsafe(no_mangle)]
pub static mut config: config_t = config_t {
    pid_file: [0; 512],
    log_file: [0; 512],
    log_level: 0,
    syslog_enabled: false,
    syslog_ident: [0; 64],
    syslog_facility: 0,
    storage_path: [0; 512],
    storage_path_hls: [0; 512],
    max_storage_size: 0,
    retention_days: 0,
    auto_delete_oldest: false,
    mp4_faststart: false,
    storage_min_free_pct: 0,
    storage_pressure_warning_pct: 0.,
    storage_pressure_critical_pct: 0.,
    storage_pressure_emergency_pct: 0.,
    generate_thumbnails: false,
    thumbnails_per_recording: 0,
    record_mp4_directly: false,
    mp4_storage_path: [0; 512],
    mp4_directory_format: [0; 32],
    mp4_segment_duration: 0,
    mp4_retention_days: 0,
    models_path: [0; 512],
    api_detection_url: [0; 512],
    api_detection_backend: [0; 32],
    api_detection_format: [0; 32],
    api_detection_detector_name: [0; 64],
    default_detection_threshold: 0,
    default_pre_detection_buffer: 0,
    default_post_detection_buffer: 0,
    default_playback_transport: [0; 24],
    detection_grace_period: 0,
    db_path: [0; 512],
    db_backup_interval_minutes: 0,
    db_backup_retention_count: 0,
    db_post_backup_script: [0; 512],
    db_startup_check: 0,
    db_backup_verify: 0,
    web_thread_pool_size: 0,
    web_port: 0,
    web_bind_ip: [0; 32],
    web_root: [0; 512],
    web_ssl_enabled: false,
    web_ssl_cert_path: [0; 512],
    web_ssl_key_path: [0; 512],
    web_auth_enabled: false,
    web_username: [0; 32],
    web_password: [0; 32],
    audio_disabled: false,
    auto_disabled: false,
    webrtc_disabled: false,
    hls_disabled: false,
    mse_disabled: false,
    auth_timeout_hours: 0,
    auth_absolute_timeout_hours: 0,
    trusted_device_days: 0,
    trusted_proxy_cidrs: [0; 1024],
    demo_mode: false,
    force_mfa_on_login: false,
    login_rate_limit_enabled: false,
    login_rate_limit_max_attempts: 0,
    login_rate_limit_window_seconds: 0,
    web_compression_enabled: false,
    web_use_minified_assets: false,
    web_cache_max_age_html: 0,
    web_cache_max_age_css: 0,
    web_cache_max_age_js: 0,
    web_cache_max_age_images: 0,
    web_cache_max_age_fonts: 0,
    web_cache_max_age_default: 0,
    onvif_discovery_enabled: false,
    onvif_discovery_interval: 0,
    onvif_discovery_network: [0; 64],
    max_streams: 0,
    streams: ::core::ptr::null_mut::<stream_config_t>(),
    buffer_size: 0,
    use_swap: false,
    swap_file: [0; 512],
    swap_size: 0,
    memory_constrained: false,
    hw_accel_enabled: false,
    hw_accel_device: [0; 32],
    go2rtc_enabled: false,
    go2rtc_binary_path: [0; 512],
    go2rtc_config_dir: [0; 512],
    go2rtc_api_port: 0,
    go2rtc_rtsp_port: 0,
    go2rtc_force_native_hls: false,
    go2rtc_proxy_max_inflight: 0,
    go2rtc_webrtc_enabled: false,
    go2rtc_webrtc_listen_port: 0,
    go2rtc_stun_enabled: false,
    go2rtc_stun_server: [0; 256],
    go2rtc_external_ip: [0; 256],
    go2rtc_ice_servers: [0; 512],
    webrtc_connection_timeout_ms: 0,
    webrtc_ice_recovery_timeout_ms: 0,
    turn_enabled: false,
    turn_server_url: [0; 256],
    turn_username: [0; 64],
    turn_password: [0; 64],
    mqtt_enabled: false,
    mqtt_broker_host: [0; 256],
    mqtt_broker_port: 0,
    mqtt_username: [0; 64],
    mqtt_password: [0; 128],
    mqtt_client_id: [0; 64],
    mqtt_topic_prefix: [0; 128],
    mqtt_tls_enabled: false,
    mqtt_keepalive: 0,
    mqtt_qos: 0,
    mqtt_retain: false,
    mqtt_ha_discovery: false,
    mqtt_ha_discovery_prefix: [0; 128],
    mqtt_ha_snapshot_interval: 0,
    health: system_health_policy_settings_t {
        enabled: false,
        profile: [0; 16],
        fast_interval_seconds: 0,
        normal_interval_seconds: 0,
        slow_interval_seconds: 0,
        device_interval_seconds: 0,
        write_probe_enabled: false,
        hardware_provider: [0; 16],
        presence_interval_seconds: 0,
        incident_retention_days: 0,
    },
    detection_engine: detection_engine_config_t {
        enabled: false,
        num_threads: 0,
        delegate: [0; 16],
    },
};
#[unsafe(no_mangle)]
pub static mut http_server: http_server_handle_t = ::core::ptr::null_mut::<http_server_t>();
unsafe extern "C" fn signal_handler(mut sig: ::core::ffi::c_int) {
    if is_coordinator_destroyed() {
        _exit(0 as ::core::ffi::c_int);
    }
    static mut shutdown_in_progress: sig_atomic_t = 0 as sig_atomic_t;
    if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const shutdown_in_progress) != 0 {
        signal_safe_write(
            b"[SIGNAL] Received signal during shutdown, forcing immediate exit\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        _exit(1 as ::core::ffi::c_int);
    }
    ::core::ptr::write_volatile(
        &raw mut shutdown_in_progress,
        1 as ::core::ffi::c_int as sig_atomic_t,
    );
    signal_safe_write(
        b"[SIGNAL] Received shutdown signal, initiating shutdown...\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    running = r#false != 0;
    request_background_abort();
}
unsafe extern "C" fn alarm_handler(mut sig: ::core::ffi::c_int) {
    signal_safe_write(
        b"[SIGNAL] Stray SIGALRM caught at top level (harmless, ignored)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
pub const SHUTDOWN_WATCHDOG_TIMEOUT_SECONDS: ::core::ffi::c_int = 900 as ::core::ffi::c_int;
unsafe extern "C" fn shutdown_watchdog_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut shutdown_started: bool = r#false != 0;
    let mut shutdown_started_at: timespec = timespec {
        tv_sec: 0 as __time_t,
        tv_nsec: 0,
    };
    loop {
        sleep(1 as ::core::ffi::c_uint);
        if !running {
            let mut now: timespec = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            clock_gettime(CLOCK_MONOTONIC, &raw mut now);
            if !shutdown_started {
                shutdown_started_at = now;
                shutdown_started = r#true != 0;
            } else if now.tv_sec - shutdown_started_at.tv_sec
                >= SHUTDOWN_WATCHDOG_TIMEOUT_SECONDS as __time_t
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Shutdown watchdog: graceful shutdown did not complete within %d seconds, force-killing process group\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    900 as ::core::ffi::c_int,
                );
                kill(0 as __pid_t, SIGKILL);
                _exit(EXIT_FAILURE);
            }
        }
    }
}
unsafe extern "C" fn start_shutdown_watchdog_thread() {
    let mut thread: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed_11::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    if pthread_create(
        &raw mut thread,
        &raw mut attr,
        Some(
            shutdown_watchdog_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start shutdown watchdog thread: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    pthread_attr_destroy(&raw mut attr);
}
unsafe extern "C" fn init_signals() {
    let mut uts_info: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    let mut is_linux_4_4: bool = r#false != 0;
    if uname(&raw mut uts_info) == 0 as ::core::ffi::c_int {
        if strncmp(
            &raw mut uts_info.release as *mut ::core::ffi::c_char,
            b"4.4\0".as_ptr() as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Detected Linux 4.4 kernel, using compatible signal handling\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            is_linux_4_4 = r#true != 0;
        }
    }
    if !daemon_mode || !is_linux_4_4 {
        let mut sa: sigaction = sigaction {
            __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &raw mut sa as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sigaction>(),
        );
        sa.__sigaction_handler.sa_handler =
            Some(signal_handler as unsafe extern "C" fn(::core::ffi::c_int) -> ())
                as __sighandler_t;
        sa.sa_flags = 0 as ::core::ffi::c_int;
        sigaction(SIGINT, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
        sigaction(SIGTERM, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
        sigaction(SIGHUP, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
        let mut sa_alarm: sigaction = sigaction {
            __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &raw mut sa_alarm as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sigaction>(),
        );
        sa_alarm.__sigaction_handler.sa_handler =
            Some(alarm_handler as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
        sa_alarm.sa_flags = SA_RESTART;
        sigaction(
            SIGALRM,
            &raw mut sa_alarm,
            ::core::ptr::null_mut::<sigaction>(),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Running in daemon mode on Linux 4.4, signal handlers will be set up by daemon.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut sa_pipe: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &raw mut sa_pipe as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sigaction>(),
    );
    sa_pipe.__sigaction_handler.sa_handler = ::core::mem::transmute::<
        ::libc::intptr_t,
        __sighandler_t,
    >(1 as ::core::ffi::c_int as ::libc::intptr_t);
    sa_pipe.sa_flags = SA_RESTART;
    sigaction(
        SIGPIPE,
        &raw mut sa_pipe,
        ::core::ptr::null_mut::<sigaction>(),
    );
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&raw mut set);
    sigaddset(&raw mut set, SIGPIPE);
    pthread_sigmask(
        SIG_BLOCK,
        &raw mut set,
        ::core::ptr::null_mut::<__sigset_t>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Signal handlers initialized with improved handling\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn is_lightnvr_process(mut pid: pid_t) -> bool {
    let mut path: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"/proc/%d/comm\0".as_ptr() as *const ::core::ffi::c_char,
        pid,
    );
    let mut fp: *mut FILE = fopen(
        &raw mut path as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if fp.is_null() {
        return r#true != 0;
    }
    let mut comm: [::core::ffi::c_char; 32] = [
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
    let mut ok: bool = !fgets(
        &raw mut comm as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as ::core::ffi::c_int,
        fp,
    )
    .is_null();
    fclose(fp);
    if !ok {
        return r#true != 0;
    }
    comm[strcspn(
        &raw mut comm as *mut ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
    ) as usize] = '\0' as ::core::ffi::c_char;
    return strcmp(
        &raw mut comm as *mut ::core::ffi::c_char,
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn check_and_kill_existing_instance(
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fp: *mut FILE = fopen(pid_file, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if fp.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut pid_buf: [::core::ffi::c_char; 32] = [
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
    if fgets(
        &raw mut pid_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as ::core::ffi::c_int,
        fp,
    )
    .is_null()
    {
        fclose(fp);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid PID file format\0".as_ptr() as *const ::core::ffi::c_char,
        );
        unlink(pid_file);
        return 0 as ::core::ffi::c_int;
    }
    fclose(fp);
    let mut end_ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pid_val: ::core::ffi::c_long = strtol(
        &raw mut pid_buf as *mut ::core::ffi::c_char,
        &raw mut end_ptr,
        10 as ::core::ffi::c_int,
    );
    if end_ptr == &raw mut pid_buf as *mut ::core::ffi::c_char
        || pid_val <= 0 as ::core::ffi::c_long
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid PID file format\0".as_ptr() as *const ::core::ffi::c_char,
        );
        unlink(pid_file);
        return 0 as ::core::ffi::c_int;
    }
    let mut existing_pid: pid_t = pid_val as pid_t;
    if kill(existing_pid, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        if !is_lightnvr_process(existing_pid) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"PID %d from stale PID file is not a LightNVR process (PID reuse after container restart); removing stale PID file\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                existing_pid,
            );
            unlink(pid_file);
            return 0 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Another instance with PID %d appears to be running\0".as_ptr()
                as *const ::core::ffi::c_char,
            existing_pid,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempting to terminate previous instance (PID: %d) with SIGTERM\0".as_ptr()
                as *const ::core::ffi::c_char,
            existing_pid,
        );
        if kill(existing_pid, SIGTERM) == 0 as ::core::ffi::c_int {
            let mut timeout: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
            loop {
                let c2rust_fresh0 = timeout;
                timeout -= 1;
                if !(c2rust_fresh0 > 0 as ::core::ffi::c_int
                    && kill(existing_pid, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int)
                {
                    break;
                }
                sleep(1 as ::core::ffi::c_uint);
            }
            if timeout <= 0 as ::core::ffi::c_int
                && kill(existing_pid, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Process didn't terminate gracefully within timeout, using SIGKILL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                kill(existing_pid, SIGKILL);
                sleep(1 as ::core::ffi::c_uint);
            }
            timeout = 5 as ::core::ffi::c_int;
            loop {
                let c2rust_fresh1 = timeout;
                timeout -= 1;
                if c2rust_fresh1 <= 0 as ::core::ffi::c_int {
                    break;
                }
                let mut test_fd: ::core::ffi::c_int = open(pid_file, O_RDWR);
                if test_fd < 0 as ::core::ffi::c_int {
                    if *__errno_location() == ENOENT {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Previous instance terminated and PID file released\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return 0 as ::core::ffi::c_int;
                    }
                } else {
                    if lockf(test_fd, F_TLOCK, 0 as __off_t) == 0 as ::core::ffi::c_int {
                        close(test_fd);
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Previous instance terminated and PID file lock released\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        unlink(pid_file);
                        return 0 as ::core::ffi::c_int;
                    }
                    close(test_fd);
                }
                sleep(1 as ::core::ffi::c_uint);
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Previous instance may have terminated but PID file is still locked or inaccessible\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            if unlink(pid_file) == 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Removed potentially stale PID file\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_int;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to remove PID file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
                return -1 as ::core::ffi::c_int;
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to terminate previous instance: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Removing stale PID file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        unlink(pid_file);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn daemonize(mut pid_file: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = init_daemon(pid_file);
    if fd < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    daemon_mode = r#true != 0;
    running = r#true != 0;
    return fd;
}
unsafe extern "C" fn detect_container_mode() -> bool {
    if access(
        b"/.dockerenv\0".as_ptr() as *const ::core::ffi::c_char,
        F_OK,
    ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detected Docker container (/.dockerenv exists)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    if !getenv(b"KUBERNETES_SERVICE_HOST\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detected Kubernetes container (KUBERNETES_SERVICE_HOST set)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    let mut cgroup: *mut FILE = fopen(
        b"/proc/1/cgroup\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !cgroup.is_null() {
        let mut line: [::core::ffi::c_char; 256] = [0; 256];
        let mut in_container: bool = r#false != 0;
        while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
            cgroup,
        )
        .is_null()
        {
            if !(!strstr(
                &raw mut line as *mut ::core::ffi::c_char,
                b"docker\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
                || !strstr(
                    &raw mut line as *mut ::core::ffi::c_char,
                    b"kubepods\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                || !strstr(
                    &raw mut line as *mut ::core::ffi::c_char,
                    b"containerd\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                || !strstr(
                    &raw mut line as *mut ::core::ffi::c_char,
                    b"lxc\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null())
            {
                continue;
            }
            in_container = r#true != 0;
            break;
        }
        fclose(cgroup);
        if in_container {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Detected container environment (cgroup indicators)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#true != 0;
        }
    }
    if getpid() == 1 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Running as PID 1 - likely in a container\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_restart() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"request_restart() called - setting restart_requested=true and running=false\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    restart_requested = r#true != 0;
    running = r#false != 0;
    request_background_abort();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"request_restart() completed - restart_requested=%d, running=%d\0".as_ptr()
            as *const ::core::ffi::c_char,
        restart_requested as ::core::ffi::c_int,
        running as ::core::ffi::c_int,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_restart_requested() -> bool {
    return restart_requested;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut default_recording_root: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut default_storage_target_uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut health_overrides: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut health_overrides_length: size_t = 0;
    let mut health_overrides_result: ::core::ffi::c_int = 0;
    let mut health_policy: system_health_policy_t = system_health_policy_t {
        generation: 0,
        settings: system_health_policy_settings_t {
            enabled: false,
            profile: [0; 16],
            fast_interval_seconds: 0,
            normal_interval_seconds: 0,
            slow_interval_seconds: 0,
            device_interval_seconds: 0,
            write_probe_enabled: false,
            hardware_provider: [0; 16],
            presence_interval_seconds: 0,
            incident_retention_days: 0,
        },
        conditions: [system_health_condition_policy_t {
            condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
            profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
            enabled: false,
            overridden: false,
            direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            warning_threshold: 0.,
            critical_threshold: 0.,
            recovery_threshold: 0.,
            warning_for_seconds: 0,
            critical_for_seconds: 0,
            recovery_for_seconds: 0,
        }; 27],
    };
    let mut health_policy_error: [::core::ffi::c_char; 256] = [0; 256];
    let mut health_options: system_health_options_t = system_health_options_t {
        enabled: false,
        register_builtin_collectors: false,
        tier_interval_ms: [0; 4],
        collector_deadline_ms: [0; 4],
        proc_root: [0; 1024],
        sys_root: [0; 1024],
        cgroup_root: [0; 1024],
        root_path: [0; 1024],
        recording_path: [0; 1024],
        hardware_provider: [0; 16],
    };
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
    let mut server_config: http_server_config_t = http_server_config_t {
        port: 0,
        bind_ip: ::core::ptr::null::<::core::ffi::c_char>(),
        web_root: ::core::ptr::null::<::core::ffi::c_char>(),
        auth_enabled: false,
        username: [0; 32],
        password: [0; 32],
        cors_enabled: false,
        allowed_origins: [0; 256],
        allowed_methods: [0; 256],
        allowed_headers: [0; 256],
        ssl_enabled: false,
        cert_path: [0; 512],
        key_path: [0; 512],
        max_connections: 0,
        connection_timeout: 0,
        daemon_mode: false,
        pid_file: [0; 512],
    };
    let mut service_check_init_time: time_t = 0;
    let mut pid_fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if prctl(
        PR_SET_THP_DISABLE,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Note: prctl(PR_SET_THP_DISABLE) not supported on this kernel\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    saved_argc = argc;
    saved_argv = argv as *mut *mut ::core::ffi::c_char;
    printf(
        b"LightNVR v%s - Lightweight NVR\n\0".as_ptr() as *const ::core::ffi::c_char,
        LIGHTNVR_VERSION_STRING.as_ptr(),
    );
    printf(
        b"Build date: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        LIGHTNVR_BUILD_DATE.as_ptr(),
    );
    if init_logger() != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Failed to initialize logger\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return EXIT_FAILURE;
    }
    let mut custom_config_path: [::core::ffi::c_char; 512] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut verbose_mode: bool = r#false != 0;
    let mut generate_go2rtc_config_only: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-d\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--daemon\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            daemon_mode = r#true != 0;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-c\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--config\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            if (i + 1 as ::core::ffi::c_int) < argc {
                safe_strcpy(
                    &raw mut custom_config_path as *mut ::core::ffi::c_char,
                    *argv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    MAX_PATH_LENGTH as size_t,
                    0 as size_t,
                );
                i += 1;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Missing config file path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return EXIT_FAILURE;
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"--generate-go2rtc-config\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            generate_go2rtc_config_only = r#true != 0;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--verbose\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            verbose_mode = r#true != 0;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-h\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--help\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            printf(
                b"Usage: %s [options]\n\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0isize),
            );
            printf(b"Options:\n\0".as_ptr() as *const ::core::ffi::c_char);
            printf(
                b"  -d, --daemon        Run as daemon\n\0".as_ptr() as *const ::core::ffi::c_char
            );
            printf(
                b"  -c, --config FILE   Use config file\n\0".as_ptr() as *const ::core::ffi::c_char
            );
            printf(b"  --generate-go2rtc-config\n\0".as_ptr() as *const ::core::ffi::c_char);
            printf(
                b"                     Generate go2rtc.yaml from saved settings and exit\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            printf(
                b"  --verbose           Enable verbose logging (debug level)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            printf(
                b"  -h, --help          Show this help\n\0".as_ptr() as *const ::core::ffi::c_char
            );
            printf(b"  -v, --version       Show version\n\0".as_ptr() as *const ::core::ffi::c_char);
            return EXIT_SUCCESS;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-v\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--version\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            return EXIT_SUCCESS;
        }
        i += 1;
    }
    if verbose_mode {
        set_log_level(log_level_t::LOG_LEVEL_DEBUG);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Verbose logging enabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if custom_config_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        set_custom_config_path(&raw mut custom_config_path as *mut ::core::ffi::c_char);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using custom config path: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut custom_config_path as *mut ::core::ffi::c_char,
        );
    }
    if load_config(&raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return EXIT_FAILURE;
    }
    if config.log_file[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if set_log_file(&raw mut config.log_file as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to set log file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.log_file as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Logging to file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.log_file as *mut ::core::ffi::c_char,
            );
        }
    }
    fprintf(
        stderr,
        b"Setting log level from config: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        config.log_level,
    );
    set_log_level(log_level_t(config.log_level as ::core::ffi::c_uint));
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Log level set to %d (%s)\0".as_ptr() as *const ::core::ffi::c_char,
        config.log_level,
        get_log_level_string(log_level_t(config.log_level as ::core::ffi::c_uint)),
    );
    if config.syslog_enabled {
        if enable_syslog(
            &raw mut config.syslog_ident as *mut ::core::ffi::c_char,
            config.syslog_facility,
        ) == 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Syslog enabled (ident: %s, facility: %d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.syslog_ident as *mut ::core::ffi::c_char,
                config.syslog_facility,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to enable syslog\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    config_copy(&raw mut g_config, &raw mut config);
    if generate_go2rtc_config_only {
        if curl_init_global() != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize libcurl globally for go2rtc config generation\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return EXIT_FAILURE;
        }
        let mut db_initialized_for_generation: bool = r#false != 0;
        if config.db_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            if init_database_ex(
                &raw mut config.db_path as *mut ::core::ffi::c_char,
                C2Rust_Unnamed_12::DB_INIT_READ_ONLY.0,
            ) == 0 as ::core::ffi::c_int
            {
                db_initialized_for_generation = r#true != 0;
                if load_stream_configs(&raw mut config) < 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to load stream configurations while generating go2rtc config\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                config_copy(&raw mut g_config, &raw mut config);
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to initialize database while generating go2rtc config; DB-backed overrides unavailable\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
        let mut go2rtc_binary: *const ::core::ffi::c_char = if config.go2rtc_binary_path[0usize]
            as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            &raw mut config.go2rtc_binary_path as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        let mut go2rtc_config_dir: *const ::core::ffi::c_char =
            if config.go2rtc_config_dir[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                &raw mut config.go2rtc_config_dir as *mut ::core::ffi::c_char
            } else {
                b"/etc/lightnvr/go2rtc\0".as_ptr() as *const ::core::ffi::c_char
            };
        let mut generated: bool = go2rtc_process_generate_startup_config(
            go2rtc_binary,
            go2rtc_config_dir,
            config.go2rtc_api_port,
        );
        if db_initialized_for_generation {
            shutdown_database();
        }
        lightnvr_sentry_shutdown();
        curl_cleanup_global();
        return if generated as ::core::ffi::c_int != 0 {
            EXIT_SUCCESS
        } else {
            EXIT_FAILURE
        };
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"LightNVR v%s starting up\0".as_ptr() as *const ::core::ffi::c_char,
        b"0.48.2\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if init_shutdown_coordinator() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize shutdown coordinator\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return EXIT_FAILURE;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Shutdown coordinator initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    init_signals();
    if check_and_kill_existing_instance(&raw mut config.pid_file as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to handle existing instance\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return EXIT_FAILURE;
    }
    if daemon_mode {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Starting in daemon mode\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pid_fd = daemonize(&raw mut config.pid_file as *mut ::core::ffi::c_char);
        if pid_fd < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to daemonize\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return EXIT_FAILURE;
        }
    } else {
        pid_fd = write_pid_file(&raw mut config.pid_file as *mut ::core::ffi::c_char);
        if pid_fd < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create PID file\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return EXIT_FAILURE;
        }
    }
    start_shutdown_watchdog_thread();
    container_mode = detect_container_mode();
    if container_mode {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Container mode detected - restart will exit and rely on container orchestrator\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Native mode detected - restart will use execv()\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if curl_init_global() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize libcurl globally\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return EXIT_FAILURE;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"libcurl initialized globally\0".as_ptr() as *const ::core::ffi::c_char,
    );
    lightnvr_sentry_init();
    storage_startup_reclaim_if_full(
        &raw mut config.storage_path as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    '_cleanup: {
        if init_database(&raw mut config.db_path as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize database\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if db_authorization_verify_action_catalog() != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Authorization action catalog does not match this build\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            audit_log_decision_modes_init();
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Initializing schema cache...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            init_schema_cache();
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Schema cache initialized\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if load_stream_configs(&raw mut config) < 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to load stream configurations from database\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            config_copy(&raw mut g_config, &raw mut config);
            default_recording_root = if config.record_mp4_directly as ::core::ffi::c_int != 0
                && config.mp4_storage_path[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                &raw mut config.mp4_storage_path as *mut ::core::ffi::c_char
            } else {
                &raw mut config.storage_path as *mut ::core::ffi::c_char
            };
            default_storage_target_uuid = [0; 37];
            if db_storage_target_bootstrap_default(
                default_recording_root,
                &raw mut default_storage_target_uuid as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to initialize the default storage target; continuing without storage target attribution\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if event_identity_init() != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to initialize persistent event identity\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else if mqtt_event_adapter_register(&raw mut g_config) != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to register MQTT event compatibility adapter\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else if event_bus_init(0 as size_t, 0 as size_t) != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to initialize asynchronous event bus\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Asynchronous event pipeline initialized successfully\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if init_storage_manager(
                &raw mut config.storage_path as *mut ::core::ffi::c_char,
                config.max_storage_size,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to initialize storage manager\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                set_retention_days(config.retention_days);
                storage_target_refresh_health_and_publish();
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Storage manager initialized\0".as_ptr() as *const ::core::ffi::c_char,
                );
                let mut observation_reconcile: observation_reconcile_result_t =
                    observation_reconcile_result_t {
                        tmp_files_deleted: 0,
                        stale_files_deleted: 0,
                        pending_interrupted: 0,
                        ready_missing: 0,
                        epoch_regenerated: false,
                    };
                if observation_retention_startup(
                    &raw mut config.storage_path as *mut ::core::ffi::c_char,
                    &raw mut observation_reconcile,
                ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Observation evidence reconciliation incomplete; evidence eviction disabled\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                let mut evidence_config: observation_evidence_config_t =
                    observation_evidence_config_t {
                        storage_path: ::core::ptr::null::<::core::ffi::c_char>(),
                        global_max_bytes: 0,
                        global_max_items: 0,
                        per_camera_max_items: 0,
                        per_camera_max_bytes: 0,
                    };
                let mut queue_bytes: [::core::ffi::c_char; 32] = [
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
                evidence_config.storage_path =
                    &raw mut config.storage_path as *mut ::core::ffi::c_char;
                if db_get_system_setting(
                    b"observation_global_queue_bytes\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut queue_bytes as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
                    && queue_bytes[0usize] as ::core::ffi::c_int != 0
                {
                    let mut value: ::core::ffi::c_longlong = strtoll(
                        &raw mut queue_bytes as *mut ::core::ffi::c_char,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                    );
                    if value > 0 as ::core::ffi::c_longlong {
                        evidence_config.global_max_bytes = value as int64_t;
                    }
                }
                if observation_evidence_init(&raw mut evidence_config) != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to initialize observation evidence; evidence admission disabled\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    observation_evidence_set_enabled_probe(
                        Some(
                            observation_evidence_stream_enabled_probe
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_char,
                                    *mut ::core::ffi::c_void,
                                ) -> bool,
                        ),
                        NULL,
                    );
                }
                if observation_relay_start(::core::ptr::null::<observation_relay_options_t>())
                    != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to start observation publication relay; staged observation changes stay pending\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                health_overrides = ::core::ptr::null_mut::<::core::ffi::c_char>();
                health_overrides_length = 0 as size_t;
                health_overrides_result = db_get_system_setting_alloc(
                    SYSTEM_HEALTH_OVERRIDES_SETTING_KEY.as_ptr(),
                    &raw mut health_overrides,
                    &raw mut health_overrides_length,
                );
                health_policy = system_health_policy_t {
                    generation: 0,
                    settings: system_health_policy_settings_t {
                        enabled: false,
                        profile: [0; 16],
                        fast_interval_seconds: 0,
                        normal_interval_seconds: 0,
                        slow_interval_seconds: 0,
                        device_interval_seconds: 0,
                        write_probe_enabled: false,
                        hardware_provider: [0; 16],
                        presence_interval_seconds: 0,
                        incident_retention_days: 0,
                    },
                    conditions: [system_health_condition_policy_t {
                        condition:
                            system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
                        profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
                        enabled: false,
                        overridden: false,
                        direction:
                            system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
                        unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
                        warning_threshold: 0.,
                        critical_threshold: 0.,
                        recovery_threshold: 0.,
                        warning_for_seconds: 0,
                        critical_for_seconds: 0,
                        recovery_for_seconds: 0,
                    }; 27],
                };
                health_policy_error = [0; 256];
                if health_overrides_result < 0 as ::core::ffi::c_int
                    || system_health_policy_build(
                        &raw mut g_config.health,
                        if health_overrides_result == 0 as ::core::ffi::c_int {
                            health_overrides
                        } else {
                            ::core::ptr::null_mut::<::core::ffi::c_char>()
                        },
                        &raw mut health_policy,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        0 as size_t,
                        &raw mut health_policy_error as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Host health policy unavailable: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        if health_overrides_result < 0 as ::core::ffi::c_int {
                            b"stored overrides could not be read\0".as_ptr()
                                as *const ::core::ffi::c_char
                        } else {
                            &raw mut health_policy_error as *mut ::core::ffi::c_char
                        },
                    );
                } else if system_health_policy_replace(
                    &raw mut health_policy,
                    &raw mut health_policy_error as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Host health policy could not be activated: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut health_policy_error as *mut ::core::ffi::c_char,
                    );
                }
                free(health_overrides as *mut ::core::ffi::c_void);
                health_options = system_health_options_t {
                    enabled: false,
                    register_builtin_collectors: false,
                    tier_interval_ms: [0; 4],
                    collector_deadline_ms: [0; 4],
                    proc_root: [0; 1024],
                    sys_root: [0; 1024],
                    cgroup_root: [0; 1024],
                    root_path: [0; 1024],
                    recording_path: [0; 1024],
                    hardware_provider: [0; 16],
                };
                if system_health_options_from_policy(
                    &raw mut g_config.health,
                    default_recording_root,
                    &raw mut health_options,
                ) != 0 as ::core::ffi::c_int
                    || system_health_init(&raw mut health_options) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Host health sampler could not be initialized\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Starting recording sync thread...\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if start_recording_sync_thread(60 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to start recording sync thread, file sizes may not be accurate\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Recording sync thread started\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                st = stat {
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
                if stat(
                    &raw mut config.web_root as *mut ::core::ffi::c_char,
                    &raw mut st,
                ) != 0 as ::core::ffi::c_int
                    || !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Web root directory %s does not exist or is not a directory\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut config.web_root as *mut ::core::ffi::c_char,
                    );
                    if strncmp(
                        &raw mut config.web_root as *mut ::core::ffi::c_char,
                        b"/var/\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as size_t,
                    ) == 0 as ::core::ffi::c_int
                        || strncmp(
                            &raw mut config.web_root as *mut ::core::ffi::c_char,
                            b"/tmp/\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as size_t,
                        ) == 0 as ::core::ffi::c_int
                        || strncmp(
                            &raw mut config.web_root as *mut ::core::ffi::c_char,
                            b"/run/\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        let mut storage_web_path: [::core::ffi::c_char; 512] = [0; 512];
                        snprintf(
                            &raw mut storage_web_path as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            b"%s/web\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut config.storage_path as *mut ::core::ffi::c_char,
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Web root is in system directory (%s), redirecting to storage path (%s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut config.web_root as *mut ::core::ffi::c_char,
                            &raw mut storage_web_path as *mut ::core::ffi::c_char,
                        );
                        if mkdir_recursive(&raw mut storage_web_path as *mut ::core::ffi::c_char)
                            != 0
                        {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to create web root in storage path: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                strerror(*__errno_location()),
                            );
                            break '_cleanup;
                        } else {
                            if ensure_path(&raw mut config.web_root as *mut ::core::ffi::c_char)
                                != 0
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to create parent directory for web root symlink\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            if symlink(
                                &raw mut storage_web_path as *mut ::core::ffi::c_char,
                                &raw mut config.web_root as *mut ::core::ffi::c_char,
                            ) != 0 as ::core::ffi::c_int
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_ERROR,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to create symlink from %s to %s: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut config.web_root as *mut ::core::ffi::c_char,
                                    &raw mut storage_web_path as *mut ::core::ffi::c_char,
                                    strerror(*__errno_location()),
                                );
                                safe_strcpy(
                                    &raw mut config.web_root as *mut ::core::ffi::c_char,
                                    &raw mut storage_web_path as *mut ::core::ffi::c_char,
                                    MAX_PATH_LENGTH as size_t,
                                    0 as size_t,
                                );
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Using storage path directly for web root: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut config.web_root as *mut ::core::ffi::c_char,
                                );
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Created symlink from %s to %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut config.web_root as *mut ::core::ffi::c_char,
                                    &raw mut storage_web_path as *mut ::core::ffi::c_char,
                                );
                            }
                        }
                    } else if mkdir_recursive(&raw mut config.web_root as *mut ::core::ffi::c_char)
                        != 0
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to create web root directory\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        break '_cleanup;
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Created web root directory: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut config.web_root as *mut ::core::ffi::c_char,
                        );
                    }
                }
                if system_health_evaluator_service_start() != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Host health evaluator could not be started\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if system_health_start() != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Host health sampler workers could not be started\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if g_config.health.enabled {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Host health sampler started\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                if init_stream_state_manager(config.max_streams) != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to initialize stream state manager\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if init_stream_state_adapter() != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to initialize stream state adapter\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if init_stream_manager(config.max_streams) != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to initialize stream manager\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if metrics_init(config.max_streams) != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to initialize metrics subsystem\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else {
                    player_telemetry_init();
                    if !config.go2rtc_enabled {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"go2rtc is disabled in configuration. HLS will connect directly to camera streams.\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"WebRTC live view will not be available. Enable go2rtc in settings if needed.\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    } else {
                        go2rtc_process_validate_existing_override_on_upgrade();
                        if !go2rtc_integration_full_start() {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to start go2rtc integration.\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Ensure go2rtc is installed and accessible (scripts/install_go2rtc.sh).\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"LiveView and WebRTC streaming will not be available until go2rtc is properly configured.\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    init_transcoding_backend();
                    init_timestamp_trackers();
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Timestamp trackers initialized\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    init_hls_streaming_backend();
                    init_mp4_recording_backend();
                    if init_detection_integration() != 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to initialize detection system\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Detection system initialized successfully\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    init_detection_stream_system();
                    if init_onvif_discovery() != 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to initialize ONVIF discovery module\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"ONVIF discovery module initialized successfully\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        if config.onvif_discovery_enabled {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Starting ONVIF discovery on network %s with interval %d seconds\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut config.onvif_discovery_network as *mut ::core::ffi::c_char,
                                config.onvif_discovery_interval,
                            );
                            if start_onvif_discovery(
                                &raw mut config.onvif_discovery_network as *mut ::core::ffi::c_char,
                                config.onvif_discovery_interval,
                            ) != 0 as ::core::ffi::c_int
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_ERROR,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to start ONVIF discovery\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"ONVIF discovery started successfully\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        }
                    }
                    if init_auth_system() != 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to initialize authentication system\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Authentication system initialized successfully\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        if batch_delete_progress_init() != 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to initialize batch delete progress tracking\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Batch delete progress tracking initialized successfully\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        if mqtt_init(&raw mut config) != 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to initialize MQTT runtime\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"MQTT runtime initialized successfully\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            if mqtt_delivery_worker_start() != 0 as ::core::ffi::c_int {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_ERROR,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to start durable MQTT delivery worker\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            if config.mqtt_enabled {
                                if mqtt_connect() != 0 as ::core::ffi::c_int {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_WARN,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Failed to connect to MQTT broker, will retry automatically\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Connected to MQTT broker\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            }
                        }
                        server_config = http_server_config_t {
                            port: config.web_port,
                            bind_ip: &raw mut config.web_bind_ip as *mut ::core::ffi::c_char,
                            web_root: &raw mut config.web_root as *mut ::core::ffi::c_char,
                            auth_enabled: config.web_auth_enabled,
                            username: [0; 32],
                            password: [0; 32],
                            cors_enabled: r#true != 0,
                            allowed_origins: [0; 256],
                            allowed_methods: [0; 256],
                            allowed_headers: [0; 256],
                            ssl_enabled: config.web_ssl_enabled,
                            cert_path: [0; 512],
                            key_path: [0; 512],
                            max_connections: 100 as ::core::ffi::c_int,
                            connection_timeout: 30 as ::core::ffi::c_int,
                            daemon_mode: daemon_mode,
                            pid_file: [0; 512],
                        };
                        safe_strcpy(
                            &raw mut server_config.cert_path as *mut ::core::ffi::c_char,
                            &raw mut config.web_ssl_cert_path as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            0 as size_t,
                        );
                        safe_strcpy(
                            &raw mut server_config.key_path as *mut ::core::ffi::c_char,
                            &raw mut config.web_ssl_key_path as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            0 as size_t,
                        );
                        safe_strcpy(
                            &raw mut server_config.allowed_origins as *mut ::core::ffi::c_char,
                            b"*\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                            0 as size_t,
                        );
                        safe_strcpy(
                            &raw mut server_config.allowed_methods as *mut ::core::ffi::c_char,
                            b"GET, POST, PUT, DELETE, OPTIONS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                            0 as size_t,
                        );
                        safe_strcpy(
                            &raw mut server_config.allowed_headers as *mut ::core::ffi::c_char,
                            b"Content-Type, Authorization\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                            0 as size_t,
                        );
                        if config.web_auth_enabled {
                            safe_strcpy(
                                &raw mut server_config.username as *mut ::core::ffi::c_char,
                                &raw mut config.web_username as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                                0 as size_t,
                            );
                            safe_strcpy(
                                &raw mut server_config.password as *mut ::core::ffi::c_char,
                                &raw mut config.web_password as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                                0 as size_t,
                            );
                        }
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Initializing web server on %s:%d (daemon_mode: %s)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut config.web_bind_ip as *mut ::core::ffi::c_char,
                            config.web_port,
                            if daemon_mode as ::core::ffi::c_int != 0 {
                                b"true\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"false\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        );
                        http_server = libuv_server_init(&raw mut server_config);
                        if http_server.is_null() {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to initialize libuv web server\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"libuv web server initialized successfully\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            if register_all_libuv_handlers(http_server) != 0 as ::core::ffi::c_int {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_ERROR,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to register API handlers\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                http_server_destroy(http_server);
                                http_server = ::core::ptr::null_mut::<http_server_t>();
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"API handlers registered successfully\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                if register_static_file_handler(http_server)
                                    != 0 as ::core::ffi::c_int
                                {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_ERROR,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Failed to register static file handler\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    http_server_destroy(http_server);
                                    http_server = ::core::ptr::null_mut::<http_server_t>();
                                } else {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Static file handler registered successfully\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Starting web server...\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    if http_server_start(http_server) != 0 as ::core::ffi::c_int {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_ERROR,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Failed to start libuv web server on %s:%d\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut config.web_bind_ip as *mut ::core::ffi::c_char,
                                            config.web_port,
                                        );
                                        http_server_destroy(http_server);
                                        http_server = ::core::ptr::null_mut::<http_server_t>();
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"libuv web server started successfully on %s:%d\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut config.web_bind_ip as *mut ::core::ffi::c_char,
                                            config.web_port,
                                        );
                                        init_health_check_system();
                                        start_health_check_thread();
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Web server health check system started\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        if daemon_mode {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_INFO,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Daemon mode: Verifying port %d is accessible...\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                config.web_port,
                                            );
                                            sleep(1 as ::core::ffi::c_uint);
                                            let mut test_socket: ::core::ffi::c_int = socket(
                                                AF_INET,
                                                __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
                                                0 as ::core::ffi::c_int,
                                            );
                                            if test_socket >= 0 as ::core::ffi::c_int {
                                                let mut addr: sockaddr_in = sockaddr_in {
                                                    sin_family: 0,
                                                    sin_port: 0,
                                                    sin_addr: in_addr { s_addr: 0 },
                                                    sin_zero: [0; 8],
                                                };
                                                memset(
                                                    &raw mut addr as *mut ::core::ffi::c_void,
                                                    0 as ::core::ffi::c_int,
                                                    ::core::mem::size_of::<sockaddr_in>(),
                                                );
                                                addr.sin_family = AF_INET as sa_family_t;
                                                addr.sin_addr.s_addr = __bswap_32(
                                                    0x7f000001 as ::core::ffi::c_int as __uint32_t,
                                                )
                                                    as in_addr_t;
                                                addr.sin_port =
                                                    __bswap_16(config.web_port as __uint16_t)
                                                        as in_port_t;
                                                if connect(
                                                    test_socket,
                                                    __CONST_SOCKADDR_ARG {
                                                        __sockaddr__: &raw mut addr
                                                            as *mut sockaddr,
                                                    },
                                                    ::core::mem::size_of::<sockaddr_in>()
                                                        as socklen_t,
                                                ) == 0 as ::core::ffi::c_int
                                                {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Port %d verification successful - server is accessible\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        config.web_port,
                                                    );
                                                } else {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_WARN,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Port %d verification failed - server may not be accessible: %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        config.web_port,
                                                        strerror(*__errno_location()),
                                                    );
                                                }
                                                close(test_socket);
                                            }
                                        }
                                        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        while i_0 < g_config.max_streams {
                                            if (*config.streams.offset(i_0 as isize)).name[0usize]
                                                as ::core::ffi::c_int
                                                != '\0' as ::core::ffi::c_int
                                                && (*config.streams.offset(i_0 as isize)).enabled
                                                    as ::core::ffi::c_int
                                                    != 0
                                                && (*config.streams.offset(i_0 as isize))
                                                    .detection_based_recording
                                                    as ::core::ffi::c_int
                                                    != 0
                                                && is_detection_recording_scheduled(
                                                    config.streams.offset(i_0 as isize),
                                                )
                                                    as ::core::ffi::c_int
                                                    != 0
                                            {
                                                let mut model_path: [::core::ffi::c_char; 512] =
                                                    [0; 512];
                                                let mut is_api_based: bool = strcmp(
                                                    &raw mut (*config.streams.offset(i_0 as isize))
                                                        .detection_model
                                                        as *mut ::core::ffi::c_char,
                                                    b"api-detection\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                ) == 0
                                                    as ::core::ffi::c_int
                                                    || strcmp(
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .detection_model
                                                            as *mut ::core::ffi::c_char,
                                                        b"motion\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    ) == 0 as ::core::ffi::c_int
                                                    || strcmp(
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .detection_model
                                                            as *mut ::core::ffi::c_char,
                                                        b"onvif\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    ) == 0 as ::core::ffi::c_int
                                                    || strncmp(
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .detection_model
                                                            as *mut ::core::ffi::c_char,
                                                        b"http://\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        7 as size_t,
                                                    ) == 0 as ::core::ffi::c_int
                                                    || strncmp(
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .detection_model
                                                            as *mut ::core::ffi::c_char,
                                                        b"https://\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        8 as size_t,
                                                    ) == 0 as ::core::ffi::c_int;
                                                if (*config.streams.offset(i_0 as isize))
                                                    .detection_model[0usize]
                                                    as ::core::ffi::c_int
                                                    == '\0' as ::core::ffi::c_int
                                                {
                                                    model_path[0usize] =
                                                        '\0' as ::core::ffi::c_char;
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"No local detection model configured for stream %s; external triggers remain available\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut (*config.streams.offset(i_0 as isize)).name
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                } else if is_api_based {
                                                    safe_strcpy(
                                                        &raw mut model_path
                                                            as *mut ::core::ffi::c_char,
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .detection_model
                                                            as *mut ::core::ffi::c_char,
                                                        MAX_PATH_LENGTH as size_t,
                                                        0 as size_t,
                                                    );
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Using built-in/API detection for stream %s: %s\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        &raw mut (*config.streams.offset(i_0 as isize)).name
                                                            as *mut ::core::ffi::c_char,
                                                        &raw mut model_path as *mut ::core::ffi::c_char,
                                                    );
                                                } else if (*config.streams.offset(i_0 as isize))
                                                    .detection_model[0usize]
                                                    as ::core::ffi::c_int
                                                    != '/' as ::core::ffi::c_int
                                                {
                                                    let mut models_dir: *const ::core::ffi::c_char =
                                                        if config.models_path[0usize]
                                                            as ::core::ffi::c_int
                                                            != '\0' as ::core::ffi::c_int
                                                        {
                                                            &raw mut config.models_path
                                                                as *mut ::core::ffi::c_char
                                                        } else {
                                                            b"/etc/lightnvr/models\0".as_ptr()
                                                                as *const ::core::ffi::c_char
                                                        };
                                                    snprintf(
                                                        &raw mut model_path
                                                            as *mut ::core::ffi::c_char,
                                                        ::core::mem::size_of::<
                                                            [::core::ffi::c_char; 512],
                                                        >(
                                                        ),
                                                        b"%s/%s\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        models_dir,
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .detection_model
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                    let mut model_file: *mut FILE = fopen(
                                                        &raw mut model_path
                                                            as *mut ::core::ffi::c_char,
                                                        b"r\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    if !model_file.is_null() {
                                                        fclose(model_file);
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_INFO,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Detection model found: %s\0".as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                            &raw mut model_path
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                    } else {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_ERROR,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Detection model not found: %s\0"
                                                                .as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                            &raw mut model_path
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_ERROR,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Detection will not work properly!\0"
                                                                .as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if mkdir_recursive(models_dir) != 0 {
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Failed to create models directory: %s\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                strerror(*__errno_location()),
                                                            );
                                                        } else {
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_INFO,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Created models directory: %s\0"
                                                                    .as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                models_dir,
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    safe_strcpy(
                                                        &raw mut model_path
                                                            as *mut ::core::ffi::c_char,
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .detection_model
                                                            as *mut ::core::ffi::c_char,
                                                        MAX_PATH_LENGTH as size_t,
                                                        0 as size_t,
                                                    );
                                                    let mut model_file_0: *mut FILE = fopen(
                                                        &raw mut model_path
                                                            as *mut ::core::ffi::c_char,
                                                        b"r\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    if !model_file_0.is_null() {
                                                        fclose(model_file_0);
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_INFO,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Detection model found: %s\0".as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                            &raw mut model_path
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                    } else {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_ERROR,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Detection model not found: %s\0"
                                                                .as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                            &raw mut model_path
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                    }
                                                }
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Starting detection-based recording for stream %s with model %s\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut (*config.streams.offset(i_0 as isize)).name
                                                        as *mut ::core::ffi::c_char,
                                                    &raw mut model_path as *mut ::core::ffi::c_char,
                                                );
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Starting detection stream reader for stream %s with model %s\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut (*config.streams.offset(i_0 as isize)).name
                                                        as *mut ::core::ffi::c_char,
                                                    &raw mut model_path as *mut ::core::ffi::c_char,
                                                );
                                                let mut detection_interval: ::core::ffi::c_int =
                                                    if (*config.streams.offset(i_0 as isize))
                                                        .detection_interval
                                                        > 0 as ::core::ffi::c_int
                                                    {
                                                        (*config.streams.offset(i_0 as isize))
                                                            .detection_interval
                                                    } else {
                                                        10 as ::core::ffi::c_int
                                                    };
                                                let mut result: ::core::ffi::c_int =
                                                    start_detection_stream_reader(
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .name
                                                            as *mut ::core::ffi::c_char,
                                                        detection_interval,
                                                    );
                                                if result == 0 as ::core::ffi::c_int {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Successfully started detection stream reader for stream %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut (*config.streams.offset(i_0 as isize)).name
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                    if is_detection_stream_reader_running(
                                                        &raw mut (*config
                                                            .streams
                                                            .offset(i_0 as isize))
                                                        .name
                                                            as *mut ::core::ffi::c_char,
                                                    ) != 0
                                                    {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_INFO,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Confirmed detection stream reader is running for %s\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            &raw mut (*config.streams.offset(i_0 as isize)).name
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                    } else {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_WARN,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Detection stream reader reported as not running for %s despite successful start\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            &raw mut (*config.streams.offset(i_0 as isize)).name
                                                                as *mut ::core::ffi::c_char,
                                                        );
                                                    }
                                                } else {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_ERROR,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Failed to start detection stream reader for stream %s: error code %d\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut (*config.streams.offset(i_0 as isize)).name
                                                            as *mut ::core::ffi::c_char,
                                                        result,
                                                    );
                                                }
                                                let mut annotation_only: bool =
                                                    (*config.streams.offset(i_0 as isize)).record;
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Directly starting unified detection thread for stream %s with model %s (annotation_only=%s)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut (*config.streams.offset(i_0 as isize)).name
                                                        as *mut ::core::ffi::c_char,
                                                    &raw mut model_path as *mut ::core::ffi::c_char,
                                                    if annotation_only as ::core::ffi::c_int != 0 {
                                                        b"true\0".as_ptr() as *const ::core::ffi::c_char
                                                    } else {
                                                        b"false\0".as_ptr() as *const ::core::ffi::c_char
                                                    },
                                                );
                                                if start_unified_detection_thread(
                                                    &raw mut (*config.streams.offset(i_0 as isize))
                                                        .name
                                                        as *mut ::core::ffi::c_char,
                                                    &raw mut model_path as *mut ::core::ffi::c_char,
                                                    (*config.streams.offset(i_0 as isize))
                                                        .detection_threshold,
                                                    (*config.streams.offset(i_0 as isize))
                                                        .pre_detection_buffer,
                                                    (*config.streams.offset(i_0 as isize))
                                                        .post_detection_buffer,
                                                    annotation_only,
                                                ) != 0 as ::core::ffi::c_int
                                                {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_WARN,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Failed to start unified detection thread for stream %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut (*config.streams.offset(i_0 as isize)).name
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                } else {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Successfully started unified detection thread for stream %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut (*config.streams.offset(i_0 as isize)).name
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                }
                                            }
                                            i_0 += 1;
                                        }
                                        if config.go2rtc_enabled {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_INFO,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Waiting 5 seconds for go2rtc to settle before starting recordings...\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            sleep(5 as ::core::ffi::c_uint);
                                        }
                                        check_and_ensure_services();
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"LightNVR initialized successfully\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        service_check_init_time =
                                            time(::core::ptr::null_mut::<time_t>());
                                        while running {
                                            static mut last_log_time: time_t = 0 as time_t;
                                            static mut last_status_time: time_t = 0 as time_t;
                                            static mut last_ffmpeg_leak_check_time: time_t =
                                                0 as time_t;
                                            static mut last_service_check_time: time_t =
                                                0 as time_t;
                                            static mut last_db_backup_check_time: time_t =
                                                0 as time_t;
                                            static mut last_audit_summary_flush_time: time_t =
                                                0 as time_t;
                                            let mut now: time_t =
                                                time(::core::ptr::null_mut::<time_t>());
                                            if last_service_check_time == 0 as time_t {
                                                last_service_check_time = service_check_init_time;
                                            }
                                            if now - last_log_time > 60 as time_t {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Daemon is still running... (running=%d, restart_requested=%d)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    running as ::core::ffi::c_int,
                                                    restart_requested as ::core::ffi::c_int,
                                                );
                                                last_log_time = now;
                                            }
                                            if now - last_status_time > 300 as time_t {
                                                print_detection_stream_status();
                                                last_status_time = now;
                                            }
                                            if now - last_ffmpeg_leak_check_time > 600 as time_t {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Checking for FFmpeg memory leaks...\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                let mut allocation_count: ::core::ffi::c_int =
                                                    ffmpeg_get_allocation_count();
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Current FFmpeg allocations: %d\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    allocation_count,
                                                );
                                                if allocation_count > 100 as ::core::ffi::c_int {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_WARN,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Potential FFmpeg memory leak detected: %d allocations\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        allocation_count,
                                                    );
                                                    ffmpeg_dump_allocations();
                                                }
                                                last_ffmpeg_leak_check_time = now;
                                            }
                                            if now - last_service_check_time > 30 as time_t {
                                                check_and_ensure_services();
                                                last_service_check_time = now;
                                            }
                                            if now - last_audit_summary_flush_time > 60 as time_t {
                                                audit_log_flush_summaries(r#true != 0);
                                                last_audit_summary_flush_time = now;
                                            }
                                            if now - last_db_backup_check_time > 60 as time_t {
                                                if maybe_run_scheduled_database_backup()
                                                    < 0 as ::core::ffi::c_int
                                                {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_WARN,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Scheduled database backup could not be started\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                                last_db_backup_check_time = now;
                                            }
                                            if restart_requested {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Main loop detected restart_requested=true, running=%d - exiting loop immediately\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    running as ::core::ffi::c_int,
                                                );
                                                break;
                                            } else {
                                                let mut i_1: ::core::ffi::c_int =
                                                    0 as ::core::ffi::c_int;
                                                while i_1 < 10 as ::core::ffi::c_int
                                                    && running as ::core::ffi::c_int != 0
                                                {
                                                    usleep(
                                                        100000 as ::core::ffi::c_int
                                                            as __useconds_t,
                                                    );
                                                    i_1 += 1;
                                                }
                                            }
                                        }
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Shutting down LightNVR... (running=%d, restart_requested=%d)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            running as ::core::ffi::c_int,
                                            restart_requested as ::core::ffi::c_int,
                                        );
                                        initiate_shutdown();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting cleanup process...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    request_background_abort();
    shutdown_recording_transcode();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stopping web server to prevent requests during shutdown...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !http_server.is_null() {
        http_server_stop(http_server);
        http_server_destroy(http_server);
        http_server = ::core::ptr::null_mut::<http_server_t>();
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stopping health check system...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cleanup_health_check_system();
    let mut block_mask: sigset_t = __sigset_t { __val: [0; 16] };
    let mut old_mask: sigset_t = __sigset_t { __val: [0; 16] };
    sigfillset(&raw mut block_mask);
    sigdelset(&raw mut block_mask, SIGUSR1);
    sigdelset(&raw mut block_mask, SIGALRM);
    sigdelset(&raw mut block_mask, SIGKILL);
    pthread_sigmask(SIG_BLOCK, &raw mut block_mask, &raw mut old_mask);
    let mut cleanup_pid: pid_t = fork();
    if cleanup_pid == 0 as ::core::ffi::c_int {
        if setpgid(0 as __pid_t, 0 as __pid_t) != 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"Watchdog: Failed to create new process group: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        let mut sa_default: sigaction = sigaction {
            __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &raw mut sa_default as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sigaction>(),
        );
        sa_default.__sigaction_handler.sa_handler = SIG_DFL;
        sigaction(
            SIGTERM,
            &raw mut sa_default,
            ::core::ptr::null_mut::<sigaction>(),
        );
        sigaction(
            SIGINT,
            &raw mut sa_default,
            ::core::ptr::null_mut::<sigaction>(),
        );
        sigaction(
            SIGALRM,
            &raw mut sa_default,
            ::core::ptr::null_mut::<sigaction>(),
        );
        sigaction(
            SIGUSR1,
            &raw mut sa_default,
            ::core::ptr::null_mut::<sigaction>(),
        );
        let mut empty_mask: sigset_t = __sigset_t { __val: [0; 16] };
        sigemptyset(&raw mut empty_mask);
        pthread_sigmask(
            SIG_SETMASK,
            &raw mut empty_mask,
            ::core::ptr::null_mut::<__sigset_t>(),
        );
        let mut parent_pid: pid_t = getppid();
        sleep(SHUTDOWN_WATCHDOG_TIMEOUT_SECONDS as ::core::ffi::c_uint);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleanup process phase 1 timed out after %d seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            900 as ::core::ffi::c_int,
        );
        kill(parent_pid, SIGUSR1);
        sleep(15 as ::core::ffi::c_uint);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleanup process phase 2 timed out after 15 more seconds, forcing exit\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        kill(parent_pid, SIGKILL);
        if restart_requested {
            if container_mode {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Restart requested in container mode after forced cleanup - exiting for orchestrator restart\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                exit(EXIT_SUCCESS);
            } else if !saved_argv.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Restart was requested, re-executing LightNVR after forced cleanup...\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                usleep(2000000 as ::core::ffi::c_int as __useconds_t);
                let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_2 < 10 as ::core::ffi::c_int {
                    if kill(parent_pid, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
                        break;
                    }
                    usleep(500000 as ::core::ffi::c_int as __useconds_t);
                    i_2 += 1;
                }
                let mut exe_path: [::core::ffi::c_char; 512] = [0; 512];
                let mut len: ssize_t = readlink(
                    b"/proc/self/exe\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut exe_path as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1 as size_t),
                );
                if len > 0 as ssize_t {
                    exe_path[len as usize] = '\0' as ::core::ffi::c_char;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Executing: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut exe_path as *mut ::core::ffi::c_char,
                    );
                    execv(&raw mut exe_path as *mut ::core::ffi::c_char, saved_argv);
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to restart after forced cleanup: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to get executable path for restart: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                }
            }
        }
        exit(EXIT_FAILURE);
    } else if cleanup_pid > 0 as ::core::ffi::c_int {
        let mut sa_usr1: sigaction = sigaction {
            __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &raw mut sa_usr1 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sigaction>(),
        );
        sa_usr1.__sigaction_handler.sa_handler =
            Some(alarm_handler as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
        sigaction(
            SIGUSR1,
            &raw mut sa_usr1,
            ::core::ptr::null_mut::<sigaction>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down host health and telemetry...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        system_health_shutdown();
        system_health_evaluator_service_shutdown(r#true != 0);
        metrics_shutdown();
        player_telemetry_shutdown();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up go2rtc integration...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        go2rtc_integration_cleanup();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Starting shutdown sequence for all components...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        usleep(250000 as ::core::ffi::c_int as __useconds_t);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopping all detection stream readers...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < g_config.max_streams {
            if (*config.streams.offset(i_3 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
                && (*config.streams.offset(i_3 as isize)).detection_based_recording
                    as ::core::ffi::c_int
                    != 0
                && (*config.streams.offset(i_3 as isize)).detection_model[0usize]
                    as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stopping detection stream reader for: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*config.streams.offset(i_3 as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                stop_detection_stream_reader(
                    &raw mut (*config.streams.offset(i_3 as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                let mut component_name: [::core::ffi::c_char; 128] = [0; 128];
                snprintf(
                    &raw mut component_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                    b"detection_thread_%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*config.streams.offset(i_3 as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                let mut component_id: ::core::ffi::c_int =
                    find_component_id_by_name(&raw mut component_name as *mut ::core::ffi::c_char);
                if component_id >= 0 as ::core::ffi::c_int {
                    update_component_state(component_id, component_state_t::COMPONENT_STOPPED);
                }
            }
            i_3 += 1;
        }
        usleep(500000 as ::core::ffi::c_int as __useconds_t);
        let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_4 < g_config.max_streams {
            if (*config.streams.offset(i_4 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                let mut stream: stream_handle_t = get_stream_by_name(
                    &raw mut (*config.streams.offset(i_4 as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                if !stream.is_null() {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Stopping stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*config.streams.offset(i_4 as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    stop_stream(stream);
                }
            }
            i_4 += 1;
        }
        usleep(1500000 as ::core::ffi::c_int as __useconds_t);
        close_all_mp4_writers();
        let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_5 < g_config.max_streams {
            if (*config.streams.offset(i_5 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
                && (*config.streams.offset(i_5 as isize)).record as ::core::ffi::c_int != 0
            {
                let mut component_name_0: [::core::ffi::c_char; 128] = [0; 128];
                snprintf(
                    &raw mut component_name_0 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                    b"mp4_writer_%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*config.streams.offset(i_5 as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                let mut component_id_0: ::core::ffi::c_int = find_component_id_by_name(
                    &raw mut component_name_0 as *mut ::core::ffi::c_char,
                );
                if component_id_0 >= 0 as ::core::ffi::c_int {
                    update_component_state(component_id_0, component_state_t::COMPONENT_STOPPED);
                }
            }
            i_5 += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up HLS directories...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cleanup_hls_directories();
        let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_6 < g_config.max_streams {
            if (*config.streams.offset(i_6 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                let mut component_name_1: [::core::ffi::c_char; 128] = [0; 128];
                snprintf(
                    &raw mut component_name_1 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                    b"hls_writer_%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*config.streams.offset(i_6 as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                let mut component_id_1: ::core::ffi::c_int = find_component_id_by_name(
                    &raw mut component_name_1 as *mut ::core::ffi::c_char,
                );
                if component_id_1 >= 0 as ::core::ffi::c_int {
                    update_component_state(component_id_1, component_state_t::COMPONENT_STOPPED);
                }
            }
            i_6 += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up detection stream system...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_detection_stream_system();
        usleep(200000 as ::core::ffi::c_int as __useconds_t);
        cleanup_all_hls_writers();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up HLS streaming backend...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cleanup_hls_streaming_backend();
        usleep(200000 as ::core::ffi::c_int as __useconds_t);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up MP4 recording backend...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cleanup_mp4_recording_backend();
        usleep(200000 as ::core::ffi::c_int as __useconds_t);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up transcoding backend...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cleanup_transcoding_backend();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up detection resources...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cleanup_detection_resources();
        observation_relay_stop();
        observation_evidence_shutdown(OBS_EVIDENCE_DEFAULT_DRAIN_MS);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Draining asynchronous event pipeline...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        event_bus_shutdown(r#true != 0);
        mqtt_event_adapter_unregister();
        event_identity_shutdown();
        mqtt_delivery_worker_shutdown();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up MQTT client...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mqtt_cleanup();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down ONVIF discovery module...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_onvif_discovery();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down stream manager...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_stream_manager();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down stream state adapter...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_stream_state_adapter();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down stream state manager...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_stream_state_manager();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down storage manager...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_storage_manager();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down recording sync thread...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        stop_recording_sync_thread();
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        audit_log_shutdown_summaries();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Ensuring all database operations are complete...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Freeing schema cache...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free_schema_cache();
        usleep(100000 as ::core::ffi::c_int as __useconds_t);
        if db_scheduled_backup_in_flight() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting up to %d s for the in-flight scheduled database backup to abort...\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                60 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int / 1000 as ::core::ffi::c_int,
            );
            if !db_scheduled_backup_wait_idle(DB_SCHEDULED_BACKUP_JOIN_TIMEOUT_MS) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Scheduled database backup worker did not stop within %d s, continuing shutdown\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    60 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int
                        / 1000 as ::core::ffi::c_int,
                );
            }
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up batch delete progress...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        batch_delete_progress_cleanup();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down database...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_database();
        usleep(100000 as ::core::ffi::c_int as __useconds_t);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Performing final SQLite memory cleanup...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_release_memory(INT_MAX);
        sqlite3_shutdown();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for all components to stop...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !wait_for_all_components_stopped(5 as ::core::ffi::c_int) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Not all components stopped within timeout, continuing anyway\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up shutdown coordinator...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_coordinator_cleanup();
        usleep(100000 as ::core::ffi::c_int as __useconds_t);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Performing additional cleanup before go2rtc...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up go2rtc stream...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        go2rtc_stream_cleanup();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up libcurl globally...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        lightnvr_sentry_shutdown();
        curl_cleanup_global();
        kill(cleanup_pid, SIGKILL);
        waitpid(
            cleanup_pid,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        pthread_sigmask(
            SIG_SETMASK,
            &raw mut old_mask,
            ::core::ptr::null_mut::<__sigset_t>(),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create watchdog process for cleanup timeout\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down host health and telemetry...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        system_health_shutdown();
        system_health_evaluator_service_shutdown(r#true != 0);
        metrics_shutdown();
        player_telemetry_shutdown();
        let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_7 < g_config.max_streams {
            if (*config.streams.offset(i_7 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                let mut stream_0: stream_handle_t = get_stream_by_name(
                    &raw mut (*config.streams.offset(i_7 as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                if !stream_0.is_null() {
                    stop_stream(stream_0);
                }
            }
            i_7 += 1;
        }
        usleep(200000 as ::core::ffi::c_int as __useconds_t);
        close_all_mp4_writers();
        shutdown_detection_stream_system();
        cleanup_mp4_recording_backend();
        cleanup_hls_streaming_backend();
        cleanup_transcoding_backend();
        observation_relay_stop();
        observation_evidence_shutdown(OBS_EVIDENCE_DEFAULT_DRAIN_MS);
        event_bus_shutdown(r#true != 0);
        mqtt_event_adapter_unregister();
        event_identity_shutdown();
        mqtt_delivery_worker_shutdown();
        mqtt_cleanup();
        batch_delete_progress_cleanup();
        shutdown_stream_manager();
        shutdown_stream_state_adapter();
        shutdown_stream_state_manager();
        shutdown_storage_manager();
        audit_log_shutdown_summaries();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Ensuring all database operations are complete...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Freeing schema cache...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free_schema_cache();
        usleep(100000 as ::core::ffi::c_int as __useconds_t);
        if db_scheduled_backup_in_flight() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting up to %d s for the in-flight scheduled database backup to abort...\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                60 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int / 1000 as ::core::ffi::c_int,
            );
            if !db_scheduled_backup_wait_idle(DB_SCHEDULED_BACKUP_JOIN_TIMEOUT_MS) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Scheduled database backup worker did not stop within %d s, continuing shutdown\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    60 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int
                        / 1000 as ::core::ffi::c_int,
                );
            }
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutting down database...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_database();
        usleep(100000 as ::core::ffi::c_int as __useconds_t);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Performing final SQLite memory cleanup...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_release_memory(INT_MAX);
        sqlite3_shutdown();
        shutdown_coordinator_cleanup();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up libcurl globally...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        lightnvr_sentry_shutdown();
        curl_cleanup_global();
        pthread_sigmask(
            SIG_SETMASK,
            &raw mut old_mask,
            ::core::ptr::null_mut::<__sigset_t>(),
        );
    }
    remove_pid_file(pid_fd, &raw mut config.pid_file as *mut ::core::ffi::c_char);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleanup complete, shutting down\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if restart_requested {
        if container_mode {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Restart requested in container mode - exiting cleanly for orchestrator restart\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            shutdown_logger();
            return EXIT_SUCCESS;
        } else if !saved_argv.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Restart requested in native mode, re-executing LightNVR...\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            shutdown_logger();
            let mut exe_path_0: [::core::ffi::c_char; 512] = [0; 512];
            let mut len_0: ssize_t = readlink(
                b"/proc/self/exe\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut exe_path_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1 as size_t),
            );
            if len_0 > 0 as ssize_t {
                exe_path_0[len_0 as usize] = '\0' as ::core::ffi::c_char;
                usleep(500000 as ::core::ffi::c_int as __useconds_t);
                execv(&raw mut exe_path_0 as *mut ::core::ffi::c_char, saved_argv);
                fprintf(
                    stderr,
                    b"Failed to restart: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            } else {
                fprintf(
                    stderr,
                    b"Failed to get executable path for restart: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
            return EXIT_FAILURE;
        }
    }
    shutdown_logger();
    return EXIT_SUCCESS;
}
unsafe extern "C" fn check_and_ensure_services() {
    if is_shutdown_initiated() as ::core::ffi::c_int != 0
        || is_background_abort_requested() as ::core::ffi::c_int != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Skipping service check during shutdown\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_config: *const config_t = get_streaming_config();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Running periodic service check (%d max streams)\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.max_streams,
    );
    let mut recordings_started: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if is_background_abort_requested() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Aborting periodic service check early: restart/shutdown requested\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if (*(*current_config).streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Service check for stream %s: enabled=%d, record=%d, streaming_enabled=%d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(*current_config).streams.offset(i as isize)).name
                    as *mut ::core::ffi::c_char,
                (*(*current_config).streams.offset(i as isize)).enabled as ::core::ffi::c_int,
                (*(*current_config).streams.offset(i as isize)).record as ::core::ffi::c_int,
                (*(*current_config).streams.offset(i as isize)).streaming_enabled
                    as ::core::ffi::c_int,
            );
        }
        if (*(*current_config).streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && (*(*current_config).streams.offset(i as isize)).enabled as ::core::ffi::c_int != 0
            && (*(*current_config).streams.offset(i as isize)).record as ::core::ffi::c_int != 0
        {
            let mut skip_scheduled_recording: bool = (*(*current_config).streams.offset(i as isize))
                .record_on_schedule
                as ::core::ffi::c_int
                != 0
                && !is_recording_scheduled((*current_config).streams.offset(i as isize));
            if skip_scheduled_recording {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Service check: stream '%s' has record_on_schedule=true but current time is outside scheduled window \xE2\x80\x94 skipping recording\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*(*current_config).streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                );
            } else {
                let mut recording_state: ::core::ffi::c_int = get_recording_state(
                    &raw mut (*(*current_config).streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Recording state for stream %s: %d (1=active, 0=inactive)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*(*current_config).streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                    recording_state,
                );
                if recording_state == 0 as ::core::ffi::c_int {
                    if recordings_started > 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Staggering recording start for stream '%s' \xE2\x80\x94 waiting 3 seconds after previous recording start (%d started so far)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut (*(*current_config).streams.offset(i as isize)).name
                                as *mut ::core::ffi::c_char,
                            recordings_started,
                        );
                        sleep(3 as ::core::ffi::c_uint);
                    }
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Ensuring MP4 recording is active for stream: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*(*current_config).streams.offset(i as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    let mut rec_result: ::core::ffi::c_int = go2rtc_integration_start_recording(
                        &raw mut (*(*current_config).streams.offset(i as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    if rec_result != 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to start MP4 recording for stream: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*(*current_config).streams.offset(i as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Successfully started MP4 recording for stream: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*(*current_config).streams.offset(i as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                        recordings_started += 1;
                    }
                }
            }
        }
        if (*(*current_config).streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && (*(*current_config).streams.offset(i as isize)).enabled as ::core::ffi::c_int != 0
            && (*(*current_config).streams.offset(i as isize)).streaming_enabled
                as ::core::ffi::c_int
                != 0
        {
            if stream_start_hls(
                &raw mut (*(*current_config).streams.offset(i as isize)).name
                    as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to start HLS streaming for stream: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*(*current_config).streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        if (*(*current_config).streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && (*(*current_config).streams.offset(i as isize)).enabled as ::core::ffi::c_int != 0
            && (*(*current_config).streams.offset(i as isize)).detection_based_recording
                as ::core::ffi::c_int
                != 0
            && is_detection_recording_scheduled((*current_config).streams.offset(i as isize))
                as ::core::ffi::c_int
                != 0
        {
            let mut annotation_only: bool = (*(*current_config).streams.offset(i as isize)).record;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Ensuring detection-based recording is active for stream: %s (annotation_only=%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(*current_config).streams.offset(i as isize)).name
                    as *mut ::core::ffi::c_char,
                if annotation_only as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if start_unified_detection_thread(
                &raw mut (*(*current_config).streams.offset(i as isize)).name
                    as *mut ::core::ffi::c_char,
                &raw mut (*(*current_config).streams.offset(i as isize)).detection_model
                    as *mut ::core::ffi::c_char,
                (*(*current_config).streams.offset(i as isize)).detection_threshold,
                (*(*current_config).streams.offset(i as isize)).pre_detection_buffer,
                (*(*current_config).streams.offset(i as isize)).post_detection_buffer,
                annotation_only,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to start detection-based recording for stream: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*(*current_config).streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Successfully started detection-based recording for stream: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*(*current_config).streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
}
pub const DB_SCHEDULED_BACKUP_JOIN_TIMEOUT_MS: ::core::ffi::c_int =
    60 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIGUSR1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGKILL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGALRM: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const LIGHTNVR_VERSION_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"0.48.2\0") };
pub const LIGHTNVR_BUILD_DATE: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*crate::build_info::LIGHTNVR_BUILD_DATE)
};
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
