# Storage fault qualification evidence

T26 passed its bounded private-storage qualification on 2026-11-05. No host
filesystem was filled or remounted. Fault-boundary self-tests, all four real-server
fault cases and four existing native storage regression tests passed against the
fresh task-owned Debug ASan/UBSan build after correcting a local FFmpeg header/
runtime mismatch. This is workstation evidence, not device qualification.

## Method

The harness creates an owned server, loopback port, database and recording tree.
Startup requires its own PID file and successful bind log before any HTTP request.
It changes the bootstrap password, creates a disabled synthetic camera through
the API, stops the server, and inserts one completed protected recording into
the actual current schema. FFmpeg generates a two-second playable MP4; this
offline fixture step is explicit and is not an assertion about media ingestion.

The running server then observes each fault through its real storage-target
probe API. A forced controller cleanup must finish within 30 seconds. The
protected row's durable fields and SHA-256 of its file are compared during
every fault and after restoration. Restart must preserve them, SQLite
`quick_check` must return `ok`, and FFmpeg must fully decode the surviving file.

| Case | Boundary | Required observable result |
| --- | --- | --- |
| Full | Test-local injected zero blocks and ENOSPC | Probe 422; target unavailable with no-space explanation; controller emergency pressure with zero available bytes |
| Read-only | Test-local injected read-only flag and EROFS | Probe 422; target unavailable with read-only explanation |
| Permission | Real kernel EACCES, directory mode 0500, UID 1000 | Independent create fails; probe 422 with permission explanation |
| Unavailable | Real kernel ENOENT after private directory rename | Probe 422 with missing-directory explanation; protected file remains in parked tree |

All four cases must recover to a successful write probe after restoration.
Unrelated paths and an unpreloaded process are checked by the independent fault
helper. The preload affects only this server and exact fixture root. ASan's
actual linked runtime is placed first in preload order; sanitizer findings,
unexpected signals and shutdown timeouts fail the run. The separate periodic
host-health collector is disabled so its clean-environment subprocess probes
cannot overwrite the deliberately injected direct-probe observation.

This is direct-probe/controller/retention evidence. It is not a claim of real
disk exhaustion, physical media qualification, read-only mount behavior, or
FFmpeg recording-write recovery. T25/T27 supply the separate recording and
lifecycle evidence. Detailed fixture scope is in
[storage-fixtures/README.md](../../../tests/reliability/storage-fixtures/README.md).

## Initial baseline run

```sh
python3 tests/reliability/storage-faults.py --binary build/eng-api-T2/bin/lightnvr --self-test
python3 tests/reliability/storage-faults.py --binary build/eng-api-T2/bin/lightnvr
```

- Self-test passed: `/tmp/lightnvr-storage-fault-p0eegnej/result.json`.
- Four-case real-server matrix passed:
  `/tmp/lightnvr-storage-fault-yf56hwmj/result.json`.
- Binary SHA-256:
  `b02db6c8d4503f9305aa78c768daff3806e9cec5c30286f4987338a53aa1aefd`,
  identical to the fresh binary in [baseline.md](baseline.md).
- Each fault/probe/cleanup observation completed in about 0.15 seconds on this
  workstation. This is measured evidence, not a production timing budget.
- Protected MP4 SHA-256 remained
  `d1e8b5dd24c05b1fe93e5482c1b8d5cd60c8b169af20447af520d31fb9e1d294`.
- Each target recovered to `healthy`; no spontaneous sanitizer error occurred.
- Result JSON includes the full run-time source manifest and digest. Because
  other native work was in progress, that observed source digest is not asserted
  to be the T2 binary's compilation provenance; the T2 baseline records that.

## Final validation

The first task-owned build exposed an environment problem: `/usr/local` supplied
FFmpeg 61 headers and stale pkg-config metadata while the executable linked the
system FFmpeg 62 libraries. Reconciliation crashed during its test encoder setup.
The failed logs are retained at `/tmp/lightnvr-eng-T26-reconcile-ctest.log`; this
mismatched build is not accepted reconciliation evidence. No product change was
made to suppress the failure. Selecting matching system headers and pkg-config
metadata corrected it. A separate compiled probe confirmed avcodec 62/62,
avformat 62/62 and avutil 60/60 (header/runtime).

```sh
PKG_CONFIG_PATH= PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig \
cmake --fresh -S . -B build/eng-api-T26 -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON \
  -DENABLE_SOD=OFF -DENABLE_LITERT=OFF -DENABLE_GO2RTC=OFF \
  -DENABLE_MQTT=OFF -DENABLE_SSL=ON \
  '-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu' \
  '-DCMAKE_CXX_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
cmake --build build/eng-api-T26 --target lightnvr test_storage_retention_sqlite \
  test_storage_target_pressure_cleanup test_storage_migration_publication \
  test_storage_reconcile_incomplete -j6
cc -isystem /usr/include/x86_64-linux-gnu \
  tests/reliability/storage-fixtures/ffmpeg_abi_probe.c \
  -o /tmp/lightnvr-T26-ffmpeg-probe -lavformat -lavcodec -lavutil
/tmp/lightnvr-T26-ffmpeg-probe
python3 tests/reliability/storage-faults.py --binary build/eng-api-T26/bin/lightnvr --self-test
python3 tests/reliability/storage-faults.py --binary build/eng-api-T26/bin/lightnvr
ctest --test-dir build/eng-api-T26 --output-on-failure --no-tests=error \
  -R '^(test_storage_retention_sqlite|test_storage_target_pressure_cleanup|test_storage_migration_publication|test_storage_reconcile_incomplete)$'
```

These include-path overrides are specific to this x86-64 Debian host's conflicting
installations; other hosts should select their own coherent toolchain. CMake
reported SSL enabled and go2rtc/SOD/LiteRT/MQTT disabled. Native flags retained
`-fsanitize=address,undefined`; no sanitizer was disabled. Libyaml was not available
for this JSON/storage-only build, and YAML override behavior is outside T26.

| Evidence | Result / artifact |
| --- | --- |
| Fault isolation self-tests | Passed; `/tmp/lightnvr-storage-fault-016hwwqy/result.json` |
| Final four-case server matrix | Passed; `/tmp/lightnvr-storage-fault-lvbulyag/result.json` |
| Native regression selection | 4/4 passed, 2.56 s; `/tmp/lightnvr-eng-T26-corrected-ctest.log` |
| Build/configuration | `/tmp/lightnvr-eng-T26-corrected-build.log`, `/tmp/lightnvr-eng-T26-corrected-configure.log` |
| FFmpeg ABI probe | `/tmp/lightnvr-eng-T26-ffmpeg-abi.log` |

Corrected binary SHA-256:
`f9594df3130363b8a9aec064a605ea0a1304b4d8b30ab2aa17d71cd9c60bb973`.
Native source was frozen during compilation: the complete sorted source manifests
before and after match at
`977cbee5420579550223463b23329c7403fc00be69aa3269f85d192f22fb1c09`.
They are retained as `/tmp/lightnvr-eng-T26-corrected-source-before.json` and
`/tmp/lightnvr-eng-T26-corrected-source-after.json`. This identifies the actual
uncommitted implementation at compilation, not just baseline HEAD `dea6094a`.

The full fault reached emergency cleanup and its filesystem fallback with no
eligible DB recordings; the protected file and row still survived unchanged.
Every case recovered to `healthy`, a subsequent server restart preserved identity,
and the two-second protected file decoded fully. For directory disappearance,
the target API correctly reports `unavailable`; the legacy aggregate storage
health endpoint retains its preceding pressure value when `statvfs` fails. The
harness asserts the target-specific error rather than misrepresenting that cached
aggregate value as proof the missing directory is healthy.

No production code was changed by T26. All owned processes exited, fault controls
were restored, and private artifacts were retained for review. T26's accepted
scope does not resolve the separate interrupted-recording timing issue found by T25.
