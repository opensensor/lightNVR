# Tech Context: LightNVR

## Core Technologies

- **Backend Language:** C (Standard: C11 or later assumed, check `CMakeLists.txt` if specific version needed)
- **Build System:** CMake
- **Database:** SQLite (embedded)
- **Web Server:** Mongoose (embedded library)
- **JSON Parsing:** cJSON (embedded library)
- **Video Processing:** FFmpeg (external dependency, linked dynamically or statically depending on build)
- **Object Detection (Optional):** SOD (libsodium fork, potentially embedded or linked)

## Frontend Technologies

- **UI Framework:** Preact
- **Styling:** Tailwind CSS
- **Build Tool:** Vite (for frontend asset bundling/optimization)
- **Core Language:** JavaScript (ES Modules)

## Development Setup & Build

- **OS:** Linux (various distributions)
- **Compiler:** GCC or Clang (check `CMakeLists.txt` for requirements)
- **Dependencies:**
    - CMake
    - Make (or Ninja)
    - C Compiler (GCC/Clang)
    - FFmpeg development libraries (libavcodec, libavformat, libavutil, etc.)
    - SQLite3 development libraries (libsqlite3-dev)
    - (Optional) Node.js/npm for frontend development/building
- **Build Process:**
    1. Clone repository.
    2. Run `./scripts/build.sh` (handles CMake configuration, compilation, and frontend build via Vite).
    3. Output binaries/libraries are typically placed in a `build/` directory.
- **Installation:** `./scripts/install.sh` copies binaries, configuration files, and systemd service files to standard system locations (e.g., `/usr/local/bin`, `/etc/lightnvr`, `/lib/systemd/system`).
- **Docker:** Dockerfiles (`Dockerfile`, `Dockerfile.alpine`) are provided for containerized builds and deployment. Uses multi-stage builds.

## Key Libraries & Dependencies (Backend C)

- **Mongoose:** (`src/web/mongoose.*`, `include/web/mongoose.*`) - Handles HTTP/WebSocket server logic.
- **cJSON:** (Likely embedded or included directly) - Used for parsing/generating JSON in API handlers.
- **FFmpeg:** (External system library) - Core for video decoding, encoding (if transcoding), and stream handling (RTSP).
- **SQLite3:** (External system library) - Used by the database subsystem.
- **ezxml:** (`src/ezxml/`, `include/ezxml.h`) - XML parsing (likely for ONVIF).
- **SOD:** (`src/sod/`, `include/sod/`) - Object detection library.
- **pthread:** Standard POSIX threads for multi-threading.

## Technical Constraints & Considerations

- **Memory Limits:** The primary constraint, especially on target devices (e.g., 256MB RAM). All development must prioritize memory efficiency.
- **Cross-Compilation:** May be required for building for specific embedded targets (e.g., ARM, MIPS). Build scripts might need adaptation.
- **FFmpeg Version Compatibility:** Ensure compatibility with FFmpeg versions commonly found on target Linux distributions or provide guidance on building/linking specific versions.
- **Real-time Performance:** Video processing and recording must meet real-time requirements to avoid frame drops or recording gaps.
- **Filesystem Performance:** Disk I/O for recording can be a bottleneck, especially on SD cards or slow storage. Efficient writing patterns are necessary.
