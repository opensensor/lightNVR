# libuv + llhttp HTTP Server Migration

## Overview

This document outlines the architecture and migration path from Mongoose to libuv + llhttp for the lightNVR HTTP server. This change addresses scalability limitations with concurrent video streaming and API traffic.

## Why libuv + llhttp?

| Aspect | Mongoose | libuv + llhttp |
|--------|----------|----------------|
| Concurrency Model | Thread-per-connection or thread pool | Event-driven, non-blocking I/O |
| Scaling | Context switching overhead at scale | Near-linear horizontal scaling |
| Memory | Higher per-connection overhead | Minimal per-connection state |
| Integration | HTTP-only | Unified event loop for HTTP, file I/O, timers, signals |
| Control | Library owns event loop | Application owns event loop |

For a media server like lightNVR handling concurrent video streams, API requests, and disk I/O, the unified event loop architecture is significantly more efficient.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Application Layer                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐  │
│  │ API Handlers    │  │ Stream Handlers │  │ Static File Handler │  │
│  │ (request_       │  │ (HLS, WebRTC)   │  │ (async file I/O)    │  │
│  │  handler_t)     │  │                 │  │                     │  │
│  └────────┬────────┘  └────────┬────────┘  └──────────┬──────────┘  │
│           │                    │                      │             │
│           └────────────────────┼──────────────────────┘             │
│                                ▼                                    │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │              HTTP Server Abstraction Layer                   │    │
│  │  ┌─────────────────┐  ┌──────────────────────────────────┐  │    │
│  │  │ http_request_t  │  │ http_response_t                  │  │    │
│  │  │ http_server_t   │  │ request_handler_t                │  │    │
│  │  └─────────────────┘  └──────────────────────────────────┘  │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                │                                    │
│           ┌────────────────────┼────────────────────┐               │
│           ▼                    ▼                    ▼               │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐  │
│  │ mongoose_server │  │ libuv_server    │  │ (future backends)   │  │
│  │ (current)       │  │ (new)           │  │                     │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

## Key Components

### 1. libuv_server.h / libuv_server.c

The new backend implementing `http_server_handle_t` interface:

```c
// Same interface as mongoose_server_init()
http_server_handle_t libuv_server_init(const http_server_config_t *config);
```

Internal structure:
```c
typedef struct libuv_server {
    uv_loop_t *loop;              // Event loop (can be shared with other subsystems)
    uv_tcp_t listener;            // TCP listener handle
    http_server_config_t config;  // Server configuration
    bool running;                 // Server running flag
    
    // Handler registry (same as current)
    struct {
        char path[256];
        char method[16];
        request_handler_t handler;
    } *handlers;
    int handler_count;
    int handler_capacity;
    
    // TLS context (optional)
    void *tls_ctx;
} libuv_server_t;
```

### 2. Connection State

```c
typedef struct libuv_connection {
    uv_tcp_t handle;              // TCP handle
    uv_buf_t read_buf;            // Read buffer
    llhttp_t parser;              // HTTP parser instance
    llhttp_settings_t settings;   // Parser callbacks
    
    http_request_t request;       // Parsed request (reused)
    http_response_t response;     // Response being built
    
    libuv_server_t *server;       // Back-pointer to server
    
    // TLS state (optional)
    void *tls_session;
    
    // Request state
    bool headers_complete;
    bool message_complete;
    size_t body_offset;
} libuv_connection_t;
```

### 3. File Serving (Async)

```c
typedef struct file_serve_ctx {
    uv_fs_t open_req;
    uv_fs_t read_req;
    uv_fs_t stat_req;
    uv_fs_t close_req;
    
    libuv_connection_t *conn;
    uv_file fd;
    char *buffer;
    size_t buffer_size;
    size_t offset;
    size_t remaining;
    
    // Range request support
    size_t range_start;
    size_t range_end;
} file_serve_ctx_t;

int libuv_serve_file(libuv_connection_t *conn, const char *path,
                     const char *content_type, const char *extra_headers);
```

## Migration Strategy

### Phase 1: Parallel Implementation
- Implement libuv_server alongside mongoose_server
- Both implement same `http_server_handle_t` interface
- Build-time switch via CMake option: `-DHTTP_BACKEND=libuv`

### Phase 2: Handler Compatibility
- All handlers using `http_request_t`/`http_response_t` work unchanged
- Handlers still using Mongoose types need conversion (10 files remaining)

### Phase 3: Extended Abstractions
- Add `http_response_serve_file()` to abstraction layer
- Add `http_response_set_cookie()` for auth handlers
- Add `http_response_redirect()` for login flows

### Phase 4: Unified Event Loop
- Share `uv_loop_t` with RTSP handling, file I/O, timers
- Remove separate threading for video stream management
- Single event loop per core for maximum efficiency

## Dependencies

```cmake
# New dependencies for libuv backend
pkg_check_modules(LIBUV REQUIRED libuv)

# llhttp is header-only or can be built from source
# https://github.com/nodejs/llhttp
```

## Build Configuration

```cmake
option(HTTP_BACKEND "HTTP server backend: mongoose or libuv" "mongoose")

if(HTTP_BACKEND STREQUAL "libuv")
    pkg_check_modules(LIBUV REQUIRED libuv)
    add_definitions(-DHTTP_BACKEND_LIBUV)
    set(HTTP_SERVER_SOURCES
        src/web/libuv_server.c
        src/web/libuv_connection.c
        src/web/libuv_file_serve.c
        src/web/llhttp_adapter.c
    )
else()
    # Current Mongoose implementation
    add_definitions(-DHTTP_BACKEND_MONGOOSE)
    set(HTTP_SERVER_SOURCES
        src/web/mongoose_server.c
        src/web/mongoose_server_*.c
    )
endif()
```

## Implementation Status

| Component | Status | File(s) |
|-----------|--------|---------|
| libuv_server.h interface | ✅ Complete | `include/web/libuv_server.h` |
| libuv_connection.h | ✅ Complete | `include/web/libuv_connection.h` |
| Core event loop | ✅ Complete | `src/web/libuv_server.c` |
| llhttp request parsing | ✅ Complete | `src/web/libuv_connection.c` |
| Response serialization | ✅ Complete | `src/web/libuv_response.c` |
| Async file serving | ✅ Complete | `src/web/libuv_file_serve.c` |
| TLS integration | ⏸️ Deferred | Headers have placeholder |
| Build system (CMake) | ✅ Complete | `CMakeLists.txt` |

## Building with libuv Backend

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libuv1-dev

# Build with libuv backend
mkdir build && cd build
cmake -DHTTP_BACKEND=libuv ..
make

# Or keep using Mongoose (default)
cmake ..
make
```

## Next Steps

1. **Test basic HTTP handling** - Verify request/response flow works
2. **Port remaining handlers** - Convert 10 files still using Mongoose types
3. **Add TLS support** - Integrate OpenSSL/mbedTLS when base is stable
4. **Benchmark** - Compare throughput vs Mongoose under concurrent load
5. **Unified event loop** - Share loop with RTSP, timers, etc.

