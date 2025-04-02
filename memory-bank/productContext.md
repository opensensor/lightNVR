# Product Context: LightNVR

## Problem Solved

Standard NVR software often requires significant system resources (CPU, RAM), making it unsuitable for low-power, resource-constrained devices like Single Board Computers (SBCs) or specific System-on-Chips (SoCs) like the Ingenic A1. Users with such hardware need a reliable NVR solution that operates efficiently within tight memory limits (e.g., 256MB RAM).

LightNVR addresses this by providing a lightweight, memory-optimized NVR specifically designed for these environments, while still being usable on more powerful Linux systems.

## How It Should Work (User Experience)

- **Setup:** Simple installation (build from source or Docker) and straightforward configuration via a text file (`/etc/lightnvr/lightnvr.conf`) or the web UI.
- **Management:** Users should be able to easily add, configure, and manage IP camera streams (RTSP/ONVIF) through the web interface.
- **Live Viewing:** Provide a stable live view of connected camera streams via the web interface (HLS or MJPEG).
- **Recording:** Reliably record streams to local storage (MP4/MKV) based on user configuration (continuous or event-based if detection is enabled).
- **Playback:** Allow users to browse, search, and playback recorded footage through the web interface.
- **Storage:** Automatically manage disk space by deleting old recordings based on retention policies.
- **Performance:** The system should remain responsive and stable even when handling multiple streams on low-resource hardware. Resource usage (CPU, RAM) should be minimized.
- **Reliability:** The NVR should automatically recover from network interruptions or system restarts, ensuring continuous operation with minimal data loss.
- **Detection (Optional):** If enabled, provide basic motion or object detection capabilities (via SOD) and allow recording based on these events.

## Core User Goals

- Monitor IP cameras reliably on low-power hardware.
- Record footage continuously or based on events without overwhelming the system.
- Easily access live views and recorded footage via a web browser.
- Configure and manage the NVR system with minimal complexity.
- Trust the system to run unattended for long periods.
