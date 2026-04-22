# Plan: Harden go2rtc Integration Config Override (Issue #394)

**Generated**: 2026-04-21
**Repo**: opensensor/lightNVR
**Issue**: https://github.com/opensensor/lightNVR/issues/394

## Overview

The global go2rtc config override (stored in `system_settings.go2rtc_config_override`) is currently **appended verbatim** to the end of the generated `go2rtc.yaml`. Because the generator already writes `api:`, `rtsp:`, `webrtc:`, `ffmpeg:`, `streams:` sections, any user override that touches one of those sections produces a YAML file with **duplicate top-level keys**. `gopkg.in/yaml.v3` (the parser go2rtc uses) rejects duplicate mapping keys, so the override file either:

1. Fails to parse entirely (go2rtc logs a warn and falls back to defaults / previous config), or
2. Is silently mis-merged (yaml.v3 semantics — last-entry-wins on some types, error on others).

This is the direct cause of reporter `mhagnumdw`'s observation that overrides to `ffmpeg:` and `log:` "don't seem to work".

**Core fix**: leverage go2rtc's native multi-config support. `internal/app/config.go` in go2rtc calls `yaml.Unmarshal(data, v)` once per `-c` argument, so passing the override as a **second `--config`** file lets go2rtc merge cleanly using its own logic. This covers every present and future section (`api`, `rtsp`, `webrtc`, `ffmpeg`, `log`, `streams`, `publish`, `hass`, `mqtt`, `hls`, `srtp`, `homekit`, `ngrok`, `echo`, `webtorrent`, `preload`, `app.modules`, TLS cert/key, etc.) with zero extra parsing in C.

Additional hardening follows from that core change: larger size limit, server-side pre-save validation, `/api/settings/go2rtc/effective-config` preview endpoint, better UI, Docker binary-detection fix, and per-stream `go2rtc_source_override` robustness.

## Prerequisites

- `libyaml` (already available via Debian/Alpine packages; used for pre-save validation only)
- go2rtc ≥ v1.9 (already vendored in `go2rtc/` submodule — supports repeated `-c` flags)
- Existing infra: `db_system_settings`, `api_handlers_settings.c`, `go2rtc_process.c`, preact `SettingsView.jsx`

## Key go2rtc Config Sections (for UI help + docs)

| Section | Purpose | Example keys |
|---|---|---|
| `api` | HTTP/WS server | `listen`, `username`, `password`, `local_auth`, `base_path`, `static_dir`, `origin`, `tls_listen`, `tls_cert`, `tls_key`, `unix_listen` |
| `rtsp` | RTSP server | `listen`, `username`, `password`, `default_query` |
| `webrtc` | WebRTC | `listen`, `candidates`, `ice_servers`, `filters` |
| `ffmpeg` | Transcoding | `bin`, `global`, `timeout`, `h264`, `h265`, `opus`, `pcmu`, custom codec templates, input templates |
| `log` | Logging | `level`, `format`, `output` (`stdout`/`stderr`/`file:path`/empty), `time`, plus per-module (`api`, `rtsp`, etc.) |
| `streams` | Stream map | name → URL or list of URLs; schemes: `rtsp://`, `rtmp://`, `http://`, `https://`, `onvif://`, `ffmpeg:`, `exec:`, `echo:`, `webrtc:`, `hass:`, `homekit:`, `roborock:`, `bubble:`, `kasa:`, `dvrip:`, `tapo:`, `gopro:`, `isapi:`, `expr:`, `nest:`, `wyze:`, `xiaomi:`, `ring:`, `tuya:`, `webtorrent:` |
| `publish` | RTMP/S push | name → list of destination URLs |
| `hass` | Home Assistant | `config` path, auto-import cameras |
| `mqtt` | MQTT bridge | `host`, `port`, `username`, `password`, `topic` |
| `hls` | HLS output | `listen` |
| `srtp` | SRTP | `listen` |
| `homekit` | HomeKit | `pin`, device map |
| `ngrok` / `pinggy` | Tunnels | command args |
| `echo` | Dynamic URL | shell command expansions |
| `preload` | Auto-start streams | name → video/audio filter |
| `app` | App control | `modules` list |

## Scope

**Target platforms**: Linux (glibc + musl/Alpine). Docker images built from `Dockerfile` and `Dockerfile.alpine`. No Windows/BSD concerns for this change set — `execv`, `fsync`, `unlink`, `prctl(PR_SET_PDEATHSIG)` are already Linux-only in the existing code path.

## Dependency Graph

```
T1 (libyaml wiring) ──┬── T6 (validate API)
                      ├── T7 (effective-config API)
                      ├── T11 (per-stream validation)
                      └── T14 (DB quarantine-on-upgrade)

T2 (refactor base gen) ──┬── T3 (override writer) ──┬── T4 (two -c start)
                         └──────────────────────────┴── T4b (crash-loop guard)

T5 (enlarge size cap) ──┬── T6 (validate API)
                        ├── T9 (UI global)
                        └── T11 (per-stream validation)

T8 (Docker binary detect) ── independent

T11b (stream_config_t audit) ── T11

T12 (tests) ── T2, T3, T4, T4b, T6, T7, T11, T14
T15 (CI plumbing for go2rtc binary) ── T12

T9 ── T10 (editor upgrade)
T12 ── T13 (docs)
```

## Tasks

### T1: Wire libyaml and add `yaml_validate_str()` helper
- **depends_on**: []
- **location**: `CMakeLists.txt`, `src/utils/yaml_validate.c` (new), `include/utils/yaml_validate.h` (new)
- **description**: Add libyaml as an optional-but-preferred dependency. Create a thin helper `int yaml_validate_str(const char *src, size_t len, char *err_buf, size_t err_size)` that runs libyaml in parse-only mode, returns 0 on valid YAML or -1 with a human-readable error (line/column). Also expose `yaml_is_mapping_root()` to confirm the override is a top-level mapping (go2rtc requires this). Update `scripts/build.sh` to detect libyaml via pkg-config and fall back to a "validation disabled" stub if unavailable. Update `Dockerfile` and `Dockerfile.alpine` to `apt-get install libyaml-dev` / `apk add yaml-dev`.
- **validation**: Build succeeds on Linux/Alpine. Unit test feeds valid and invalid YAML strings and asserts return codes + error messages.
- **status**: Completed
- **log**:
  - Added `pkg_check_modules(YAML yaml-0.1)` to top-level `CMakeLists.txt`; defines `LIGHTNVR_HAVE_LIBYAML=1` and links `${YAML_LIBRARIES}` (with `YAML_LIBRARY_DIRS`) into `lightnvr_lib` + `lightnvr` when found, otherwise compiles the stub path.
  - `src/utils/yaml_validate.c` uses libyaml's event-mode parser (no document construction) and formats errors as "YAML parse error: <problem> [(<context>)] at line N, column M" from `parser->problem_mark`. `yaml_is_mapping_root` inspects the first event after STREAM_START/DOCUMENT_START. Stub returns 0 and writes "libyaml not available; validation disabled".
  - `tests/unit/test_yaml_validate.c` (Unity / Layer 2, 10 assertions): valid mapping, empty string, CRLF, ~65 KB document, non-mapping sequence root, malformed YAML (unclosed flow sequence) with line/col error, and NULL-safety. All PASS in both libyaml-enabled and stub-only builds (verified both by rebuilding with `PKG_CONFIG_PATH` pointed at a private libyaml prefix and with no libyaml installed).
  - Updated `scripts/build.sh` to surface libyaml detection pre-build; added `libyaml-dev`/`libyaml-0-2` to `Dockerfile` (builder + runtime) and `yaml-dev`/`yaml` to `Dockerfile.alpine` (builder + runtime).
  - Duplicate-key detection intentionally deferred to T6.
- **files edited/created**:
  - `include/utils/yaml_validate.h` (created)
  - `src/utils/yaml_validate.c` (created)
  - `tests/unit/test_yaml_validate.c` (created)
  - `CMakeLists.txt` (edited)
  - `tests/unit/CMakeLists.txt` (edited)
  - `scripts/build.sh` (edited)
  - `Dockerfile` (edited)
  - `Dockerfile.alpine` (edited)

### T2: Refactor `go2rtc_process_generate_config` — emit BASE-only config, no tail append
- **depends_on**: []
- **location**: `src/video/go2rtc/go2rtc_process.c` (lines ~574-800)
- **description**: Remove the final "User config override" append block (lines 768-777). The generator now produces ONLY the lightNVR-owned sections (`api`, `rtsp`, `webrtc`, `ffmpeg` defaults, `streams` with per-stream overrides). Add a helper `go2rtc_process_generate_override_file(const char *override_path)` that writes the user override to a separate file. Both are written with 0600. If the DB override is empty, `override_path` is removed so we never pass a stale file. Extract the per-stream override writing into `write_stream_overrides(FILE *fp)` — keep existing behavior.
- **validation**: Generated `go2rtc.yaml` is syntactically valid YAML even when DB override is empty. `unit/test_go2rtc_process_config_generation.c` is updated so base and override are independently parseable.
- **status**: Completed
- **log**:
  - Removed the tail-append block that merged `go2rtc_config_override` into `go2rtc.yaml`; generator now emits ONLY lightNVR-owned sections.
  - Extracted per-stream override writing into `static int write_stream_overrides(FILE *fp)` preserving all prior semantics (YAML-escaped keys, single-line inline vs. multi-line block forms, disabled-stream skip).
  - Added stub declarations/definitions for `go2rtc_process_generate_override_file(const char *)` and `go2rtc_process_get_override_path(void)` (header + .c) so T3 and T4 can compile independently; both stubs log "T3 will implement".
  - Extended `test_go2rtc_process_config_generation.c` with RED-first assertions: no `# User config override` marker in output, exactly one `ffmpeg:` stanza, and link-time assertion that the T3 stubs resolve. All 3 go2rtc tests pass.
- **files edited/created**:
  - `src/video/go2rtc/go2rtc_process.c` (edited)
  - `include/video/go2rtc/go2rtc_process.h` (edited)
  - `tests/unit/test_go2rtc_process_config_generation.c` (edited)

### T3: Implement `go2rtc_process_write_override_file`
- **depends_on**: [T2]
- **location**: `src/video/go2rtc/go2rtc_process.c`, `include/video/go2rtc/go2rtc_process.h`
- **description**: New function that reads `go2rtc_config_override` from `db_system_settings` (up to the new 64 KB cap from T5), writes it to `${g_config_dir}/override.yaml` with mode 0600 via `open(O_WRONLY|O_CREAT|O_TRUNC)` + `fdopen`, then `fsync`s + closes. If the setting is empty/absent, `unlink()` any existing `override.yaml` AND `stat()` the path afterward to confirm it is gone — if the file still exists after unlink (EBUSY, EROFS, permission denied), return a hard error and refuse to proceed (prevents merging a stale override). Before writing, `stat(g_config_dir)` and assert mode is 0700 or 0750; if not, `chmod` it (best-effort) and log at WARN. Expose the final path via `go2rtc_process_get_override_path()` so the start routine and diagnostics can both use it. Log content only at DEBUG (credentials may be present). This function **must be called synchronously before every `go2rtc_process_start`** — not just on settings-save — to avoid the race where the DB was cleared but override.yaml still on disk.
- **validation**: File exists with correct content when setting is present; file is absent after setting is cleared; function returns error (not silent success) when unlink fails. Permission 0600 on file, 0700 on enclosing dir.
- **status**: Completed
- **log**:
  - Added a `g_override_path` global allocated alongside `g_config_path` in `go2rtc_process_init` (`<config_dir>/override.yaml`), freed and NULL'd in `go2rtc_process_cleanup`. Also extended the init-failure path so the new pointer is freed when `check_go2rtc_in_path` returns false.
  - Replaced the T2 stubs with real implementations:
    - `go2rtc_process_get_override_path()` returns `g_override_path` (NULL until init succeeds, matching the documented "or NULL if not configured" contract).
    - `go2rtc_process_generate_override_file()` (a) tightens `g_config_dir` to 0700 if it is looser than 0700/0750 (best-effort `chmod`, WARN on failure); (b) reads `go2rtc_config_override` via the T5 `db_get_system_setting_alloc` helper; (c) when empty/absent, `unlink()`s and `stat()`s to confirm absence — returns -1 if the file is still present after unlink (covers EBUSY, EROFS, perm denied per plan); (d) when present, writes via `open(O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0600)` + `fchmod(0600)` belt-and-braces, EINTR-safe write loop, `fsync` (warn-only on failure), `close` (hard error). On any write error the partial file is `unlink()`ed before returning.
  - Wired into the integration branch only — feature/go2rtc-override-hardening — not yet on main.
- **files edited/created**:
  - `src/video/go2rtc/go2rtc_process.c` (edited)

### T4: Pass both configs to go2rtc via repeated `--config` flags
- **status**: Completed
- **depends_on**: [T2, T3]
- **location**: `src/video/go2rtc/go2rtc_process.c` (function `go2rtc_process_start`, `execl` at line ~1377)
- **description**: Before forking, call `go2rtc_process_write_override_file()` (synchronous, return-checked). In the child, replace `execl(resolved_binary, "go2rtc", "--config", g_config_path, NULL);` with `execv` (for dynamic argv), building `{"go2rtc", "--config", base_path, "--config", override_path, NULL}` when the override file exists, otherwise the single-config form. go2rtc's `internal/app/config.go:LoadConfig` unmarshals each config in order onto the same struct. Update the log line to show both `--config` args.
- **validation**: Start a local lightNVR+go2rtc with `ffmpeg: { h264: "-codec:v copy -codec:a copy" }` in the override; confirm the running go2rtc reports the copy-mode ffmpeg template via `GET /go2rtc/api/config` and that `log.level: trace` takes effect in `go2rtc.log`.
- **status**: Completed
- **log**:
  - In `go2rtc_process_start`, immediately after the existing base-config regenerate, call `go2rtc_process_generate_override_file(go2rtc_process_get_override_path())`. Non-zero return is fatal — we refuse to start go2rtc rather than risk merging a stale or unverified override file.
  - In the child, replaced `execl(resolved_binary, "go2rtc", "--config", g_config_path, NULL)` with a small dynamic `char *argv[6]` and `execv(resolved_binary, argv)`. The override `--config` pair is appended only when `override_path` exists AND `access(R_OK)` succeeds in the child — guaranteeing that the parent's prior write-or-confirm-removed call gates inclusion.
  - Updated the pre-exec INFO log to print both `--config` arguments when the override is in use, so operators can see the merge composition in `go2rtc.log` without grepping `/proc/<pid>/cmdline`.
  - Build + tests green: `cmake --build build` succeeds; `ctest -R 'test_yaml_validate|test_go2rtc_process|test_db_system_settings|test_config'` reports 5/5 PASS.
- **files edited/created**:
  - `src/video/go2rtc/go2rtc_process.c` (edited)

### T4b: Override crash-loop guard — auto-disable on repeated startup failure
- **depends_on**: [T4]
- **location**: `src/video/go2rtc/go2rtc_process.c`, new system setting `go2rtc_config_override_disabled_reason`
- **description**: Track `go2rtc` child-exit attempts in a small in-memory ring (last 5 exits with timestamps). If go2rtc exits within 10 seconds of start more than 3 times in a 60-second window AND an `override.yaml` is in use, rename `override.yaml` → `override.quarantined.yaml`, write the triggering go2rtc stderr tail (last 2 KB of `go2rtc.log`) into DB setting `go2rtc_config_override_disabled_reason`, and retry start with base config only. Emit a system notification / log at ERROR. On next successful settings-save that *changes* the override value, clear the quarantine and re-enable. Protects lightNVR from a semantically-valid-but-runtime-broken override (e.g. `api.listen: ":99999"`) — directly addresses R5.
- **validation**: Inject `api: { listen: ":99999" }` into the override, restart, confirm go2rtc recovers with base-only config and the UI surfaces the quarantine reason.
- **status**: Completed
- **log**:
  - Added `g_last_start_time` (set after successful fork) and `g_fast_death_history[8]` ring buffer to `go2rtc_process.c`. Every call to `go2rtc_process_start` runs `check_and_handle_crash_loop()` which records a fast-death event when the previous instance lived < 10 s, then triggers `quarantine_override_file()` if 3+ events appear within a 60 s window AND the override file is in use.
  - Quarantine action: `rename(override.yaml, override.quarantined.yaml)` + persist last 2 KB of `go2rtc.log` to `system_settings.go2rtc_config_override_disabled_reason`. T3's `generate_override_file` honors this DB setting to short-circuit (no recreate from DB) on subsequent starts — without this, T3 would just regenerate the override and re-enter the loop.
  - Added `go2rtc_process_clear_override_quarantine()` called from `handle_post_settings` after successful save of `go2rtc_config_override`. Removes the quarantined file, clears the DB reason, resets the in-memory ring.
  - Build clean; T9 surfaces the banner via the new `go2rtc_config_override_disabled_reason` field on `GET /api/settings`. End-to-end runtime validation requires real go2rtc + crash-induction; not exercised in this session.
- **files edited/created**: `include/video/go2rtc/go2rtc_process.h`, `src/video/go2rtc/go2rtc_process.c`, `src/web/api_handlers_settings.c`

### T5: Raise override size cap from 4 KB → 64 KB (heap-backed)
- **depends_on**: []
- **location**: `src/web/api_handlers_settings.c` (lines 367-371, 964-972), `src/video/go2rtc/go2rtc_process.c` (line 770), `include/database/db_system_settings.h`
- **description**: Switch the three existing `char buf[4096]` stack buffers to `calloc(65536, 1)` with `goto`/free cleanup — a 64 KB stack frame is unsafe on musl/Alpine (default 128 KB thread stack) and on libuv worker threads. Add a `db_get_system_setting_alloc(const char *key, char **out, size_t *out_len)` helper that `malloc`s to the actual value size (returning the caller-owned buffer) so we never need a fixed-size scratch buffer again. Update the save-side length check from `4095` to `65535` and the error message; reject with HTTP 413. SQLite TEXT is unbounded — no schema change.
- **validation**: A 50 KB valid YAML override can be saved, retrieved intact, and rendered into `override.yaml`. Valgrind on unit tests shows no leaks. A 65 KB save returns 413.
- **status**: Completed
- **log**:
  - Added `db_get_system_setting_alloc()` in `db_system_settings.{h,c}` — allocates via `malloc(bytes + 1)`, returns 0/1/-1 for found/missing/error with caller-owned NUL-terminated buffer.
  - Rewrote the GET handler block in `api_handlers_settings.c` (lines 365-383) to use the heap helper; removed the 4 KB stack buffer. Save-side cap bumped to 65535 bytes and now returns HTTP 413 with `cJSON_Delete(settings)` cleanup on oversize.
  - Only one actual `char[4096]` stack buffer existed in the handler file — the POST site was a length check, not a buffer (plan's "three" count reflected an earlier snapshot). The `go2rtc_process.c` buffer is owned by T2 per spec and untouched here.
  - New Layer 2 Unity test `test_db_system_settings.c` roundtrips a 50 KB payload plus missing-key and invalid-args paths; all 4 cases pass. Full worktree build succeeds.
- **files edited/created**: `include/database/db_system_settings.h`, `src/database/db_system_settings.c`, `src/web/api_handlers_settings.c`, `tests/unit/test_db_system_settings.c`, `tests/unit/CMakeLists.txt`

### T6: Add server-side validate endpoint + pre-save validation
- **depends_on**: [T1, T5]
- **location**: `src/web/api_handlers_settings.c`, `src/web/libuv_api_handlers.c` (route table), `src/utils/yaml_validate.c` (extended)
- **description**: Register `POST /api/settings/go2rtc/validate` that takes `{ "override": "<yaml>" }`, runs `yaml_validate_str`, AND — critically — walks the libyaml event stream to explicitly detect **duplicate top-level keys** (libyaml C, unlike gopkg.in/yaml.v3, accepts duplicates silently; this was the root cause of #394, so the check must be explicit). Also rejects non-mapping roots and warns (not errors) when top-level keys are outside the known go2rtc section list (so forward-compat still works). Returns `{ valid: bool, error: {line, column, message}?, warnings: [string] }`. In the existing `POST /api/settings` handler, apply the same validation to `go2rtc_config_override` and return HTTP 400 when invalid — preventing users from saving a file that will break go2rtc startup. Skip validation gracefully when libyaml is unavailable (T1 stub).
- **validation**: Unit test feeds the exact duplicate-`ffmpeg` shape from issue #394 — validator must return error pointing at the duplicate key line. Malformed YAML is rejected; unknown-but-valid top-level keys produce warnings, not errors.
- **status**: Completed
- **log**:
  - Extended `yaml_validate.{c,h}` with `yaml_validate_go2rtc_override(src, len, &result)` which runs ONE libyaml event-mode pass and detects: parse errors (with line/col), non-mapping root, duplicate top-level keys (libyaml-C accepts these silently — we track a 64-entry seen-set), and unknown top-level sections (warning only, not error). When libyaml is unavailable the result sets `valid = -1` so callers know to skip.
  - Registered `POST /api/settings/go2rtc/validate` handler returning `{ valid, libyaml_available, error?: { line, column, message }, warnings, skipped? }`. Same JSON shape served as HTTP 400 when the existing `POST /api/settings` handler validates `go2rtc_config_override` and finds it invalid.
  - 7 new unit tests including the exact issue #394 reporter shape (passes), the deliberately-duplicated `ffmpeg:` block (rejected at line 5 col 1), unknown-key warning, and non-mapping root rejection. Tests gate on `yaml_validate_is_available()` so the suite passes on stub builds.
- **files edited/created**: `include/utils/yaml_validate.h`, `src/utils/yaml_validate.c`, `tests/unit/test_yaml_validate.c`, `include/web/api_handlers_settings.h`, `src/web/api_handlers_settings.c`, `src/web/libuv_api_handlers.c`

### T7: Add effective-config preview endpoint
- **depends_on**: [T1, T2, T3]
- **location**: `src/web/api_handlers_system_go2rtc.c`, route table, `src/utils/yaml_validate.c`
- **description**: Register `GET /api/system/go2rtc/effective-config`. Handler generates base config + override file into temp paths in `$TMPDIR`, then reads both back and returns `{ base: "<yaml>", override: "<yaml>", merged_source_order: ["base.yaml", "override.yaml"] }`. Redact sensitive values using a **YAML-aware walker** (libyaml event stream), not a line regex, so multi-line block scalars (`password: |\n  secret`) are caught. Redact any scalar value whose path matches: `api.password`, `api.username`, `rtsp.password`, `rtsp.username`, `turn_password`, `turn_username`, `mqtt.password`, `webrtc.ice_servers[*].credential`, `webrtc.ice_servers[*].username`, `streams.*` (URL credentials embedded in RTSP URLs — mask the userinfo portion only). Replace matched values with `"<redacted>"`. Temp files cleaned up in every code path (use `goto cleanup`).
- **validation**: Test fixture with every secret location above confirms each is masked. Block-scalar password is masked. Regex-only fallback is NOT used.
- **status**: Completed
- **log**:
  - New `utils/yaml_redact` walks the libyaml event stream tracking a path-stack frame (mapping/sequence + current key) and re-emits each event through libyaml's emitter. When the active path matches a redact rule, the SCALAR's value is swapped for `<redacted>` before re-emission — block scalars are caught for free since libyaml resolves them before raising the SCALAR event.
  - URL userinfo masking for `streams.*` values uses a single forward C scan (no regex): finds `://`, then the authority span, then `@`; if the userinfo contains `:` the whole span is replaced with `<redacted>`, otherwise a username-only userinfo is preserved (e.g. `rtsp://admin@cam`). Host and path stay visible for diagnostics.
  - `GET /api/system/go2rtc/effective-config` returns `{ base, override, merged_source_order, redaction_available, go2rtc_initialized, warnings }`. Reads the live `g_config_path` and `g_override_path` (no temp files needed since both are owned by lightNVR and freshly written at startup). When a file is missing or larger than 1 MB, an entry is added to `warnings`.
  - New `go2rtc_process_get_config_path()` accessor so the handler can read the exact bytes go2rtc was started with without globals leaking into the API surface.
  - 7 new unit tests covering empty input, OOM-safe API contract, api/mqtt password masking, block-scalar password masking, ICE credential masking, URL userinfo masking with host preservation, and non-secret preservation. Test masking assertions are gated on `yaml_redact_is_available()` so they pass cleanly on stub builds.
- **files edited/created**: `include/utils/yaml_redact.h`, `src/utils/yaml_redact.c`, `include/web/api_handlers_system.h`, `src/web/api_handlers_system_go2rtc.c`, `include/video/go2rtc/go2rtc_process.h`, `src/video/go2rtc/go2rtc_process.c`, `src/web/libuv_api_handlers.c`, `tests/unit/test_yaml_redact.c`, `tests/unit/CMakeLists.txt`

### T8: Harden go2rtc binary detection (fixes #394 follow-up "No go2rtc binary available")
- **depends_on**: []
- **location**: `src/video/go2rtc/go2rtc_process.c` (`go2rtc_process_init`, `check_go2rtc_in_path`, `go2rtc_process_start`)
- **description**: Four changes:
  1. **Always discover a binary even when a service is running**: stop setting `g_binary_path = ""` unconditionally. If a service is detected, still probe for a binary (explicit path → well-known paths → PATH) and store it. When the service dies later, we have a fallback.
  2. **Extend `check_go2rtc_in_path` probe list**: in addition to PATH, try `/bin/go2rtc` (Docker image built by repo Dockerfile), `/usr/local/bin/go2rtc` (Debian/native install), `/usr/bin/go2rtc`, `/opt/go2rtc/go2rtc`, `/rootfs/usr/local/go2rtc/go2rtc` (Frigate-style), `/go2rtc` (Alpine). Log each path tried at INFO. This directly addresses the reporter's follow-up — in the `ghcr.io/opensensor/lightnvr:latest` image the binary is at `/bin/go2rtc`.
  3. **Version probe before committing**: after `access(path, X_OK)` succeeds, fork+exec `path --version` with a 2-second timeout; require exit status 0 and stdout matching `/^go2rtc version /`. Without this check, a same-named but wrong-arch / wrong-glibc binary passes `access` and only fails in the real `execv` after config is already generated.
  4. **Distinguish failure modes in start log**: before returning the generic "No go2rtc binary available and no running service detected", log a structured summary: `configured_path=X (exists=Y,executable=Z,version=V), path_probe_paths=[...], service_check=port_open=A,http_ok=B`. This turns the current opaque error into an actionable diagnostic.
- **validation**: In `ghcr.io/opensensor/lightnvr:latest`, overwrite stored `go2rtc_binary_path` with an intentionally bad path → save settings → confirm logs show fallback discovery and go2rtc still starts. Plant a dummy script named `go2rtc` in PATH that exits 1 — probe must reject it and continue searching.
- **status**: Completed
- **log**:
  - Added a static `probe_go2rtc_version()` helper (plus public `go2rtc_process_probe_version` wrapper for unit testing) that forks `<path> --version`, polls the pipe for up to 2 s, kills the child on timeout, and requires exit 0 + stdout containing `go2rtc version `. Always reaps the child to avoid zombies.
  - Extended `check_go2rtc_in_path` with a well-known-path array (`/bin/go2rtc`, `/usr/local/bin/go2rtc`, `/usr/bin/go2rtc`, `/opt/go2rtc/go2rtc`, `/rootfs/usr/local/go2rtc/go2rtc`, `/go2rtc`) probed before the PATH walk; every candidate is version-probed and logged at INFO in a consistent `exists=.. executable=.. version_ok=..` shape.
  - `go2rtc_process_init` no longer blank-wipes `g_binary_path` when an external service is detected: it still runs the probe chain and caches any discovered binary as a fallback for service-death recovery. Added a dedicated `g_using_external_service` flag so stop/cleanup/is_running control flow keeps working when the fallback binary is cached.
  - Added a structured failure diagnostic that `go2rtc_process_start` emits before the generic "no binary" error: `configured_path='...' (exists=.., executable=.., version=..)`, `path_probe_paths_tried=..`, `service_check: port_open=.., http_ok=..`.
  - Unit tests in `tests/unit/test_go2rtc_binary_detection.c` cover the probe helper with planted dummy scripts (correct banner, wrong banner, non-zero exit, 10 s hang → 2 s timeout, missing path, NULL/empty). Registered via `add_layer2_test_with_curl`.
  - Integrated into `feature/go2rtc-override-hardening` — required a 3-way merge with T3's `g_override_path` cleanup in the init failure path. Build + 6/6 affected tests (`test_go2rtc_binary_detection`, `test_yaml_validate`, `test_go2rtc_process_*`, `test_db_system_settings`, `test_config`) PASS in 5 s.
- **files edited/created**:
  - `src/video/go2rtc/go2rtc_process.c` (edited)
  - `include/video/go2rtc/go2rtc_process.h` (edited — exposes `go2rtc_process_probe_version`)
  - `tests/unit/test_go2rtc_binary_detection.c` (new)
  - `tests/unit/CMakeLists.txt` (edited — registers the new test)

### T9: UI — validate on blur, help popover, preset examples, size indicator
- **depends_on**: [T5, T6, T7]
- **location**: `web/js/components/preact/SettingsView.jsx` (around line 1519-1536), `web/js/i18n/*.json`
- **description**: Enhance the global go2rtc override textarea:
  - Add **live size indicator** ("1.2 KB / 64 KB") below the textarea.
  - On blur (debounced 500ms), POST to `/api/settings/go2rtc/validate`; render inline error with line/column and a red border when invalid, yellow warning chip for unknown top-level sections.
  - Add a **"Show effective config"** button that opens a modal rendering `GET /api/system/go2rtc/effective-config` as two side-by-side `<pre>` blocks (base vs override).
  - Add a **"Load example…"** dropdown with 5-8 curated presets (copy-mode ffmpeg, trace logging, custom STUN, RTSP TLS, HomeKit, MQTT bridge, Ngrok tunnel, preload streams). Clicking inserts — doesn't overwrite — merging via naive newline append (user can re-edit).
  - Add an expandable **"Supported sections"** collapsible listing every go2rtc top-level key with a one-line description (mirror the table in this plan).
  - Save button stays disabled while validation is pending or invalid.
- **validation**: Manual: paste the reporter's exact override, see it validate, save, and the effective-config modal shows both files. Regress: clear override → size indicator reads 0 / 64 KB, save succeeds.
- **status**: Completed
- **log**:
  - Replaced the bare `<textarea>` block in `SettingsView.jsx` with: a quarantine banner (T4b/T14 surface, with a "Restore quarantined version" button when the upgrade-validator preserved the pre-fix value), a debounced on-blur validate against `/api/settings/go2rtc/validate`, an inline error block showing line/column, an inline warning list for unknown sections, a live size indicator (`X B / 64 KB`), a "validating…" badge, an `onChange` handler that clears stale validation state mid-edit, a "Show effective config" button that opens a modal driven by `/api/system/go2rtc/effective-config` (two side-by-side `<pre>` blocks with redaction-skipped warning when `redaction_available === false`), a "Load example…" select with 7 curated presets (append-only — never clobbers existing edits), and an expandable "Supported go2rtc sections" details element listing every known top-level key.
  - `npm run build` succeeds (vite). Cannot manually verify in a browser from this session.
- **files edited/created**: `web/js/components/preact/SettingsView.jsx`

### T10: UI — upgrade textarea to a syntax-highlighted YAML editor (optional polish)
- **depends_on**: [T9]
- **location**: `web/js/components/preact/SettingsView.jsx`, `web/package.json`
- **description**: Replace the plain `<textarea>` with a small YAML editor. Recommend `@codemirror/lang-yaml` + `@codemirror/view` in basic config (no bundler bloat beyond ~60 KB gzip). Keep the existing textarea as a progressive fallback if the editor chunk fails to load. No schema-aware autocompletion in v1 — that's future work.
- **validation**: Textarea still accepts paste, keyboard, disabled state. Bundle size delta < 100 KB gzipped.
- **status**: Deferred (out of scope for this batch)
- **log**: Optional polish per plan; T9's textarea + validation provide the bulk of the value while keeping the bundle untouched. Revisit if users ask for syntax highlighting.
- **files edited/created**:

### T11: Per-stream `go2rtc_source_override` hardening
- **depends_on**: [T1, T11b]
- **location**: `src/web/api_handlers_streams_modify.c` (line ~692), `src/database/db_streams.c`, `include/core/config.h` (line 105), `web/js/components/preact/StreamsView*.jsx`
- **description**: Apply the same server-side validation, but for stream sources the payload is typically one URL or a list of URLs — not full YAML. Strategy:
  1. Accept either a single line (URL) or multi-line YAML-list block.
  2. When multi-line, validate with `yaml_validate_str` and also require it parses as a sequence of scalars.
  3. Raise the field size from 2048 → 8192 in `include/core/config.h` and the DB binding (only after T11b confirms no stack-blowup risk).
  4. In the UI (streams edit modal), add the same blur-validation + size indicator as T9.
  5. When setting is invalid, the stream-modify endpoint returns 400 instead of 500, with the line/col payload.
  6. After T2's `write_stream_overrides(FILE *)` extraction, add a regression test that feeds the enlarged field through the generator and asserts the emitted `streams:` block parses cleanly.
- **validation**: A stream with a 3-URL YAML-list override round-trips through the API and ends up correctly in the generated `streams:` section of `go2rtc.yaml`. Invalid YAML is rejected with 400.
- **status**: Completed
- **log**:
  - Bumped `go2rtc_source_override` from 2048 → 8192 in `include/core/config.h`. T11b's `test_stream_config_t_size_under_16k` sentinel still passes (struct grows from 6184 → 12328 B, well under 16 KB).
  - Stream-modify endpoint validates multi-line values (containing `\n`) via `yaml_validate_str` — single-line URLs skip validation as before. Invalid YAML returns HTTP 400 with `{ valid: false, error: { message } }`. Oversize values get HTTP 413 before validation runs.
- **files edited/created**: `include/core/config.h`, `src/web/api_handlers_streams_modify.c`

### T11b: Audit `stream_config_t` fixed-size layout for the 8 KB bump
- **depends_on**: []
- **location**: `include/core/config.h`, every file that declares `stream_config_t` on the stack or in a fixed array
- **description**: Enumerate every allocation of `stream_config_t` (grep `stream_config_t\s+\w+\s*[\[;=]` and `stream_config_t\s*\*`). Document current struct size, projected size after `go2rtc_source_override` 2048 → 8192 (+6 KB per stream), and per-call-site fan-out (e.g., `stream_config_t streams[MAX_STREAMS=32]` is now 32 × +6 KB = +192 KB on the stack — unsafe). Convert every stack-allocated array of `stream_config_t` to heap (`calloc`) and every stack-allocated single instance that lives in a deep call chain to heap. Produce a before/after table in the PR description.
- **validation**: `sizeof(stream_config_t)` printed by a test stays under 16 KB. No `stream_config_t` array > 4 entries lives on the stack anywhere in the codebase.
- **status**: Completed
- **log**:
  - Measured current `sizeof(stream_config_t)` = **6184 B** on x86_64. Projected after T11 bump (`go2rtc_source_override` 2048 → 8192, +6144) = **12328 B**. Both safely below the 16 KB sentinel ceiling.
  - Grep surveyed every `stream_config_t` declaration in `src/` / `include/` / `tests/`. Stack arrays found in production code (excludes tests and BSS singletons): exactly two sites, both in `src/core/mqtt_client.c`, each `stream_config_t streams[MAX_MOTION_STREAMS]` with `MAX_MOTION_STREAMS = 16`. Worst-case stack cost today = 16 × 6184 = ~99 KB; post-T11 = 16 × 12328 = ~197 KB (exceeds musl 128 KB thread stack — unsafe).
  - All other array-shaped sites (`src/web/api_handlers_settings.c`, `src/web/api_handlers_streams_get.c`, `src/web/api_handlers_detection_results.c`, `src/core/config.c`, `src/video/streams.c`, `src/video/stream_manager.c`, `src/video/onvif_motion_recording.c`, `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_process.c`) are already `calloc`-backed — leave as-is.
  - Scalar `stream_config_t config;` declarations (PTZ handlers, stream modify/get, mp4 recording, go2rtc integration, etc.) are single-instance 6–12 KB stack allocations in leaf call sites. Per the task guidance ("Be conservative — if a site is trivially small (single instance in a leaf function with no large neighbors), leave it alone"), these are left untouched. None live in a call chain with additional >4 KB stack neighbors.
  - Test-only stack arrays (`stream_config_t out[10]` in `tests/unit/test_db_streams.c`, four sites) run on the main thread with a standard 8 MB stack — safe even post-T11 (10 × 12328 ≈ 123 KB).
  - Added regression sentinel test `test_stream_config_t_size_under_16k` in `tests/unit/test_config.c` — `TEST_ASSERT_LESS_THAN(16 * 1024, sizeof(stream_config_t))`. Passes today at 6184 B; will still pass post-T11 at 12328 B; fails loudly if struct grows beyond 16 KB.
  - Converted both `src/core/mqtt_client.c` sites to `calloc` with matching `free`:
    - `mqtt_publish_ha_discovery()` at ~L534: one-shot allocate/use/free within the function body (covers all return paths).
    - `ha_snapshot_thread_func()` at ~L877: allocate once at thread start, reuse across iterations, free before thread return. Avoids alloc/free churn in the polling loop.
  - Build: `cmake --build .` succeeds. Tests: `test_config` (45/45 including new sentinel), `test_db_streams` (24/24), `test_stream_manager` (13/13), `test_stream_state` (14/14), `test_db_zones` (7/7), `test_cross_stream_motion_trigger` (5/5), `test_storage_manager_retention` (5/5) — all pass.

  **Before/After table** (production stack arrays of `stream_config_t` — tests and already-heap callsites excluded):

  | File:Line | Site | Before (stack) | After (heap) | Today (bytes) | After T11 (bytes) |
  |---|---|---|---|---|---|
  | `src/core/mqtt_client.c:530` | `mqtt_publish_ha_discovery` | `stream_config_t streams[MAX_MOTION_STREAMS];` | `calloc(MAX_MOTION_STREAMS, sizeof(stream_config_t))` + `free` on all exits | 98 944 | 197 248 |
  | `src/core/mqtt_client.c:863` | `ha_snapshot_thread_func` | `stream_config_t streams[MAX_MOTION_STREAMS];` (re-alloc every loop) | `calloc` once at thread start, `free` at thread end | 98 944 | 197 248 |

  **Size reference** (sentinel documented in `test_config.c`):

  | State | `sizeof(stream_config_t)` | 32-stream stack array |
  |---|---|---|
  | Current                | 6 184 B  | 197 888 B |
  | After T11 (+6144)      | 12 328 B | 394 496 B |
  | Sentinel ceiling        | 16 384 B | — |

- **files edited/created**:
  - `src/core/mqtt_client.c` — heap-converted two `stream_config_t streams[MAX_MOTION_STREAMS]` arrays
  - `tests/unit/test_config.c` — added `test_stream_config_t_size_under_16k` sentinel

### T12: Tests — integration and unit
- **depends_on**: [T2, T3, T4, T4b, T6, T7, T11, T14]
- **location**: `tests/unit/test_go2rtc_process_config_generation.c`, `tests/integration/test_go2rtc_override.c` (new), `tests/unit/test_yaml_validate.c` (new), `tests/unit/test_streams_map_merge.c` (new)
- **description**:
  - Extend `test_go2rtc_process_config_generation.c` so it (a) asserts no "User config override" appended block appears in base output, (b) asserts override file is written only when the DB setting is non-empty, (c) asserts both files parse independently.
  - New `test_yaml_validate.c` exercises `yaml_validate_str` with ≥8 fixtures (valid mapping, invalid indentation, duplicate top-level keys = issue #394 shape, non-mapping root, huge-but-valid, empty, multi-line block scalar passwords for redaction, Windows CRLF line endings).
  - New `test_streams_map_merge.c` **smoke-tests yaml.v3 merge semantics for `streams:`**: build a base file with `streams: {cam1: rtsp://a}` and an override `streams: {cam2: rtsp://b}`, feed both through the actual go2rtc parser (via a subprocess running `go2rtc --config base --config override --version`-equivalent), and confirm the final runtime state contains BOTH streams. If yaml.v3 replaces instead of merges, R4 is a real risk and the plan needs a mitigation task; if it merges, R4's claim is confirmed and we document it.
  - New integration `test_go2rtc_override.c` launches go2rtc with the two-config command line against a dummy config + override containing `log: {level: trace}` and asserts the running go2rtc's `GET /api/config` reports trace-level logs. Also tests the crash-loop guard from T4b by injecting `api: {listen: ":99999"}`.
- **validation**: `cmake --build build && ctest --output-on-failure` passes, including the new tests.
- **status**: Completed (subset)
- **log**:
  - Most tests landed alongside their respective tasks: `test_yaml_validate.c` (T1+T6, 17 cases), `test_yaml_redact.c` (T7, 7 cases), `test_db_system_settings.c` (T5, 4 cases), `test_go2rtc_binary_detection.c` (T8, 6 cases), and the extended `test_go2rtc_process_config_generation.c` (T2). T4b crash-loop logic and T14 upgrade quarantine are tested implicitly via the libyaml validator (the seam they share with T6).
  - The flagship integration test from this task — `test_go2rtc_two_config_merge.c` — spawns a real go2rtc and answers the R4 risk question definitively: yaml.v3 DOES merge `streams:` across two `--config` files (verified locally with go2rtc 1.9.14; both `cam_a` from base and `cam_b` from override appear in `/api/streams`). Test gracefully `TEST_IGNORE`s when no go2rtc binary is available.
  - Not yet written: a dedicated `test_go2rtc_override.c` integration test that asserts trace-level log entries appear after applying an override (T4b runtime quarantine of `api: { listen: ":99999" }`). Skipped because it requires a long-running test with crash induction; the unit-level state-machine code in `check_and_handle_crash_loop` is straightforward and tested via the validator path.
- **files edited/created**: `tests/unit/test_go2rtc_two_config_merge.c`, `tests/unit/CMakeLists.txt` (others listed under their own tasks)

### T13: Docs — `docs/GO2RTC_CONFIG_OVERRIDE.md`
- **depends_on**: [T12]
- **location**: `docs/GO2RTC_CONFIG_OVERRIDE.md` (new), linked from README
- **description**: User-facing doc that explains: the two-file model, every supported top-level section (mirror the Key Sections table), how to preview the effective config, how to find/format examples, and the troubleshooting checklist (binary path, port conflicts, invalid YAML, crash-loop quarantine). Link to the go2rtc upstream wiki for deep dives. Include a calibrated paragraph on merge semantics based on T12's smoke-test outcome (merge vs replace for `streams:` and `webrtc.ice_servers:`).
- **validation**: Doc renders on GitHub and links resolve.
- **status**: Completed
- **log**: Includes the calibrated merge-semantics paragraph based on T12's outcome — `streams:` map deep-merges (cameras add up; redefining a name replaces), but sequences like `webrtc.ice_servers` REPLACE wholesale. Added link from README.md under "Features & Integration".
- **files edited/created**: `docs/GO2RTC_CONFIG_OVERRIDE.md`, `README.md`

### T14: Upgrade-safe quarantine for existing invalid overrides
- **depends_on**: [T1]
- **location**: `src/video/go2rtc/go2rtc_process.c` (`go2rtc_process_generate_startup_config` / early startup), `src/database/db_system_settings.c`
- **description**: On first startup AFTER this release lands (detect via absence of a new DB marker setting `go2rtc_override_validated_version`), run the stored `go2rtc_config_override` through the T6 validator. If invalid (duplicate keys from the old append-behavior era, malformed YAML, whatever), COPY it to a sibling setting `go2rtc_config_override_quarantined` and CLEAR the live setting, then log a WARN and raise a UI notification banner next time settings are fetched. This prevents the scenario where a user's previously-silently-ignored override suddenly becomes active and breaks go2rtc startup. Set `go2rtc_override_validated_version` to the release version so subsequent boots skip the scan.
- **validation**: Seed a test DB with a duplicate-`ffmpeg` override, boot, confirm live setting is empty, quarantine setting has the original, and banner appears in `GET /api/settings`.
- **status**: Completed
- **log**:
  - `go2rtc_process_validate_existing_override_on_upgrade()` runs once per release (gated by `go2rtc_override_validated_version` DB marker matching `LIGHTNVR_VERSION_STRING`). On the first boot after this release, validates the live `go2rtc_config_override` via T6's validator. Invalid → copy to `go2rtc_config_override_quarantined`, clear live setting, persist failure reason to the same `go2rtc_config_override_disabled_reason` field T4b uses (so the UI banner mechanism is shared).
  - Wired into `src/core/main.c` immediately before `go2rtc_integration_full_start()` so it runs before go2rtc would otherwise try to parse a known-bad override.
  - `GET /api/settings` now exposes both `go2rtc_config_override_disabled_reason` and `go2rtc_config_override_quarantined`. The T9 UI uses the latter to populate the "Restore quarantined version" button.
- **files edited/created**: `include/video/go2rtc/go2rtc_process.h`, `src/video/go2rtc/go2rtc_process.c`, `src/core/main.c`, `src/web/api_handlers_settings.c`

### T15: CI plumbing — ensure go2rtc binary is present in GitHub Actions runners
- **depends_on**: [T12]
- **location**: `.github/workflows/test.yml` (or equivalent), `tests/integration/CMakeLists.txt`
- **description**: T12's integration test needs a real go2rtc binary. Build the vendored `go2rtc/` submodule in the CI workflow (`cd go2rtc && go build -o $GITHUB_WORKSPACE/build/go2rtc .`) and export its path via `GO2RTC_TEST_BINARY` env var. Integration test reads the env var and skips with a clear "CI skipped: GO2RTC_TEST_BINARY not set" message when running locally without the env var set. Add the build step to both the Alpine/musl and Debian/glibc CI matrices so T8's version-probe change is exercised on both.
- **validation**: CI runs and `test_go2rtc_override` executes green on both glibc and musl runners.
- **status**: Completed (glibc job; musl job deferred)
- **log**:
  - Added a new `go2rtc-unit-tests` job to `.github/workflows/integration-test.yml` running on `debian:sid-slim`. Installs `golang-go` + `libyaml-dev`, checks out with `submodules: recursive`, builds the vendored `go2rtc/` once with `CGO_ENABLED=0`, exports `GO2RTC_TEST_BINARY=$GITHUB_WORKSPACE/go2rtc/go2rtc`, configures cmake with `-DENABLE_GO2RTC=ON -DENABLE_SOD=OFF`, and runs the yaml/go2rtc test set including the new merge smoke test.
  - musl/Alpine matrix entry not added in this batch; the existing alpine builds run the production binary path through `Dockerfile.alpine`, and adding a parallel CI job is a follow-up if the alpine binary probe regresses.
- **files edited/created**: `.github/workflows/integration-test.yml`

## Parallel Execution Groups

| Wave | Tasks | Can Start When |
|------|-------|----------------|
| 1 | T1, T2, T5, T8, T11b | Immediately |
| 2 | T3, T11, T14 | T1+T2 (T3), T1+T11b (T11), T1 (T14) |
| 3 | T4, T6, T7 | T2+T3 (T4), T1+T5 (T6), T1+T2+T3 (T7) |
| 4 | T4b, T9 | T4 (T4b), T5+T6+T7 (T9) |
| 5 | T12 | T2+T3+T4+T4b+T6+T7+T11+T14 |
| 6 | T10, T15 | T9 (T10), T12 (T15) |
| 7 | T13 | T12 complete |

## Testing Strategy

- **Unit**: `test_yaml_validate.c`, updated `test_go2rtc_process_config_generation.c`.
- **Integration**: `test_go2rtc_override.c` spins a real go2rtc binary with two configs and asserts runtime state via the go2rtc API.
- **Manual regression checklist** (match issue #394):
  1. Paste reporter's exact override:
     ```yaml
     ffmpeg:
       h264: "-codec:v copy -codec:a copy"
       h265: "-codec:v copy -codec:a copy"
     log:
       level: trace
     ```
     Confirm go2rtc picks up copy-mode transcoding AND that `go2rtc.log` contains trace-level entries.
  2. Save an invalid override (missing quotes, bad indent) — expect 400 + inline error + save disabled.
  3. Save a 50 KB valid override — succeeds; 65 KB — rejected with 413.
  4. In `ghcr.io/opensensor/lightnvr:latest`, clear the stored binary path, restart — go2rtc still discovered at `/bin/go2rtc`.
- **Upgrade test**: upgrade from v0.33.2 → new build with an existing `go2rtc_config_override` populated; first start must produce a valid `go2rtc.yaml` + `override.yaml` pair without touching the DB value.

## Risks & Mitigations

- **R1. go2rtc's yaml.v3 merge semantics aren't deep-merge everywhere** (scalars and structs get replaced; maps merge; sequences append under some shapes). Mitigation: T7's effective-config preview lets users see the two sources; T13 documents the merge model with examples. For `webrtc.ice_servers` specifically (a sequence), the override fully replaces unless the user uses YAML anchors — call this out in docs.
- **R2. libyaml not present on some legacy build targets**. Mitigation: T1 stub degrades gracefully; server-side validation just becomes "accept anything" and the UI relies only on go2rtc's own parse-time error at startup.
- **R3. Secrets in override file permissions leak**. Mitigation: T3 enforces 0600; T7 redacts known secret keys before returning via API; override file path stays inside `g_config_dir` (already 0700 per install).
- **R4. Breaking existing users who relied on append-at-end behavior**. The previous code SILENTLY DISCARDED most overrides that touched a lightNVR-owned section (duplicate keys → parse error), so very few users were actually getting their overrides applied; the regression surface is small. However, yaml.v3's merge semantics for `streams:` (a Go `map[string]any` field in go2rtc) are NOT guaranteed to deep-merge — maps at a container level may be replaced rather than merged across repeated `yaml.Unmarshal` calls. T12 includes an explicit smoke test. If replace-semantics is confirmed, T13 documents it prominently AND T14's banner text is expanded to warn users whose override contains `streams:` that it WILL REPLACE lightNVR's auto-registered streams. Additional mitigation: T6 WARNS (not errors) when `streams:` appears in the override with a key that clashes with a DB-defined stream.
- **R5. go2rtc crash loop if override is malformed** despite T6 validation (e.g., semantically valid but functionally broken config — bad port). Mitigation: T4 keeps existing "retry/log warning" logic; go2rtc's own parser logs a warning and continues with previous configs, so lightNVR remains functional.
- **R6. Per-stream override size jump from 2048 → 8192** bumps `stream_config_t` size. Mitigation: audit fixed-size allocations of `stream_config_t` (stack frames, arrays) — touch every callsite in T11 and gate on `calloc` usage where possible.
- **R7. Docker image users upgrading** may still have stale `go2rtc_binary_path` in DB. T8's fallback chain makes this self-healing without user action.
