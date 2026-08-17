# LightNVR Documentation

Start here. Documents are grouped by what you are trying to do, in roughly the order you
will need them.

---

## Getting started

**New to LightNVR? Follow this path:**

1. **[Install it](INSTALLATION.md)** — requirements, and every installation method
   - [Docker](DOCKER.md) — the usual choice, and the reference for every container knob
   - [Home Assistant add-on](HOME_ASSISTANT.md) — for Home Assistant OS / Supervised
   - [Windows via Podman + WSL2](WINDOWS_PODMAN.md)
   - [Build from source](BUILD.md) — for embedded targets or development
2. **Log in and change the password.** The default is `admin` / `admin` and the web server
   listens on all interfaces — see the warning in [CONFIGURATION.md](CONFIGURATION.md#web-server-settings).
3. **Add a camera.** Use ONVIF discovery if your cameras support it
   ([ONVIF_DETECTION.md](ONVIF_DETECTION.md)); otherwise add the RTSP URL by hand.
   [CAMERAS.md](CAMERAS.md) covers what LightNVR needs from a camera and how to find its
   RTSP URL reliably.
4. **Confirm it is recording,** then set a retention policy before your disk fills —
   [CONFIGURATION.md](CONFIGURATION.md) covers storage and retention settings.
5. **Optional: turn on detection.** [ZONE_CONFIGURATION.md](ZONE_CONFIGURATION.md) for
   where to look, [SOD_INTEGRATION.md](SOD_INTEGRATION.md) for what does the looking.

If something goes wrong at any step, [TROUBLESHOOTING.md](TROUBLESHOOTING.md) is the
general guide and [TROUBLESHOOTING_WEB_INTERFACE.md](TROUBLESHOOTING_WEB_INTERFACE.md)
covers a blank or broken web UI specifically.

---

## Install

| Document | What it covers |
|---|---|
| [INSTALLATION.md](INSTALLATION.md) | Requirements, all install methods, per-platform notes, upgrading, uninstalling |
| [DOCKER.md](DOCKER.md) | Container reference: volumes, networking, environment variables, first-run behavior |
| [HOME_ASSISTANT.md](HOME_ASSISTANT.md) | The Home Assistant add-on, and how it differs from plain Docker |
| [WINDOWS_PODMAN.md](WINDOWS_PODMAN.md) | Windows via Podman + WSL2 |
| [BUILD.md](BUILD.md) | Building from source, build flags, cross-compilation |

## Configure

| Document | What it covers |
|---|---|
| [CONFIGURATION.md](CONFIGURATION.md) | `lightnvr.ini` — every section and key |
| [ZONE_CONFIGURATION.md](ZONE_CONFIGURATION.md) | Polygon detection zones, class filters, thresholds |
| [REVERSE_PROXY.md](REVERSE_PROXY.md) | Running behind nginx / Caddy / Traefik, HTTPS, `trusted_proxy_cidrs` |
| [GO2RTC_CONFIG_OVERRIDE.md](GO2RTC_CONFIG_OVERRIDE.md) | Overriding the generated go2rtc configuration |

## Operate

| Document | What it covers |
|---|---|
| [TROUBLESHOOTING.md](TROUBLESHOOTING.md) | General problems: streams, recording, storage, auth |
| [TROUBLESHOOTING_WEB_INTERFACE.md](TROUBLESHOOTING_WEB_INTERFACE.md) | Blank page, 404s, asset and login problems |
| [SYSLOG.md](SYSLOG.md) | Sending logs to syslog / journald |
| [QUICKREF_Retention_API.md](QUICKREF_Retention_API.md) | Retention API quick reference |

## Integrate

| Document | What it covers |
|---|---|
| [API.md](API.md) | The REST API |
| [MQTT_INTEGRATION.md](MQTT_INTEGRATION.md) | Publishing detection events, with Home Assistant examples |
| [CAMERAS.md](CAMERAS.md) | Camera compatibility, finding RTSP URLs, vendor interop notes |
| [ONVIF_DETECTION.md](ONVIF_DETECTION.md) | ONVIF discovery and camera-side motion events |
| [GO2RTC_INTEGRATION.md](GO2RTC_INTEGRATION.md) | How LightNVR drives go2rtc |
| [SOD_INTEGRATION.md](SOD_INTEGRATION.md) | Embedded detection with SOD |
| [MOTION_BUFFER.md](MOTION_BUFFER.md) | Pre-detection buffering, so events include what led up to them |

## Develop

| Document | What it covers |
|---|---|
| [ARCHITECTURE.md](ARCHITECTURE.md) | How the whole system fits together |
| [FRONTEND.md](FRONTEND.md) | The Preact/Tailwind web UI |
| [RELEASE_PROCESS.md](RELEASE_PROCESS.md) | Cutting a release |
| [prd/](prd/) | Product requirement documents for larger pieces of work |
| [internal/](internal/) | Design notes and historical implementation writeups |

## Other

- [COMMERCIAL.md](COMMERCIAL.md) — commercial licensing and professional support

---

## Conventions

- **User-facing documents live in `docs/`.** If a document only makes sense to someone
  working on LightNVR's internals, it belongs in [`internal/`](internal/).
- **Documents describe what the code does today.** A document that describes a past change
  ("we refactored X", "the fix for Y") is a historical note — put it in `internal/`, where
  nobody will mistake it for current guidance.
- **When a change alters a default, a port, a path, or a credential, the documentation
  changes in the same PR.** See [RELEASE_PROCESS.md](RELEASE_PROCESS.md#documentation-checkpoint).
