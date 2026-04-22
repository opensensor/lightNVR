# go2rtc Config Override

LightNVR generates a `go2rtc.yaml` from your stream definitions and starts go2rtc with it.  When you need to tune go2rtc beyond what the Settings page exposes — custom ffmpeg templates, trace logging, MQTT bridges, ICE/TURN servers, HomeKit, ngrok tunnels, and so on — you supply a **config override**.

## How overrides are applied

LightNVR writes **two** YAML files into `${go2rtc_config_dir}` (default `/etc/lightnvr/go2rtc`):

| File | Owner | Contents |
| --- | --- | --- |
| `go2rtc.yaml` | LightNVR | Auto-generated `api`, `rtsp`, `webrtc`, `ffmpeg`, and `streams` sections derived from your Streams and Settings. Regenerated on every start. |
| `override.yaml` | You (Settings → "go2rtc Config Override") | Whatever you save in the textarea. Mode 0600. Removed when the field is empty. |

go2rtc is started with both files:

```
go2rtc --config /etc/lightnvr/go2rtc/go2rtc.yaml --config /etc/lightnvr/go2rtc/override.yaml
```

go2rtc loads them in order, calling `yaml.Unmarshal` once per file onto the same struct.  This means your override is merged by go2rtc's own parser, not by lightNVR — there's nothing to escape, no string concatenation, no duplicate-key risk.

> **Why two files?** Earlier versions appended your override to the bottom of `go2rtc.yaml`.  If your override touched any section lightNVR also wrote (`ffmpeg`, `log`, `streams`, …), the resulting file had duplicate top-level keys and `gopkg.in/yaml.v3` rejected the whole document — your override was silently dropped.  Two files lets each parse cleanly on its own.

## Merge semantics

We verified the runtime merge behavior with the actual go2rtc binary in `tests/unit/test_go2rtc_two_config_merge.c`:

- **Mappings** are merged key-by-key. Top-level scalars in `override.yaml` override scalars in `go2rtc.yaml`.
- **Nested maps** (e.g., `ffmpeg.h264`, `mqtt.host`) deep-merge.
- **The `streams:` map specifically** is merged: cameras you add in `override.yaml` appear alongside lightNVR's auto-generated cameras.  But if you redefine a stream name lightNVR already wrote (`streams.cam1`), your version **wins** (replace, not list-append).
- **Sequences** (e.g., `webrtc.ice_servers`, `publish.<name>`) are **replaced**, not appended.  If you set `webrtc.ice_servers` in your override, lightNVR's defaults are gone — list every server you want.

## Supported sections

Anything go2rtc accepts works in your override.  These are the top-level keys we know about; an unknown key is allowed (forward-compat) but produces a warning:

| Section | Purpose |
| --- | --- |
| `api` | HTTP/WS API server: `listen`, `username`, `password`, `base_path`, `tls_listen`, `tls_cert`, `tls_key`, `unix_listen` |
| `rtsp` | RTSP server: `listen`, `username`, `password`, `default_query` |
| `webrtc` | WebRTC: `listen`, `candidates`, `ice_servers`, `filters` |
| `ffmpeg` | Transcoding: `bin`, `global`, `timeout`, `h264`, `h265`, `opus`, custom codec/input templates |
| `log` | Logging: `level`, `format`, `output`, plus per-module overrides (`api`, `rtsp`, …) |
| `streams` | Stream definitions — **merges with lightNVR streams; redefining a name replaces** |
| `publish` | RTMP/S push destinations |
| `hass` | Home Assistant auto-import |
| `mqtt` | MQTT bridge |
| `hls` | HLS output server |
| `srtp` | SRTP server |
| `homekit` | HomeKit accessory bridge |
| `ngrok` / `pinggy` | Tunnels |
| `echo` | Dynamic-URL shell expansion |
| `preload` | Auto-start streams (name → filter spec) |
| `app` | Module enable/disable |

## Validation

Two tiers of checks happen before your override ever reaches go2rtc:

1. **Live, on-blur** — the Settings UI POSTs to `/api/settings/go2rtc/validate` 200ms after you stop typing.  Errors are inlined under the textarea with line/column.
2. **Pre-save** — `POST /api/settings` runs the same validator and rejects with HTTP 400 if it fails.

The validator catches:

- YAML syntax errors (line/column reported)
- **Duplicate top-level keys** — `gopkg.in/yaml.v3` rejects these but libyaml-C silently accepts them, so we walk the event stream ourselves
- Non-mapping root (sequence or scalar at top level)
- Unknown top-level sections (warning, not error)

When libyaml isn't compiled into the lightNVR build, validation is skipped and the UI shows a "validation skipped" badge — go2rtc itself will still error out at startup if the YAML is malformed.

## Effective-config preview

Click **Show effective config** in the Settings page (or `GET /api/system/go2rtc/effective-config`) to see exactly what go2rtc was started with.  Both files are returned with secrets redacted via a YAML-aware walker:

- `api.password`, `api.username`, `rtsp.password`, `rtsp.username`, `mqtt.password` — replaced with `<redacted>`
- `webrtc.ice_servers[*].credential` and `[*].username` — replaced with `<redacted>`
- `streams.*` URL userinfo — `rtsp://user:pass@host` becomes `rtsp://<redacted>@host` (host/path stay visible for diagnostics)

Block-scalar passwords (`password: |\n  secret`) are caught the same as inline scalars — the walker reads libyaml's resolved scalar value, not the source text.  When libyaml isn't available, redaction is a no-op pass-through and the response sets `redaction_available: false`.

## Crash-loop quarantine

A semantically-valid-but-runtime-broken override (e.g. `api: { listen: ":99999" }`) can put go2rtc into a crash loop.  LightNVR detects this:

- Track per-instance lifetime (`g_last_start_time`).  An exit within 10s of start counts as a "fast death."
- If 3+ fast-deaths happen within a 60s window AND `override.yaml` is in use, lightNVR:
  1. Renames `override.yaml` → `override.quarantined.yaml`
  2. Persists the last 2KB of `go2rtc.log` plus a header to `system_settings.go2rtc_config_override_disabled_reason`
  3. Restarts go2rtc with base config only
- The Settings page surfaces the reason as a yellow banner above the editor with a one-click "Restore quarantined version into the editor" button.
- Saving any new value to the override field clears the quarantine.

## Upgrade-time quarantine

The first boot after each lightNVR release validates your existing override against the current rules.  If it fails (most commonly because of an old append-era duplicate-key shape that was previously silently ignored):

- The original is copied to `system_settings.go2rtc_config_override_quarantined`
- The live `go2rtc_config_override` is cleared
- The same banner appears, with the "Restore" button to repopulate the editor

A `go2rtc_override_validated_version` marker prevents re-checking on subsequent boots.

## Troubleshooting

| Symptom | Check |
| --- | --- |
| "No go2rtc binary available" | LightNVR probes `/bin/go2rtc`, `/usr/local/bin/go2rtc`, `/usr/bin/go2rtc`, `/opt/go2rtc/go2rtc`, `/rootfs/usr/local/go2rtc/go2rtc`, `/go2rtc`, then `$PATH`. Each candidate is run as `<path> --version` (2 s timeout) and accepted only if stdout contains `go2rtc version `. The Settings → "go2rtc Binary Path" field overrides the probe. |
| Override saves but doesn't take effect | Check the banner — your override may be quarantined. Click "Show effective config" to confirm `override.yaml` is listed under `merged_source_order`. |
| `"port already in use"` after override save | Your override likely sets `api.listen` or `rtsp.listen` to a port another process holds. T4b will auto-quarantine after 3 fast crashes; check the banner. |
| ICE/STUN settings ignored | Sequences are replaced, not merged. Your `webrtc.ice_servers` overwrites lightNVR's defaults — list every server you want. |
| Block-scalar password not redacted in preview | Make sure libyaml is installed in your runtime container; the response field `redaction_available` tells you. The Docker images ship with `libyaml` and `libyaml-dev`. |

## Examples

### Enable trace logging
```yaml
log:
  level: trace
```

### Pass camera streams through without re-encoding
```yaml
ffmpeg:
  h264: "-codec:v copy -codec:a copy"
  h265: "-codec:v copy -codec:a copy"
```

### Bridge to MQTT
```yaml
mqtt:
  host: mqtt.example.com
  port: 1883
  username: lightnvr
  password: hunter2
```

### Add a TURN server
```yaml
webrtc:
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]
    - urls: [turn:turn.example.com:3478]
      username: USER
      credential: PASS
```

## Reference

- Source: [`src/utils/yaml_validate.c`](../src/utils/yaml_validate.c), [`src/utils/yaml_redact.c`](../src/utils/yaml_redact.c), [`src/video/go2rtc/go2rtc_process.c`](../src/video/go2rtc/go2rtc_process.c)
- Tests: [`tests/unit/test_yaml_validate.c`](../tests/unit/test_yaml_validate.c), [`tests/unit/test_yaml_redact.c`](../tests/unit/test_yaml_redact.c), [`tests/unit/test_go2rtc_two_config_merge.c`](../tests/unit/test_go2rtc_two_config_merge.c)
- Upstream: [go2rtc wiki](https://github.com/AlexxIT/go2rtc/wiki) for per-section deep dives
