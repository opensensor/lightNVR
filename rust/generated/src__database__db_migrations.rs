unsafe extern "C" {
    pub type sqlite_migrate;
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
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
    unsafe fn sqlite3_free(_: *mut ::core::ffi::c_void);
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
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
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn migrate_init(
        db: *mut sqlite3,
        config: *const migrate_config_t,
    ) -> *mut sqlite_migrate_t;
    unsafe fn migrate_free(ctx: *mut sqlite_migrate_t);
    unsafe fn migrate_up(
        ctx: *mut sqlite_migrate_t,
        stats: *mut migrate_stats_t,
    ) -> ::core::ffi::c_int;
    unsafe fn migrate_status(
        ctx: *mut sqlite_migrate_t,
        migrations: *mut migration_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn migrate_get_version(
        ctx: *const sqlite_migrate_t,
        version: *mut ::core::ffi::c_char,
        version_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type ssize_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct migrate_stats_t {
    pub total: ::core::ffi::c_int,
    pub applied: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub failed: ::core::ffi::c_int,
}
pub type sqlite_migrate_t = sqlite_migrate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct migrate_config_t {
    pub migrations_dir: *const ::core::ffi::c_char,
    pub migrations_table: *const ::core::ffi::c_char,
    pub embedded_migrations: *const migration_t,
    pub embedded_count: size_t,
    pub callback: migrate_callback_t,
    pub callback_data: *mut ::core::ffi::c_void,
    pub dry_run: bool,
    pub verbose: bool,
}
pub type migrate_callback_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        migrate_status_t,
        *mut ::core::ffi::c_void,
    ) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct migrate_status_t(pub ::core::ffi::c_uint);
impl migrate_status_t {
    pub const MIGRATE_STATUS_PENDING: Self = Self(0);
    pub const MIGRATE_STATUS_APPLIED: Self = Self(1);
    pub const MIGRATE_STATUS_FAILED: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct migration_t {
    pub version: [::core::ffi::c_char; 32],
    pub description: [::core::ffi::c_char; 256],
    pub filepath: [::core::ffi::c_char; 512],
    pub sql_up: *const ::core::ffi::c_char,
    pub sql_down: *const ::core::ffi::c_char,
    pub status: migrate_status_t,
    pub is_embedded: bool,
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
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
static mut migration_0001_up: [::core::ffi::c_char; 1310] = unsafe {
    ::core::mem::transmute::<
        [u8; 1310],
        [::core::ffi::c_char; 1310],
    >(
        *b"-- Streams configuration table\nCREATE TABLE IF NOT EXISTS streams (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    name TEXT NOT NULL UNIQUE,\n    url TEXT NOT NULL,\n    enabled INTEGER DEFAULT 1,\n    streaming_enabled INTEGER DEFAULT 1,\n    width INTEGER DEFAULT 1280,\n    height INTEGER DEFAULT 720,\n    fps INTEGER DEFAULT 30,\n    codec TEXT DEFAULT 'h264',\n    priority INTEGER DEFAULT 5,\n    record INTEGER DEFAULT 1,\n    segment_duration INTEGER DEFAULT 900\n);\n\n-- Recordings table\nCREATE TABLE IF NOT EXISTS recordings (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    stream_name TEXT NOT NULL,\n    file_path TEXT NOT NULL,\n    start_time INTEGER NOT NULL,\n    end_time INTEGER,\n    size_bytes INTEGER DEFAULT 0,\n    width INTEGER,\n    height INTEGER,\n    fps INTEGER,\n    codec TEXT,\n    is_complete INTEGER DEFAULT 0,\n    created_at INTEGER DEFAULT (strftime('%s', 'now'))\n);\n\n-- Events table\nCREATE TABLE IF NOT EXISTS events (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    type INTEGER NOT NULL,\n    timestamp INTEGER NOT NULL,\n    stream_name TEXT,\n    description TEXT NOT NULL,\n    details TEXT\n);\n\n-- Create index for efficient queries\nCREATE INDEX IF NOT EXISTS idx_recordings_stream_name ON recordings(stream_name);\nCREATE INDEX IF NOT EXISTS idx_recordings_start_time ON recordings(start_time);\0",
    )
};
static mut migration_0001_down: [::core::ffi::c_char; 189] = unsafe {
    ::core::mem::transmute::<
        [u8; 189],
        [::core::ffi::c_char; 189],
    >(
        *b"DROP TABLE IF EXISTS events;\nDROP INDEX IF EXISTS idx_recordings_start_time;\nDROP INDEX IF EXISTS idx_recordings_stream_name;\nDROP TABLE IF EXISTS recordings;\nDROP TABLE IF EXISTS streams;\0",
    )
};
static mut migration_0002_up: [::core::ffi::c_char; 422] = unsafe {
    ::core::mem::transmute::<
        [u8; 422],
        [::core::ffi::c_char; 422],
    >(
        *b"ALTER TABLE streams ADD COLUMN detection_based_recording INTEGER DEFAULT 0;\nALTER TABLE streams ADD COLUMN detection_model TEXT DEFAULT '';\nALTER TABLE streams ADD COLUMN detection_threshold REAL DEFAULT 0.5;\nALTER TABLE streams ADD COLUMN detection_interval INTEGER DEFAULT 10;\nALTER TABLE streams ADD COLUMN pre_detection_buffer INTEGER DEFAULT 0;\nALTER TABLE streams ADD COLUMN post_detection_buffer INTEGER DEFAULT 3;\0",
    )
};
static mut migration_0002_down: [::core::ffi::c_char; 172] = unsafe {
    ::core::mem::transmute::<
        [u8; 172],
        [::core::ffi::c_char; 172],
    >(
        *b"-- SQLite doesn't support DROP COLUMN before 3.35.0\n-- For older versions, we'd need to recreate the table\n-- This is a no-op for safety on older SQLite versions\nSELECT 1;\0",
    )
};
static mut migration_0003_up: [::core::ffi::c_char; 118] = unsafe {
    ::core::mem::transmute::<
        [u8; 118],
        [::core::ffi::c_char; 118],
    >(
        *b"ALTER TABLE streams ADD COLUMN protocol INTEGER DEFAULT 0;\nALTER TABLE streams ADD COLUMN is_onvif INTEGER DEFAULT 0;\0",
    )
};
static mut migration_0003_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0004_up: [::core::ffi::c_char; 63] = unsafe {
    ::core::mem::transmute::<[u8; 63], [::core::ffi::c_char; 63]>(
        *b"ALTER TABLE streams ADD COLUMN record_audio INTEGER DEFAULT 1;\0",
    )
};
static mut migration_0004_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0005_up: [::core::ffi::c_char; 625] = unsafe {
    ::core::mem::transmute::<
        [u8; 625],
        [::core::ffi::c_char; 625],
    >(
        *b"CREATE TABLE IF NOT EXISTS detections (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    stream_name TEXT NOT NULL,\n    timestamp INTEGER NOT NULL,\n    label TEXT NOT NULL,\n    confidence REAL NOT NULL,\n    x REAL,\n    y REAL,\n    width REAL,\n    height REAL,\n    recording_id INTEGER,\n    created_at INTEGER DEFAULT (strftime('%s', 'now')),\n    FOREIGN KEY (recording_id) REFERENCES recordings(id)\n);\n\nCREATE INDEX IF NOT EXISTS idx_detections_stream ON detections(stream_name);\nCREATE INDEX IF NOT EXISTS idx_detections_timestamp ON detections(timestamp);\nCREATE INDEX IF NOT EXISTS idx_detections_label ON detections(label);\0",
    )
};
static mut migration_0005_down: [::core::ffi::c_char; 167] = unsafe {
    ::core::mem::transmute::<
        [u8; 167],
        [::core::ffi::c_char; 167],
    >(
        *b"DROP INDEX IF EXISTS idx_detections_label;\nDROP INDEX IF EXISTS idx_detections_timestamp;\nDROP INDEX IF EXISTS idx_detections_stream;\nDROP TABLE IF EXISTS detections;\0",
    )
};
static mut migration_0006_up: [::core::ffi::c_char; 1097] = unsafe {
    ::core::mem::transmute::<
        [u8; 1097],
        [::core::ffi::c_char; 1097],
    >(
        *b"CREATE TABLE IF NOT EXISTS users (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    username TEXT NOT NULL UNIQUE,\n    password_hash TEXT NOT NULL,\n    salt TEXT NOT NULL,\n    role TEXT NOT NULL,\n    email TEXT,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    last_login INTEGER,\n    is_active INTEGER DEFAULT 1,\n    api_key TEXT\n);\n\n-- Create indexes for faster lookups\nCREATE UNIQUE INDEX IF NOT EXISTS idx_users_username ON users (username);\nCREATE INDEX IF NOT EXISTS idx_users_api_key ON users (api_key);\n\n-- Create sessions table for token-based authentication\nCREATE TABLE IF NOT EXISTS sessions (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    user_id INTEGER NOT NULL,\n    token TEXT NOT NULL UNIQUE,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    expires_at INTEGER NOT NULL,\n    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE\n);\n\nCREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions (token);\nCREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions (user_id);\0",
    )
};
static mut migration_0006_down: [::core::ffi::c_char; 59] = unsafe {
    ::core::mem::transmute::<[u8; 59], [::core::ffi::c_char; 59]>(
        *b"DROP TABLE IF EXISTS sessions;\nDROP TABLE IF EXISTS users;\0",
    )
};
static mut migration_0007_up: [::core::ffi::c_char; 418] = unsafe {
    ::core::mem::transmute::<
        [u8; 418],
        [::core::ffi::c_char; 418],
    >(
        *b"CREATE TABLE IF NOT EXISTS motion_settings (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    stream_name TEXT NOT NULL UNIQUE,\n    enabled INTEGER DEFAULT 0,\n    sensitivity REAL DEFAULT 0.5,\n    threshold REAL DEFAULT 0.01,\n    cooldown_seconds INTEGER DEFAULT 5,\n    min_motion_frames INTEGER DEFAULT 3,\n    created_at INTEGER DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER DEFAULT (strftime('%s', 'now'))\n);\0",
    )
};
static mut migration_0007_down: [::core::ffi::c_char; 38] = unsafe {
    ::core::mem::transmute::<[u8; 38], [::core::ffi::c_char; 38]>(
        *b"DROP TABLE IF EXISTS motion_settings;\0",
    )
};
static mut migration_0008_up: [::core::ffi::c_char; 574] = unsafe {
    ::core::mem::transmute::<
        [u8; 574],
        [::core::ffi::c_char; 574],
    >(
        *b"CREATE TABLE IF NOT EXISTS detection_zones (\n    id TEXT PRIMARY KEY,\n    stream_name TEXT NOT NULL,\n    name TEXT NOT NULL,\n    enabled INTEGER DEFAULT 1,\n    color TEXT DEFAULT '#3b82f6',\n    polygon TEXT NOT NULL,\n    filter_classes TEXT DEFAULT '',\n    min_confidence REAL DEFAULT 0.0,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE\n);\n\nCREATE INDEX IF NOT EXISTS idx_zones_stream ON detection_zones(stream_name);\0",
    )
};
static mut migration_0008_down: [::core::ffi::c_char; 77] = unsafe {
    ::core::mem::transmute::<[u8; 77], [::core::ffi::c_char; 77]>(
        *b"DROP INDEX IF EXISTS idx_zones_stream;\nDROP TABLE IF EXISTS detection_zones;\0",
    )
};
static mut migration_0009_up: [::core::ffi::c_char; 122] = unsafe {
    ::core::mem::transmute::<
        [u8; 122],
        [::core::ffi::c_char; 122],
    >(
        *b"ALTER TABLE detections ADD COLUMN track_id INTEGER DEFAULT -1;\nALTER TABLE detections ADD COLUMN zone_id TEXT DEFAULT '';\0",
    )
};
static mut migration_0009_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0010_up: [::core::ffi::c_char; 73] = unsafe {
    ::core::mem::transmute::<[u8; 73], [::core::ffi::c_char; 73]>(
        *b"ALTER TABLE recordings ADD COLUMN trigger_type TEXT DEFAULT 'scheduled';\0",
    )
};
static mut migration_0010_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0011_up: [::core::ffi::c_char; 66] = unsafe {
    ::core::mem::transmute::<[u8; 66], [::core::ffi::c_char; 66]>(
        *b"ALTER TABLE streams ADD COLUMN detection_api_url TEXT DEFAULT '';\0",
    )
};
static mut migration_0011_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0012_up: [::core::ffi::c_char; 70] = unsafe {
    ::core::mem::transmute::<[u8; 70], [::core::ffi::c_char; 70]>(
        *b"ALTER TABLE streams ADD COLUMN backchannel_enabled INTEGER DEFAULT 0;\0",
    )
};
static mut migration_0012_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0013_up: [::core::ffi::c_char; 614] = unsafe {
    ::core::mem::transmute::<
        [u8; 614],
        [::core::ffi::c_char; 614],
    >(
        *b"-- Per-stream retention settings\nALTER TABLE streams ADD COLUMN retention_days INTEGER DEFAULT 30;\nALTER TABLE streams ADD COLUMN detection_retention_days INTEGER DEFAULT 90;\nALTER TABLE streams ADD COLUMN max_storage_mb INTEGER DEFAULT 0;\n\n-- Recording protection\nALTER TABLE recordings ADD COLUMN protected INTEGER DEFAULT 0;\nALTER TABLE recordings ADD COLUMN retention_override_days INTEGER DEFAULT NULL;\n\n-- Indexes for efficient retention queries\nCREATE INDEX IF NOT EXISTS idx_recordings_protected ON recordings(protected);\nCREATE INDEX IF NOT EXISTS idx_recordings_trigger_type ON recordings(trigger_type);\0",
    )
};
static mut migration_0013_down: [::core::ffi::c_char; 107] = unsafe {
    ::core::mem::transmute::<
        [u8; 107],
        [::core::ffi::c_char; 107],
    >(
        *b"DROP INDEX IF EXISTS idx_recordings_trigger_type;\nDROP INDEX IF EXISTS idx_recordings_protected;\nSELECT 1;\0",
    )
};
static mut migration_0014_up: [::core::ffi::c_char; 305] = unsafe {
    ::core::mem::transmute::<
        [u8; 305],
        [::core::ffi::c_char; 305],
    >(
        *b"ALTER TABLE streams ADD COLUMN ptz_enabled INTEGER DEFAULT 0;\nALTER TABLE streams ADD COLUMN ptz_max_x INTEGER DEFAULT 0;\nALTER TABLE streams ADD COLUMN ptz_max_y INTEGER DEFAULT 0;\nALTER TABLE streams ADD COLUMN ptz_max_z INTEGER DEFAULT 0;\nALTER TABLE streams ADD COLUMN ptz_has_home INTEGER DEFAULT 0;\0",
    )
};
static mut migration_0014_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0015_up: [::core::ffi::c_char; 160] = unsafe {
    ::core::mem::transmute::<
        [u8; 160],
        [::core::ffi::c_char; 160],
    >(
        *b"-- Buffer strategy: 'auto', 'none', 'go2rtc', 'hls_segment', 'memory_packet', 'mmap_hybrid'\nALTER TABLE streams ADD COLUMN buffer_strategy TEXT DEFAULT 'auto';\0",
    )
};
static mut migration_0015_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0016_up: [::core::ffi::c_char; 188] = unsafe {
    ::core::mem::transmute::<
        [u8; 188],
        [::core::ffi::c_char; 188],
    >(
        *b"ALTER TABLE streams ADD COLUMN onvif_username TEXT DEFAULT '';\nALTER TABLE streams ADD COLUMN onvif_password TEXT DEFAULT '';\nALTER TABLE streams ADD COLUMN onvif_profile TEXT DEFAULT '';\0",
    )
};
static mut migration_0016_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0017_up: [::core::ffi::c_char; 82] = unsafe {
    ::core::mem::transmute::<[u8; 82], [::core::ffi::c_char; 82]>(
        *b"ALTER TABLE detections ADD COLUMN recording_id INTEGER REFERENCES recordings(id);\0",
    )
};
static mut migration_0017_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0018_up: [::core::ffi::c_char; 203] = unsafe {
    ::core::mem::transmute::<
        [u8; 203],
        [::core::ffi::c_char; 203],
    >(
        *b"-- Add ip_address column for tracking client IP\nALTER TABLE sessions ADD COLUMN ip_address TEXT;\n\n-- Add user_agent column for tracking client user agent\nALTER TABLE sessions ADD COLUMN user_agent TEXT;\0",
    )
};
static mut migration_0018_down: [::core::ffi::c_char; 203] = unsafe {
    ::core::mem::transmute::<
        [u8; 203],
        [::core::ffi::c_char; 203],
    >(
        *b"-- SQLite doesn't support DROP COLUMN directly, so we need to recreate the table\n-- For simplicity in the down migration, we'll just leave the columns\n-- (they're nullable, so this won't break anything)\0",
    )
};
static mut migration_0019_up: [::core::ffi::c_char; 211] = unsafe {
    ::core::mem::transmute::<
        [u8; 211],
        [::core::ffi::c_char; 211],
    >(
        *b"-- Add password_change_locked column to users table\n-- This allows admins to lock password changes for specific users (e.g., demo accounts)\nALTER TABLE users ADD COLUMN password_change_locked INTEGER DEFAULT 0;\0",
    )
};
static mut migration_0019_down: [::core::ffi::c_char; 214] = unsafe {
    ::core::mem::transmute::<
        [u8; 214],
        [::core::ffi::c_char; 214],
    >(
        *b"-- SQLite doesn't support DROP COLUMN directly, so we need to recreate the table\n-- For simplicity in the down migration, we'll just leave the column\n-- (it's nullable with a default, so this won't break anything)\0",
    )
};
static mut migration_0020_up: [::core::ffi::c_char; 1587] = unsafe {
    ::core::mem::transmute::<
        [u8; 1587],
        [::core::ffi::c_char; 1587],
    >(
        *b"-- Motion recording configuration table (per-stream settings)\nCREATE TABLE IF NOT EXISTS motion_recording_config (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    stream_name TEXT NOT NULL UNIQUE,\n    enabled INTEGER DEFAULT 1,\n    pre_buffer_seconds INTEGER DEFAULT 5,\n    post_buffer_seconds INTEGER DEFAULT 5,\n    max_file_duration INTEGER DEFAULT 300,\n    codec TEXT DEFAULT 'h264',\n    quality TEXT DEFAULT 'medium',\n    retention_days INTEGER DEFAULT 7,\n    max_storage_mb INTEGER DEFAULT 0,\n    created_at INTEGER NOT NULL,\n    updated_at INTEGER NOT NULL,\n    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE\n);\n\n-- Index for faster lookups\nCREATE INDEX IF NOT EXISTS idx_motion_config_stream ON motion_recording_config(stream_name);\n\n-- Motion recordings table (separate from regular recordings)\nCREATE TABLE IF NOT EXISTS motion_recordings (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    stream_name TEXT NOT NULL,\n    file_path TEXT NOT NULL UNIQUE,\n    start_time INTEGER NOT NULL,\n    end_time INTEGER,\n    size_bytes INTEGER DEFAULT 0,\n    width INTEGER,\n    height INTEGER,\n    fps INTEGER,\n    codec TEXT,\n    is_complete INTEGER DEFAULT 0,\n    created_at INTEGER NOT NULL,\n    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE\n);\n\n-- Indexes for faster queries\nCREATE INDEX IF NOT EXISTS idx_motion_recordings_stream ON motion_recordings(stream_name);\nCREATE INDEX IF NOT EXISTS idx_motion_recordings_time ON motion_recordings(start_time);\nCREATE INDEX IF NOT EXISTS idx_motion_recordings_complete ON motion_recordings(is_complete);\0",
    )
};
static mut migration_0020_down: [::core::ffi::c_char; 286] = unsafe {
    ::core::mem::transmute::<
        [u8; 286],
        [::core::ffi::c_char; 286],
    >(
        *b"DROP INDEX IF EXISTS idx_motion_recordings_complete;\nDROP INDEX IF EXISTS idx_motion_recordings_time;\nDROP INDEX IF EXISTS idx_motion_recordings_stream;\nDROP TABLE IF EXISTS motion_recordings;\nDROP INDEX IF EXISTS idx_motion_config_stream;\nDROP TABLE IF EXISTS motion_recording_config;\0",
    )
};
static mut migration_0021_up: [::core::ffi::c_char; 108] = unsafe {
    ::core::mem::transmute::<
        [u8; 108],
        [::core::ffi::c_char; 108],
    >(
        *b"ALTER TABLE users ADD COLUMN totp_secret TEXT;\nALTER TABLE users ADD COLUMN totp_enabled INTEGER DEFAULT 0;\0",
    )
};
static mut migration_0021_down: [::core::ffi::c_char; 123] = unsafe {
    ::core::mem::transmute::<
        [u8; 123],
        [::core::ffi::c_char; 123],
    >(
        *b"-- SQLite doesn't support DROP COLUMN in older versions\n-- The columns are nullable with safe defaults, so they can remain\0",
    )
};
static mut migration_0022_up: [::core::ffi::c_char; 613] = unsafe {
    ::core::mem::transmute::<
        [u8; 613],
        [::core::ffi::c_char; 613],
    >(
        *b"DROP TABLE IF EXISTS zones;\n\nCREATE TABLE IF NOT EXISTS detection_zones (\n    id TEXT PRIMARY KEY,\n    stream_name TEXT NOT NULL,\n    name TEXT NOT NULL,\n    enabled INTEGER DEFAULT 1,\n    color TEXT DEFAULT '#3b82f6',\n    polygon TEXT NOT NULL,\n    filter_classes TEXT DEFAULT '',\n    min_confidence REAL DEFAULT 0.0,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE\n);\n\nCREATE INDEX IF NOT EXISTS idx_detection_zones_stream ON detection_zones(stream_name);\0",
    )
};
static mut migration_0022_down: [::core::ffi::c_char; 87] = unsafe {
    ::core::mem::transmute::<
        [u8; 87],
        [::core::ffi::c_char; 87],
    >(
        *b"DROP INDEX IF EXISTS idx_detection_zones_stream;\nDROP TABLE IF EXISTS detection_zones;\0",
    )
};
static mut migration_0023_up: [::core::ffi::c_char; 153] = unsafe {
    ::core::mem::transmute::<
        [u8; 153],
        [::core::ffi::c_char; 153],
    >(
        *b"ALTER TABLE streams ADD COLUMN detection_object_filter TEXT DEFAULT 'none';\nALTER TABLE streams ADD COLUMN detection_object_filter_list TEXT DEFAULT '';\0",
    )
};
static mut migration_0023_down: [::core::ffi::c_char; 106] = unsafe {
    ::core::mem::transmute::<
        [u8; 106],
        [::core::ffi::c_char; 106],
    >(
        *b"-- SQLite doesn't support DROP COLUMN in older versions\n-- These columns will just be ignored if not used\0",
    )
};
static mut migration_0024_up: [::core::ffi::c_char; 1479] = unsafe {
    ::core::mem::transmute::<
        [u8; 1479],
        [::core::ffi::c_char; 1479],
    >(
        *b"-- Tiered retention columns on recordings\nALTER TABLE recordings ADD COLUMN retention_tier INTEGER DEFAULT 2;\nALTER TABLE recordings ADD COLUMN disk_pressure_eligible INTEGER DEFAULT 1;\n\n-- Tier multipliers and storage priority on streams\nALTER TABLE streams ADD COLUMN tier_critical_multiplier REAL DEFAULT 3.0;\nALTER TABLE streams ADD COLUMN tier_important_multiplier REAL DEFAULT 2.0;\nALTER TABLE streams ADD COLUMN tier_ephemeral_multiplier REAL DEFAULT 0.25;\nALTER TABLE streams ADD COLUMN storage_priority INTEGER DEFAULT 5;\n\n-- Daily storage statistics table for analytics\nCREATE TABLE IF NOT EXISTS storage_daily_stats (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    date TEXT NOT NULL,\n    stream_name TEXT NOT NULL,\n    tier INTEGER NOT NULL DEFAULT 2,\n    recording_count INTEGER NOT NULL DEFAULT 0,\n    total_bytes INTEGER NOT NULL DEFAULT 0,\n    deleted_count INTEGER NOT NULL DEFAULT 0,\n    deleted_bytes INTEGER NOT NULL DEFAULT 0,\n    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n);\n\n-- Indexes for efficient queries on daily stats\nCREATE INDEX IF NOT EXISTS idx_storage_daily_stats_stream_date ON storage_daily_stats(stream_name, date);\nCREATE INDEX IF NOT EXISTS idx_storage_daily_stats_date_tier ON storage_daily_stats(date, tier);\n\n-- Index for tier-based retention queries\nCREATE INDEX IF NOT EXISTS idx_recordings_retention_tier ON recordings(retention_tier);\nCREATE INDEX IF NOT EXISTS idx_recordings_disk_pressure ON recordings(disk_pressure_eligible);\0",
    )
};
static mut migration_0024_down: [::core::ffi::c_char; 269] = unsafe {
    ::core::mem::transmute::<
        [u8; 269],
        [::core::ffi::c_char; 269],
    >(
        *b"DROP INDEX IF EXISTS idx_recordings_disk_pressure;\nDROP INDEX IF EXISTS idx_recordings_retention_tier;\nDROP INDEX IF EXISTS idx_storage_daily_stats_date_tier;\nDROP INDEX IF EXISTS idx_storage_daily_stats_stream_date;\nDROP TABLE IF EXISTS storage_daily_stats;\nSELECT 1;\0",
    )
};
static mut migration_0025_up: [::core::ffi::c_char; 733] = unsafe {
    ::core::mem::transmute::<
        [u8; 733],
        [::core::ffi::c_char; 733],
    >(
        *b"-- Detection performance indexes (new)\nCREATE INDEX IF NOT EXISTS idx_detections_recording_id ON detections(recording_id);\n\n-- Indexes previously only in db_core.c (never migrated)\nCREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp);\nCREATE INDEX IF NOT EXISTS idx_events_type ON events(type);\nCREATE INDEX IF NOT EXISTS idx_events_stream ON events(stream_name);\nCREATE INDEX IF NOT EXISTS idx_recordings_end_time ON recordings(end_time);\nCREATE INDEX IF NOT EXISTS idx_recordings_complete_stream_start ON recordings(is_complete, stream_name, start_time);\nCREATE INDEX IF NOT EXISTS idx_streams_name ON streams(name);\nCREATE INDEX IF NOT EXISTS idx_detections_stream_timestamp ON detections(stream_name, timestamp);\0",
    )
};
static mut migration_0025_down: [::core::ffi::c_char; 369] = unsafe {
    ::core::mem::transmute::<
        [u8; 369],
        [::core::ffi::c_char; 369],
    >(
        *b"DROP INDEX IF EXISTS idx_detections_stream_timestamp;\nDROP INDEX IF EXISTS idx_streams_name;\nDROP INDEX IF EXISTS idx_recordings_complete_stream_start;\nDROP INDEX IF EXISTS idx_recordings_end_time;\nDROP INDEX IF EXISTS idx_events_stream;\nDROP INDEX IF EXISTS idx_events_type;\nDROP INDEX IF EXISTS idx_events_timestamp;\nDROP INDEX IF EXISTS idx_detections_recording_id;\0",
    )
};
static mut migration_0026_up: [::core::ffi::c_char; 147] = unsafe {
    ::core::mem::transmute::<
        [u8; 147],
        [::core::ffi::c_char; 147],
    >(
        *b"ALTER TABLE streams ADD COLUMN record_on_schedule INTEGER NOT NULL DEFAULT 0;\nALTER TABLE streams ADD COLUMN recording_schedule TEXT DEFAULT NULL;\0",
    )
};
static mut migration_0026_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0027_up: [::core::ffi::c_char; 68] = unsafe {
    ::core::mem::transmute::<[u8; 68], [::core::ffi::c_char; 68]>(
        *b"ALTER TABLE streams ADD COLUMN group_name TEXT NOT NULL DEFAULT '';\0",
    )
};
static mut migration_0027_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0028_up: [::core::ffi::c_char; 224] = unsafe {
    ::core::mem::transmute::<
        [u8; 224],
        [::core::ffi::c_char; 224],
    >(
        *b"ALTER TABLE streams ADD COLUMN tags TEXT NOT NULL DEFAULT '';UPDATE streams SET tags = group_name WHERE (tags = '' OR tags IS NULL) AND group_name IS NOT NULL AND group_name != '';ALTER TABLE streams DROP COLUMN group_name;\0",
    )
};
static mut migration_0028_down: [::core::ffi::c_char; 242] = unsafe {
    ::core::mem::transmute::<
        [u8; 242],
        [::core::ffi::c_char; 242],
    >(
        *b"ALTER TABLE streams ADD COLUMN group_name TEXT NOT NULL DEFAULT '';UPDATE streams SET group_name = CASE WHEN instr(tags, ',') > 0 THEN substr(tags, 1, instr(tags, ',') - 1) ELSE tags END WHERE tags != '';ALTER TABLE streams DROP COLUMN tags;\0",
    )
};
static mut migration_0029_up: [::core::ffi::c_char; 61] = unsafe {
    ::core::mem::transmute::<[u8; 61], [::core::ffi::c_char; 61]>(
        *b"ALTER TABLE users ADD COLUMN allowed_tags TEXT DEFAULT NULL;\0",
    )
};
static mut migration_0029_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0030_up: [::core::ffi::c_char; 259] = unsafe {
    ::core::mem::transmute::<
        [u8; 259],
        [::core::ffi::c_char; 259],
    >(
        *b"CREATE TABLE IF NOT EXISTS system_settings (\n    key   TEXT PRIMARY KEY,\n    value TEXT NOT NULL DEFAULT '',\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))\n);\nINSERT OR IGNORE INTO system_settings (key, value) VALUES ('setup_complete', '0');\0",
    )
};
static mut migration_0030_down: [::core::ffi::c_char; 38] = unsafe {
    ::core::mem::transmute::<[u8; 38], [::core::ffi::c_char; 38]>(
        *b"DROP TABLE IF EXISTS system_settings;\0",
    )
};
static mut migration_0031_up: [::core::ffi::c_char; 61] = unsafe {
    ::core::mem::transmute::<[u8; 61], [::core::ffi::c_char; 61]>(
        *b"ALTER TABLE streams ADD COLUMN onvif_port INTEGER DEFAULT 0;\0",
    )
};
static mut migration_0031_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0032_up: [::core::ffi::c_char; 540] = unsafe {
    ::core::mem::transmute::<
        [u8; 540],
        [::core::ffi::c_char; 540],
    >(
        *b"CREATE TABLE IF NOT EXISTS recording_tags (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    recording_id INTEGER NOT NULL,\n    tag TEXT NOT NULL,\n    created_at INTEGER DEFAULT (strftime('%s', 'now')),\n    FOREIGN KEY (recording_id) REFERENCES recordings(id) ON DELETE CASCADE\n);\n\nCREATE UNIQUE INDEX IF NOT EXISTS idx_recording_tags_unique ON recording_tags(recording_id, tag);\nCREATE INDEX IF NOT EXISTS idx_recording_tags_tag ON recording_tags(tag);\nCREATE INDEX IF NOT EXISTS idx_recording_tags_recording ON recording_tags(recording_id);\0",
    )
};
static mut migration_0032_down: [::core::ffi::c_char; 181] = unsafe {
    ::core::mem::transmute::<
        [u8; 181],
        [::core::ffi::c_char; 181],
    >(
        *b"DROP INDEX IF EXISTS idx_recording_tags_recording;\nDROP INDEX IF EXISTS idx_recording_tags_tag;\nDROP INDEX IF EXISTS idx_recording_tags_unique;\nDROP TABLE IF EXISTS recording_tags;\0",
    )
};
static mut migration_0033_up: [::core::ffi::c_char; 59] = unsafe {
    ::core::mem::transmute::<[u8; 59], [::core::ffi::c_char; 59]>(
        *b"DELETE FROM streams WHERE TRIM(name) = '' OR name IS NULL;\0",
    )
};
static mut migration_0033_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0034_up: [::core::ffi::c_char; 2581] = unsafe {
    ::core::mem::transmute::<
        [u8; 2581],
        [::core::ffi::c_char; 2581],
    >(
        *b"UPDATE streams\n   SET url = substr(url, 1, instr(url, '://') + 2) ||\n             substr(substr(url, instr(url, '://') + 3),\n                    instr(substr(url, instr(url, '://') + 3), '@') + 1),\n       onvif_username = CASE\n           WHEN COALESCE(onvif_username, '') = '' THEN\n               CASE\n                   WHEN instr(substr(substr(url, instr(url, '://') + 3),\n                                     1,\n                                     instr(substr(url, instr(url, '://') + 3), '@') - 1),\n                              ':') > 0 THEN\n                       substr(substr(substr(url, instr(url, '://') + 3),\n                                           1,\n                                           instr(substr(url, instr(url, '://') + 3), '@') - 1),\n                              1,\n                              instr(substr(substr(url, instr(url, '://') + 3),\n                                                  1,\n                                                  instr(substr(url, instr(url, '://') + 3), '@') - 1),\n                                     ':') - 1)\n                   ELSE\n                       substr(substr(url, instr(url, '://') + 3),\n                              1,\n                              instr(substr(url, instr(url, '://') + 3), '@') - 1)\n               END\n           ELSE onvif_username\n       END,\n       onvif_password = CASE\n           WHEN COALESCE(onvif_password, '') = '' THEN\n               CASE\n                   WHEN instr(substr(substr(url, instr(url, '://') + 3),\n                                     1,\n                                     instr(substr(url, instr(url, '://') + 3), '@') - 1),\n                              ':') > 0 THEN\n                       substr(substr(substr(url, instr(url, '://') + 3),\n                                           1,\n                                           instr(substr(url, instr(url, '://') + 3), '@') - 1),\n                              instr(substr(substr(url, instr(url, '://') + 3),\n                                                  1,\n                                                  instr(substr(url, instr(url, '://') + 3), '@') - 1),\n                                     ':') + 1)\n                   ELSE ''\n               END\n           ELSE onvif_password\n       END\n WHERE instr(url, '://') > 0\n   AND instr(substr(url, instr(url, '://') + 3), '@') > 0\n   AND (\n        is_onvif = 1 OR\n        COALESCE(onvif_profile, '') != '' OR\n        COALESCE(onvif_username, '') != '' OR\n        COALESCE(onvif_password, '') != '' OR\n        COALESCE(onvif_port, 0) > 0\n   );\0",
    )
};
static mut migration_0034_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0035_up: [::core::ffi::c_char; 1068] = unsafe {
    ::core::mem::transmute::<
        [u8; 1068],
        [::core::ffi::c_char; 1068],
    >(
        *b"ALTER TABLE sessions ADD COLUMN last_activity_at INTEGER;\nUPDATE sessions\n   SET last_activity_at = COALESCE(last_activity_at, created_at, strftime('%s', 'now'));\nALTER TABLE sessions ADD COLUMN idle_expires_at INTEGER;\nUPDATE sessions\n   SET idle_expires_at = COALESCE(idle_expires_at, expires_at, strftime('%s', 'now'));\nCREATE INDEX IF NOT EXISTS idx_sessions_user_last_activity\n    ON sessions(user_id, last_activity_at DESC);\nCREATE TABLE IF NOT EXISTS trusted_devices (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    user_id INTEGER NOT NULL,\n    token TEXT NOT NULL UNIQUE,\n    ip_address TEXT,\n    user_agent TEXT,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    last_used_at INTEGER,\n    expires_at INTEGER NOT NULL,\n    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE\n);\nCREATE INDEX IF NOT EXISTS idx_trusted_devices_user_id ON trusted_devices(user_id);\nCREATE INDEX IF NOT EXISTS idx_trusted_devices_token ON trusted_devices(token);\nCREATE INDEX IF NOT EXISTS idx_trusted_devices_expires_at ON trusted_devices(expires_at);\0",
    )
};
static mut migration_0035_down: [::core::ffi::c_char; 243] = unsafe {
    ::core::mem::transmute::<
        [u8; 243],
        [::core::ffi::c_char; 243],
    >(
        *b"DROP INDEX IF EXISTS idx_trusted_devices_expires_at;\nDROP INDEX IF EXISTS idx_trusted_devices_token;\nDROP INDEX IF EXISTS idx_trusted_devices_user_id;\nDROP TABLE IF EXISTS trusted_devices;\nDROP INDEX IF EXISTS idx_sessions_user_last_activity;\0",
    )
};
static mut migration_0036_up: [::core::ffi::c_char; 58] = unsafe {
    ::core::mem::transmute::<[u8; 58], [::core::ffi::c_char; 58]>(
        *b"ALTER TABLE streams ADD COLUMN admin_url TEXT DEFAULT '';\0",
    )
};
static mut migration_0036_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0037_up: [::core::ffi::c_char; 68] = unsafe {
    ::core::mem::transmute::<[u8; 68], [::core::ffi::c_char; 68]>(
        *b"ALTER TABLE users ADD COLUMN allowed_login_cidrs TEXT DEFAULT NULL;\0",
    )
};
static mut migration_0037_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0038_up: [::core::ffi::c_char; 63] = unsafe {
    ::core::mem::transmute::<[u8; 63], [::core::ffi::c_char; 63]>(
        *b"ALTER TABLE streams ADD COLUMN privacy_mode INTEGER DEFAULT 0;\0",
    )
};
static mut migration_0038_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0039_up: [::core::ffi::c_char; 70] = unsafe {
    ::core::mem::transmute::<[u8; 70], [::core::ffi::c_char; 70]>(
        *b"ALTER TABLE streams ADD COLUMN motion_trigger_source TEXT DEFAULT '';\0",
    )
};
static mut migration_0039_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0040_up: [::core::ffi::c_char; 134] = unsafe {
    ::core::mem::transmute::<
        [u8; 134],
        [::core::ffi::c_char; 134],
    >(
        *b"ALTER TABLE streams ADD COLUMN go2rtc_source_override TEXT DEFAULT '';\nALTER TABLE streams ADD COLUMN sub_stream_url TEXT DEFAULT '';\0",
    )
};
static mut migration_0040_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0041_up: [::core::ffi::c_char; 74] = unsafe {
    ::core::mem::transmute::<[u8; 74], [::core::ffi::c_char; 74]>(
        *b"ALTER TABLE streams ADD COLUMN audio_voice_enhancement INTEGER DEFAULT 0;\0",
    )
};
static mut migration_0041_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0042_up: [::core::ffi::c_char; 62] = unsafe {
    ::core::mem::transmute::<[u8; 62], [::core::ffi::c_char; 62]>(
        *b"ALTER TABLE streams ADD COLUMN detection_url TEXT DEFAULT '';\0",
    )
};
static mut migration_0042_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0043_up: [::core::ffi::c_char; 494] = unsafe {
    ::core::mem::transmute::<
        [u8; 494],
        [::core::ffi::c_char; 494],
    >(
        *b"-- Backfill disk_pressure_eligible on existing recordings.\n-- A prior bug inserted every recording with disk_pressure_eligible = 0,\n-- which left the emergency disk-pressure eviction path with no candidates\n-- and allowed disks to fill to 100%. Eligibility is derived from the\n-- protection flag: any unprotected recording may be evicted under pressure.\nUPDATE recordings SET disk_pressure_eligible = 1 WHERE protected = 0;\nUPDATE recordings SET disk_pressure_eligible = 0 WHERE protected = 1;\0",
    )
};
static mut migration_0043_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0044_up: [::core::ffi::c_char; 60] = unsafe {
    ::core::mem::transmute::<[u8; 60], [::core::ffi::c_char; 60]>(
        *b"ALTER TABLE streams ADD COLUMN publish_url TEXT DEFAULT '';\0",
    )
};
static mut migration_0044_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0045_up: [::core::ffi::c_char; 243] = unsafe {
    ::core::mem::transmute::<
        [u8; 243],
        [::core::ffi::c_char; 243],
    >(
        *b"ALTER TABLE recordings ADD COLUMN schedule_restricted INTEGER DEFAULT NULL;\nALTER TABLE streams ADD COLUMN detection_record_on_schedule INTEGER NOT NULL DEFAULT 0;\nALTER TABLE streams ADD COLUMN detection_recording_schedule TEXT DEFAULT NULL;\0",
    )
};
static mut migration_0045_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0046_up: [::core::ffi::c_char; 329] = unsafe {
    ::core::mem::transmute::<
        [u8; 329],
        [::core::ffi::c_char; 329],
    >(
        *b"ALTER TABLE detections ADD COLUMN source TEXT NOT NULL DEFAULT '';\nALTER TABLE detections ADD COLUMN event_end_time INTEGER DEFAULT NULL;\nUPDATE detections SET event_end_time = timestamp WHERE source = '';\nCREATE INDEX IF NOT EXISTS idx_detections_external_interval ON detections(stream_name, source, timestamp, event_end_time);\0",
    )
};
static mut migration_0046_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0047_up: [::core::ffi::c_char; 688] = unsafe {
    ::core::mem::transmute::<
        [u8; 688],
        [::core::ffi::c_char; 688],
    >(
        *b"CREATE INDEX IF NOT EXISTS idx_recordings_start_time ON recordings(start_time);\nCREATE INDEX IF NOT EXISTS idx_recordings_complete_stream_start ON recordings(is_complete, stream_name, start_time);\nCREATE INDEX IF NOT EXISTS idx_detections_stream_timestamp ON detections(stream_name, timestamp);\nCREATE INDEX IF NOT EXISTS idx_detections_recording_id ON detections(recording_id);\nCREATE INDEX IF NOT EXISTS idx_recordings_history_start ON recordings(start_time DESC) WHERE is_complete = 1 AND end_time IS NOT NULL;\nCREATE INDEX IF NOT EXISTS idx_detections_unlinked_stream_time_label ON detections(stream_name, timestamp, label) WHERE recording_id IS NULL AND source != 'external_motion';\0",
    )
};
static mut migration_0047_down: [::core::ffi::c_char; 115] = unsafe {
    ::core::mem::transmute::<
        [u8; 115],
        [::core::ffi::c_char; 115],
    >(
        *b"DROP INDEX IF EXISTS idx_detections_unlinked_stream_time_label;\nDROP INDEX IF EXISTS idx_recordings_history_start;\0",
    )
};
static mut migration_0048_up: [::core::ffi::c_char; 448] = unsafe {
    ::core::mem::transmute::<
        [u8; 448],
        [::core::ffi::c_char; 448],
    >(
        *b"ALTER TABLE streams ADD COLUMN camera_uuid TEXT NOT NULL DEFAULT '';\n\nUPDATE streams\nSET camera_uuid = lower(\n    hex(randomblob(4)) || '-' ||\n    hex(randomblob(2)) || '-4' ||\n    substr(hex(randomblob(2)), 2) || '-' ||\n    substr('89ab', (abs(random()) % 4) + 1, 1) ||\n    substr(hex(randomblob(2)), 2) || '-' ||\n    hex(randomblob(6))\n)\nWHERE camera_uuid = '';\n\nCREATE UNIQUE INDEX IF NOT EXISTS idx_streams_camera_uuid\nON streams(camera_uuid);\0",
    )
};
static mut migration_0048_down: [::core::ffi::c_char; 56] = unsafe {
    ::core::mem::transmute::<[u8; 56], [::core::ffi::c_char; 56]>(
        *b"DROP INDEX IF EXISTS idx_streams_camera_uuid;\nSELECT 1;\0",
    )
};
static mut migration_0049_up: [::core::ffi::c_char; 1454] = unsafe {
    ::core::mem::transmute::<
        [u8; 1454],
        [::core::ffi::c_char; 1454],
    >(
        *b"CREATE TABLE camera_locations (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    uuid TEXT NOT NULL UNIQUE,\n    parent_uuid TEXT REFERENCES camera_locations(uuid) ON DELETE RESTRICT,\n    name TEXT NOT NULL,\n    type TEXT NOT NULL DEFAULT 'area',\n    sort_order INTEGER NOT NULL DEFAULT 0,\n    metadata_json TEXT NOT NULL DEFAULT '{}',\n    is_system INTEGER NOT NULL DEFAULT 0,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))\n);\n\nCREATE UNIQUE INDEX idx_camera_locations_sibling_name\nON camera_locations(ifnull(parent_uuid, ''), name COLLATE NOCASE);\n\nCREATE INDEX idx_camera_locations_parent\nON camera_locations(parent_uuid, sort_order, name COLLATE NOCASE);\n\nINSERT INTO camera_locations (uuid, parent_uuid, name, type, is_system)\nVALUES (\n    lower(\n        hex(randomblob(4)) || '-' ||\n        hex(randomblob(2)) || '-4' ||\n        substr(hex(randomblob(2)), 2) || '-' ||\n        substr('89ab', (abs(random()) % 4) + 1, 1) ||\n        substr(hex(randomblob(2)), 2) || '-' ||\n        hex(randomblob(6))\n    ),\n    NULL,\n    'Unassigned',\n    'system',\n    1\n);\n\nALTER TABLE streams ADD COLUMN location_uuid TEXT DEFAULT NULL\nREFERENCES camera_locations(uuid) ON DELETE RESTRICT;\n\nUPDATE streams\nSET location_uuid = (SELECT uuid FROM camera_locations WHERE is_system = 1 LIMIT 1)\nWHERE location_uuid IS NULL;\n\nCREATE INDEX idx_streams_location_uuid\nON streams(location_uuid);\0",
    )
};
static mut migration_0049_down: [::core::ffi::c_char; 244] = unsafe {
    ::core::mem::transmute::<
        [u8; 244],
        [::core::ffi::c_char; 244],
    >(
        *b"DROP INDEX IF EXISTS idx_streams_location_uuid;\nUPDATE streams SET location_uuid = NULL;\nDROP INDEX IF EXISTS idx_camera_locations_parent;\nDROP INDEX IF EXISTS idx_camera_locations_sibling_name;\nDROP TABLE IF EXISTS camera_locations;\nSELECT 1;\0",
    )
};
static mut migration_0050_up: [::core::ffi::c_char; 814] = unsafe {
    ::core::mem::transmute::<
        [u8; 814],
        [::core::ffi::c_char; 814],
    >(
        *b"CREATE TABLE camera_tags (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    uuid TEXT NOT NULL UNIQUE,\n    label TEXT NOT NULL,\n    color TEXT NOT NULL DEFAULT '',\n    description TEXT NOT NULL DEFAULT '',\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))\n);\n\nCREATE UNIQUE INDEX idx_camera_tags_label\nON camera_tags(label COLLATE NOCASE);\n\nCREATE TABLE camera_tag_assignments (\n    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,\n    tag_uuid TEXT NOT NULL REFERENCES camera_tags(uuid) ON DELETE CASCADE,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    PRIMARY KEY (camera_uuid, tag_uuid)\n);\n\nCREATE INDEX idx_camera_tag_assignments_tag\nON camera_tag_assignments(tag_uuid, camera_uuid);\0",
    )
};
static mut migration_0050_down: [::core::ffi::c_char; 186] = unsafe {
    ::core::mem::transmute::<
        [u8; 186],
        [::core::ffi::c_char; 186],
    >(
        *b"DROP INDEX IF EXISTS idx_camera_tag_assignments_tag;\nDROP TABLE IF EXISTS camera_tag_assignments;\nDROP INDEX IF EXISTS idx_camera_tags_label;\nDROP TABLE IF EXISTS camera_tags;\nSELECT 1;\0",
    )
};
static mut migration_0051_up: [::core::ffi::c_char; 1163] = unsafe {
    ::core::mem::transmute::<
        [u8; 1163],
        [::core::ffi::c_char; 1163],
    >(
        *b"CREATE TABLE camera_collections (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    uuid TEXT NOT NULL UNIQUE,\n    name TEXT NOT NULL,\n    description TEXT NOT NULL DEFAULT '',\n    collection_type TEXT NOT NULL CHECK (collection_type IN ('static', 'smart')),\n    selector_json TEXT NOT NULL DEFAULT '',\n    is_shared INTEGER NOT NULL DEFAULT 1,\n    owner_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))\n);\n\nCREATE UNIQUE INDEX idx_camera_collections_name\nON camera_collections(name COLLATE NOCASE);\n\nCREATE INDEX idx_camera_collections_owner\nON camera_collections(owner_user_id, is_shared);\n\nCREATE TABLE camera_collection_members (\n    collection_uuid TEXT NOT NULL REFERENCES camera_collections(uuid) ON DELETE CASCADE,\n    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    PRIMARY KEY (collection_uuid, camera_uuid)\n);\n\nCREATE INDEX idx_camera_collection_members_camera\nON camera_collection_members(camera_uuid, collection_uuid);\0",
    )
};
static mut migration_0051_down: [::core::ffi::c_char; 259] = unsafe {
    ::core::mem::transmute::<
        [u8; 259],
        [::core::ffi::c_char; 259],
    >(
        *b"DROP INDEX IF EXISTS idx_camera_collection_members_camera;\nDROP TABLE IF EXISTS camera_collection_members;\nDROP INDEX IF EXISTS idx_camera_collections_owner;\nDROP INDEX IF EXISTS idx_camera_collections_name;\nDROP TABLE IF EXISTS camera_collections;\nSELECT 1;\0",
    )
};
static mut migration_0052_up: [::core::ffi::c_char; 5217] = unsafe {
    ::core::mem::transmute::<
        [u8; 5217],
        [::core::ffi::c_char; 5217],
    >(
        *b"ALTER TABLE users ADD COLUMN authorization_mode TEXT NOT NULL DEFAULT 'legacy' CHECK (authorization_mode IN ('legacy', 'policy'));\nCREATE TABLE authz_actions (action_key TEXT PRIMARY KEY, category TEXT NOT NULL, description TEXT NOT NULL, camera_scoped INTEGER NOT NULL DEFAULT 0, destructive INTEGER NOT NULL DEFAULT 0, sort_order INTEGER NOT NULL DEFAULT 0);\nCREATE TABLE authz_roles (uuid TEXT PRIMARY KEY, name TEXT NOT NULL UNIQUE COLLATE NOCASE, description TEXT NOT NULL DEFAULT '', is_builtin INTEGER NOT NULL DEFAULT 0, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')));\nCREATE TABLE authz_role_actions (role_uuid TEXT NOT NULL REFERENCES authz_roles(uuid) ON DELETE CASCADE, action_key TEXT NOT NULL REFERENCES authz_actions(action_key) ON DELETE RESTRICT, PRIMARY KEY (role_uuid, action_key));\nCREATE TABLE authz_grants (uuid TEXT PRIMARY KEY, user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, role_uuid TEXT NOT NULL REFERENCES authz_roles(uuid) ON DELETE RESTRICT, scope_type TEXT NOT NULL DEFAULT 'all' CHECK (scope_type IN ('all', 'selector')), selector_json TEXT, enabled INTEGER NOT NULL DEFAULT 1, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), CHECK ((scope_type = 'all' AND selector_json IS NULL) OR (scope_type = 'selector' AND selector_json IS NOT NULL)));\nCREATE INDEX idx_authz_grants_user ON authz_grants(user_id, enabled);\nCREATE INDEX idx_authz_grants_role ON authz_grants(role_uuid);\nCREATE TABLE authz_policy_state (id INTEGER PRIMARY KEY CHECK (id = 1), version INTEGER NOT NULL DEFAULT 1, updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')));\nINSERT INTO authz_policy_state (id, version) VALUES (1, 1);\nINSERT INTO authz_actions (action_key, category, description, camera_scoped, destructive, sort_order) VALUES ('live.view','Live video','View a camera live stream',1,0,10),('audio.listen','Live video','Listen to camera audio',1,0,20),('audio.talk','Live video','Transmit audio to a camera',1,0,30),('recordings.replay','Recordings','Replay recorded video',1,0,40),('recordings.export','Recordings','Download or export recorded video',1,0,50),('snapshot.create','Recordings','Create or download a camera snapshot',1,0,60),('ptz.control','Camera operation','Move PTZ cameras and manage presets',1,0,70),('evidence.protect','Recordings','Protect or release recordings from retention',1,1,80),('recording.delete','Recordings','Permanently delete recordings',1,1,90),('camera.configure','Camera administration','Add, change, or remove camera configuration',1,1,100),('fleet.execute_job','Camera administration','Execute a bulk fleet operation',1,1,110),('storage.configure','System administration','Change storage and retention configuration',0,1,120),('events.configure','System administration','Change event routes and destinations',0,1,130),('users.manage','System administration','Manage users, roles, and grants',0,1,140),('system.admin','System administration','Change or control the lightNVR system',0,1,150);\nINSERT INTO authz_roles (uuid,name,description,is_builtin) VALUES ('00000000-0000-4000-8000-000000000001','Administrator','All lightNVR actions over every resource',1),('00000000-0000-4000-8000-000000000002','Operator','Day-to-day camera and recording operation',1),('00000000-0000-4000-8000-000000000003','Viewer','Read-only live and recording access',1),('00000000-0000-4000-8000-000000000004','API Operator','Programmatic camera and recording operation',1);\nINSERT INTO authz_role_actions (role_uuid,action_key) SELECT '00000000-0000-4000-8000-000000000001',action_key FROM authz_actions;\nINSERT INTO authz_role_actions (role_uuid,action_key) VALUES ('00000000-0000-4000-8000-000000000002','live.view'),('00000000-0000-4000-8000-000000000002','audio.listen'),('00000000-0000-4000-8000-000000000002','audio.talk'),('00000000-0000-4000-8000-000000000002','recordings.replay'),('00000000-0000-4000-8000-000000000002','recordings.export'),('00000000-0000-4000-8000-000000000002','snapshot.create'),('00000000-0000-4000-8000-000000000002','ptz.control'),('00000000-0000-4000-8000-000000000002','evidence.protect'),('00000000-0000-4000-8000-000000000002','recording.delete'),('00000000-0000-4000-8000-000000000002','camera.configure'),('00000000-0000-4000-8000-000000000003','live.view'),('00000000-0000-4000-8000-000000000003','audio.listen'),('00000000-0000-4000-8000-000000000003','recordings.replay'),('00000000-0000-4000-8000-000000000003','recordings.export'),('00000000-0000-4000-8000-000000000003','snapshot.create'),('00000000-0000-4000-8000-000000000004','live.view'),('00000000-0000-4000-8000-000000000004','audio.listen'),('00000000-0000-4000-8000-000000000004','audio.talk'),('00000000-0000-4000-8000-000000000004','recordings.replay'),('00000000-0000-4000-8000-000000000004','recordings.export'),('00000000-0000-4000-8000-000000000004','snapshot.create'),('00000000-0000-4000-8000-000000000004','ptz.control'),('00000000-0000-4000-8000-000000000004','evidence.protect'),('00000000-0000-4000-8000-000000000004','recording.delete'),('00000000-0000-4000-8000-000000000004','camera.configure');\0",
    )
};
static mut migration_0052_down: [::core::ffi::c_char; 285] = unsafe {
    ::core::mem::transmute::<
        [u8; 285],
        [::core::ffi::c_char; 285],
    >(
        *b"DROP TABLE IF EXISTS authz_policy_state;\nDROP INDEX IF EXISTS idx_authz_grants_role;\nDROP INDEX IF EXISTS idx_authz_grants_user;\nDROP TABLE IF EXISTS authz_grants;\nDROP TABLE IF EXISTS authz_role_actions;\nDROP TABLE IF EXISTS authz_roles;\nDROP TABLE IF EXISTS authz_actions;\nSELECT 1;\0",
    )
};
static mut migration_0053_up: [::core::ffi::c_char; 1394] = unsafe {
    ::core::mem::transmute::<
        [u8; 1394],
        [::core::ffi::c_char; 1394],
    >(
        *b"CREATE TABLE authz_grants_new (uuid TEXT PRIMARY KEY, user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, role_uuid TEXT NOT NULL REFERENCES authz_roles(uuid) ON DELETE RESTRICT, scope_type TEXT NOT NULL DEFAULT 'all' CHECK (scope_type IN ('all','selector','collection')), selector_json TEXT, collection_uuid TEXT REFERENCES camera_collections(uuid) ON DELETE RESTRICT, enabled INTEGER NOT NULL DEFAULT 1, created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), CHECK ((scope_type='all' AND selector_json IS NULL AND collection_uuid IS NULL) OR (scope_type='selector' AND selector_json IS NOT NULL AND collection_uuid IS NULL) OR (scope_type='collection' AND selector_json IS NULL AND collection_uuid IS NOT NULL)));\nINSERT INTO authz_grants_new (uuid,user_id,role_uuid,scope_type,selector_json,enabled,created_at,updated_at) SELECT uuid,user_id,role_uuid,scope_type,selector_json,enabled,created_at,updated_at FROM authz_grants;\nDROP INDEX idx_authz_grants_role;\nDROP INDEX idx_authz_grants_user;\nDROP TABLE authz_grants;\nALTER TABLE authz_grants_new RENAME TO authz_grants;\nCREATE INDEX idx_authz_grants_user ON authz_grants(user_id,enabled);\nCREATE INDEX idx_authz_grants_role ON authz_grants(role_uuid);\nCREATE INDEX idx_authz_grants_collection ON authz_grants(collection_uuid) WHERE collection_uuid IS NOT NULL;\0",
    )
};
static mut migration_0053_down: [::core::ffi::c_char; 1133] = unsafe {
    ::core::mem::transmute::<
        [u8; 1133],
        [::core::ffi::c_char; 1133],
    >(
        *b"CREATE TABLE authz_grants_old (uuid TEXT PRIMARY KEY, user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, role_uuid TEXT NOT NULL REFERENCES authz_roles(uuid) ON DELETE RESTRICT, scope_type TEXT NOT NULL DEFAULT 'all' CHECK (scope_type IN ('all','selector')), selector_json TEXT, enabled INTEGER NOT NULL DEFAULT 1, created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), CHECK ((scope_type='all' AND selector_json IS NULL) OR (scope_type='selector' AND selector_json IS NOT NULL)));\nINSERT INTO authz_grants_old (uuid,user_id,role_uuid,scope_type,selector_json,enabled,created_at,updated_at) SELECT uuid,user_id,role_uuid,scope_type,selector_json,enabled,created_at,updated_at FROM authz_grants WHERE scope_type!='collection';\nDROP INDEX idx_authz_grants_collection;\nDROP INDEX idx_authz_grants_role;\nDROP INDEX idx_authz_grants_user;\nDROP TABLE authz_grants;\nALTER TABLE authz_grants_old RENAME TO authz_grants;\nCREATE INDEX idx_authz_grants_user ON authz_grants(user_id,enabled);\nCREATE INDEX idx_authz_grants_role ON authz_grants(role_uuid);\nSELECT 1;\0",
    )
};
static mut migration_0054_up: [::core::ffi::c_char; 1116] = unsafe {
    ::core::mem::transmute::<
        [u8; 1116],
        [::core::ffi::c_char; 1116],
    >(
        *b"CREATE TABLE authz_api_tokens (uuid TEXT PRIMARY KEY, user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, created_by_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL, description TEXT NOT NULL, token_prefix TEXT NOT NULL, token_hash TEXT NOT NULL UNIQUE, action_mask INTEGER NOT NULL CHECK (action_mask>0), scope_type TEXT NOT NULL CHECK (scope_type IN ('all','selector','collection')), selector_json TEXT, collection_uuid TEXT REFERENCES camera_collections(uuid) ON DELETE RESTRICT, expires_at INTEGER NOT NULL, revoked_at INTEGER, last_used_at INTEGER, created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), CHECK ((scope_type='all' AND selector_json IS NULL AND collection_uuid IS NULL) OR (scope_type='selector' AND selector_json IS NOT NULL AND collection_uuid IS NULL) OR (scope_type='collection' AND selector_json IS NULL AND collection_uuid IS NOT NULL)), CHECK (expires_at>created_at));\nCREATE INDEX idx_authz_api_tokens_user ON authz_api_tokens(user_id,created_at DESC);\nCREATE INDEX idx_authz_api_tokens_active ON authz_api_tokens(token_hash,expires_at) WHERE revoked_at IS NULL;\0",
    )
};
static mut migration_0054_down: [::core::ffi::c_char; 147] = unsafe {
    ::core::mem::transmute::<
        [u8; 147],
        [::core::ffi::c_char; 147],
    >(
        *b"DROP INDEX IF EXISTS idx_authz_api_tokens_active;\nDROP INDEX IF EXISTS idx_authz_api_tokens_user;\nDROP TABLE IF EXISTS authz_api_tokens;\nSELECT 1;\0",
    )
};
static mut migration_0055_up: [::core::ffi::c_char; 1041] = unsafe {
    ::core::mem::transmute::<
        [u8; 1041],
        [::core::ffi::c_char; 1041],
    >(
        *b"CREATE TABLE audit_events (id INTEGER PRIMARY KEY AUTOINCREMENT, uuid TEXT NOT NULL UNIQUE, occurred_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), request_id TEXT NOT NULL, principal_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL, principal_username TEXT NOT NULL DEFAULT '', auth_method TEXT NOT NULL DEFAULT 'unknown', api_token_uuid TEXT, action TEXT NOT NULL, target_type TEXT, target_uuid TEXT, outcome TEXT NOT NULL CHECK (outcome IN ('allowed','denied','success','failure','error')), remote_address TEXT, details_json TEXT NOT NULL DEFAULT '{}');\nCREATE INDEX idx_audit_events_occurred ON audit_events(occurred_at DESC,id DESC);\nCREATE INDEX idx_audit_events_principal ON audit_events(principal_user_id,occurred_at DESC);\nCREATE INDEX idx_audit_events_action_outcome ON audit_events(action,outcome,occurred_at DESC);\nCREATE INDEX idx_audit_events_target ON audit_events(target_uuid,occurred_at DESC) WHERE target_uuid IS NOT NULL;\nINSERT OR IGNORE INTO system_settings(key,value) VALUES ('audit_retention_days','365');\0",
    )
};
static mut migration_0055_down: [::core::ffi::c_char; 304] = unsafe {
    ::core::mem::transmute::<
        [u8; 304],
        [::core::ffi::c_char; 304],
    >(
        *b"DELETE FROM system_settings WHERE key='audit_retention_days';\nDROP INDEX IF EXISTS idx_audit_events_target;\nDROP INDEX IF EXISTS idx_audit_events_action_outcome;\nDROP INDEX IF EXISTS idx_audit_events_principal;\nDROP INDEX IF EXISTS idx_audit_events_occurred;\nDROP TABLE IF EXISTS audit_events;\nSELECT 1;\0",
    )
};
static mut migration_0056_up: [::core::ffi::c_char; 1210] = unsafe {
    ::core::mem::transmute::<
        [u8; 1210],
        [::core::ffi::c_char; 1210],
    >(
        *b"ALTER TABLE authz_actions ADD COLUMN bit_index INTEGER NOT NULL DEFAULT -1;\nUPDATE authz_actions SET bit_index = CASE action_key WHEN 'live.view' THEN 0 WHEN 'audio.listen' THEN 1 WHEN 'audio.talk' THEN 2 WHEN 'recordings.replay' THEN 3 WHEN 'recordings.export' THEN 4 WHEN 'snapshot.create' THEN 5 WHEN 'ptz.control' THEN 6 WHEN 'evidence.protect' THEN 7 WHEN 'recording.delete' THEN 8 WHEN 'camera.configure' THEN 9 WHEN 'fleet.execute_job' THEN 10 WHEN 'storage.configure' THEN 11 WHEN 'events.configure' THEN 12 WHEN 'users.manage' THEN 13 WHEN 'system.admin' THEN 14 ELSE -1 END;\nCREATE UNIQUE INDEX idx_authz_actions_bit_index ON authz_actions(bit_index);\nCREATE TRIGGER trg_streams_camera_uuid_insert BEFORE INSERT ON streams FOR EACH ROW WHEN NEW.camera_uuid IS NULL OR NEW.camera_uuid = '' BEGIN SELECT RAISE(ABORT, 'streams.camera_uuid must be a generated UUID'); END;\nCREATE TRIGGER trg_streams_camera_uuid_update BEFORE UPDATE OF camera_uuid ON streams FOR EACH ROW WHEN NEW.camera_uuid IS NULL OR NEW.camera_uuid = '' BEGIN SELECT RAISE(ABORT, 'streams.camera_uuid must be a generated UUID'); END;\nINSERT OR IGNORE INTO system_settings (key, value) VALUES ('camera_tags_backfill_completed', '0');\0",
    )
};
static mut migration_0056_down: [::core::ffi::c_char; 244] = unsafe {
    ::core::mem::transmute::<
        [u8; 244],
        [::core::ffi::c_char; 244],
    >(
        *b"DROP TRIGGER IF EXISTS trg_streams_camera_uuid_update;\nDROP TRIGGER IF EXISTS trg_streams_camera_uuid_insert;\nDROP INDEX IF EXISTS idx_authz_actions_bit_index;\nDELETE FROM system_settings WHERE key = 'camera_tags_backfill_completed';\nSELECT 1;\0",
    )
};
static mut migration_0057_up: [::core::ffi::c_char; 1294] = unsafe {
    ::core::mem::transmute::<
        [u8; 1294],
        [::core::ffi::c_char; 1294],
    >(
        *b"CREATE TABLE event_outbox (id INTEGER PRIMARY KEY AUTOINCREMENT, event_id TEXT NOT NULL, event_source TEXT NOT NULL, event_type TEXT NOT NULL, subject TEXT NOT NULL, destination TEXT NOT NULL, topic TEXT NOT NULL, envelope_json TEXT NOT NULL, envelope_bytes INTEGER NOT NULL CHECK (envelope_bytes>0), severity INTEGER NOT NULL CHECK (severity BETWEEN 0 AND 3), state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending','delivering','delivered','dead')), attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count>=0), next_attempt_at INTEGER NOT NULL, expires_at INTEGER NOT NULL, created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), last_attempt_at INTEGER, lease_expires_at INTEGER, delivered_at INTEGER, dead_at INTEGER, last_error TEXT NOT NULL DEFAULT '', UNIQUE(event_source,event_id,destination));\nCREATE INDEX idx_event_outbox_due ON event_outbox(destination,state,next_attempt_at,severity DESC,id) WHERE state='pending';\nCREATE INDEX idx_event_outbox_lease ON event_outbox(state,lease_expires_at) WHERE state='delivering';\nCREATE INDEX idx_event_outbox_expiry ON event_outbox(state,expires_at);\nCREATE INDEX idx_event_outbox_terminal ON event_outbox(state,updated_at,id) WHERE state IN ('delivered','dead');\0",
    )
};
static mut migration_0057_down: [::core::ffi::c_char; 227] = unsafe {
    ::core::mem::transmute::<
        [u8; 227],
        [::core::ffi::c_char; 227],
    >(
        *b"DROP INDEX IF EXISTS idx_event_outbox_terminal;\nDROP INDEX IF EXISTS idx_event_outbox_expiry;\nDROP INDEX IF EXISTS idx_event_outbox_lease;\nDROP INDEX IF EXISTS idx_event_outbox_due;\nDROP TABLE IF EXISTS event_outbox;\nSELECT 1;\0",
    )
};
static mut migration_0058_up: [::core::ffi::c_char; 1596] = unsafe {
    ::core::mem::transmute::<
        [u8; 1596],
        [::core::ffi::c_char; 1596],
    >(
        *b"CREATE TABLE event_routes (id INTEGER PRIMARY KEY AUTOINCREMENT, uuid TEXT NOT NULL UNIQUE, name TEXT NOT NULL, description TEXT NOT NULL DEFAULT '', enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0,1)), destination_key TEXT NOT NULL DEFAULT 'mqtt:default', scope_type TEXT NOT NULL DEFAULT 'all' CHECK (scope_type IN ('all','selector')), selector_json TEXT, predicate_json TEXT NOT NULL DEFAULT '{\"version\":1}', schedule_json TEXT NOT NULL DEFAULT '{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}', debounce_seconds INTEGER NOT NULL DEFAULT 0 CHECK (debounce_seconds BETWEEN 0 AND 86400), cooldown_seconds INTEGER NOT NULL DEFAULT 0 CHECK (cooldown_seconds BETWEEN 0 AND 604800), grouping_window_seconds INTEGER NOT NULL DEFAULT 0 CHECK (grouping_window_seconds BETWEEN 0 AND 3600), max_events_per_minute INTEGER NOT NULL DEFAULT 0 CHECK (max_events_per_minute BETWEEN 0 AND 60000), revision INTEGER NOT NULL DEFAULT 1 CHECK (revision>=1), created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')), CHECK ((scope_type='all' AND selector_json IS NULL) OR (scope_type='selector' AND selector_json IS NOT NULL)));\nCREATE UNIQUE INDEX idx_event_routes_name ON event_routes(name COLLATE NOCASE);\nCREATE INDEX idx_event_routes_delivery ON event_routes(enabled,destination_key,uuid);\nCREATE TABLE event_route_types (route_uuid TEXT NOT NULL REFERENCES event_routes(uuid) ON DELETE CASCADE, event_type TEXT NOT NULL, PRIMARY KEY(route_uuid,event_type));\nCREATE INDEX idx_event_route_types_type ON event_route_types(event_type,route_uuid);\0",
    )
};
static mut migration_0058_down: [::core::ffi::c_char; 226] = unsafe {
    ::core::mem::transmute::<
        [u8; 226],
        [::core::ffi::c_char; 226],
    >(
        *b"DROP INDEX IF EXISTS idx_event_route_types_type;\nDROP TABLE IF EXISTS event_route_types;\nDROP INDEX IF EXISTS idx_event_routes_delivery;\nDROP INDEX IF EXISTS idx_event_routes_name;\nDROP TABLE IF EXISTS event_routes;\nSELECT 1;\0",
    )
};
static mut migration_0059_up: [::core::ffi::c_char; 988] = unsafe {
    ::core::mem::transmute::<
        [u8; 988],
        [::core::ffi::c_char; 988],
    >(
        *b"CREATE TABLE event_route_suppression_state (route_uuid TEXT NOT NULL REFERENCES event_routes(uuid) ON DELETE CASCADE, event_type TEXT NOT NULL, subject TEXT NOT NULL, last_observed_at INTEGER NOT NULL DEFAULT 0 CHECK(last_observed_at>=0), last_allowed_at INTEGER NOT NULL DEFAULT 0 CHECK(last_allowed_at>=0), rate_window_started_at INTEGER NOT NULL DEFAULT 0 CHECK(rate_window_started_at>=0), rate_window_count INTEGER NOT NULL DEFAULT 0 CHECK(rate_window_count>=0), group_started_at INTEGER NOT NULL DEFAULT 0 CHECK(group_started_at>=0), suppressed_count INTEGER NOT NULL DEFAULT 0 CHECK(suppressed_count>=0), last_allowed_event_id TEXT NOT NULL DEFAULT '', last_reason TEXT NOT NULL DEFAULT 'allowed' CHECK(last_reason IN ('allowed','debounce','cooldown','grouping','rate')), updated_at INTEGER NOT NULL DEFAULT(strftime('%s','now')), PRIMARY KEY(route_uuid,event_type,subject));\nCREATE INDEX idx_event_route_suppression_updated ON event_route_suppression_state(updated_at,route_uuid);\0",
    )
};
static mut migration_0059_down: [::core::ffi::c_char; 120] = unsafe {
    ::core::mem::transmute::<
        [u8; 120],
        [::core::ffi::c_char; 120],
    >(
        *b"DROP INDEX IF EXISTS idx_event_route_suppression_updated;\nDROP TABLE IF EXISTS event_route_suppression_state;\nSELECT 1;\0",
    )
};
static mut migration_0060_up: [::core::ffi::c_char; 1319] = unsafe {
    ::core::mem::transmute::<
        [u8; 1319],
        [::core::ffi::c_char; 1319],
    >(
        *b"CREATE TABLE event_destinations (uuid TEXT PRIMARY KEY, name TEXT NOT NULL, description TEXT NOT NULL DEFAULT '', enabled INTEGER NOT NULL DEFAULT 1 CHECK(enabled IN (0,1)), destination_type TEXT NOT NULL DEFAULT 'mqtt' CHECK(destination_type='mqtt'), broker_host TEXT NOT NULL, broker_port INTEGER NOT NULL CHECK(broker_port BETWEEN 1 AND 65535), client_id TEXT NOT NULL, topic_template TEXT NOT NULL, username TEXT NOT NULL DEFAULT '', password TEXT NOT NULL DEFAULT '', tls_mode TEXT NOT NULL DEFAULT 'system' CHECK(tls_mode IN ('disabled','system','custom_ca','mutual')), ca_file TEXT NOT NULL DEFAULT '', cert_file TEXT NOT NULL DEFAULT '', key_file TEXT NOT NULL DEFAULT '', keepalive_seconds INTEGER NOT NULL DEFAULT 60 CHECK(keepalive_seconds BETWEEN 5 AND 3600), qos INTEGER NOT NULL DEFAULT 1 CHECK(qos BETWEEN 0 AND 2), revision INTEGER NOT NULL DEFAULT 1 CHECK(revision>=1), created_at INTEGER NOT NULL DEFAULT(strftime('%s','now')), updated_at INTEGER NOT NULL DEFAULT(strftime('%s','now')));\nCREATE UNIQUE INDEX idx_event_destinations_name ON event_destinations(name COLLATE NOCASE);\nCREATE UNIQUE INDEX idx_event_destinations_client ON event_destinations(broker_host COLLATE NOCASE,broker_port,client_id);\nCREATE INDEX idx_event_destinations_enabled ON event_destinations(enabled,destination_type,uuid);\0",
    )
};
static mut migration_0060_down: [::core::ffi::c_char; 206] = unsafe {
    ::core::mem::transmute::<
        [u8; 206],
        [::core::ffi::c_char; 206],
    >(
        *b"DROP INDEX IF EXISTS idx_event_destinations_enabled;\nDROP INDEX IF EXISTS idx_event_destinations_client;\nDROP INDEX IF EXISTS idx_event_destinations_name;\nDROP TABLE IF EXISTS event_destinations;\nSELECT 1;\0",
    )
};
static mut migration_0061_up: [::core::ffi::c_char; 822] = unsafe {
    ::core::mem::transmute::<
        [u8; 822],
        [::core::ffi::c_char; 822],
    >(
        *b"ALTER TABLE recordings ADD COLUMN camera_uuid TEXT;\nALTER TABLE detections ADD COLUMN camera_uuid TEXT;\nUPDATE recordings SET camera_uuid = (SELECT streams.camera_uuid FROM streams WHERE streams.name = recordings.stream_name) WHERE camera_uuid IS NULL AND EXISTS (SELECT 1 FROM streams WHERE streams.name = recordings.stream_name);\nUPDATE detections SET camera_uuid = COALESCE((SELECT recordings.camera_uuid FROM recordings WHERE recordings.id = detections.recording_id), (SELECT streams.camera_uuid FROM streams WHERE streams.name = detections.stream_name)) WHERE camera_uuid IS NULL;\nCREATE INDEX idx_recordings_camera_time ON recordings(camera_uuid, start_time, end_time) WHERE camera_uuid IS NOT NULL;\nCREATE INDEX idx_detections_camera_time_id ON detections(camera_uuid, timestamp, id) WHERE camera_uuid IS NOT NULL;\0",
    )
};
static mut migration_0061_down: [::core::ffi::c_char; 111] = unsafe {
    ::core::mem::transmute::<
        [u8; 111],
        [::core::ffi::c_char; 111],
    >(
        *b"DROP INDEX IF EXISTS idx_detections_camera_time_id;\nDROP INDEX IF EXISTS idx_recordings_camera_time;\nSELECT 1;\0",
    )
};
static mut migration_0062_up: [::core::ffi::c_char; 438] = unsafe {
    ::core::mem::transmute::<
        [u8; 438],
        [::core::ffi::c_char; 438],
    >(
        *b"CREATE INDEX idx_detections_camera_label_time_id ON detections(camera_uuid, label, timestamp DESC, id DESC) WHERE camera_uuid IS NOT NULL;\nCREATE INDEX idx_detections_camera_zone_time_id ON detections(camera_uuid, zone_id, timestamp DESC, id DESC) WHERE camera_uuid IS NOT NULL AND zone_id != '';\nCREATE INDEX idx_detections_camera_source_time_id ON detections(camera_uuid, source, timestamp DESC, id DESC) WHERE camera_uuid IS NOT NULL;\0",
    )
};
static mut migration_0062_down: [::core::ffi::c_char; 174] = unsafe {
    ::core::mem::transmute::<
        [u8; 174],
        [::core::ffi::c_char; 174],
    >(
        *b"DROP INDEX IF EXISTS idx_detections_camera_source_time_id;\nDROP INDEX IF EXISTS idx_detections_camera_zone_time_id;\nDROP INDEX IF EXISTS idx_detections_camera_label_time_id;\0",
    )
};
static mut migration_0063_up: [::core::ffi::c_char; 117] = unsafe {
    ::core::mem::transmute::<
        [u8; 117],
        [::core::ffi::c_char; 117],
    >(
        *b"ALTER TABLE users ADD COLUMN must_change_password INTEGER NOT NULL DEFAULT 0 CHECK (must_change_password IN (0, 1));\0",
    )
};
static mut migration_0063_down: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SELECT 1;\0") };
static mut migration_0064_up: [::core::ffi::c_char; 1105] = unsafe {
    ::core::mem::transmute::<
        [u8; 1105],
        [::core::ffi::c_char; 1105],
    >(
        *b"CREATE TABLE investigation_bookmarks (uuid TEXT PRIMARY KEY,owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,title TEXT NOT NULL,note TEXT NOT NULL DEFAULT '',start_time INTEGER NOT NULL,end_time INTEGER NOT NULL,cursor_time INTEGER NOT NULL,primary_camera_uuid TEXT NOT NULL,filters_json TEXT NOT NULL DEFAULT '{}',representative_result_json TEXT,revision INTEGER NOT NULL DEFAULT 1,created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')) ,updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')) ,CHECK (end_time > start_time),CHECK (cursor_time >= start_time AND cursor_time <= end_time));CREATE INDEX idx_investigation_bookmarks_owner_updated ON investigation_bookmarks(owner_user_id,updated_at DESC,uuid);CREATE TABLE investigation_bookmark_cameras (bookmark_uuid TEXT NOT NULL REFERENCES investigation_bookmarks(uuid) ON DELETE CASCADE,camera_uuid TEXT NOT NULL,sort_order INTEGER NOT NULL,PRIMARY KEY (bookmark_uuid,camera_uuid),UNIQUE (bookmark_uuid,sort_order));CREATE INDEX idx_investigation_bookmark_cameras_camera ON investigation_bookmark_cameras(camera_uuid,bookmark_uuid);\0",
    )
};
static mut migration_0064_down: [::core::ffi::c_char; 233] = unsafe {
    ::core::mem::transmute::<
        [u8; 233],
        [::core::ffi::c_char; 233],
    >(
        *b"DROP INDEX IF EXISTS idx_investigation_bookmark_cameras_camera;DROP TABLE IF EXISTS investigation_bookmark_cameras;DROP INDEX IF EXISTS idx_investigation_bookmarks_owner_updated;DROP TABLE IF EXISTS investigation_bookmarks;SELECT 1;\0",
    )
};
static mut migration_0065_up: [::core::ffi::c_char; 3353] = unsafe {
    ::core::mem::transmute::<
        [u8; 3353],
        [::core::ffi::c_char; 3353],
    >(
        *b"CREATE TABLE IF NOT EXISTS storage_targets(uuid TEXT PRIMARY KEY,name TEXT NOT NULL COLLATE NOCASE,target_type TEXT NOT NULL DEFAULT 'filesystem' CHECK(target_type IN('filesystem')),root_path TEXT NOT NULL,enabled INTEGER NOT NULL DEFAULT 1 CHECK(enabled IN(0,1)),is_default INTEGER NOT NULL DEFAULT 0 CHECK(is_default IN(0,1)),storage_class TEXT NOT NULL DEFAULT 'hot' CHECK(storage_class IN('hot','warm','cold')),reserve_bytes INTEGER NOT NULL DEFAULT 0 CHECK(reserve_bytes>=0),high_watermark_pct REAL NOT NULL DEFAULT 90.0 CHECK(high_watermark_pct>0 AND high_watermark_pct<100),low_watermark_pct REAL NOT NULL DEFAULT 80.0 CHECK(low_watermark_pct>=0 AND low_watermark_pct<high_watermark_pct),health_status TEXT NOT NULL DEFAULT 'unknown' CHECK(health_status IN('unknown','healthy','degraded','unavailable','disabled')),capacity_bytes INTEGER NOT NULL DEFAULT 0,available_bytes INTEGER NOT NULL DEFAULT 0,filesystem_device INTEGER NOT NULL DEFAULT 0,last_probe_at INTEGER,last_success_at INTEGER,last_error TEXT NOT NULL DEFAULT '',recording_count INTEGER NOT NULL DEFAULT 0 CHECK(recording_count>=0),recording_bytes INTEGER NOT NULL DEFAULT 0 CHECK(recording_bytes>=0),revision INTEGER NOT NULL DEFAULT 1,created_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),updated_at INTEGER NOT NULL DEFAULT(strftime('%s','now')));CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_targets_name ON storage_targets(name COLLATE NOCASE);CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_targets_root_path ON storage_targets(root_path);CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_targets_one_default ON storage_targets(is_default) WHERE is_default=1;CREATE INDEX IF NOT EXISTS idx_storage_targets_enabled ON storage_targets(enabled,name COLLATE NOCASE);ALTER TABLE recordings ADD COLUMN storage_target_uuid TEXT REFERENCES storage_targets(uuid) ON DELETE SET NULL;ALTER TABLE recordings ADD COLUMN object_key TEXT;ALTER TABLE recordings ADD COLUMN placement_reason TEXT;ALTER TABLE recordings ADD COLUMN storage_policy_version INTEGER;CREATE INDEX IF NOT EXISTS idx_recordings_storage_target_time ON recordings(storage_target_uuid,start_time DESC);CREATE TRIGGER IF NOT EXISTS trg_recordings_storage_target_insert AFTER INSERT ON recordings FOR EACH ROW WHEN NEW.storage_target_uuid IS NOT NULL BEGIN UPDATE storage_targets SET recording_count=recording_count+1,recording_bytes=recording_bytes+MAX(COALESCE(NEW.size_bytes,0),0) WHERE uuid=NEW.storage_target_uuid;END;CREATE TRIGGER IF NOT EXISTS trg_recordings_storage_target_delete AFTER DELETE ON recordings FOR EACH ROW WHEN OLD.storage_target_uuid IS NOT NULL BEGIN UPDATE storage_targets SET recording_count=MAX(recording_count-1,0),recording_bytes=MAX(recording_bytes-MAX(COALESCE(OLD.size_bytes,0),0),0) WHERE uuid=OLD.storage_target_uuid;END;CREATE TRIGGER IF NOT EXISTS trg_recordings_storage_target_update AFTER UPDATE OF storage_target_uuid,size_bytes ON recordings FOR EACH ROW BEGIN UPDATE storage_targets SET recording_count=MAX(recording_count-1,0),recording_bytes=MAX(recording_bytes-MAX(COALESCE(OLD.size_bytes,0),0),0) WHERE OLD.storage_target_uuid IS NOT NULL AND uuid=OLD.storage_target_uuid;UPDATE storage_targets SET recording_count=recording_count+1,recording_bytes=recording_bytes+MAX(COALESCE(NEW.size_bytes,0),0) WHERE NEW.storage_target_uuid IS NOT NULL AND uuid=NEW.storage_target_uuid;END;\0",
    )
};
static mut migration_0065_down: [::core::ffi::c_char; 473] = unsafe {
    ::core::mem::transmute::<
        [u8; 473],
        [::core::ffi::c_char; 473],
    >(
        *b"DROP TRIGGER IF EXISTS trg_recordings_storage_target_update;DROP TRIGGER IF EXISTS trg_recordings_storage_target_delete;DROP TRIGGER IF EXISTS trg_recordings_storage_target_insert;DROP INDEX IF EXISTS idx_recordings_storage_target_time;DROP INDEX IF EXISTS idx_storage_targets_enabled;DROP INDEX IF EXISTS idx_storage_targets_one_default;DROP INDEX IF EXISTS idx_storage_targets_root_path;DROP INDEX IF EXISTS idx_storage_targets_name;DROP TABLE IF EXISTS storage_targets;\0",
    )
};
static mut migration_0066_up: [::core::ffi::c_char; 1278] = unsafe {
    ::core::mem::transmute::<
        [u8; 1278],
        [::core::ffi::c_char; 1278],
    >(
        *b"ALTER TABLE storage_targets ADD COLUMN mount_required INTEGER NOT NULL DEFAULT 0 CHECK(mount_required IN(0,1));ALTER TABLE storage_targets ADD COLUMN mount_guard_path TEXT NOT NULL DEFAULT '';CREATE TABLE IF NOT EXISTS storage_policies(uuid TEXT PRIMARY KEY,name TEXT NOT NULL COLLATE NOCASE,enabled INTEGER NOT NULL DEFAULT 1 CHECK(enabled IN(0,1)),priority INTEGER NOT NULL DEFAULT 100,selector_json TEXT NOT NULL,primary_target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,fallback_mode TEXT NOT NULL DEFAULT 'default' CHECK(fallback_mode IN('default','target','pause','fail')),fallback_target_uuid TEXT REFERENCES storage_targets(uuid) ON DELETE RESTRICT,revision INTEGER NOT NULL DEFAULT 1,created_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),updated_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),CHECK((fallback_mode='target' AND fallback_target_uuid IS NOT NULL) OR(fallback_mode!='target' AND fallback_target_uuid IS NULL)),CHECK(fallback_target_uuid IS NULL OR fallback_target_uuid!=primary_target_uuid));CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_policies_name ON storage_policies(name COLLATE NOCASE);CREATE INDEX IF NOT EXISTS idx_storage_policies_evaluation ON storage_policies(enabled,priority DESC,name COLLATE NOCASE,uuid);\0",
    )
};
static mut migration_0066_down: [::core::ffi::c_char; 139] = unsafe {
    ::core::mem::transmute::<
        [u8; 139],
        [::core::ffi::c_char; 139],
    >(
        *b"DROP INDEX IF EXISTS idx_storage_policies_evaluation;DROP INDEX IF EXISTS idx_storage_policies_name;DROP TABLE IF EXISTS storage_policies;\0",
    )
};
static mut migration_0067_up: [::core::ffi::c_char; 933] = unsafe {
    ::core::mem::transmute::<
        [u8; 933],
        [::core::ffi::c_char; 933],
    >(
        *b"CREATE TABLE fleet_saved_views(uuid TEXT PRIMARY KEY,owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,name TEXT NOT NULL COLLATE NOCASE,is_shared INTEGER NOT NULL DEFAULT 0 CHECK(is_shared IN(0,1)),selector_json TEXT NOT NULL,search_text TEXT NOT NULL DEFAULT '',collection_uuid TEXT,columns_json TEXT NOT NULL DEFAULT '[]',sort_by TEXT NOT NULL DEFAULT 'name' CHECK(sort_by IN('name','camera_uuid','location','health','enabled','recording_mode','address')),sort_order TEXT NOT NULL DEFAULT 'asc' CHECK(sort_order IN('asc','desc')),revision INTEGER NOT NULL DEFAULT 1,created_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),updated_at INTEGER NOT NULL DEFAULT(strftime('%s','now')));CREATE UNIQUE INDEX idx_fleet_saved_views_owner_name ON fleet_saved_views(COALESCE(owner_user_id,0),name COLLATE NOCASE);CREATE INDEX idx_fleet_saved_views_visible ON fleet_saved_views(is_shared,owner_user_id,name COLLATE NOCASE,uuid);\0",
    )
};
static mut migration_0067_down: [::core::ffi::c_char; 145] = unsafe {
    ::core::mem::transmute::<
        [u8; 145],
        [::core::ffi::c_char; 145],
    >(
        *b"DROP INDEX IF EXISTS idx_fleet_saved_views_visible;DROP INDEX IF EXISTS idx_fleet_saved_views_owner_name;DROP TABLE IF EXISTS fleet_saved_views;\0",
    )
};
static mut migration_0068_up: [::core::ffi::c_char; 1729] = unsafe {
    ::core::mem::transmute::<
        [u8; 1729],
        [::core::ffi::c_char; 1729],
    >(
        *b"CREATE TABLE storage_migration_jobs(uuid TEXT PRIMARY KEY,recording_id INTEGER NOT NULL REFERENCES recordings(id) ON DELETE CASCADE,owner_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,operation TEXT NOT NULL DEFAULT 'move' CHECK(operation IN('move')),source_target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,source_object_key TEXT NOT NULL,destination_target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,destination_object_key TEXT NOT NULL,state TEXT NOT NULL DEFAULT 'queued' CHECK(state IN('queued','copying','verifying','committing','cleanup_pending','retry_wait','completed','failed','cancelled')),checksum_mode TEXT NOT NULL DEFAULT 'sha256' CHECK(checksum_mode IN('sha256')),checksum TEXT NOT NULL DEFAULT '',bytes_total INTEGER NOT NULL DEFAULT 0 CHECK(bytes_total>=0),bytes_copied INTEGER NOT NULL DEFAULT 0 CHECK(bytes_copied>=0),attempt_count INTEGER NOT NULL DEFAULT 0 CHECK(attempt_count>=0),max_attempts INTEGER NOT NULL DEFAULT 5 CHECK(max_attempts BETWEEN 1 AND 20),next_attempt_at INTEGER NOT NULL DEFAULT 0,last_error TEXT NOT NULL DEFAULT '',revision INTEGER NOT NULL DEFAULT 1,created_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),updated_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),started_at INTEGER,completed_at INTEGER,CHECK(source_target_uuid<>destination_target_uuid));CREATE UNIQUE INDEX idx_storage_migration_one_active_recording ON storage_migration_jobs(recording_id) WHERE state NOT IN('completed','failed','cancelled');CREATE INDEX idx_storage_migration_due ON storage_migration_jobs(state,next_attempt_at,created_at);CREATE INDEX idx_storage_migration_destination ON storage_migration_jobs(destination_target_uuid,state);\0",
    )
};
static mut migration_0068_down: [::core::ffi::c_char; 211] = unsafe {
    ::core::mem::transmute::<
        [u8; 211],
        [::core::ffi::c_char; 211],
    >(
        *b"DROP INDEX IF EXISTS idx_storage_migration_destination;DROP INDEX IF EXISTS idx_storage_migration_due;DROP INDEX IF EXISTS idx_storage_migration_one_active_recording;DROP TABLE IF EXISTS storage_migration_jobs;\0",
    )
};
static mut migration_0069_up: [::core::ffi::c_char; 186] = unsafe {
    ::core::mem::transmute::<
        [u8; 186],
        [::core::ffi::c_char; 186],
    >(
        *b"ALTER TABLE streams ADD COLUMN playback_transport TEXT NOT NULL DEFAULT 'auto' CHECK(playback_transport IN('auto','webrtc_only','mse_only','hls_only','webrtc_then_mse','mse_then_hls'));\0",
    )
};
static mut migration_0069_down: [::core::ffi::c_char; 80] = unsafe {
    ::core::mem::transmute::<[u8; 80], [::core::ffi::c_char; 80]>(
        *b"-- SQLite cannot drop a column while retaining compatibility with older builds.\0",
    )
};
static mut migration_0070_up: [::core::ffi::c_char; 2259] = unsafe {
    ::core::mem::transmute::<
        [u8; 2259],
        [::core::ffi::c_char; 2259],
    >(
        *b"CREATE TABLE onvif_discovery_inventory(uuid TEXT PRIMARY KEY,endpoint TEXT NOT NULL,device_service TEXT NOT NULL,media_service TEXT NOT NULL DEFAULT '',ptz_service TEXT NOT NULL DEFAULT '',imaging_service TEXT NOT NULL DEFAULT '',manufacturer TEXT NOT NULL DEFAULT '',model TEXT NOT NULL DEFAULT '',firmware_version TEXT NOT NULL DEFAULT '',serial_number TEXT NOT NULL DEFAULT '',hardware_id TEXT NOT NULL DEFAULT '',ip_address TEXT NOT NULL DEFAULT '',mac_address TEXT NOT NULL DEFAULT '' COLLATE NOCASE,first_seen_at INTEGER NOT NULL,last_seen_at INTEGER NOT NULL,last_scan_network TEXT NOT NULL DEFAULT 'auto',online INTEGER NOT NULL DEFAULT 1 CHECK(online IN(0,1)),claim_state TEXT NOT NULL DEFAULT 'unclaimed' CHECK(claim_state IN('unclaimed','claimed','ignored')),claimed_camera_uuid TEXT REFERENCES streams(camera_uuid) ON DELETE SET NULL,duplicate_suspected INTEGER NOT NULL DEFAULT 0 CHECK(duplicate_suspected IN(0,1)),revision INTEGER NOT NULL DEFAULT 1 CHECK(revision>=1),updated_at INTEGER NOT NULL DEFAULT(strftime('%s','now')));CREATE INDEX idx_onvif_inventory_seen ON onvif_discovery_inventory(last_seen_at DESC,uuid);CREATE INDEX idx_onvif_inventory_serial ON onvif_discovery_inventory(serial_number) WHERE serial_number!='';CREATE INDEX idx_onvif_inventory_mac ON onvif_discovery_inventory(mac_address) WHERE mac_address!='';CREATE INDEX idx_onvif_inventory_endpoint ON onvif_discovery_inventory(endpoint);CREATE INDEX idx_onvif_inventory_claim ON onvif_discovery_inventory(claim_state,online,last_seen_at DESC);CREATE TABLE onvif_discovery_addresses(inventory_uuid TEXT NOT NULL REFERENCES onvif_discovery_inventory(uuid) ON DELETE CASCADE,address_type TEXT NOT NULL CHECK(address_type IN('endpoint','service','ip')),address TEXT NOT NULL,first_seen_at INTEGER NOT NULL,last_seen_at INTEGER NOT NULL,PRIMARY KEY(inventory_uuid,address_type,address));CREATE INDEX idx_onvif_discovery_addresses_address ON onvif_discovery_addresses(address,inventory_uuid);CREATE TRIGGER trg_onvif_inventory_unclaim_deleted_stream BEFORE DELETE ON streams FOR EACH ROW BEGIN UPDATE onvif_discovery_inventory SET claim_state='unclaimed',claimed_camera_uuid=NULL,revision=revision+1,updated_at=strftime('%s','now') WHERE claimed_camera_uuid=OLD.camera_uuid;END;\0",
    )
};
static mut migration_0070_down: [::core::ffi::c_char; 456] = unsafe {
    ::core::mem::transmute::<
        [u8; 456],
        [::core::ffi::c_char; 456],
    >(
        *b"DROP TRIGGER IF EXISTS trg_onvif_inventory_unclaim_deleted_stream;DROP INDEX IF EXISTS idx_onvif_discovery_addresses_address;DROP TABLE IF EXISTS onvif_discovery_addresses;DROP INDEX IF EXISTS idx_onvif_inventory_claim;DROP INDEX IF EXISTS idx_onvif_inventory_endpoint;DROP INDEX IF EXISTS idx_onvif_inventory_mac;DROP INDEX IF EXISTS idx_onvif_inventory_serial;DROP INDEX IF EXISTS idx_onvif_inventory_seen;DROP TABLE IF EXISTS onvif_discovery_inventory;\0",
    )
};
static mut migration_0071_up: [::core::ffi::c_char; 5394] = unsafe {
    ::core::mem::transmute::<
        [u8; 5394],
        [::core::ffi::c_char; 5394],
    >(
        *b"ALTER TABLE storage_targets ADD COLUMN migration_bandwidth_bps INTEGER NOT NULL DEFAULT 0\n    CHECK (migration_bandwidth_bps = 0 OR migration_bandwidth_bps >= 65536);\nALTER TABLE storage_targets ADD COLUMN archival_window_start_minute INTEGER NOT NULL DEFAULT 0\n    CHECK (archival_window_start_minute BETWEEN 0 AND 1439);\nALTER TABLE storage_targets ADD COLUMN archival_window_end_minute INTEGER NOT NULL DEFAULT 0\n    CHECK (archival_window_end_minute BETWEEN 0 AND 1439);\nALTER TABLE storage_targets ADD COLUMN replica_count INTEGER NOT NULL DEFAULT 0\n    CHECK (replica_count >= 0);\nALTER TABLE storage_targets ADD COLUMN replica_bytes INTEGER NOT NULL DEFAULT 0\n    CHECK (replica_bytes >= 0);\n\nCREATE TABLE storage_migration_jobs_v2 (\n    uuid TEXT PRIMARY KEY,\n    recording_id INTEGER NOT NULL\n        REFERENCES recordings(id) ON DELETE CASCADE,\n    owner_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,\n    operation TEXT NOT NULL DEFAULT 'move'\n        CHECK (operation IN ('move', 'copy')),\n    source_target_uuid TEXT NOT NULL\n        REFERENCES storage_targets(uuid) ON DELETE RESTRICT,\n    source_object_key TEXT NOT NULL,\n    destination_target_uuid TEXT NOT NULL\n        REFERENCES storage_targets(uuid) ON DELETE RESTRICT,\n    destination_object_key TEXT NOT NULL,\n    state TEXT NOT NULL DEFAULT 'queued'\n        CHECK (state IN ('queued', 'copying', 'verifying', 'committing',\n                         'cleanup_pending', 'retry_wait', 'completed',\n                         'failed', 'cancelled')),\n    checksum_mode TEXT NOT NULL DEFAULT 'sha256'\n        CHECK (checksum_mode IN ('sha256')),\n    checksum TEXT NOT NULL DEFAULT '',\n    bytes_total INTEGER NOT NULL DEFAULT 0 CHECK (bytes_total >= 0),\n    bytes_copied INTEGER NOT NULL DEFAULT 0 CHECK (bytes_copied >= 0),\n    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),\n    max_attempts INTEGER NOT NULL DEFAULT 5 CHECK (max_attempts BETWEEN 1 AND 20),\n    next_attempt_at INTEGER NOT NULL DEFAULT 0,\n    last_error TEXT NOT NULL DEFAULT '',\n    cancel_requested INTEGER NOT NULL DEFAULT 0 CHECK (cancel_requested IN (0, 1)),\n    bandwidth_limit_bps INTEGER NOT NULL DEFAULT 0\n        CHECK (bandwidth_limit_bps = 0 OR bandwidth_limit_bps >= 65536),\n    window_start_minute INTEGER NOT NULL DEFAULT 0\n        CHECK (window_start_minute BETWEEN 0 AND 1439),\n    window_end_minute INTEGER NOT NULL DEFAULT 0\n        CHECK (window_end_minute BETWEEN 0 AND 1439),\n    revision INTEGER NOT NULL DEFAULT 1,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    started_at INTEGER,\n    completed_at INTEGER,\n    CHECK (source_target_uuid <> destination_target_uuid)\n);\n\nINSERT INTO storage_migration_jobs_v2(\n    uuid, recording_id, owner_user_id, operation, source_target_uuid,\n    source_object_key, destination_target_uuid, destination_object_key, state,\n    checksum_mode, checksum, bytes_total, bytes_copied, attempt_count,\n    max_attempts, next_attempt_at, last_error, revision, created_at, updated_at,\n    started_at, completed_at)\nSELECT uuid, recording_id, owner_user_id, operation, source_target_uuid,\n       source_object_key, destination_target_uuid, destination_object_key, state,\n       checksum_mode, checksum, bytes_total, bytes_copied, attempt_count,\n       max_attempts, next_attempt_at, last_error, revision, created_at, updated_at,\n       started_at, completed_at\nFROM storage_migration_jobs;\n\nDROP TABLE storage_migration_jobs;\nALTER TABLE storage_migration_jobs_v2 RENAME TO storage_migration_jobs;\n\nCREATE UNIQUE INDEX idx_storage_migration_one_active_recording\n    ON storage_migration_jobs(recording_id)\n    WHERE state NOT IN ('completed', 'failed', 'cancelled');\nCREATE INDEX idx_storage_migration_due\n    ON storage_migration_jobs(state, next_attempt_at, created_at);\nCREATE INDEX idx_storage_migration_destination\n    ON storage_migration_jobs(destination_target_uuid, state);\n\nCREATE TABLE storage_recording_copies (\n    uuid TEXT PRIMARY KEY,\n    recording_id INTEGER NOT NULL\n        REFERENCES recordings(id) ON DELETE CASCADE,\n    target_uuid TEXT NOT NULL\n        REFERENCES storage_targets(uuid) ON DELETE RESTRICT,\n    object_key TEXT NOT NULL,\n    checksum_mode TEXT NOT NULL DEFAULT 'sha256'\n        CHECK (checksum_mode IN ('sha256')),\n    checksum TEXT NOT NULL,\n    size_bytes INTEGER NOT NULL CHECK (size_bytes >= 0),\n    verified_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    UNIQUE(recording_id, target_uuid),\n    UNIQUE(target_uuid, object_key)\n);\n\nCREATE INDEX idx_storage_recording_copies_recording\n    ON storage_recording_copies(recording_id);\nCREATE INDEX idx_storage_recording_copies_target\n    ON storage_recording_copies(target_uuid, recording_id);\n\nCREATE TRIGGER trg_storage_recording_copy_insert\nAFTER INSERT ON storage_recording_copies\nBEGIN\n    UPDATE storage_targets\n    SET replica_count = replica_count + 1,\n        replica_bytes = replica_bytes + NEW.size_bytes\n    WHERE uuid = NEW.target_uuid;\nEND;\n\nCREATE TRIGGER trg_storage_recording_copy_delete\nAFTER DELETE ON storage_recording_copies\nBEGIN\n    UPDATE storage_targets\n    SET replica_count = MAX(replica_count - 1, 0),\n        replica_bytes = MAX(replica_bytes - OLD.size_bytes, 0)\n    WHERE uuid = OLD.target_uuid;\nEND;\0",
    )
};
static mut migration_0071_down: [::core::ffi::c_char; 2946] = unsafe {
    ::core::mem::transmute::<
        [u8; 2946],
        [::core::ffi::c_char; 2946],
    >(
        *b"DROP TRIGGER IF EXISTS trg_storage_recording_copy_delete;\nDROP TRIGGER IF EXISTS trg_storage_recording_copy_insert;\nDROP INDEX IF EXISTS idx_storage_recording_copies_target;\nDROP INDEX IF EXISTS idx_storage_recording_copies_recording;\nDROP TABLE IF EXISTS storage_recording_copies;\n\nCREATE TABLE storage_migration_jobs_v1 (\n    uuid TEXT PRIMARY KEY,\n    recording_id INTEGER NOT NULL REFERENCES recordings(id) ON DELETE CASCADE,\n    owner_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,\n    operation TEXT NOT NULL DEFAULT 'move' CHECK (operation IN ('move')),\n    source_target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,\n    source_object_key TEXT NOT NULL,\n    destination_target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,\n    destination_object_key TEXT NOT NULL,\n    state TEXT NOT NULL DEFAULT 'queued'\n        CHECK (state IN ('queued', 'copying', 'verifying', 'committing',\n                         'cleanup_pending', 'retry_wait', 'completed',\n                         'failed', 'cancelled')),\n    checksum_mode TEXT NOT NULL DEFAULT 'sha256' CHECK (checksum_mode IN ('sha256')),\n    checksum TEXT NOT NULL DEFAULT '',\n    bytes_total INTEGER NOT NULL DEFAULT 0 CHECK (bytes_total >= 0),\n    bytes_copied INTEGER NOT NULL DEFAULT 0 CHECK (bytes_copied >= 0),\n    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),\n    max_attempts INTEGER NOT NULL DEFAULT 5 CHECK (max_attempts BETWEEN 1 AND 20),\n    next_attempt_at INTEGER NOT NULL DEFAULT 0,\n    last_error TEXT NOT NULL DEFAULT '',\n    revision INTEGER NOT NULL DEFAULT 1,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    started_at INTEGER,\n    completed_at INTEGER,\n    CHECK (source_target_uuid <> destination_target_uuid)\n);\nINSERT INTO storage_migration_jobs_v1\nSELECT uuid, recording_id, owner_user_id, operation, source_target_uuid,\n       source_object_key, destination_target_uuid, destination_object_key, state,\n       checksum_mode, checksum, bytes_total, bytes_copied, attempt_count,\n       max_attempts, next_attempt_at, last_error, revision, created_at, updated_at,\n       started_at, completed_at\nFROM storage_migration_jobs WHERE operation = 'move';\nDROP TABLE storage_migration_jobs;\nALTER TABLE storage_migration_jobs_v1 RENAME TO storage_migration_jobs;\nCREATE UNIQUE INDEX idx_storage_migration_one_active_recording\n    ON storage_migration_jobs(recording_id)\n    WHERE state NOT IN ('completed', 'failed', 'cancelled');\nCREATE INDEX idx_storage_migration_due\n    ON storage_migration_jobs(state, next_attempt_at, created_at);\nCREATE INDEX idx_storage_migration_destination\n    ON storage_migration_jobs(destination_target_uuid, state);\n\n-- Target control columns are intentionally retained on rollback; older code\n-- ignores additive columns and preserving them avoids a lossy table rebuild.\0",
    )
};
static mut migration_0072_up: [::core::ffi::c_char; 2963] = unsafe {
    ::core::mem::transmute::<
        [u8; 2963],
        [::core::ffi::c_char; 2963],
    >(
        *b"CREATE TABLE storage_pools (\n    uuid TEXT PRIMARY KEY,\n    name TEXT NOT NULL UNIQUE COLLATE NOCASE,\n    strategy TEXT NOT NULL DEFAULT 'most_free' CHECK (strategy IN ('most_free', 'round_robin', 'priority')),\n    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),\n    allocation_cursor INTEGER NOT NULL DEFAULT 0 CHECK (allocation_cursor >= 0),\n    revision INTEGER NOT NULL DEFAULT 1,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))\n);\nCREATE TABLE storage_pool_members (\n    pool_uuid TEXT NOT NULL REFERENCES storage_pools(uuid) ON DELETE CASCADE,\n    target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,\n    position INTEGER NOT NULL DEFAULT 0,\n    weight INTEGER NOT NULL DEFAULT 1 CHECK (weight BETWEEN 1 AND 1000),\n    PRIMARY KEY(pool_uuid, target_uuid)\n);\nCREATE INDEX idx_storage_pool_members_order ON storage_pool_members(pool_uuid, position, target_uuid);\nALTER TABLE storage_policies ADD COLUMN primary_pool_uuid TEXT REFERENCES storage_pools(uuid) ON DELETE RESTRICT;\nALTER TABLE storage_policies ADD COLUMN minimum_retention_days INTEGER NOT NULL DEFAULT 0 CHECK (minimum_retention_days BETWEEN 0 AND 36500);\nALTER TABLE storage_policies ADD COLUMN desired_retention_days INTEGER NOT NULL DEFAULT 0 CHECK (desired_retention_days BETWEEN 0 AND 36500);\nALTER TABLE storage_policies ADD COLUMN maximum_retention_days INTEGER NOT NULL DEFAULT 0 CHECK (maximum_retention_days BETWEEN 0 AND 36500);\nALTER TABLE storage_policies ADD COLUMN required_copy_count INTEGER NOT NULL DEFAULT 1 CHECK (required_copy_count BETWEEN 1 AND 8);\nALTER TABLE storage_policies ADD COLUMN replication_pool_uuid TEXT REFERENCES storage_pools(uuid) ON DELETE RESTRICT;\nALTER TABLE storage_policies ADD COLUMN migration_after_days INTEGER NOT NULL DEFAULT 0 CHECK (migration_after_days BETWEEN 0 AND 36500);\nALTER TABLE storage_policies ADD COLUMN migration_target_uuid TEXT REFERENCES storage_targets(uuid) ON DELETE RESTRICT;\nALTER TABLE storage_policies ADD COLUMN pressure_priority INTEGER NOT NULL DEFAULT 100 CHECK (pressure_priority BETWEEN -1000000 AND 1000000);\nCREATE TABLE storage_policy_violations (\n    uuid TEXT PRIMARY KEY,\n    policy_uuid TEXT NOT NULL REFERENCES storage_policies(uuid) ON DELETE CASCADE,\n    recording_id INTEGER REFERENCES recordings(id) ON DELETE CASCADE,\n    camera_uuid TEXT,\n    scope_key TEXT NOT NULL,\n    violation_type TEXT NOT NULL CHECK (violation_type IN ('copy_count', 'minimum_retention', 'target_unavailable', 'migration_failed')),\n    details TEXT NOT NULL DEFAULT '',\n    first_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    last_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),\n    resolved_at INTEGER,\n    UNIQUE(policy_uuid, scope_key, violation_type)\n);\nCREATE INDEX idx_storage_policy_violations_active ON storage_policy_violations(policy_uuid, resolved_at, violation_type);\0",
    )
};
static mut migration_0072_down: [::core::ffi::c_char; 107] = unsafe {
    ::core::mem::transmute::<
        [u8; 107],
        [::core::ffi::c_char; 107],
    >(
        *b"DROP INDEX IF EXISTS idx_storage_policy_violations_active;\nDROP TABLE IF EXISTS storage_policy_violations;\0",
    )
};
static mut migration_0073_up: [::core::ffi::c_char; 104] = unsafe {
    ::core::mem::transmute::<
        [u8; 104],
        [::core::ffi::c_char; 104],
    >(
        *b"ALTER TABLE streams ADD COLUMN eptz_config TEXT NOT NULL DEFAULT '' CHECK(length(eptz_config) <= 1023);\0",
    )
};
static mut migration_0073_down: [::core::ffi::c_char; 80] = unsafe {
    ::core::mem::transmute::<[u8; 80], [::core::ffi::c_char; 80]>(
        *b"-- SQLite cannot drop a column while retaining compatibility with older builds.\0",
    )
};
static mut migration_0074_up: [::core::ffi::c_char; 1284] = unsafe {
    ::core::mem::transmute::<
        [u8; 1284],
        [::core::ffi::c_char; 1284],
    >(
        *b"CREATE TABLE user_workspace_preferences (\n    owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,\n    workspace_key TEXT NOT NULL CHECK (workspace_key IN ('live.navigator','investigation')),\n    is_visible INTEGER NOT NULL CHECK (is_visible IN (0, 1)),\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))\n);\nCREATE UNIQUE INDEX idx_user_workspace_preferences_owner_key ON user_workspace_preferences(COALESCE(owner_user_id, 0), workspace_key);\nCREATE TABLE camera_observations (\n    camera_uuid TEXT PRIMARY KEY REFERENCES streams(camera_uuid) ON DELETE CASCADE,\n    first_video_at INTEGER NOT NULL DEFAULT 0,\n    last_video_at INTEGER NOT NULL DEFAULT 0,\n    last_recording_at INTEGER NOT NULL DEFAULT 0,\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))\n);\nCREATE INDEX idx_camera_observations_last_video ON camera_observations(last_video_at DESC, camera_uuid);\nINSERT INTO camera_observations(camera_uuid, first_video_at, last_video_at, last_recording_at) SELECT r.camera_uuid, MIN(r.start_time), MAX(COALESCE(r.end_time, r.start_time)), MAX(COALESCE(r.end_time, r.start_time)) FROM recordings r JOIN streams s ON s.camera_uuid = r.camera_uuid WHERE r.camera_uuid IS NOT NULL AND r.camera_uuid <> '' AND r.is_complete = 1 GROUP BY r.camera_uuid;\0",
    )
};
static mut migration_0074_down: [::core::ffi::c_char; 211] = unsafe {
    ::core::mem::transmute::<
        [u8; 211],
        [::core::ffi::c_char; 211],
    >(
        *b"DROP INDEX IF EXISTS idx_camera_observations_last_video;\nDROP TABLE IF EXISTS camera_observations;\nDROP INDEX IF EXISTS idx_user_workspace_preferences_owner_key;\nDROP TABLE IF EXISTS user_workspace_preferences;\0",
    )
};
static mut migration_0075_up: [::core::ffi::c_char; 978] = unsafe {
    ::core::mem::transmute::<
        [u8; 978],
        [::core::ffi::c_char; 978],
    >(
        *b"CREATE TABLE live_saved_layouts (uuid TEXT PRIMARY KEY,owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,name TEXT NOT NULL COLLATE NOCASE,is_shared INTEGER NOT NULL DEFAULT 0 CHECK (is_shared IN (0,1)),location_uuid TEXT REFERENCES camera_locations(uuid) ON DELETE SET NULL,availability TEXT NOT NULL DEFAULT 'live' CHECK (availability IN ('all','live','offline','never_connected','disabled')),columns INTEGER NOT NULL CHECK (columns BETWEEN 1 AND 9),rows INTEGER NOT NULL CHECK (rows BETWEEN 1 AND 9),camera_slots_json TEXT NOT NULL DEFAULT '[]',revision INTEGER NOT NULL DEFAULT 1,created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),CHECK (columns * rows <= 36));CREATE UNIQUE INDEX idx_live_saved_layouts_owner_name ON live_saved_layouts(COALESCE(owner_user_id,0),name COLLATE NOCASE);CREATE INDEX idx_live_saved_layouts_visible ON live_saved_layouts(is_shared,owner_user_id,name COLLATE NOCASE);\0",
    )
};
static mut migration_0075_down: [::core::ffi::c_char; 148] = unsafe {
    ::core::mem::transmute::<
        [u8; 148],
        [::core::ffi::c_char; 148],
    >(
        *b"DROP INDEX IF EXISTS idx_live_saved_layouts_visible;DROP INDEX IF EXISTS idx_live_saved_layouts_owner_name;DROP TABLE IF EXISTS live_saved_layouts;\0",
    )
};
static mut migration_0076_up: [::core::ffi::c_char; 1354] = unsafe {
    ::core::mem::transmute::<
        [u8; 1354],
        [::core::ffi::c_char; 1354],
    >(
        *b"CREATE TABLE operator_floor_plans (uuid TEXT PRIMARY KEY,name TEXT NOT NULL COLLATE NOCASE,location_uuid TEXT REFERENCES camera_locations(uuid) ON DELETE SET NULL,parent_plan_uuid TEXT REFERENCES operator_floor_plans(uuid) ON DELETE SET NULL,canvas_width INTEGER NOT NULL DEFAULT 1200 CHECK (canvas_width BETWEEN 400 AND 4000),canvas_height INTEGER NOT NULL DEFAULT 800 CHECK (canvas_height BETWEEN 300 AND 4000),revision INTEGER NOT NULL DEFAULT 1,created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')));CREATE UNIQUE INDEX idx_operator_floor_plans_name ON operator_floor_plans(name COLLATE NOCASE);CREATE INDEX idx_operator_floor_plans_location ON operator_floor_plans(location_uuid,name COLLATE NOCASE);CREATE TABLE operator_floor_plan_cameras (plan_uuid TEXT NOT NULL REFERENCES operator_floor_plans(uuid) ON DELETE CASCADE,camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,x REAL NOT NULL CHECK (x >= 0.0 AND x <= 1.0),y REAL NOT NULL CHECK (y >= 0.0 AND y <= 1.0),rotation REAL NOT NULL DEFAULT 0.0 CHECK (rotation >= -180.0 AND rotation <= 180.0),fov REAL NOT NULL DEFAULT 65.0 CHECK (fov >= 1.0 AND fov <= 180.0),PRIMARY KEY (plan_uuid,camera_uuid));CREATE INDEX idx_operator_floor_plan_cameras_camera ON operator_floor_plan_cameras(camera_uuid,plan_uuid);\0",
    )
};
static mut migration_0076_down: [::core::ffi::c_char; 258] = unsafe {
    ::core::mem::transmute::<
        [u8; 258],
        [::core::ffi::c_char; 258],
    >(
        *b"DROP INDEX IF EXISTS idx_operator_floor_plan_cameras_camera;DROP TABLE IF EXISTS operator_floor_plan_cameras;DROP INDEX IF EXISTS idx_operator_floor_plans_location;DROP INDEX IF EXISTS idx_operator_floor_plans_name;DROP TABLE IF EXISTS operator_floor_plans;\0",
    )
};
static mut migration_0077_up: [::core::ffi::c_char; 3589] = unsafe {
    ::core::mem::transmute::<
        [u8; 3589],
        [::core::ffi::c_char; 3589],
    >(
        *b"CREATE TABLE stream_detection_engines(id INTEGER PRIMARY KEY AUTOINCREMENT,stream_id INTEGER NOT NULL REFERENCES streams(id) ON DELETE CASCADE,engine_key TEXT NOT NULL,engine_type TEXT NOT NULL CHECK(engine_type IN('motion','object','onvif','api','external')),model_path TEXT NOT NULL DEFAULT '',enabled INTEGER NOT NULL DEFAULT 1 CHECK(enabled IN(0,1)),threshold REAL NOT NULL DEFAULT 0.5 CHECK(threshold>=0.0 AND threshold<=1.0),interval_seconds INTEGER NOT NULL DEFAULT 5 CHECK(interval_seconds BETWEEN 1 AND 86400),sort_order INTEGER NOT NULL DEFAULT 0,config_json TEXT NOT NULL DEFAULT '{}',created_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),updated_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),UNIQUE(stream_id,engine_key));CREATE INDEX idx_stream_detection_engines_stream_order ON stream_detection_engines(stream_id,enabled DESC,sort_order,id);INSERT INTO stream_detection_engines(stream_id,engine_key,engine_type,model_path,enabled,threshold,interval_seconds,sort_order,config_json) SELECT id,'legacy-primary',CASE WHEN detection_model='motion' THEN 'motion' WHEN detection_model='onvif' THEN 'onvif' WHEN detection_model='api-detection' OR detection_model LIKE 'http://%' OR detection_model LIKE 'https://%' THEN 'api' ELSE 'object' END,detection_model,1,CASE WHEN detection_threshold BETWEEN 0.0 AND 1.0 THEN detection_threshold ELSE 0.5 END,CASE WHEN detection_interval BETWEEN 1 AND 86400 THEN detection_interval ELSE 5 END,0,'{}' FROM streams WHERE detection_model IS NOT NULL AND trim(detection_model)<>'';CREATE TRIGGER trg_stream_detection_engines_after_insert AFTER INSERT ON streams WHEN NEW.detection_model IS NOT NULL AND trim(NEW.detection_model)<>'' BEGIN INSERT INTO stream_detection_engines(stream_id,engine_key,engine_type,model_path,enabled,threshold,interval_seconds,sort_order,config_json) VALUES(NEW.id,'legacy-primary',CASE WHEN NEW.detection_model='motion' THEN 'motion' WHEN NEW.detection_model='onvif' THEN 'onvif' WHEN NEW.detection_model='api-detection' OR NEW.detection_model LIKE 'http://%' OR NEW.detection_model LIKE 'https://%' THEN 'api' ELSE 'object' END,NEW.detection_model,1,CASE WHEN NEW.detection_threshold BETWEEN 0.0 AND 1.0 THEN NEW.detection_threshold ELSE 0.5 END,CASE WHEN NEW.detection_interval BETWEEN 1 AND 86400 THEN NEW.detection_interval ELSE 5 END,0,'{}'); END;CREATE TRIGGER trg_stream_detection_engines_after_update AFTER UPDATE OF detection_model,detection_threshold,detection_interval ON streams BEGIN DELETE FROM stream_detection_engines WHERE stream_id=NEW.id AND engine_key='legacy-primary' AND(NEW.detection_model IS NULL OR trim(NEW.detection_model)='');INSERT INTO stream_detection_engines(stream_id,engine_key,engine_type,model_path,enabled,threshold,interval_seconds,sort_order,config_json) SELECT NEW.id,'legacy-primary',CASE WHEN NEW.detection_model='motion' THEN 'motion' WHEN NEW.detection_model='onvif' THEN 'onvif' WHEN NEW.detection_model='api-detection' OR NEW.detection_model LIKE 'http://%' OR NEW.detection_model LIKE 'https://%' THEN 'api' ELSE 'object' END,NEW.detection_model,1,CASE WHEN NEW.detection_threshold BETWEEN 0.0 AND 1.0 THEN NEW.detection_threshold ELSE 0.5 END,CASE WHEN NEW.detection_interval BETWEEN 1 AND 86400 THEN NEW.detection_interval ELSE 5 END,0,'{}' WHERE NEW.detection_model IS NOT NULL AND trim(NEW.detection_model)<>'' ON CONFLICT(stream_id,engine_key) DO UPDATE SET engine_type=excluded.engine_type,model_path=excluded.model_path,enabled=excluded.enabled,threshold=excluded.threshold,interval_seconds=excluded.interval_seconds,updated_at=strftime('%s','now'); END;\0",
    )
};
static mut migration_0077_down: [::core::ffi::c_char; 240] = unsafe {
    ::core::mem::transmute::<
        [u8; 240],
        [::core::ffi::c_char; 240],
    >(
        *b"DROP TRIGGER IF EXISTS trg_stream_detection_engines_after_update;DROP TRIGGER IF EXISTS trg_stream_detection_engines_after_insert;DROP INDEX IF EXISTS idx_stream_detection_engines_stream_order;DROP TABLE IF EXISTS stream_detection_engines;\0",
    )
};
static mut migration_0078_up: [::core::ffi::c_char; 1930] = unsafe {
    ::core::mem::transmute::<
        [u8; 1930],
        [::core::ffi::c_char; 1930],
    >(
        *b"CREATE TABLE lpr_reads(uuid TEXT PRIMARY KEY,camera_uuid TEXT NOT NULL,stream_name TEXT NOT NULL,observed_at_ms INTEGER NOT NULL,received_at_ms INTEGER NOT NULL,source TEXT NOT NULL CHECK(source IN('onvif_profile_m','onvif_vendor','vendor_api','metadata_track')),vendor_topic TEXT NOT NULL DEFAULT '',plate_nonce BLOB NOT NULL CHECK(length(plate_nonce)=12),plate_ciphertext BLOB NOT NULL,plate_tag BLOB NOT NULL CHECK(length(plate_tag)=16),plate_exact_hmac BLOB NOT NULL CHECK(length(plate_exact_hmac)=32),dedupe_hmac BLOB NOT NULL CHECK(length(dedupe_hmac)=32),confidence REAL CHECK(confidence IS NULL OR(confidence>=0.0 AND confidence<=1.0)),country TEXT,region TEXT,plate_type TEXT,direction TEXT,lane TEXT,vehicle_type TEXT,vehicle_color TEXT,object_id TEXT,correlation_id TEXT,bbox_left REAL,bbox_top REAL,bbox_right REAL,bbox_bottom REAL,recording_id INTEGER REFERENCES recordings(id) ON DELETE SET NULL,created_at INTEGER NOT NULL DEFAULT(strftime('%s','now')),UNIQUE(dedupe_hmac));CREATE INDEX idx_lpr_reads_camera_time ON lpr_reads(camera_uuid,observed_at_ms DESC);CREATE INDEX idx_lpr_reads_exact_time ON lpr_reads(plate_exact_hmac,observed_at_ms DESC);CREATE INDEX idx_lpr_reads_retention ON lpr_reads(received_at_ms);INSERT OR IGNORE INTO system_settings(key,value) VALUES('lpr_retention_days','30');INSERT INTO authz_actions(action_key,category,description,camera_scoped,destructive,sort_order,bit_index) VALUES('lpr.read','License plates','View protected plate values',1,0,160,15),('lpr.search','License plates','Search protected plate reads',1,0,170,16),('lpr.export','License plates','Export protected plate reads',1,0,180,17),('lpr.delete','License plates','Permanently delete protected plate reads',1,1,190,18);INSERT INTO authz_role_actions(role_uuid,action_key) SELECT '00000000-0000-4000-8000-000000000001',action_key FROM authz_actions WHERE action_key IN('lpr.read','lpr.search','lpr.export','lpr.delete');\0",
    )
};
static mut migration_0078_down: [::core::ffi::c_char; 428] = unsafe {
    ::core::mem::transmute::<
        [u8; 428],
        [::core::ffi::c_char; 428],
    >(
        *b"DELETE FROM authz_role_actions WHERE action_key IN('lpr.read','lpr.search','lpr.export','lpr.delete');DELETE FROM authz_actions WHERE action_key IN('lpr.read','lpr.search','lpr.export','lpr.delete');DELETE FROM system_settings WHERE key='lpr_retention_days';DROP INDEX IF EXISTS idx_lpr_reads_retention;DROP INDEX IF EXISTS idx_lpr_reads_exact_time;DROP INDEX IF EXISTS idx_lpr_reads_camera_time;DROP TABLE IF EXISTS lpr_reads;\0",
    )
};
static mut migration_0079_up: [::core::ffi::c_char; 133] = unsafe {
    ::core::mem::transmute::<
        [u8; 133],
        [::core::ffi::c_char; 133],
    >(
        *b"CREATE INDEX IF NOT EXISTS idx_detections_stream_label ON detections(stream_name,label) WHERE label IS NOT NULL AND TRIM(label)<>'';\0",
    )
};
static mut migration_0079_down: [::core::ffi::c_char; 50] = unsafe {
    ::core::mem::transmute::<[u8; 50], [::core::ffi::c_char; 50]>(
        *b"DROP INDEX IF EXISTS idx_detections_stream_label;\0",
    )
};
static mut migration_0080_up: [::core::ffi::c_char; 148] = unsafe {
    ::core::mem::transmute::<
        [u8; 148],
        [::core::ffi::c_char; 148],
    >(
        *b"ALTER TABLE operator_floor_plans ADD COLUMN background_mime TEXT CHECK (background_mime IS NULL OR background_mime IN ('image/png', 'image/jpeg'));\0",
    )
};
static mut migration_0080_down: [::core::ffi::c_char; 62] = unsafe {
    ::core::mem::transmute::<[u8; 62], [::core::ffi::c_char; 62]>(
        *b"ALTER TABLE operator_floor_plans DROP COLUMN background_mime;\0",
    )
};
static mut migration_0081_up: [::core::ffi::c_char; 6151] = unsafe {
    ::core::mem::transmute::<
        [u8; 6151],
        [::core::ffi::c_char; 6151],
    >(
        *b"CREATE TABLE system_health_process_runs (\n    run_id TEXT PRIMARY KEY\n        CHECK (length(run_id) = 36 AND substr(run_id, 9, 1) = '-'\n            AND substr(run_id, 14, 1) = '-' AND substr(run_id, 19, 1) = '-'\n            AND substr(run_id, 24, 1) = '-'),\n    boot_id TEXT NOT NULL\n        CHECK (length(boot_id) BETWEEN 1 AND 63\n            AND boot_id NOT GLOB '*[^A-Za-z0-9_.:-]*'),\n    started_at_ms INTEGER NOT NULL CHECK (started_at_ms > 0),\n    closed_at_ms INTEGER CHECK (\n        closed_at_ms IS NULL OR closed_at_ms >= started_at_ms),\n    clean_close INTEGER NOT NULL DEFAULT 0 CHECK (clean_close IN (0, 1)),\n    CHECK ((clean_close = 0 AND closed_at_ms IS NULL)\n        OR (clean_close = 1 AND closed_at_ms IS NOT NULL))\n);\n\nCREATE INDEX idx_system_health_runs_latest\nON system_health_process_runs(started_at_ms DESC, run_id DESC);\n\nCREATE TABLE system_health_incidents (\n    uuid TEXT PRIMARY KEY\n        CHECK (length(uuid) = 36 AND substr(uuid, 9, 1) = '-'\n            AND substr(uuid, 14, 1) = '-' AND substr(uuid, 19, 1) = '-'\n            AND substr(uuid, 24, 1) = '-'),\n    condition_code TEXT NOT NULL CHECK (condition_code IN (\n        'memory.available_low', 'memory.oom_kill', 'memory.swap_thrash',\n        'cpu.saturation', 'cpu.throttled', 'io.pressure',\n        'filesystem.bytes_low', 'filesystem.inodes_low',\n        'filesystem.read_only', 'filesystem.write_failed', 'thermal.high',\n        'network.link_down', 'network.error_rate',\n        'clock.unsynchronized', 'clock.jump', 'process.fd_exhaustion',\n        'process.pid_exhaustion', 'process.allocation_failed',\n        'storage.device_prefail', 'storage.device_critical',\n        'hardware.ecc_corrected', 'hardware.ecc_uncorrectable',\n        'hardware.fan_failed', 'hardware.power_unstable',\n        'health.collector_stale', 'system.unexpected_restart',\n        'event.delivery_degraded')),\n    subject TEXT NOT NULL CHECK (length(subject) BETWEEN 1 AND 63\n        AND subject NOT GLOB '*[^A-Za-z0-9_.:-]*'),\n    scope TEXT NOT NULL CHECK (\n        scope IN ('process', 'container', 'host', 'filesystem', 'device')),\n    state TEXT NOT NULL CHECK (state IN ('open', 'recovering', 'closed')),\n    severity TEXT NOT NULL CHECK (severity IN ('warning', 'error', 'critical')),\n    first_seen_at_ms INTEGER NOT NULL CHECK (first_seen_at_ms > 0),\n    last_seen_at_ms INTEGER NOT NULL CHECK (last_seen_at_ms >= first_seen_at_ms),\n    closed_at_ms INTEGER,\n    last_observation_json TEXT NOT NULL\n        CHECK (length(last_observation_json) BETWEEN 2 AND 2048),\n    alert_event_id TEXT CHECK (alert_event_id IS NULL OR\n        (length(alert_event_id) = 36 AND substr(alert_event_id, 9, 1) = '-'\n            AND substr(alert_event_id, 14, 1) = '-'\n            AND substr(alert_event_id, 19, 1) = '-'\n            AND substr(alert_event_id, 24, 1) = '-')),\n    recovery_event_id TEXT CHECK (recovery_event_id IS NULL OR\n        (length(recovery_event_id) = 36 AND substr(recovery_event_id, 9, 1) = '-'\n            AND substr(recovery_event_id, 14, 1) = '-'\n            AND substr(recovery_event_id, 19, 1) = '-'\n            AND substr(recovery_event_id, 24, 1) = '-')),\n    reconciliation_state TEXT NOT NULL DEFAULT 'none' CHECK (\n        reconciliation_state IN ('none', 'alert_pending', 'recovery_pending',\n                                 'reconciled', 'delivery_failed')),\n    boot_id TEXT NOT NULL CHECK (length(boot_id) BETWEEN 1 AND 63\n        AND boot_id NOT GLOB '*[^A-Za-z0-9_.:-]*'),\n    run_id TEXT NOT NULL CHECK (length(run_id) = 36\n        AND substr(run_id, 9, 1) = '-' AND substr(run_id, 14, 1) = '-'\n        AND substr(run_id, 19, 1) = '-' AND substr(run_id, 24, 1) = '-'),\n    revision INTEGER NOT NULL DEFAULT 1 CHECK (revision >= 1),\n    CHECK ((state = 'closed' AND closed_at_ms IS NOT NULL\n            AND closed_at_ms >= last_seen_at_ms)\n        OR (state <> 'closed' AND closed_at_ms IS NULL))\n);\n\nCREATE UNIQUE INDEX idx_system_health_incidents_active\nON system_health_incidents(condition_code, subject)\nWHERE state <> 'closed';\n\nCREATE INDEX idx_system_health_incidents_list\nON system_health_incidents(last_seen_at_ms DESC, uuid DESC);\n\nCREATE TABLE system_health_incident_transitions (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    transition_uuid TEXT NOT NULL UNIQUE CHECK (length(transition_uuid) = 36\n        AND substr(transition_uuid, 9, 1) = '-'\n        AND substr(transition_uuid, 14, 1) = '-'\n        AND substr(transition_uuid, 19, 1) = '-'\n        AND substr(transition_uuid, 24, 1) = '-'),\n    incident_uuid TEXT NOT NULL REFERENCES system_health_incidents(uuid)\n        ON DELETE CASCADE,\n    kind TEXT NOT NULL CHECK (\n        kind IN ('open', 'escalation', 'material_change', 'recovery', 'one_shot')),\n    from_state TEXT CHECK (\n        from_state IS NULL OR from_state IN ('open', 'recovering', 'closed')),\n    to_state TEXT NOT NULL CHECK (to_state IN ('open', 'recovering', 'closed')),\n    severity TEXT NOT NULL CHECK (severity IN ('warning', 'error', 'critical')),\n    observed_at_ms INTEGER NOT NULL CHECK (observed_at_ms > 0),\n    safe_observation_json TEXT NOT NULL\n        CHECK (length(safe_observation_json) BETWEEN 2 AND 2048),\n    event_id TEXT CHECK (event_id IS NULL OR (length(event_id) = 36\n        AND substr(event_id, 9, 1) = '-' AND substr(event_id, 14, 1) = '-'\n        AND substr(event_id, 19, 1) = '-' AND substr(event_id, 24, 1) = '-')),\n    reconciliation_state TEXT NOT NULL CHECK (\n        reconciliation_state IN ('none', 'alert_pending', 'recovery_pending',\n                                 'reconciled', 'delivery_failed')),\n    boot_id TEXT NOT NULL CHECK (length(boot_id) BETWEEN 1 AND 63\n        AND boot_id NOT GLOB '*[^A-Za-z0-9_.:-]*'),\n    run_id TEXT NOT NULL CHECK (length(run_id) = 36\n        AND substr(run_id, 9, 1) = '-' AND substr(run_id, 14, 1) = '-'\n        AND substr(run_id, 19, 1) = '-' AND substr(run_id, 24, 1) = '-')\n);\n\nCREATE INDEX idx_system_health_transitions_incident\nON system_health_incident_transitions(\n    incident_uuid, observed_at_ms DESC, id DESC);\n\nCREATE INDEX idx_system_health_transitions_retention\nON system_health_incident_transitions(observed_at_ms, id);\0",
    )
};
static mut migration_0081_down: [::core::ffi::c_char; 439] = unsafe {
    ::core::mem::transmute::<
        [u8; 439],
        [::core::ffi::c_char; 439],
    >(
        *b"DROP INDEX IF EXISTS idx_system_health_transitions_retention;\nDROP INDEX IF EXISTS idx_system_health_transitions_incident;\nDROP TABLE IF EXISTS system_health_incident_transitions;\nDROP INDEX IF EXISTS idx_system_health_incidents_list;\nDROP INDEX IF EXISTS idx_system_health_incidents_active;\nDROP TABLE IF EXISTS system_health_incidents;\nDROP INDEX IF EXISTS idx_system_health_runs_latest;\nDROP TABLE IF EXISTS system_health_process_runs;\0",
    )
};
static mut migration_0082_up: [::core::ffi::c_char; 90] = unsafe {
    ::core::mem::transmute::<
        [u8; 90],
        [::core::ffi::c_char; 90],
    >(
        *b"ALTER TABLE event_destinations\nADD COLUMN status_topic_template TEXT NOT NULL DEFAULT '';\0",
    )
};
static mut migration_0082_down: [::core::ffi::c_char; 66] = unsafe {
    ::core::mem::transmute::<[u8; 66], [::core::ffi::c_char; 66]>(
        *b"ALTER TABLE event_destinations DROP COLUMN status_topic_template;\0",
    )
};
static mut migration_0083_up: [::core::ffi::c_char; 142] = unsafe {
    ::core::mem::transmute::<
        [u8; 142],
        [::core::ffi::c_char; 142],
    >(
        *b"CREATE INDEX IF NOT EXISTS idx_detections_open_external_motion\nON detections(id)\nWHERE source = 'external_motion' AND event_end_time IS NULL;\0",
    )
};
static mut migration_0083_down: [::core::ffi::c_char; 58] = unsafe {
    ::core::mem::transmute::<[u8; 58], [::core::ffi::c_char; 58]>(
        *b"DROP INDEX IF EXISTS idx_detections_open_external_motion;\0",
    )
};
static mut migration_0084_up: [::core::ffi::c_char; 6077] = unsafe {
    ::core::mem::transmute::<
        [u8; 6077],
        [::core::ffi::c_char; 6077],
    >(
        *b"ALTER TABLE storage_targets RENAME COLUMN target_type TO legacy_target_type;\nALTER TABLE storage_targets ADD COLUMN target_type TEXT NOT NULL DEFAULT 'filesystem'\n    CHECK (target_type IN ('filesystem','s3'));\nALTER TABLE storage_targets ADD COLUMN endpoint TEXT NOT NULL DEFAULT '';\nALTER TABLE storage_targets ADD COLUMN region TEXT NOT NULL DEFAULT '';\nALTER TABLE storage_targets ADD COLUMN bucket TEXT NOT NULL DEFAULT '';\nALTER TABLE storage_targets ADD COLUMN credential_ref TEXT NOT NULL DEFAULT '';\nALTER TABLE storage_targets ADD COLUMN archive_budget_bytes INTEGER NOT NULL DEFAULT 0\n    CHECK (archive_budget_bytes >= 0);\n\nALTER TABLE storage_policies ADD COLUMN archive_after_seconds INTEGER NOT NULL DEFAULT -1\n    CHECK (archive_after_seconds BETWEEN -1 AND 2147483647);\nALTER TABLE storage_policies ADD COLUMN hot_residency_seconds INTEGER NOT NULL DEFAULT -1\n    CHECK (hot_residency_seconds BETWEEN -1 AND 2147483647);\nALTER TABLE storage_policies ADD COLUMN archive_protected INTEGER NOT NULL DEFAULT 0\n    CHECK (archive_protected IN (0,1));\nALTER TABLE storage_policies ADD COLUMN archive_on_pressure INTEGER NOT NULL DEFAULT 0\n    CHECK (archive_on_pressure IN (0,1));\n\nALTER TABLE recordings ADD COLUMN storage_policy_uuid TEXT;\nUPDATE recordings SET storage_policy_uuid=substr(placement_reason,instr(placement_reason,':')+1)\n    WHERE placement_reason LIKE 'policy-%';\nCREATE TRIGGER trg_recording_policy_identity AFTER INSERT ON recordings\nWHEN NEW.storage_policy_uuid IS NULL AND NEW.placement_reason LIKE 'policy-%'\nBEGIN\n    UPDATE recordings SET storage_policy_uuid=substr(NEW.placement_reason,instr(NEW.placement_reason,':')+1)\n        WHERE id=NEW.id;\nEND;\nALTER TABLE recordings ADD COLUMN archive_checksum TEXT NOT NULL DEFAULT '';\nALTER TABLE recordings ADD COLUMN deletion_pending INTEGER NOT NULL DEFAULT 0\n    CHECK (deletion_pending IN (0,1));\n\nCREATE TABLE storage_deletions (\n    uuid TEXT PRIMARY KEY,\n    recording_id INTEGER NOT NULL,\n    camera_uuid TEXT NOT NULL DEFAULT '',\n    stream_name TEXT NOT NULL DEFAULT '',\n    reason TEXT NOT NULL,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),\n    completed_at INTEGER\n);\nCREATE TABLE storage_deletion_objects (\n    id INTEGER PRIMARY KEY,\n    deletion_uuid TEXT NOT NULL REFERENCES storage_deletions(uuid) ON DELETE CASCADE,\n    target_uuid TEXT REFERENCES storage_targets(uuid) ON DELETE RESTRICT,\n    original_target_uuid TEXT NOT NULL DEFAULT '',\n    upload_id TEXT NOT NULL DEFAULT '',\n    object_key TEXT NOT NULL DEFAULT '',\n    file_path TEXT NOT NULL DEFAULT '',\n    size_bytes INTEGER NOT NULL DEFAULT 0,\n    state TEXT NOT NULL DEFAULT 'pending' CHECK(state IN ('pending','retry_wait','completed')),\n    attempt_count INTEGER NOT NULL DEFAULT 0,\n    next_attempt_at INTEGER NOT NULL DEFAULT 0,\n    last_error TEXT NOT NULL DEFAULT '',\n    updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now'))\n);\nCREATE TRIGGER trg_deletion_target_identity AFTER INSERT ON storage_deletion_objects\nBEGIN\n    UPDATE storage_deletion_objects SET original_target_uuid=COALESCE(NEW.target_uuid,'') WHERE id=NEW.id;\nEND;\nCREATE INDEX idx_storage_deletion_due ON storage_deletion_objects(state,next_attempt_at,id);\nCREATE INDEX idx_storage_deletion_recording ON storage_deletions(recording_id);\n\nALTER TABLE storage_migration_jobs ADD COLUMN upload_id TEXT NOT NULL DEFAULT '';\nALTER TABLE storage_migration_jobs ADD COLUMN upload_parts TEXT NOT NULL DEFAULT '';\nALTER TABLE storage_migration_jobs ADD COLUMN artifacts_cleaned INTEGER NOT NULL DEFAULT 0 CHECK(artifacts_cleaned IN(0,1));\n\nCREATE TABLE storage_retrieval_jobs (\n    recording_id INTEGER PRIMARY KEY REFERENCES recordings(id) ON DELETE CASCADE,\n    target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,\n    object_key TEXT NOT NULL,\n    checksum TEXT NOT NULL,\n    size_bytes INTEGER NOT NULL,\n    file_path TEXT NOT NULL,\n    state TEXT NOT NULL DEFAULT 'queued' CHECK(state IN ('queued','fetching','ready','failed')),\n    last_access_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),\n    next_attempt_at INTEGER NOT NULL DEFAULT 0,\n    last_error TEXT NOT NULL DEFAULT ''\n);\nCREATE TABLE storage_read_leases (\n    recording_id INTEGER PRIMARY KEY REFERENCES recordings(id) ON DELETE CASCADE,\n    expires_at INTEGER NOT NULL\n);\n\n-- Retain the policy revision selected at capture. Legacy unattributed rows\n-- continue using the existing stream/system rules.\nCREATE TABLE storage_policy_versions AS SELECT * FROM storage_policies;\nCREATE UNIQUE INDEX idx_storage_policy_version ON storage_policy_versions(uuid,revision);\nCREATE TRIGGER trg_storage_policy_snapshot_insert AFTER INSERT ON storage_policies\nBEGIN\n    INSERT INTO storage_policy_versions SELECT * FROM storage_policies WHERE uuid=NEW.uuid;\nEND;\nCREATE TRIGGER trg_storage_policy_snapshot_update AFTER UPDATE ON storage_policies\nWHEN NEW.revision<>OLD.revision\nBEGIN\n    INSERT INTO storage_policy_versions SELECT * FROM storage_policies WHERE uuid=NEW.uuid;\nEND;\nCREATE TRIGGER trg_storage_policy_snapshot_delete AFTER DELETE ON storage_policies\nBEGIN\n    DELETE FROM storage_policy_versions WHERE uuid=OLD.uuid AND NOT EXISTS(SELECT 1 FROM recordings r\n        WHERE r.storage_policy_uuid=storage_policy_versions.uuid AND r.storage_policy_version=storage_policy_versions.revision);\nEND;\nUPDATE recordings SET storage_policy_version=(SELECT revision FROM storage_policies p\n    WHERE p.uuid=recordings.storage_policy_uuid) WHERE storage_policy_uuid IN(SELECT uuid FROM storage_policies);\nCREATE VIEW storage_recording_policies AS\nSELECT r.id AS recording_id,v.* FROM recordings r JOIN storage_policy_versions v\n    ON v.uuid=r.storage_policy_uuid AND v.revision=r.storage_policy_version\nUNION ALL\nSELECT r.id AS recording_id,p.* FROM recordings r JOIN storage_policies p\n    ON p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1))\n    WHERE NOT EXISTS(SELECT 1 FROM storage_policy_versions v WHERE v.uuid=r.storage_policy_uuid\n        AND v.revision=r.storage_policy_version);\n\0",
    )
};
static mut migration_0084_down: [::core::ffi::c_char; 95] = unsafe {
    ::core::mem::transmute::<
        [u8; 95],
        [::core::ffi::c_char; 95],
    >(
        *b"-- Retain additive metadata and deletion inventory: removing it would orphan media.\nSELECT 1;\n\0",
    )
};
static mut migration_0085_up: [::core::ffi::c_char; 117] = unsafe {
    ::core::mem::transmute::<
        [u8; 117],
        [::core::ffi::c_char; 117],
    >(
        *b"CREATE INDEX IF NOT EXISTS idx_storage_migration_recording_state\n    ON storage_migration_jobs(recording_id, state);\0",
    )
};
static mut migration_0085_down: [::core::ffi::c_char; 60] = unsafe {
    ::core::mem::transmute::<[u8; 60], [::core::ffi::c_char; 60]>(
        *b"DROP INDEX IF EXISTS idx_storage_migration_recording_state;\0",
    )
};
static mut migration_0086_up: [::core::ffi::c_char; 474] = unsafe {
    ::core::mem::transmute::<
        [u8; 474],
        [::core::ffi::c_char; 474],
    >(
        *b"CREATE INDEX IF NOT EXISTS idx_recordings_deletion_pending\n    ON recordings(deletion_pending) WHERE deletion_pending = 1;\nCREATE INDEX IF NOT EXISTS idx_storage_deletion_object_deletion\n    ON storage_deletion_objects(deletion_uuid);\nCREATE INDEX IF NOT EXISTS idx_storage_deletion_object_open\n    ON storage_deletion_objects(next_attempt_at, id) WHERE state <> 'completed';\nCREATE INDEX IF NOT EXISTS idx_storage_deletion_completed\n    ON storage_deletions(completed_at);\0",
    )
};
static mut migration_0086_down: [::core::ffi::c_char; 221] = unsafe {
    ::core::mem::transmute::<
        [u8; 221],
        [::core::ffi::c_char; 221],
    >(
        *b"DROP INDEX IF EXISTS idx_storage_deletion_completed;\nDROP INDEX IF EXISTS idx_storage_deletion_object_open;\nDROP INDEX IF EXISTS idx_storage_deletion_object_deletion;\nDROP INDEX IF EXISTS idx_recordings_deletion_pending;\0",
    )
};
static mut migration_0087_up: [::core::ffi::c_char; 62] = unsafe {
    ::core::mem::transmute::<[u8; 62], [::core::ffi::c_char; 62]>(
        *b"ALTER TABLE operator_floor_plans ADD COLUMN sketch_json TEXT;\0",
    )
};
static mut migration_0087_down: [::core::ffi::c_char; 58] = unsafe {
    ::core::mem::transmute::<[u8; 58], [::core::ffi::c_char; 58]>(
        *b"ALTER TABLE operator_floor_plans DROP COLUMN sketch_json;\0",
    )
};
static mut migration_0088_up: [::core::ffi::c_char; 426] = unsafe {
    ::core::mem::transmute::<
        [u8; 426],
        [::core::ffi::c_char; 426],
    >(
        *b"CREATE TABLE camera_favorites (\n    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,\n    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,\n    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))\n);\n\nCREATE UNIQUE INDEX idx_camera_favorites_user_camera\nON camera_favorites(COALESCE(user_id, 0), camera_uuid);\n\nCREATE INDEX idx_camera_favorites_camera\nON camera_favorites(camera_uuid);\0",
    )
};
static mut migration_0088_down: [::core::ffi::c_char; 144] = unsafe {
    ::core::mem::transmute::<
        [u8; 144],
        [::core::ffi::c_char; 144],
    >(
        *b"DROP INDEX IF EXISTS idx_camera_favorites_camera;\nDROP INDEX IF EXISTS idx_camera_favorites_user_camera;\nDROP TABLE IF EXISTS camera_favorites;\0",
    )
};
static mut migration_0089_up: [::core::ffi::c_char; 700] = unsafe {
    ::core::mem::transmute::<
        [u8; 700],
        [::core::ffi::c_char; 700],
    >(
        *b"CREATE INDEX idx_recordings_storage_policy_revision\n    ON recordings(storage_policy_uuid, storage_policy_version);\nDROP VIEW storage_recording_policies;\nCREATE VIEW storage_recording_policies AS\nSELECT r.id AS recording_id,v.* FROM storage_policy_versions v CROSS JOIN recordings r\n    ON v.uuid=r.storage_policy_uuid AND v.revision=r.storage_policy_version\nUNION ALL\nSELECT r.id AS recording_id,p.* FROM storage_policies p CROSS JOIN recordings r\n    ON p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1))\n    WHERE NOT EXISTS(SELECT 1 FROM storage_policy_versions v WHERE v.uuid=r.storage_policy_uuid\n        AND v.revision=r.storage_policy_version);\0",
    )
};
static mut migration_0089_down: [::core::ffi::c_char; 623] = unsafe {
    ::core::mem::transmute::<
        [u8; 623],
        [::core::ffi::c_char; 623],
    >(
        *b"DROP VIEW storage_recording_policies;\nCREATE VIEW storage_recording_policies AS\nSELECT r.id AS recording_id,v.* FROM recordings r JOIN storage_policy_versions v\n    ON v.uuid=r.storage_policy_uuid AND v.revision=r.storage_policy_version\nUNION ALL\nSELECT r.id AS recording_id,p.* FROM recordings r JOIN storage_policies p\n    ON p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1))\n    WHERE NOT EXISTS(SELECT 1 FROM storage_policy_versions v WHERE v.uuid=r.storage_policy_uuid\n        AND v.revision=r.storage_policy_version);\nDROP INDEX idx_recordings_storage_policy_revision;\0",
    )
};
static mut migration_0090_up: [::core::ffi::c_char; 193] = unsafe {
    ::core::mem::transmute::<
        [u8; 193],
        [::core::ffi::c_char; 193],
    >(
        *b"CREATE INDEX IF NOT EXISTS idx_detections_external_motion_camera_end\n    ON detections(camera_uuid, event_end_time, timestamp)\n    WHERE source = 'external_motion' AND camera_uuid IS NOT NULL;\0",
    )
};
static mut migration_0090_down: [::core::ffi::c_char; 64] = unsafe {
    ::core::mem::transmute::<[u8; 64], [::core::ffi::c_char; 64]>(
        *b"DROP INDEX IF EXISTS idx_detections_external_motion_camera_end;\0",
    )
};
static mut migration_0091_up: [::core::ffi::c_char; 75] = unsafe {
    ::core::mem::transmute::<[u8; 75], [::core::ffi::c_char; 75]>(
        *b"ALTER TABLE recordings ADD COLUMN playback_issue TEXT NOT NULL DEFAULT '';\0",
    )
};
static mut migration_0091_down: [::core::ffi::c_char; 51] = unsafe {
    ::core::mem::transmute::<[u8; 51], [::core::ffi::c_char; 51]>(
        *b"ALTER TABLE recordings DROP COLUMN playback_issue;\0",
    )
};
static mut migration_0092_up: [::core::ffi::c_char; 191] = unsafe {
    ::core::mem::transmute::<
        [u8; 191],
        [::core::ffi::c_char; 191],
    >(
        *b"CREATE INDEX IF NOT EXISTS idx_detections_legacy_stream_label\nON detections(stream_name, label)\nWHERE (camera_uuid IS NULL OR camera_uuid = '')\n  AND label IS NOT NULL AND TRIM(label) <> '';\0",
    )
};
static mut migration_0092_down: [::core::ffi::c_char; 57] = unsafe {
    ::core::mem::transmute::<[u8; 57], [::core::ffi::c_char; 57]>(
        *b"DROP INDEX IF EXISTS idx_detections_legacy_stream_label;\0",
    )
};
static mut migration_0093_up: [::core::ffi::c_char; 260] = unsafe {
    ::core::mem::transmute::<
        [u8; 260],
        [::core::ffi::c_char; 260],
    >(
        *b"DELETE FROM sessions WHERE user_id IN (SELECT id FROM users WHERE totp_enabled = 1);\nALTER TABLE sessions ADD COLUMN mfa_pending INTEGER NOT NULL DEFAULT 0 CHECK (mfa_pending IN (0, 1));\nALTER TABLE sessions ADD COLUMN mfa_attempts INTEGER NOT NULL DEFAULT 0;\0",
    )
};
static mut migration_0093_down: [::core::ffi::c_char; 137] = unsafe {
    ::core::mem::transmute::<
        [u8; 137],
        [::core::ffi::c_char; 137],
    >(
        *b"DELETE FROM sessions WHERE mfa_pending = 1;\nALTER TABLE sessions DROP COLUMN mfa_attempts;\nALTER TABLE sessions DROP COLUMN mfa_pending;\0",
    )
};
static mut migration_0094_up: [::core::ffi::c_char; 349] = unsafe {
    ::core::mem::transmute::<
        [u8; 349],
        [::core::ffi::c_char; 349],
    >(
        *b"ALTER TABLE users ADD COLUMN totp_last_used_step INTEGER NOT NULL DEFAULT -1;\nALTER TABLE users ADD COLUMN totp_failed_attempts INTEGER NOT NULL DEFAULT 0 CHECK (totp_failed_attempts >= 0);\nALTER TABLE users ADD COLUMN totp_window_started_at INTEGER NOT NULL DEFAULT 0;\nALTER TABLE users ADD COLUMN totp_setup_expires_at INTEGER NOT NULL DEFAULT 0;\0",
    )
};
static mut migration_0094_down: [::core::ffi::c_char; 323] = unsafe {
    ::core::mem::transmute::<
        [u8; 323],
        [::core::ffi::c_char; 323],
    >(
        *b"-- Pending challenges must not survive removal of replay protection.\nDELETE FROM sessions WHERE mfa_pending = 1;\nALTER TABLE users DROP COLUMN totp_setup_expires_at;\nALTER TABLE users DROP COLUMN totp_window_started_at;\nALTER TABLE users DROP COLUMN totp_failed_attempts;\nALTER TABLE users DROP COLUMN totp_last_used_step;\0",
    )
};
static mut embedded_migrations_data: [migration_t; 94] = unsafe {
    [
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0001\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"initial_schema\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0001_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0001_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0002\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_detection_columns\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0002_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0002_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0003\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_protocol_onvif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0003_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0003_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0004\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_record_audio\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0004_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0004_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0005\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_detections_table\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0005_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0005_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0006\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_users_table\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0006_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0006_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0007\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_motion_settings\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0007_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0007_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0008\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_zones_table\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0008_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0008_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0009\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_detection_tracking\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0009_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0009_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0010\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_trigger_type\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0010_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0010_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0011\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_detection_api_url\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0011_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0011_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0012\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_backchannel\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0012_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0012_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0013\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_retention_policy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0013_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0013_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0014\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_ptz_support\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0014_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0014_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0015\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_buffer_strategy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0015_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0015_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0016\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_onvif_credentials\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0016_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0016_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0017\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_recording_id_to_detections\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0017_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0017_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0018\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_session_tracking\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0018_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0018_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0019\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_password_change_locked\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0019_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0019_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0020\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_motion_recording_config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0020_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0020_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0021\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_totp_mfa\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0021_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0021_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0022\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"fix_detection_zones_table\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0022_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0022_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0023\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_detection_object_filters\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0023_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0023_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0024\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"enhanced_storage_management\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0024_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0024_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0025\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_detection_performance_indexes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0025_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0025_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0026\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_recording_schedule\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0026_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0026_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0027\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_stream_group\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0027_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0027_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0028\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_stream_tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0028_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0028_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0029\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_user_allowed_tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0029_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0029_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0030\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_system_settings\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0030_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0030_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0031\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_onvif_port\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0031_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0031_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0032\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_recording_tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0032_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0032_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0033\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"cleanup_blank_stream_names\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0033_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0033_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0034\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"repair_onvif_embedded_credentials\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0034_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0034_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0035\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"modernize_auth_sessions\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0035_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0035_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0036\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_stream_admin_url\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0036_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0036_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0037\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_user_allowed_login_cidrs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0037_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0037_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0038\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_privacy_mode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0038_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0038_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0039\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_motion_trigger_source\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0039_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0039_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0040\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_go2rtc_source_override_and_sub_stream_url\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0040_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0040_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0041\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_audio_voice_enhancement\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0041_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0041_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0042\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_detection_url\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0042_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0042_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0043\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"backfill_disk_pressure_eligible\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0043_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0043_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0044\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_stream_publish_url\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0044_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0044_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0045\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_recording_schedule_metadata_and_detection_schedule\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0045_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0045_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0046\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_detection_event_intervals\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0046_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0046_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0047\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"optimize_recording_history\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0047_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0047_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0048\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_camera_uuid\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0048_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0048_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0049\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_camera_locations\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0049_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0049_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0050\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_normalized_camera_tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0050_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0050_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0051\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_camera_collections\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0051_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0051_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0052\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_authorization_policy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0052_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0052_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0053\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_collection_authorization_scope\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0053_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0053_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0054\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_scoped_api_tokens\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0054_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0054_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0055\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_audit_events\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0055_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0055_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0056\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"harden_authorization_metadata\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0056_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0056_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0057\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_event_outbox\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0057_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0057_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0058\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_event_routes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0058_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0058_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0059\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_event_route_suppression\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0059_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0059_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0060\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_event_destinations\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0060_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0060_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0061\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_capture_camera_identity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0061_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0061_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0062\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_investigation_search_indexes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0062_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0062_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0063\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_must_change_password\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0063_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0063_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0064\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_investigation_bookmarks\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0064_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0064_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0065\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_storage_targets\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0065_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0065_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0066\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_storage_placement_policies\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0066_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0066_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0067\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_fleet_saved_views\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0067_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0067_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0068\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_storage_migration_jobs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0068_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0068_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0069\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_stream_playback_transport\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0069_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0069_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0070\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_onvif_discovery_inventory\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0070_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0070_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0071\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"complete_storage_migration_controls\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0071_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0071_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0072\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_storage_pools_and_lifecycle\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0072_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0072_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0073\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_stream_eptz_config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0073_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0073_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0074\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_operator_workspace_foundations\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0074_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0074_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0075\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_live_saved_layouts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0075_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0075_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0076\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_operator_floor_plans\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0076_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0076_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0077\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_stream_detection_engines\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0077_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0077_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0078\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_protected_lpr_reads\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0078_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0078_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0079\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_scoped_detection_label_index\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0079_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0079_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0080\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_floor_plan_background\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0080_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0080_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0081\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_system_health_incidents\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0081_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0081_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0082\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_event_destination_status_topic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0082_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0082_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0083\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"index_open_external_motion\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0083_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0083_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0084\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_external_recording_archive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0084_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0084_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0085\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"index_recording_migration_lookups\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0085_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0085_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0086\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"index_storage_deletion_ledger\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0086_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0086_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0087\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_floor_plan_sketch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0087_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0087_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0088\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_camera_favorites\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0088_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0088_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0089\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"optimize_storage_recording_policies\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0089_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0089_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0090\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"index_external_motion_intervals\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0090_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0090_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0091\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"add_recording_playback_issue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0091_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0091_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0092\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"index_legacy_detection_labels\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0092_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0092_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0093\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"separate_mfa_challenges\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0093_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0093_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
        migration_t {
            version: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"0094\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            description: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"totp_replay_and_attempts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            filepath: [0; 512],
            sql_up: &raw const migration_0094_up as *const ::core::ffi::c_char,
            sql_down: &raw const migration_0094_down as *const ::core::ffi::c_char,
            status: migrate_status_t::MIGRATE_STATUS_PENDING,
            is_embedded: r#true != 0,
        },
    ]
};
pub const EMBEDDED_MIGRATIONS_COUNT: ::core::ffi::c_int = 94 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const DEFAULT_MIGRATIONS_DIR: [::core::ffi::c_char; 31] = unsafe {
    ::core::mem::transmute::<[u8; 31], [::core::ffi::c_char; 31]>(
        *b"/usr/share/lightnvr/migrations\0",
    )
};
pub const LOCAL_MIGRATIONS_DIR: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"./db/migrations\0") };
static mut LEGACY_VERSION_MAP: [*const ::core::ffi::c_char; 20] = [
    ::core::ptr::null::<::core::ffi::c_char>(),
    b"0001\0".as_ptr() as *const ::core::ffi::c_char,
    b"0002\0".as_ptr() as *const ::core::ffi::c_char,
    b"0003\0".as_ptr() as *const ::core::ffi::c_char,
    b"0004\0".as_ptr() as *const ::core::ffi::c_char,
    b"0005\0".as_ptr() as *const ::core::ffi::c_char,
    b"0006\0".as_ptr() as *const ::core::ffi::c_char,
    b"0007\0".as_ptr() as *const ::core::ffi::c_char,
    b"0008\0".as_ptr() as *const ::core::ffi::c_char,
    b"0009\0".as_ptr() as *const ::core::ffi::c_char,
    b"0010\0".as_ptr() as *const ::core::ffi::c_char,
    b"0011\0".as_ptr() as *const ::core::ffi::c_char,
    b"0012\0".as_ptr() as *const ::core::ffi::c_char,
    b"0013\0".as_ptr() as *const ::core::ffi::c_char,
    b"0014\0".as_ptr() as *const ::core::ffi::c_char,
    b"0015\0".as_ptr() as *const ::core::ffi::c_char,
    b"0016\0".as_ptr() as *const ::core::ffi::c_char,
    b"0017\0".as_ptr() as *const ::core::ffi::c_char,
    b"0018\0".as_ptr() as *const ::core::ffi::c_char,
    b"0019\0".as_ptr() as *const ::core::ffi::c_char,
];
pub const LEGACY_VERSION_COUNT: usize = ::core::mem::size_of::<[*const ::core::ffi::c_char; 20]>()
    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>());
unsafe extern "C" fn find_migrations_dir() -> *const ::core::ffi::c_char {
    let mut env_path: *const ::core::ffi::c_char =
        getenv(b"LIGHTNVR_MIGRATIONS_DIR\0".as_ptr() as *const ::core::ffi::c_char);
    if !env_path.is_null() && access(env_path, R_OK) == 0 as ::core::ffi::c_int {
        return env_path;
    }
    if access(LOCAL_MIGRATIONS_DIR.as_ptr(), R_OK) == 0 as ::core::ffi::c_int {
        return LOCAL_MIGRATIONS_DIR.as_ptr();
    }
    if access(DEFAULT_MIGRATIONS_DIR.as_ptr(), R_OK) == 0 as ::core::ffi::c_int {
        return DEFAULT_MIGRATIONS_DIR.as_ptr();
    }
    let mut exe_path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut len: ssize_t = readlink(
        b"/proc/self/exe\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut exe_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1 as size_t),
    );
    if len > 0 as ssize_t {
        exe_path[len as usize] = '\0' as ::core::ffi::c_char;
        let mut last_slash: *mut ::core::ffi::c_char = strrchr(
            &raw mut exe_path as *mut ::core::ffi::c_char,
            '/' as ::core::ffi::c_int,
        );
        if !last_slash.is_null() {
            *last_slash = '\0' as ::core::ffi::c_char;
            static mut migrations_path: [::core::ffi::c_char; 4096] = [0; 4096];
            snprintf(
                &raw mut migrations_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                b"%s/../share/lightnvr/migrations\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut exe_path as *mut ::core::ffi::c_char,
            );
            if access(&raw mut migrations_path as *mut ::core::ffi::c_char, R_OK)
                == 0 as ::core::ffi::c_int
            {
                return &raw mut migrations_path as *mut ::core::ffi::c_char;
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"No migrations directory found, using embedded migrations only\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn migration_callback(
    mut version: *const ::core::ffi::c_char,
    mut description: *const ::core::ffi::c_char,
    mut status: migrate_status_t,
    mut user_data: *mut ::core::ffi::c_void,
) {
    let mut status_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    match status {
        migrate_status_t::MIGRATE_STATUS_APPLIED => {
            status_str = b"APPLIED\0".as_ptr() as *const ::core::ffi::c_char;
        }
        migrate_status_t::MIGRATE_STATUS_FAILED => {
            status_str = b"FAILED\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {
            status_str = b"PENDING\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Migration %s_%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
        version,
        description,
        status_str,
    );
}
unsafe extern "C" fn get_legacy_schema_version(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut check_sql: *const ::core::ffi::c_char =
        b"SELECT name FROM sqlite_master WHERE type='table' AND name='schema_version';\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        check_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return 0 as ::core::ffi::c_int;
    }
    let mut has_legacy_table: bool = sqlite3_step(stmt) == SQLITE_ROW;
    sqlite3_finalize(stmt);
    if !has_legacy_table {
        return 0 as ::core::ffi::c_int;
    }
    let mut version_sql: *const ::core::ffi::c_char =
        b"SELECT version FROM schema_version WHERE id = 1;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        version_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return 0 as ::core::ffi::c_int;
    }
    let mut version: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if sqlite3_step(stmt) == SQLITE_ROW {
        version = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    sqlite3_finalize(stmt);
    return version;
}
unsafe extern "C" fn has_new_migrations_table(mut db: *mut sqlite3) -> bool {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT name FROM sqlite_master WHERE type='table' AND name='schema_migrations';\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return r#false != 0;
    }
    let mut exists: bool = sqlite3_step(stmt) == SQLITE_ROW;
    sqlite3_finalize(stmt);
    return exists;
}
unsafe extern "C" fn migrate_from_legacy_schema(
    mut db: *mut sqlite3,
    mut legacy_version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Migrating from legacy schema version %d to new migration system\0".as_ptr()
            as *const ::core::ffi::c_char,
        legacy_version,
    );
    let mut create_sql: *const ::core::ffi::c_char = b"CREATE TABLE IF NOT EXISTS schema_migrations (  version TEXT PRIMARY KEY,  applied_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')));\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = sqlite3_exec(db, create_sql, None, NULL, &raw mut err_msg);
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create schema_migrations table: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut insert_sql: *const ::core::ffi::c_char =
        b"INSERT OR IGNORE INTO schema_migrations (version) VALUES (?);\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        insert_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare insert statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut migrated: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i <= legacy_version && i < LEGACY_VERSION_COUNT as ::core::ffi::c_int {
        if !LEGACY_VERSION_MAP[i as usize].is_null() {
            sqlite3_reset(stmt);
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                LEGACY_VERSION_MAP[i as usize],
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            rc = sqlite3_step(stmt);
            if rc != SQLITE_DONE {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to insert migration record for version %d: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    i,
                    sqlite3_errmsg(db),
                );
            } else {
                migrated += 1;
            }
        }
        i += 1;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Migrated %d legacy schema versions to new migration system\0".as_ptr()
            as *const ::core::ffi::c_char,
        migrated,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_database_migrations() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot run migrations: no database connection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !has_new_migrations_table(db) {
        let mut legacy_version: ::core::ffi::c_int = get_legacy_schema_version(db);
        if legacy_version > 0 as ::core::ffi::c_int {
            if migrate_from_legacy_schema(db, legacy_version) != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to migrate from legacy schema system\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
    }
    let mut migrations_dir: *const ::core::ffi::c_char = find_migrations_dir();
    let mut config: migrate_config_t = migrate_config_t {
        migrations_dir: migrations_dir,
        migrations_table: b"schema_migrations\0".as_ptr() as *const ::core::ffi::c_char,
        embedded_migrations: &raw const embedded_migrations_data as *const migration_t,
        embedded_count: EMBEDDED_MIGRATIONS_COUNT as size_t,
        callback: Some(
            migration_callback
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    migrate_status_t,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        callback_data: NULL,
        dry_run: r#false != 0,
        verbose: r#true != 0,
    };
    let mut ctx: *mut sqlite_migrate_t = migrate_init(db, &raw mut config);
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize migration system\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stats: migrate_stats_t = migrate_stats_t {
        total: 0,
        applied: 0,
        pending: 0,
        failed: 0,
    };
    let mut result: ::core::ffi::c_int = migrate_up(ctx, &raw mut stats);
    if result == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migrations complete: %d total, %d applied, %d pending\0".as_ptr()
                as *const ::core::ffi::c_char,
            stats.total,
            stats.applied,
            stats.pending,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migration failed: %d migrations failed\0".as_ptr() as *const ::core::ffi::c_char,
            stats.failed,
        );
    }
    migrate_free(ctx);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_database_version(
    mut version: *mut ::core::ffi::c_char,
    mut version_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut migrations_dir: *const ::core::ffi::c_char = find_migrations_dir();
    let mut config: migrate_config_t = migrate_config_t {
        migrations_dir: migrations_dir,
        migrations_table: b"schema_migrations\0".as_ptr() as *const ::core::ffi::c_char,
        embedded_migrations: ::core::ptr::null::<migration_t>(),
        embedded_count: 0,
        callback: None,
        callback_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dry_run: false,
        verbose: r#false != 0,
    };
    let mut ctx: *mut sqlite_migrate_t = migrate_init(db, &raw mut config);
    if ctx.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = migrate_get_version(ctx, version, version_len as size_t);
    migrate_free(ctx);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_migration_status() {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot print migration status: no database connection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut migrations_dir: *const ::core::ffi::c_char = find_migrations_dir();
    let mut config: migrate_config_t = migrate_config_t {
        migrations_dir: migrations_dir,
        migrations_table: b"schema_migrations\0".as_ptr() as *const ::core::ffi::c_char,
        embedded_migrations: ::core::ptr::null::<migration_t>(),
        embedded_count: 0,
        callback: None,
        callback_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dry_run: false,
        verbose: r#false != 0,
    };
    let mut ctx: *mut sqlite_migrate_t = migrate_init(db, &raw mut config);
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize migration system\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut migrations: [migration_t; 100] = [migration_t {
        version: [0; 32],
        description: [0; 256],
        filepath: [0; 512],
        sql_up: ::core::ptr::null::<::core::ffi::c_char>(),
        sql_down: ::core::ptr::null::<::core::ffi::c_char>(),
        status: migrate_status_t::MIGRATE_STATUS_PENDING,
        is_embedded: false,
    }; 100];
    let mut count: ::core::ffi::c_int = migrate_status(
        ctx,
        &raw mut migrations as *mut migration_t,
        100 as ::core::ffi::c_int,
    );
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get migration status\0".as_ptr() as *const ::core::ffi::c_char,
        );
        migrate_free(ctx);
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"=== Migration Status ===\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut status_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        match migrations[i as usize].status {
            migrate_status_t::MIGRATE_STATUS_APPLIED => {
                status_str = b"[x]\0".as_ptr() as *const ::core::ffi::c_char;
            }
            migrate_status_t::MIGRATE_STATUS_FAILED => {
                status_str = b"[!]\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {
                status_str = b"[ ]\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s %s_%s\0".as_ptr() as *const ::core::ffi::c_char,
            status_str,
            &raw mut (*(&raw mut migrations as *mut migration_t).offset(i as isize)).version
                as *mut ::core::ffi::c_char,
            &raw mut (*(&raw mut migrations as *mut migration_t).offset(i as isize)).description
                as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"========================\0".as_ptr() as *const ::core::ffi::c_char,
    );
    migrate_free(ctx);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
