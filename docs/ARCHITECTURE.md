# LightNVR Architecture

This document describes the architecture and internal design of the LightNVR system.

## Overview

LightNVR is designed with a modular architecture that prioritizes memory efficiency and reliability. The system is composed of several key components that work together to provide a complete Network Video Recorder solution.

*Note: A system architecture diagram should be created based on the specifications in [images/system-architecture.txt](images/system-architecture.txt)*

## Core Components

### Core System

The core system is responsible for:
- Application lifecycle management
- Configuration loading and validation
- Signal handling and graceful shutdown
- Daemon mode operation
- PID file management
- Logging

Key files:
- `src/core/main.c`: Main entry point and application lifecycle
- `src/core/config.c`: Configuration loading and management
- `src/core/daemon.c`: Daemon mode functionality
- `src/core/logger.c`: Logging system

### Video Subsystem

The video subsystem handles:
- Stream management (connecting to cameras)
- Video decoding and processing
- Frame buffering and memory management
- Recording to disk

Key files:
- `src/video/stream_manager.c`: Manages stream connections and state
- `src/video/streams.c`: Stream implementation
- `src/video/hls_writer.c`: HLS (HTTP Live Streaming) recording
- `src/video/mp4_writer.c`: MP4 recording

### Storage Subsystem

The storage subsystem is responsible for:
- Managing recording storage
- Implementing retention policies
- Disk space management
- File organization

Key files:
- `src/storage/storage_manager.c`: Storage management implementation

### Database Subsystem

The database subsystem handles:
- Storing stream configurations
- Recording metadata
- System settings
- User authentication

Key files:
- `src/database/database_manager.c`: Database operations

### Web Interface

The web interface provides:
- User interface for managing the system
- Live view of camera streams
- Recording playback
- System configuration
- REST API for programmatic access

Key files:
- `src/web/web_server.c`: Web server implementation
- `src/web/api_handlers.c`: API request handling
- `src/web/api_handlers_*.c`: Specific API endpoint implementations
- `web/`: HTML, CSS, and JavaScript files for the web interface

## Memory Management

LightNVR is designed to be memory-efficient, with several strategies employed:

### Stream Buffering

- Configurable buffer sizes to balance memory usage and performance
- Intelligent frame dropping when memory is constrained
- Priority-based resource allocation

### Memory Pools

- Pre-allocated memory pools for frequently used objects
- Avoids fragmentation from frequent allocations/deallocations
- Configurable pool sizes based on available system memory

### Swap Support

- Optional swap file for additional virtual memory
- Configurable swap size
- Used for non-critical operations to free physical memory for stream processing

## Thread Model

LightNVR uses a multi-threaded architecture to efficiently handle multiple streams:

1. **Main Thread**: Application lifecycle, signal handling, and periodic tasks
2. **Stream Threads**: One thread per active stream for receiving and processing video
3. **Recording Threads**: Separate threads for writing recordings to disk
4. **Web Server Thread**: Handles HTTP requests for the web interface
5. **Thread Pool**: For handling API requests and other tasks

Thread synchronization is handled using mutexes and condition variables to ensure thread safety while minimizing contention.

## Data Flow

### Stream Processing Flow

1. Stream connection is established (RTSP/ONVIF)
2. Frames are received and decoded
3. Frames are processed (optional: motion detection, analytics)
4. Frames are buffered for live viewing
5. If recording is enabled, frames are sent to the recording subsystem
6. Recording subsystem writes frames to disk in the configured format (MP4/HLS)

### Web Interface Flow

1. User accesses web interface via browser
2. Web server authenticates the user (if enabled)
3. Web server serves the appropriate HTML/CSS/JS files
4. Client-side JavaScript makes API requests to fetch data
5. API handlers process requests and return JSON responses
6. For live viewing, HLS or MJPEG streams are served

## Database Schema

LightNVR uses SQLite for data storage with the following main tables:

### Streams Table

Stores stream configuration:
```sql
CREATE TABLE streams (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    fps INTEGER NOT NULL,
    codec TEXT NOT NULL,
    priority INTEGER NOT NULL,
    record INTEGER NOT NULL,
    segment_duration INTEGER NOT NULL
);
```

### Recordings Table

Stores recording metadata:
```sql
CREATE TABLE recordings (
    id INTEGER PRIMARY KEY,
    stream_id INTEGER NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    duration INTEGER NOT NULL,
    size INTEGER NOT NULL,
    format TEXT NOT NULL,
    path TEXT NOT NULL,
    FOREIGN KEY (stream_id) REFERENCES streams(id)
);
```

### Settings Table

Stores system settings:
```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

### Users Table

Stores user authentication information:
```sql
CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL,
    role TEXT NOT NULL
);
```

## API Design

The LightNVR API follows RESTful principles:

- Resources are identified by URLs
- Standard HTTP methods (GET, POST, PUT, DELETE) for CRUD operations
- JSON for data exchange
- Authentication via HTTP Basic Auth
- Proper status codes for success/error conditions

See [API.md](API.md) for detailed API documentation.

## Configuration System

The configuration system uses a simple key-value format:

- Configuration is loaded from a file at startup
- Default values are provided for all settings
- Configuration can be updated via the API
- Changes are persisted to the database
- Some settings require a restart to take effect

See [CONFIGURATION.md](CONFIGURATION.md) for detailed configuration documentation.

## Memory Optimization for Ingenic A1

The Ingenic A1 SoC has only 256MB of RAM, requiring specific optimizations:

1. **Staggered Initialization**: Streams are initialized one at a time to prevent memory spikes
2. **Reduced Buffer Sizes**: Default buffer sizes are reduced for memory-constrained environments
3. **Frame Dropping**: Intelligent frame dropping when memory is low
4. **Resolution Limiting**: Automatic downscaling of high-resolution streams
5. **Swap Support**: Optional swap file for additional virtual memory
6. **Priority System**: Ensures critical streams get resources when memory is constrained

## Error Handling and Recovery

LightNVR is designed to be robust and self-healing:

1. **Stream Reconnection**: Automatically reconnects to streams after network issues
2. **Watchdog Timer**: Monitors system health and restarts components if necessary
3. **Graceful Degradation**: Reduces functionality rather than crashing when resources are constrained
4. **Safe Shutdown**: Ensures recordings are properly finalized during shutdown
5. **Crash Recovery**: Recovers state from database after unexpected shutdowns

## Security Considerations

LightNVR implements several security measures:

1. **Authentication**: Optional HTTP Basic Auth for web interface and API
2. **Password Hashing**: Passwords are stored as salted hashes
3. **Input Validation**: All user input is validated to prevent injection attacks
4. **Resource Limiting**: Rate limiting to prevent DoS attacks
5. **Minimal Dependencies**: Reduces attack surface by minimizing external dependencies

## Future Architecture Enhancements

Planned architectural improvements:

1. **Plugin System**: Allow extending functionality through plugins
2. **Clustering**: Support for distributed operation across multiple nodes
3. **Hardware Acceleration**: Better support for hardware-accelerated video processing
4. **Event System**: More sophisticated event handling and notifications
5. **Analytics Integration**: Support for video analytics and AI-based detection
