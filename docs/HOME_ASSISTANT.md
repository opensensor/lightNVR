# LightNVR on Home Assistant

*Verified against LightNVR 0.37.x.*

If you run Home Assistant OS or Supervised, LightNVR is available as an add-on. You get it
from the Add-on Store, it starts with the rest of your system, its config and database ride
along in your Home Assistant backups, and its detections can drive automations.

The add-on lives in its own repository:
**[opensensor/lightnvr-hassio-addons](https://github.com/opensensor/lightnvr-hassio-addons)**

[![Open your Home Assistant instance and add this repository.](https://my.home-assistant.io/badges/supervisor_add_addon_repository.svg)](https://my.home-assistant.io/redirect/supervisor_add_addon_repository/?repository_url=https%3A%2F%2Fgithub.com%2Fopensensor%2Flightnvr-hassio-addons)

---

## Is the add-on the right choice?

| | Add-on | Plain Docker |
|---|---|---|
| Home Assistant OS / Supervised | ✅ Use this | Not possible — no Docker access |
| Home Assistant Container / Core | Not available | ✅ [DOCKER.md](DOCKER.md) |
| Separate machine or NAS | — | ✅ [DOCKER.md](DOCKER.md) |

An NVR is I/O-heavy and runs continuously. If your Home Assistant box is a small Pi that is
already busy, running LightNVR somewhere else and integrating over
[MQTT](#feeding-detections-into-home-assistant) is the better setup — the add-on is a
convenience, not a requirement.

**Architecture note:** the add-on is built for `amd64` and `aarch64` only. Home Assistant
dropped 32-bit ARM in 2025.12, so a 32-bit Pi OS install cannot run it. The upstream image
still publishes `arm/v7` for plain Docker users.

## Install

1. **Settings → Add-ons → Add-on Store**
2. ⋮ menu (top right) → **Repositories**
3. Add `https://github.com/opensensor/lightnvr-hassio-addons`

   (or click the badge above, which does steps 1–3 for you)
4. Find **LightNVR** in the store → **Install** → **Start**
5. **Open Web UI**

The install pulls the published multi-arch image and layers a thin launcher on it. No
compilation — a minute or two depending on your connection.

> ⚠️ **The default credentials are `admin` / `admin`, and the add-on runs with host
> networking**, so the web UI is reachable from anywhere on your LAN the moment it starts.
> Change the password under **Settings → Users** before you do anything else.

## What the add-on does differently

If you have read [DOCKER.md](DOCKER.md), these are the four places the add-on deviates.
Everything else about LightNVR behaves identically.

**Port 7800, not 8080.** 8080 is contended on a typical Home Assistant host, so the add-on
defaults its web UI to 7800. The **Open Web UI** button is pinned to that default — if you
change `web_port`, browse to `http://<host-ip>:<your-port>/` directly instead.

**Host networking.** Required, not cosmetic: ONVIF WS-Discovery and mDNS are multicast and
do not survive a bridge network, and go2rtc's WebRTC candidates need to be directly
reachable for live view to connect. The consequence is that LightNVR's ports appear on the
host itself — 7800, 8554 RTSP, 8555 TCP+UDP WebRTC, and 1984 for the go2rtc API.

**Recordings live outside backups.** This is the important one:

| Data | Location | In Home Assistant backups |
|---|---|---|
| Config, go2rtc config | add-on `/data/config` | ✅ Yes |
| SQLite database | add-on `/data/lib/database` | ✅ Yes |
| Detection models | add-on `/data/lib/models` | ✅ Yes |
| **Recordings** | **`/media/lightnvr/recordings`** | ❌ **No** |

Recordings go under Home Assistant's `media` folder deliberately — a week of continuous
video would otherwise land in every backup you take. Your settings and database *are*
backed up. Recordings survive updates and even an uninstall; delete `/media/lightnvr`
by hand if you want the space back.

**Camera discovery needs your subnet.** If ONVIF discovery comes up empty, set the
`onvif_network` option to your camera network in CIDR form (`192.168.1.0/24`) and restart.

## Configuration

Add-on options are documented in
**[the add-on's DOCS.md](https://github.com/opensensor/lightnvr-hassio-addons/blob/main/lightnvr/DOCS.md)**
— `web_port`, `max_streams`, `go2rtc_config_persist`, `onvif_network`, and `timezone`. That
page is the authoritative reference and tracks the add-on's own releases.

Everything else — streams, detection, zones, retention — is configured inside LightNVR's
web UI, exactly as documented in [CONFIGURATION.md](CONFIGURATION.md) and
[ZONE_CONFIGURATION.md](ZONE_CONFIGURATION.md). The add-on options only cover what has to
be decided before the process starts.

## Feeding detections into Home Assistant

Installing the add-on gets you a recorder next to Home Assistant; it does not by itself
make Home Assistant *aware* of anything. For that, point LightNVR's MQTT integration at
your broker (the Mosquitto add-on works) and detections become events you can automate on —
turn on lights when a person is detected, send a notification with the snapshot, and so on.

See **[MQTT Integration → Home Assistant](MQTT_INTEGRATION.md#home-assistant)** for the
broker setup, topic layout, and ready-made automation and MQTT-camera examples.

## Updating

Add-on updates appear in the Add-on Store like any other. Config, database, and recordings
all live on Home Assistant-managed storage, so they carry across updates untouched.

## Getting help

- **Something wrong with the NVR** — streams, recording, detection, the web UI:
  [opensensor/lightnvr/issues](https://github.com/opensensor/lightnvr/issues)
- **Something wrong with the packaging** — install fails, add-on will not start, options
  not applying: [opensensor/lightnvr-hassio-addons](https://github.com/opensensor/lightnvr-hassio-addons)

The add-on **Log** tab is the first place to look either way; the launcher prints the
resolved storage paths on start. [TROUBLESHOOTING.md](TROUBLESHOOTING.md) applies to
everything above the packaging layer.
