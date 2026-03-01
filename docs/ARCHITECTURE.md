# LightNVR Architecture

This document describes the architecture and internal design of the LightNVR system.

## Overview

LightNVR is designed with a modular architecture that prioritizes memory efficiency and reliability. The system is composed of several key components that work together to provide a complete Network Video Recorder solution.

The architecture centers on **go2rtc** as the primary streaming backbone, with LightNVR providing configuration management, recording, detection, and a unified web interface.

## Overall Architecture
![LightNVR Overall Architecture](images/arch-overall.svg)

## Thread Architecture
![LightNVR Thread Architecture](images/arch-thread.svg)

## State Management Architecture
![LightNVR State Management](images/arch-state.svg)

## High-Level Data Flow

```
┌─────────────┐     RTSP/ONVIF      ┌─────────────────────────────────────────────────┐
│   Cameras   │────────────────────▶│                    go2rtc                       │
│             │                     │  (Managed by LightNVR as child process)         │
└─────────────┘                     │                                                 │
                                    │  ┌─────────┐  ┌─────────┐  ┌─────────────────┐  │
                                    │  │  RTSP   │  │  WebRTC │  │  HLS/frame.jpeg │  │
                                    │  │ Server  │  │ Server  │  │    Endpoints    │  │
                                    │  │ :8554   │  │         │  │     :1984       │  │
                                    │  └────┬────┘  └────┬────┘  └────────┬────────┘  │
                                    └───────┼───────────┼─────────────────┼───────────┘
                                            │           │                 │
            ┌───────────────────────────────┼───────────┼─────────────────┼───────────┐
            │                   LightNVR    │           │                 │           │
            │                               ▼           ▼                 ▼           │
            │  ┌──────────────────────────────────────────────────────────────────┐   │
            │  │                        Web Interface                              │   │
            │  │  Live View (WebRTC/HLS) │ Recordings │ Settings │ Detection      │   │
            │  └──────────────────────────────────────────────────────────────────┘   │
            │                               │                     │                   │
            │              ┌────────────────┴───────┐   ┌─────────┴─────────┐         │
            │              ▼                        ▼   ▼                   ▼         │
            │  ┌─────────────────┐    ┌─────────────────────┐   ┌────────────────┐    │
            │  │  MP4 Recording  │    │   HLS Streaming     │   │   Detection    │    │
            │  │   (FFmpeg lib)  │    │   (FFmpeg lib)      │   │   (TFLite/     │    │
            │  │                 │    │                     │   │  SOD/API)      │    │
            │  └────────┬────────┘    └──────────┬──────────┘   └───────┬────────┘    │
            │           │                        │                      │             │
            │           ▼                        ▼                      ▼             │
            │  ┌──────────────────────────────────────────────────────────────────┐   │
            │  │                      SQLite Database                              │   │
            │  │  streams │ recordings │ detections │ zones │ users │ settings    │   │
            │  └──────────────────────────────────────────────────────────────────┘   │
            │           │                        │                                    │
            │           ▼                        ▼                                    │
            │  ┌──────────────────────────────────────────────────────────────────┐   │
            │  │                      Storage (/var/lib/lightnvr)                  │   │
            │  │  recordings/mp4/  │  recordings/hls/  │  lightnvr.db              │   │
            │  └──────────────────────────────────────────────────────────────────┘   │
            └─────────────────────────────────────────────────────────────────────────┘
```

### Core System

The core system is responsible for:
- Application lifecycle management
- Configuration loading and validation
- Signal handling and graceful shutdown
- Daemon mode operation
- PID file management
- Logging
- **go2rtc process management** (starting, monitoring, stopping)

Key files:
- `src/core/main.c`: Main entry point and application lifecycle
- `src/core/config.c`: Configuration loading and management
- `src/core/daemon.c`: Daemon mode functionality
- `src/core/logger.c`: Logging system with syslog support
- `src/core/logger_json.c`: JSON-formatted logging output
- `src/core/shutdown_coordinator.c`: Coordinated shutdown of all components
- `src/core/mqtt_client.c`: MQTT client for detection event publishing
- `src/core/curl_init.c`: Shared libcurl initialization

### go2rtc Integration (Primary Streaming)

**go2rtc is the heart of LightNVR's streaming architecture.** It handles:
- Direct camera connections (RTSP, ONVIF, HTTP, etc.)
- WebRTC streaming for ultra-low-latency live viewing
- RTSP server for local re-streaming (used by MP4/HLS recorders)
- HLS segment generation
- Frame extraction via `frame.jpeg` endpoint (used for detection)
- Two-way audio (backchannel) support

Key files:
- `src/video/go2rtc/go2rtc_process.c`: Process lifecycle management
- `src/video/go2rtc/go2rtc_stream.c`: Stream registration and API communication
- `src/video/go2rtc/go2rtc_integration.c`: Integration layer with LightNVR
- `src/video/go2rtc/go2rtc_consumer.c`: Recording consumer management
- `src/video/go2rtc/go2rtc_api.c`: HTTP API communication with go2rtc
- `src/video/go2rtc/go2rtc_snapshot.c`: Snapshot/frame extraction from go2rtc
- `src/video/go2rtc/dns_cleanup.c`: DNS resource cleanup for go2rtc

See [GO2RTC_INTEGRATION.md](GO2RTC_INTEGRATION.md) for detailed documentation.

### Video Subsystem

The video subsystem handles:
- Stream management (stream state, configuration)
- **go2rtc coordination** (registering streams, getting RTSP URLs)
- Recording to disk (MP4 and HLS formats)
- Detection-based recording triggers

Key files:
- `src/video/stream_manager.c`: Manages stream state and configuration
- `src/video/streams.c`: Stream state implementation
- `src/video/stream_state.c`: Stream state machine
- `src/video/stream_state_adapter.c`: Stream state adapter layer
- `src/video/stream_protocol.c`: Protocol-specific handling (TCP/UDP)
- `src/video/stream_transcoding.c`: Stream transcoding support
- `src/video/hls_writer.c`: HLS (HTTP Live Streaming) recording
- `src/video/hls_streaming.c`: HLS streaming management
- `src/video/hls/hls_unified_thread.c`: Unified HLS writer thread
- `src/video/hls/hls_context.c`: HLS context management
- `src/video/hls/hls_directory.c`: HLS directory/segment management
- `src/video/hls/hls_api.c`: HLS API integration
- `src/video/mp4_writer.c`: MP4 recording core writer
- `src/video/mp4_writer_core.c`: MP4 writer core implementation
- `src/video/mp4_writer_thread.c`: MP4 writer thread management
- `src/video/mp4_writer_utils.c`: MP4 writer utilities
- `src/video/mp4_recording_core.c`: MP4 recording core logic
- `src/video/mp4_recording_writer.c`: MP4 recording writer
- `src/video/mp4_segment_recorder.c`: Segmented MP4 recording
- `src/video/recording.c`: Recording orchestration

### Detection Subsystem

The detection subsystem provides object detection and motion detection:
- **Object Detection**: TensorFlow Lite models, SOD (Simple Object Detection), or external API
- **Motion Detection**: ONVIF-based motion events from cameras
- **Detection Zones**: Polygon-based regions of interest per stream
- **Detection-triggered recording**: Start/stop recordings based on detection events
- **Unified Detection Thread**: Per-stream thread with pre/post detection buffering

#### Recording Modes

LightNVR supports four recording modes based on the `record` and `detection_based_recording` flags:

| `record` | `detection_based_recording` | Behavior |
|----------|----------------------------|----------|
| `false` | `false` | No recording, no detection |
| `true` | `false` | Continuous recording only (MP4 with `trigger_type='scheduled'`) |
| `false` | `true` | Detection-only recording (MP4 created only when detections occur) |
| `true` | `true` | **Annotation mode**: Continuous recording + detection annotations linked via `recording_id` |

In **annotation mode**, the unified detection thread runs but does NOT create separate MP4 files. Instead, detections are stored in the database with a `recording_id` foreign key linking to the ongoing continuous recording.

Key files:
- `src/video/unified_detection_thread.c`: Unified per-stream detection thread with buffering and annotation mode
- `src/video/detection.c`: Core detection logic
- `src/video/detection_config.c`: Detection configuration management
- `src/video/detection_embedded.c`: Embedded detection (SOD/TFLite)
- `src/video/detection_integration.c`: Model loading (TFLite/SOD/API)
- `src/video/detection_model.c`: Detection model management
- `src/video/detection_stream.c`: Per-stream detection state
- `src/video/api_detection.c`: External API-based detection
- `src/video/sod_detection.c`: SOD detection implementation
- `src/video/sod_integration.c`: SOD library integration
- `src/video/sod_realnet.c`: SOD RealNet face detection
- `src/video/onvif_detection.c`: ONVIF event-based detection
- `src/video/onvif_detection_integration.c`: ONVIF detection integration layer
- `src/video/onvif_motion_recording.c`: ONVIF motion-triggered recording
- `src/video/motion_detection.c`: Motion detection processing
- `src/video/motion_storage_manager.c`: Motion recording storage management
- `src/video/pre_detection_buffer.c`: Pre-detection circular buffer for capture before events
- `src/video/packet_buffer.c`: Packet-level ring buffer
- `src/video/buffer_strategy_go2rtc.c`: go2rtc-based buffer strategy
- `src/video/buffer_strategy_hls_segment.c`: HLS segment buffer strategy
- `src/video/buffer_strategy_memory_packet.c`: In-memory packet buffer strategy
- `src/video/buffer_strategy_mmap.c`: Memory-mapped buffer strategy
- `src/database/db_detections.c`: Detection storage with optional recording_id linkage

See [SOD_INTEGRATION.md](SOD_INTEGRATION.md) and [ONVIF_MOTION_RECORDING.md](ONVIF_MOTION_RECORDING.md) for details.

### Storage Subsystem

The storage subsystem is responsible for:
- Managing recording storage with per-stream directories
- Implementing retention policies (standard vs detection recordings)
- Disk space management and automatic cleanup
- File organization by date

Key files:
- `src/storage/storage_manager.c`: Storage management implementation
- `src/storage/storage_manager_streams.c`: Per-stream storage management
- `src/storage/storage_manager_streams_cache.c`: Stream storage cache
- `src/database/db_recordings.c`: Recording metadata and retention queries

### Database Subsystem

The database subsystem handles:
- Stream configurations (SQLite)
- Recording metadata with trigger types
- Detection results and zones
- User authentication and sessions
- ONVIF motion configuration

Key files:
- `src/database/db_core.c`: Database initialization and connection management
- `src/database/db_schema.c`: Schema version management
- `src/database/db_migrations.c`: Migration execution engine
- `src/database/db_streams.c`: Stream configuration CRUD
- `src/database/db_recordings.c`: Recording metadata operations
- `src/database/db_recordings_sync.c`: Recording filesystem synchronization
- `src/database/db_detections.c`: Detection result storage
- `src/database/db_zones.c`: Detection zone management
- `src/database/db_auth.c`: User authentication and session management
- `src/database/db_events.c`: Event logging
- `src/database/db_motion_config.c`: Motion recording configuration
- `src/database/db_maintenance.c`: Database maintenance tasks
- `src/database/db_backup.c`: Database backup support
- `src/database/db_query_builder.c`: Dynamic query builder
- `src/database/db_transaction.c`: Transaction management
- `src/database/db_schema_cache.c`: Schema caching for performance

### Web Interface

The web interface provides:
- **Live viewing** via WebRTC (low latency) or HLS (compatibility)
- Recording playback with timeline scrubbing
- Stream configuration with detection zone editor
- ONVIF device discovery and auto-configuration
- PTZ (Pan-Tilt-Zoom) camera controls
- System settings, user management, and TOTP/MFA
- REST API for programmatic access

Key files:
- `src/web/libuv_server.c`: Web server implementation using libuv + llhttp
- `src/web/libuv_api_handlers.c`: API route registration
- `src/web/libuv_connection.c`: Connection management
- `src/web/libuv_file_serve.c`: Static file serving with gzip support
- `src/web/libuv_response.c`: HTTP response helpers
- `src/web/api_handlers_streams_get.c`: Stream GET endpoints
- `src/web/api_handlers_streams_modify.c`: Stream POST/PUT/DELETE endpoints
- `src/web/api_handlers_streams_test.c`: Stream connection testing
- `src/web/api_handlers_recordings_backend_agnostic.c`: Recording CRUD endpoints
- `src/web/api_handlers_recordings_list.c`: Recording list/filter
- `src/web/api_handlers_recordings_playback.c`: Recording playback
- `src/web/api_handlers_recordings_download.c`: Recording file download
- `src/web/api_handlers_recordings_files_backend_agnostic.c`: Recording file operations
- `src/web/api_handlers_recordings_sync.c`: Recording filesystem sync
- `src/web/api_handlers_detection.c`: Detection API endpoints
- `src/web/api_handlers_detection_results.c`: Detection result queries
- `src/web/api_handlers_detection_models.c`: Detection model listing
- `src/web/api_handlers_motion.c`: Motion detection API
- `src/web/api_handlers_retention.c`: Retention policy API
- `src/web/api_handlers_timeline.c`: Timeline segments and playback
- `src/web/api_handlers_auth_backend_agnostic.c`: Login/logout/verify
- `src/web/api_handlers_users_backend_agnostic.c`: User CRUD, API keys, password management
- `src/web/api_handlers_totp.c`: TOTP/MFA setup and verification
- `src/web/api_handlers_zones.c`: Detection zone management
- `src/web/api_handlers_settings.c`: System settings
- `src/web/api_handlers_system.c`: System info/restart/shutdown
- `src/web/api_handlers_system_logs.c`: Log viewing and management
- `src/web/api_handlers_onvif_backend_agnostic.c`: ONVIF discovery and device management
- `src/web/api_handlers_ptz.c`: PTZ camera control
- `src/web/api_handlers_go2rtc_proxy.c`: go2rtc API proxy
- `src/web/api_handlers_ice_servers.c`: WebRTC ICE server configuration
- `src/web/api_handlers_streaming.c`: HLS streaming endpoints
- `src/web/api_handlers_health.c`: Health check endpoints
- `web/js/components/preact/`: Preact components for the SPA
- `web/js/pages/`: Page-level entry points

## Memory Management

LightNVR is designed to be memory-efficient, with several strategies employed:

### go2rtc Memory Efficiency

- go2rtc handles all camera connections and transcoding
- LightNVR only receives re-streamed video for recording
- Frame extraction for detection uses JPEG snapshots (not decoded video)

### Motion Buffer (Pre-Event Recording)

- Circular buffer of AVPackets for ONVIF motion detection
- Configurable duration (5-30 seconds)
- Memory-efficient packet cloning
- See [MOTION_BUFFER.md](MOTION_BUFFER.md)

### Swap Support

- Optional swap file for additional virtual memory
- Configurable swap size
- Used for non-critical operations to free physical memory for stream processing

## Thread Model

LightNVR uses a multi-threaded architecture:

1. **Main Thread**: Application lifecycle, signal handling, shutdown coordination
2. **go2rtc Process**: Separate process managed by LightNVR (handles all camera I/O)
3. **HTTP Server Thread**: HTTP/WebSocket event loop (libuv)
4. **Per-Request Threads**: API requests handled in thread pool
5. **HLS Writer Threads**: One per stream writing HLS segments
6. **MP4 Recording Threads**: One per active MP4 recording
7. **Detection Threads**: One per stream with detection enabled
8. **ONVIF Motion Threads**: Event subscription threads per motion-enabled stream
9. **Health Monitor Thread**: Monitors go2rtc and stream health

Thread synchronization uses mutexes and condition variables. The shutdown coordinator ensures ordered cleanup.

## Data Flow

### Stream Processing Flow (go2rtc-based)

```
Camera (RTSP/ONVIF)
       │
       ▼
   go2rtc ──────────────────────────────┬─────────────────────┐
       │                                │                     │
       │ WebRTC                         │ HLS                 │ RTSP
       ▼                                ▼                     ▼
   Browser                      LightNVR HLS          LightNVR MP4
  (Live View)                    Writer Thread         Recording
                                       │                     │
                                       ▼                     ▼
                               HLS Segments (.ts)      MP4 Files
                                       │                     │
                               (Detection Thread          Database
                                analyzes segments)        Metadata
```

1. **Camera → go2rtc**: go2rtc connects to cameras via RTSP/ONVIF/HTTP
2. **go2rtc → Browser**: WebRTC for low-latency live viewing
3. **go2rtc → HLS Writer**: Consumes go2rtc's RTSP output, writes .ts segments
4. **go2rtc → MP4 Recorder**: Consumes go2rtc's RTSP output, writes MP4 files
5. **HLS Segments → Detection**: Detection thread analyzes newest segments
6. **Detection → Recording Trigger**: Detections can start/stop MP4 recordings

### Web Interface Flow

1. User accesses web interface via browser
2. Web server authenticates the user (session token or Basic Auth)
3. Web server serves the SPA (Single Page Application)
4. Client-side Preact app makes API requests
5. **Live viewing**: Browser connects directly to go2rtc for WebRTC/HLS
6. **Recording playback**: Browser requests MP4/HLS from LightNVR API

## Database Schema

LightNVR uses SQLite for data storage. The schema is managed via numbered migration files in `db/migrations/` (0001 through 0022). Below is the cumulative schema after all migrations.

### Streams Table

Stores stream configuration:
```sql
CREATE TABLE streams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    enabled INTEGER DEFAULT 1,
    streaming_enabled INTEGER DEFAULT 1,
    width INTEGER DEFAULT 1280,
    height INTEGER DEFAULT 720,
    fps INTEGER DEFAULT 30,
    codec TEXT DEFAULT 'h264',
    priority INTEGER DEFAULT 5,
    record INTEGER DEFAULT 1,
    segment_duration INTEGER DEFAULT 900,
    -- Detection (migration 0002)
    detection_based_recording INTEGER DEFAULT 0,
    detection_model TEXT DEFAULT '',
    detection_threshold REAL DEFAULT 0.5,
    detection_interval INTEGER DEFAULT 10,
    pre_detection_buffer INTEGER DEFAULT 0,
    post_detection_buffer INTEGER DEFAULT 3,
    -- Protocol/ONVIF (migration 0003)
    protocol INTEGER DEFAULT 0,
    is_onvif INTEGER DEFAULT 0,
    -- Audio (migration 0004)
    record_audio INTEGER DEFAULT 1,
    -- Detection API (migration 0011)
    detection_api_url TEXT DEFAULT '',
    -- Backchannel (migration 0012)
    backchannel_enabled INTEGER DEFAULT 0,
    -- Retention (migration 0013)
    retention_days INTEGER DEFAULT 30,
    detection_retention_days INTEGER DEFAULT 90,
    max_storage_mb INTEGER DEFAULT 0,
    -- PTZ (migration 0014)
    ptz_enabled INTEGER DEFAULT 0,
    ptz_max_x INTEGER DEFAULT 0,
    ptz_max_y INTEGER DEFAULT 0,
    ptz_max_z INTEGER DEFAULT 0,
    ptz_has_home INTEGER DEFAULT 0,
    -- Buffer strategy (migration 0015)
    buffer_strategy TEXT DEFAULT 'auto',
    -- ONVIF credentials (migration 0016)
    onvif_username TEXT DEFAULT '',
    onvif_password TEXT DEFAULT '',
    onvif_profile TEXT DEFAULT ''
);
```

### Recordings Table

Stores recording metadata:
```sql
CREATE TABLE recordings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER,
    size_bytes INTEGER DEFAULT 0,
    width INTEGER,
    height INTEGER,
    codec TEXT,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    -- Trigger type (migration 0010)
    trigger_type TEXT DEFAULT 'scheduled',  -- 'scheduled', 'detection', 'motion'
    -- Protection and retention (migration 0013)
    protected INTEGER DEFAULT 0,
    retention_override_days INTEGER DEFAULT NULL
);
```

### Detections Table

Stores detection results:
```sql
CREATE TABLE detections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    label TEXT NOT NULL,
    confidence REAL NOT NULL,
    x REAL, y REAL, width REAL, height REAL,
    recording_id INTEGER,  -- Links to continuous recording in annotation mode
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    -- Tracking (migration 0009)
    track_id INTEGER DEFAULT -1,
    zone_id TEXT DEFAULT '',
    FOREIGN KEY (recording_id) REFERENCES recordings(id)
);
```

### Detection Zones Table

Stores polygon-based detection regions:
```sql
CREATE TABLE detection_zones (
    id TEXT PRIMARY KEY,
    stream_name TEXT NOT NULL,
    name TEXT NOT NULL,
    enabled INTEGER DEFAULT 1,
    color TEXT DEFAULT '#3b82f6',
    polygon TEXT NOT NULL,      -- JSON array of {x, y} normalized points
    filter_classes TEXT DEFAULT '',
    min_confidence REAL DEFAULT 0.0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE
);
```

### Users Table

Stores user authentication information:
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL,
    role TEXT NOT NULL,  -- 'admin', 'user', 'viewer'
    email TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    last_login INTEGER,
    is_active INTEGER DEFAULT 1,
    api_key TEXT,
    -- Password lock (migration 0019)
    password_change_locked INTEGER DEFAULT 0,
    -- TOTP/MFA (migration 0021)
    totp_secret TEXT,
    totp_enabled INTEGER DEFAULT 0
);
```

### Sessions Table

Stores active user sessions:
```sql
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    expires_at INTEGER NOT NULL,
    -- Session tracking (migration 0018)
    ip_address TEXT,
    user_agent TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

### Motion Recording Config Table

Stores per-stream motion recording settings (migration 0020):
```sql
CREATE TABLE motion_recording_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL UNIQUE,
    enabled INTEGER DEFAULT 1,
    pre_buffer_seconds INTEGER DEFAULT 5,
    post_buffer_seconds INTEGER DEFAULT 5,
    max_file_duration INTEGER DEFAULT 300,
    codec TEXT DEFAULT 'h264',
    quality TEXT DEFAULT 'medium',
    retention_days INTEGER DEFAULT 7,
    max_storage_mb INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE
);
```

### Motion Recordings Table

Stores motion-triggered recordings (migration 0020):
```sql
CREATE TABLE motion_recordings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL,
    file_path TEXT NOT NULL UNIQUE,
    start_time INTEGER NOT NULL,
    end_time INTEGER,
    size_bytes INTEGER DEFAULT 0,
    width INTEGER,
    height INTEGER,
    fps INTEGER,
    codec TEXT,
    is_complete INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE
);
```

## API Design

The LightNVR API follows RESTful principles:

- Resources are identified by URLs
- Standard HTTP methods (GET, POST, PUT, DELETE) for CRUD operations
- JSON for data exchange using cJSON library
- Authentication via session tokens or HTTP Basic Auth
- Proper status codes for success/error conditions

See [API.md](API.md) for detailed API documentation.

## Frontend Architecture

The frontend is a multi-page application with Preact components, built with Vite:

- **Preact** for component-based UI development
- **Tailwind CSS** for styling and responsive design with theme customization
- **Vite** for build tooling and bundling
- **WebRTC** for low-latency live viewing (via go2rtc)
- **HLS.js** for fallback streaming compatibility
- **@tanstack/query** for data fetching and caching

Key pages (`web/js/pages/`):
- `index-page.jsx`: Live view entry point
- `recordings-page.jsx`: Recording browser
- `streams-page.jsx`: Stream management
- `settings-page.jsx`: System settings
- `system-page.jsx`: System information and logs
- `users-page.jsx`: User management
- `timeline-page.jsx`: Timeline playback
- `hls-page.jsx`: Direct HLS viewer
- `login-page.jsx`: Authentication

Key components (`web/js/components/preact/`):
- `LiveView.jsx`: Live camera grid with WebRTC/HLS/MSE
- `WebRTCVideoCell.jsx`, `HLSVideoCell.jsx`, `MSEVideoCell.jsx`: Video player cells
- `RecordingsView.jsx`: Recording browser and playback
- `recordings/`: Modular recordings sub-components (filters, table, pagination)
- `StreamConfigModal.jsx`: Stream configuration with detection zones
- `StreamsView.jsx`: Stream list and management
- `SettingsView.jsx`: System settings
- `SystemView.jsx`: System information dashboard
- `system/`: System sub-components (logs, memory, streams info, controls)
- `timeline/`: Timeline player components (segments, ruler, cursor, controls)
- `UsersView.jsx`: User management
- `users/`: User sub-components (add/edit/delete modals, TOTP setup, API keys)
- `ZoneEditor.jsx`: Detection zone polygon editor
- `DetectionOverlay.jsx`: Detection result overlay on video
- `PTZControls.jsx`: Pan-Tilt-Zoom camera controls
- `ThemeCustomizer.jsx`: Theme selection and dark mode
- `Toast.jsx`, `ToastContainer.jsx`: Notification system
- `SnapshotManager.jsx`: Camera snapshot capture
- `FullscreenManager.jsx`: Fullscreen video management
- `UI.jsx`: Shared UI primitives

Utilities (`web/js/utils/`):
- `auth-utils.js`: Authentication helpers
- `dom-utils.js`: DOM manipulation utilities
- `settings-utils.js`: Settings management
- `theme-init.js`: Theme initialization
- `url-utils.js`: URL parameter handling

## Configuration System

Configuration uses INI format files (`.ini`) with the following sections:

- **`[general]`**: PID file, log file, log level, syslog settings
- **`[storage]`**: Recording paths, retention, MP4 direct recording options
- **`[database]`**: SQLite database path
- **`[web]`**: Port, web root, authentication, session timeout, thread pool
- **`[streams]`**: Stream configuration
- **`[models]`**: Detection model storage path
- **`[api_detection]`**: External detection API URL, backend, confidence, class filter
- **`[memory]`**: Buffer size, swap settings
- **`[hardware]`**: Hardware acceleration
- **`[go2rtc]`**: WebRTC settings, STUN/TURN, ICE servers, proxy limits
- **`[mqtt]`**: MQTT broker connection, topic prefix, TLS, QoS
- **`[onvif]`**: ONVIF discovery settings and network

Stream configurations are stored in the SQLite database and managed via the API/web UI. System settings can be updated via the API or web UI and are persisted immediately. Some settings (like go2rtc port) require a restart.

See [CONFIGURATION.md](CONFIGURATION.md) for detailed configuration documentation.

## Memory Optimization for Ingenic A1

The Ingenic A1 SoC has only 256MB of RAM, requiring specific optimizations:

1. **go2rtc Efficiency**: go2rtc handles camera connections, reducing LightNVR memory usage
2. **Staggered Initialization**: Streams are initialized one at a time to prevent memory spikes
3. **JPEG-based Detection**: Uses go2rtc's `frame.jpeg` endpoint instead of decoding video
4. **Swap Support**: Optional swap file for additional virtual memory
5. **Priority System**: Ensures critical streams get resources when memory is constrained

## Shutdown Coordination System

LightNVR implements a robust shutdown coordination system to ensure clean and orderly shutdown of all components.

The shutdown coordination system follows a priority-based approach to ensure components shut down in the correct dependency order:

1. **Detection Threads** (highest priority - 100): Shut down first since they depend on both MP4 writers and HLS streaming
2. **MP4 Writers** (medium priority - 80): Shut down second since they depend on HLS streaming
3. **HLS Streaming Threads** (lowest priority - 60): Shut down last as they are the foundation
4. **go2rtc Process** (final): Terminated after all consumers are stopped

### Key Components

1. **Shutdown Coordinator** (`include/core/shutdown_coordinator.h` and `src/core/shutdown_coordinator.c`)
   - Centralized management of component registration and shutdown sequence
   - Atomic operations for thread safety with minimal mutex usage
   - Component state tracking during shutdown
   - Priority-based shutdown sequencing
   - Timeout mechanism to prevent hanging

2. **Component Integration**
   - Components register with the coordinator during initialization
   - Components check for shutdown signals in their main loops
   - Components update their state when exiting
   - Coordinator tracks component states during shutdown

### Shutdown Sequence

1. Main process initiates shutdown (via signal handler or user request)
2. Coordinator sets shutdown flag and notifies components in priority order
3. Components check shutdown flag in their main loops and begin cleanup
4. Components release shared resources and update their state
5. Coordinator waits for all components to stop (with timeout)
6. go2rtc process is terminated
7. Main process performs final cleanup when all components are stopped

This system prevents race conditions, deadlocks, and memory corruption during shutdown by ensuring components are stopped in the correct order and resources are properly released.

## Error Handling and Recovery

LightNVR is designed to be robust and self-healing:

1. **go2rtc Health Monitoring**: Monitors go2rtc API and restarts on failure
2. **Stream Reconnection**: go2rtc automatically reconnects to streams after network issues
3. **Graceful Degradation**: Reduces functionality rather than crashing when resources are constrained
4. **Safe Shutdown**: Ensures recordings are properly finalized during shutdown
5. **Crash Recovery**: Recovers state from database after unexpected shutdowns

## Security Considerations

LightNVR implements several security measures:

1. **Session-based Authentication**: Secure token-based sessions with configurable expiration
2. **TOTP/MFA**: Two-factor authentication support via time-based one-time passwords
3. **Password Hashing**: Passwords are stored as salted hashes (mbedTLS)
4. **Input Validation**: All user input is validated to prevent injection attacks
5. **Role-based Access**: Admin, user, and viewer roles with API key support
6. **Password Lock**: Ability to lock user passwords against changes
7. **Minimal Dependencies**: Reduces attack surface by minimizing external dependencies

## Future Architecture Enhancements

Planned architectural improvements:

1. **go2rtc Native Recording**: Use go2rtc's built-in recording capabilities
2. **Hardware Acceleration**: Better support for hardware-accelerated video processing
3. **Webhooks**: HTTP webhook notifications for detection events
4. **Multi-model Detection**: Support for multiple detection models per stream

**Note:** Pre-detection buffer and MQTT event streaming have been implemented.
