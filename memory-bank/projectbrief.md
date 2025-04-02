# Project Brief: LightNVR

## Core Goal

Develop and maintain LightNVR, a tiny, memory-optimized Network Video Recorder (NVR) software written in C. The primary focus is efficiency and reliability, especially on resource-constrained Linux devices like the Ingenic A1 SoC (256MB RAM), while remaining functional on standard Linux systems.

## Key Features & Requirements

- **Cross-Platform:** Run on any Linux system (ARM, x86, MIPS).
- **Memory Efficiency:** Optimized for low RAM environments (target < 256MB).
- **Stream Handling:** Support up to 16 concurrent video streams (RTSP, basic ONVIF profile).
- **Codec Support:** H.264 (primary), H.265 (optional, resource-dependent).
- **Object Detection:** Optional integration with SOD (libsodium) for motion/object detection (RealNet/CNN models).
- **Resolution/FPS:** Support up to 1080p, configurable frame rate (1-15 FPS) per stream.
- **Recording:** Record in standard MP4/MKV formats with proper indexing.
- **Web Interface:** Provide a modern, responsive web UI (Tailwind CSS, Preact) for management and viewing.
- **Storage Management:** Implement automatic retention policies and disk space management.
- **Reliability:** Ensure automatic recovery from failures and graceful shutdown.
- **Resource Optimization:** Implement stream prioritization and resource governors.

## Target Audience

- Users with resource-constrained devices (SBCs, specific SoCs).
- Users needing a lightweight NVR solution on standard Linux systems.
- DIY security camera enthusiasts.

## Non-Goals (Potentially Future Features)

- Advanced AI/Analytics beyond basic SOD integration.
- Cloud storage integration (focus is local storage).
- Support for non-Linux operating systems.
- Complex clustering or distributed features (initially).
