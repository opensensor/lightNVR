# Camera Compatibility

*Verified against LightNVR 0.37.x.*

"Will my camera work?" is the most common question about LightNVR. The short answer: if it
speaks RTSP, yes. Everything below is about the details.

---

## What LightNVR needs

| Requirement | Notes |
|---|---|
| **RTSP** | The baseline. Any camera exposing an RTSP URL can be recorded. |
| **H.264** | The primary codec. H.265 works if your hardware has the headroom to handle it. |
| **ONVIF** (optional) | Only needed for *automatic discovery* and *camera-side motion events*. A camera without ONVIF still records fine — you add it by URL. |

Ingest goes through [go2rtc](GO2RTC_INTEGRATION.md), so anything go2rtc can pull from can
in principle be used as a stream source, including sources LightNVR does not offer in its
own UI — see [go2rtc Config Override](GO2RTC_CONFIG_OVERRIDE.md) for wiring one up.

**Not supported:** USB webcams and cloud-only cameras. If a camera only talks to its
vendor's app and exposes no local RTSP, no NVR can record it — check for an "RTSP",
"ONVIF", or "local access" toggle in its settings before assuming it is locked down. Some
vendors ship it disabled by default.

---

## Finding your camera's RTSP URL

Do not guess, and be careful with the URL lists that circulate online — they go stale and
vary by firmware. Ask the camera instead, in this order:

**1. Let ONVIF discovery do it.** If the camera supports ONVIF, LightNVR will find it and
read the stream URL from the camera itself. This is the only method that cannot be out of
date. See [ONVIF Detection](ONVIF_DETECTION.md). In a container, set
`LIGHTNVR_ONVIF_NETWORK` to your camera subnet first — discovery cannot rely on multicast
from inside a bridge network.

**2. Check the camera's own web interface.** Most expose the RTSP path under a
network/stream/advanced settings page, often alongside a port setting.

**3. Probe it.** `ffprobe` will tell you what a candidate URL actually returns:

```bash
ffprobe -rtsp_transport tcp "rtsp://user:password@192.168.1.50:554/stream1"
```

If that prints stream information, LightNVR can record it. If it hangs or errors, fix the
URL before adding it — LightNVR will fail in exactly the same way.

**Sub-streams matter.** Nearly every camera publishes a high-resolution main stream and a
lower-resolution sub-stream on a different path. Recording main and running detection on
the sub-stream is the usual way to keep CPU and disk manageable.

---

## Known vendor interop notes

These are issues that were reported against specific hardware and **have been fixed** —
listed so you can tell whether a symptom you are seeing is already handled, in which case
the answer is to upgrade.

| Reported with | Symptom | Issue |
|---|---|---|
| Hikvision | ONVIF failed where the camera returned SOAP responses using the `env:` namespace prefix | [#441](https://github.com/opensensor/lightNVR/issues/441) |
| Reolink | ONVIF not working | [#374](https://github.com/opensensor/lightNVR/issues/374) |
| Tapo (C545D and likely others) | ONVIF motion detection not firing | [#333](https://github.com/opensensor/lightNVR/issues/333) |
| Lorex | Duplicate SOAP security headers rejected by the camera | [#335](https://github.com/opensensor/lightNVR/issues/335) |
| Dahua | RTSP stream interruptions causing recording gaps | [#413](https://github.com/opensensor/lightNVR/issues/413) |

ONVIF is specified loosely enough that vendors disagree on the details, which is why these
are vendor-shaped rather than general bugs. If your camera's ONVIF misbehaves but its RTSP
is fine, add it by URL and carry on — you lose auto-discovery and camera-side motion
events, nothing else.

---

## When a camera does not work

Work down this list before opening an issue — it also gathers exactly what an issue needs.

1. **Does `ffprobe` see the stream?** (command above) If not, the problem is the URL,
   credentials, or camera-side RTSP being disabled — not LightNVR.
2. **Is it reachable from where LightNVR runs?** In a container, that is the container's
   network, not your laptop's. `docker exec <container> ffprobe ...` settles it.
3. **Does it work over TCP?** Some cameras are unreliable over UDP. Force TCP and see if
   the behavior changes.
4. **Does the camera limit concurrent connections?** Many allow only two or three RTSP
   sessions. Your phone app, a browser tab, and LightNVR can add up to a refusal that
   looks like a LightNVR bug.
5. **Check the logs.** `docker logs lightnvr` or `journalctl -u lightnvr` — go2rtc's errors
   are surfaced there and usually name the cause.

### Reporting a camera

[Open an issue](https://github.com/opensensor/lightNVR/issues) with:

- Make, model, and **firmware version** — behavior varies across firmware on the same model
- Whether ONVIF, RTSP, or both are affected
- The RTSP URL with credentials removed
- `ffprobe` output for that URL
- The relevant log lines
- LightNVR version and how you are running it (container, `.deb`, add-on)

Reports in that shape are what keep the table above accurate, and they are the only way a
camera nobody here owns gets fixed.

---

## See also

- [ONVIF Detection](ONVIF_DETECTION.md) — discovery and camera-side motion events
- [go2rtc Config Override](GO2RTC_CONFIG_OVERRIDE.md) — custom stream sources
- [Zone Configuration](ZONE_CONFIGURATION.md) — restricting detection to areas of interest
- [Troubleshooting](TROUBLESHOOTING.md) — general stream and recording problems
