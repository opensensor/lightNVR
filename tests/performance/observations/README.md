# INT_01 observation performance fixture (C4 / X2)

The repeatable recording workload behind the INT_01 performance gate
(PRD §8, A11). C4 declares it and captures the **feature-disabled** baseline.
X2 reruns it with evidence enabled and compares the two.

The C1 contract (`docs/integrations/OBSERVATIONS_CONTRACT.md` §Performance)
should link here rather than copy these values.

## Files

| File | Purpose |
| --- | --- |
| `workload.json` | Declared fixture: streams, resolution, fps, GOP, segment length, detector and cadence, window, sampling and retention. CLI flags override it, and every run records the values it actually used. |
| `run-workload.mjs` | Harness. Writes `<output>/results.json`. |
| `metrics.mjs` | Pure aggregation (percentiles, Prometheus parsing, gap counting, repeatability and release-gate checks). Unit tests live in `tests/contracts/tooling/observation-perf-metrics.test.mjs` (`npm run contracts:test-tools`). |
| `compare.mjs` | `repeat A B` (C4: p95 variance < 2%), `gate BASELINE CANDIDATE` (X2: same binary, no new gaps, p95 +≤5%, plus the evidence checks for an enabled candidate; `--advisory KEY`, `--require-saturation`, `--max-submit-seconds S`) and `pool BASE... -- CAND...` (one raw series pooled per mode). |
| `make-bird-clip.sh` | X2's detection-producing clip: the pinned cardinal photo (not committed, CC BY-SA) bouncing across a textured 1280×720 background. |
| `results/` | Committed results. Raw samples are kept so the numbers can be recomputed. |

## Workload (`int01-observations-v1`)

- **Streams.** 4 streams, each 1280×720 H.264 at 15 fps with a 15-frame GOP
  and no B-frames.
- **Source.** A deterministic 30 s `testsrc2` clip. One always-on
  `ffmpeg -re -stream_loop -1 -c copy` per stream publishes the clip into a
  standalone go2rtc (`cam0..camN-1`) on loopback.
- **lightNVR.** lightNVR pulls each stream over RTSP/TCP. Its own go2rtc
  integration is disabled, as in the contract fixture
  (`tests/contracts/fixture-server.mjs`), so recording and detection each
  open their own RTSP session.
- **Recording.** Continuous MP4 recording with 10 s segments and no audio.
  `storage.max_size = 0`, so no retention sweeps run inside the window.
- **Detector.** A local SOD CNN (`tiny20.sod`, VOC-20) in the unified
  detection thread, running every 1 s on every stream. It runs in
  annotation-only mode because continuous recording is on. The model is not
  vendored because it is 63 MB. Pass `--model`; its SHA-256 is recorded.
- **Timing.** 30 s warm-up, then a 300 s measurement window.
- **Viewers.** None.
- **Storage.** The fixture runs on a real filesystem (`--work-dir`, default
  `build/perf-observations/work`). The harness refuses tmpfs, because `/tmp`
  on the dev host is tmpfs.

## Measurements (`results.json`)

`definitions` in every results file repeats these definitions.

### Recording-path latency

lightNVR has no in-process packet-write latency histogram, so the harness uses
two black-box proxies that both sit on the recording path. They are the
headline series for repeatability and for the gate.

- **`latency.segment_interval_ms`** is the wall-clock time between the
  appearance of consecutive segment files for each stream, seen through
  inotify. A stall in the writer, in rotation, or in the catalog insert under
  the global DB mutex lengthens it.
  - `segment_interval_excess_ms` is the same value minus the nominal 10 000 ms.
- **`latency.db_probe_ms`** times `GET /api/recordings?limit=1` with a session
  cookie, every 250 ms. That request takes the same global DB mutex as the
  recording catalog writes. The plan calls out mutex hold time as an X2 risk.
  - A session cookie is used because Basic auth runs PBKDF2 on every request,
    and that cost would swamp the measurement.

### Gaps

- **`gaps.metrics_recording_gaps_total`** is the change in the in-process
  `lightnvr_recording_gaps_total` counter (5 s threshold) over the window.
- **`gaps.catalog_gaps_total`** counts places where a catalog row starts more
  than 5 s after the previous row for that stream ends.
- **`gaps.segment_interval_gaps`** counts segment intervals longer than the
  nominal 10 s plus 5 s.

### Resources

**`resources.lightnvr`** comes from `/proc/<pid>` sampled once per second:

- CPU percent of one core (distribution and average);
- RSS distribution and `VmHWM` high-water mark;
- `write_bytes` delta and rate;
- maximum thread count and maximum fd count.

The fixture's own cost (go2rtc plus publishers) is reported separately under
`resources.fixture_go2rtc_and_publishers`.

### Health and detector checks

- **`recording`** records segment counts against the expected number, the
  frame-drop and reconnect deltas, FPS samples and `streams_up_min`.
- **`detector`** counts inferences from `sod_cnn_predict returned` lines in the
  log, as inferences per stream per second against the configured cadence.

A run is marked `invalid` when any of these hold:

- a stream went down;
- the detector ran below half its configured cadence;
- a probe failed;
- a publisher exited.

## Run

```sh
# Fresh Release build against the system FFmpeg (stale /usr/local headers on the dev host break local builds)
PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig \
  cmake -S . -B build/int01-c4-release -DCMAKE_BUILD_TYPE=Release -DBUILD_TESTS=OFF
cmake --build build/int01-c4-release --target lightnvr -j

node tests/performance/observations/run-workload.mjs --model ~/tiny20.sod \
  --output build/perf-observations/disabled-run1 --mode disabled
node tests/performance/observations/run-workload.mjs --model ~/tiny20.sod \
  --output build/perf-observations/disabled-run2 --mode disabled
node tests/performance/observations/compare.mjs repeat \
  build/perf-observations/disabled-run1/results.json build/perf-observations/disabled-run2/results.json
```

The harness needs Node 26, ffmpeg/ffprobe with libx264, go2rtc on `PATH` (or
`--go2rtc`), `findmnt`/`lsblk`, and Linux `/proc`. Use `--keep` to keep the
fixture directory (server log, DB, media).

## X2: rerun with evidence enabled

1. **Use one binary and one source for both modes.**
   - Build the X2 binary. Rerun **disabled** mode on it as well. The
     committed C4 baseline proves the harness is repeatable, but the gate
     should compare the two modes on the same binary.
   - Synthetic `testsrc2` content gives **no VOC detections**. The detector
     runs, but it produces no observations, so evidence work would never
     trigger.
   - X2 must pass a clip that produces detections, for example the plan's
     recorded bird fixture, with `--source FILE`. Run both modes on that clip.
     Its SHA-256 goes into `workload.source_sha256`, and `compare.mjs` refuses
     to compare runs whose workloads differ.
2. **Enable the feature with the hooks only.** Do not edit the harness for
   this.
   - `--stream-extra stream.json` is merged into every `POST /api/streams`
     body, for example the L9 per-camera evidence opt-in.
   - `--config-extra extra.ini` is appended to the generated `lightnvr.ini`,
     for example the C1 queue or byte budgets.
   - Both are recorded in `results.feature`. Use `--mode enabled`.
   - `--system-setting KEY=VALUE` (repeatable) writes a `system_settings` row
     and restarts lightNVR once before streams are added. The global queue
     budget is read only at startup, so the saturation run uses
     `--system-setting observation_global_queue_bytes=4194304`.
3. **Extra X2 evidence (automatic).**
   - `results.feature_metrics` holds the window's L10 counters (accepted,
     dropped by reason, ready, failures, discards) and the run's
     high-water marks.
   - It also holds the configured limits, the gauge maxima over every scrape,
     and the producer submit-latency histogram
     (`lightnvr_observation_submit_latency_seconds`).
   - `results.feature_db` is read from the fixture DB after shutdown:
     observations, assets by `state:reason`, and detections.
4. **Gate.** Run `compare.mjs gate disabled.json enabled.json`. It fails on:
   - a different binary or workload;
   - any additional gap;
   - a p95 increase above 5% in a gating headline series;
   - any `evidenceGate` check, which requires all of these:
     - detections, evidence and DB observations really happened;
     - high-water marks are within the configured limits;
     - no untracked drops, and the DB holds at least the window's drops by
       reason;
     - no encode, write or DB failures;
     - the producer submit maximum is under 1 ms (`--max-submit-seconds`).

   `--require-saturation` also requires `queue_full` drops.

   The 1 ms wall-clock budget belongs to this Release performance gate.
   `test_observation_evidence` checks admission while the encoder worker is
   stalled, queue limits, and latency histogram consistency without a wall-clock
   budget, so sanitizer instrumentation and runner scheduling do not fail those
   functional checks.

## Committed baseline (C4, 2026-11-08, feature disabled)

The data is in `results/2026-11-08-disabled-run{1,2}.json`, and
`results/2026-11-08-disabled-repeatability.json` holds the output of
`compare.mjs repeat`.

**Host**

- AMD Ryzen 9 7950X, 32 logical cores, `powersave` governor.
- 128 GB RAM, kernel 7.0.0-38-generic, Ubuntu 26.04.1.
- Storage: ext4 on a Samsung 990 PRO NVMe.
- ffmpeg 8.0.1, go2rtc 1.9.14.

**Workload:** 4 × 1280×720 at 15 fps, 10 s segments, SOD `tiny20.sod` every
1 s, 30 s warm-up, 300 s window.

| Series | Run 1 | Run 2 | Variance |
| --- | --- | --- | --- |
| `segment_interval_ms` p95 | 10035.3 | 10037.9 | 0.03% (pass) |
| `db_probe_ms` p95 | 11.95 | 12.32 | **3.02% (fails the 2% criterion)** |
| Recording gaps (metrics / catalog / interval) | 0 / 0 / 0 | 0 / 0 / 0 | |
| Segments in window (expected 120) | 120 | 120 | |
| Detector inferences per stream per second (configured 1) | 1.003 | 1.003 | |
| lightNVR CPU, average percent of one core | 144.2 | 146.7 | |
| RSS high-water (KiB) | 875 932 | 874 988 | |
| Disk write rate (B/s) | 2.33 M | 2.33 M | |

**Caveats on these numbers**

- **The DB probe did not meet the 2% criterion.** The dev host was shared with
  other parallel agents during the runs. The 1-minute load average rose from
  about 3.8 to 9.25 during run 2. The HTTP round-trip p95 at about 12 ms is
  sensitive to that.
  - The segment-interval series is stable.
  - Before X2 relies on the DB-probe series at a 5% threshold, rerun on a
    quiet host, or on the dedicated perf host, with `--seconds 600` or more.
    Confirm its variance first. If it stays above 2%, treat it as advisory
    and gate only on segment interval plus gaps. Record that decision in C1.
- **Binary provenance.** `build/int01-c4-release/bin/lightnvr` was built at
  08:11 from `c46bff1c` (fresh Release, system FFmpeg 8.0.1) and
  `binary_sha256` identifies it.
  - `build.revision` in the JSON is the checkout at run time (`3c61e235`).
    Parallel tasks committed docs-only changes in between. The tree may also
    have held other agents' uncommitted, unwired edits.
  - No observation code was compiled in, so the feature is absent.

## X2 result (2026-11-08, A11): pass

The data is in `results/2026-11-08-x2-*.json`, and `results/2026-11-08-x2-gate.json`
holds every gate, the pooled comparisons, disabled repeatability, and two runs
showing the gate fails at impossible thresholds (exit 1).

**Setup**

- **Binary:** `build-int01-X2/bin/lightnvr` from `9ed02bb4`. It is a Release
  build with LiteRT on, MQTT and SOD, built against the apt FFmpeg 8.0.1
  headers through a symlink include dir.
- **Clip:** `make-bird-clip.sh` (sha256 `5bf26bbc…`; see the results).
- **Host:** the same as C4. The 1-minute load average was 2.8–7.8. The
  user's own lightNVR, a uvicorn, Mosquitto, a `llama-server` using one core,
  and other desktop work were running.
- **Runs:** 600 s each, recorded in this order:
  1. disabled-run1;
  2. enabled-defaults;
  3. enabled-saturated;
  4. disabled-run2;
  5. disabled-run3;
  6. enabled-defaults2;
  7. enabled-saturated2.

```sh
S=/path/to/scratch
tests/performance/observations/make-bird-clip.sh cardinal.jpg $S/bird.mp4
echo '{"observation_evidence_enabled": true, "observation_max_bytes": 67108864}' > $S/stream.json
COMMON="--binary build-int01-X2/bin/lightnvr --model ~/tiny20.sod --source $S/bird.mp4 --seconds 600 --warmup 30"
node tests/performance/observations/run-workload.mjs $COMMON --mode disabled --output build/perf-observations/x2-disabled
node tests/performance/observations/run-workload.mjs $COMMON --mode enabled --stream-extra $S/stream.json --output build/perf-observations/x2-enabled
node tests/performance/observations/run-workload.mjs $COMMON --mode enabled --stream-extra $S/stream.json \
  --system-setting observation_global_queue_bytes=4194304 --output build/perf-observations/x2-saturated
node tests/performance/observations/compare.mjs gate build/perf-observations/x2-disabled/results.json \
  build/perf-observations/x2-enabled/results.json --advisory latency.db_probe_ms.p95
node tests/performance/observations/compare.mjs gate build/perf-observations/x2-disabled/results.json \
  build/perf-observations/x2-saturated/results.json --advisory latency.db_probe_ms.p95 --require-saturation
```

| | Disabled (3 runs) | Enabled, defaults (2) | Enabled, saturated 4 MiB (2) |
| --- | --- | --- | --- |
| Detections per window / observations in DB | 2,408 / 0 | 2,407–2,408 / 2,524–2,532 | 2,404 / 2,529–2,532 |
| Accepted / `queue_full` / untracked | — | 2,407–2,408 / 0 / 0 | 1,063–1,073 / 1,331–1,341 / 0 |
| Recording gaps (3 sources) | 0 | 0 | 0 |
| `segment_interval_ms` p95 | 10008.9–10016.1 | 10008.5–10013.5 | 10009.3–10012.9 |
| `db_probe_ms` p95 (advisory) | 12.74–13.40 | 13.99–14.83 | 13.70–14.23 |
| Pooled `db_probe_ms` p95 vs disabled | 13.13 | 14.43 (+9.9%) | 13.98 (+6.5%) |
| Submit latency: max / p99 bucket / mean | — | 341–396 µs / ≤ 50 µs / 4.1–4.4 µs | 365–405 µs / ≤ 50 µs / 4.3–4.4 µs |
| Queue high-water: items / bytes (limit) | — | 4 / 11.1 MB (64 / 32 MiB) | 1 / 2.77 MB (64 / 4 MiB) |
| Per-camera high-water (limit 8) / drop ring max (256) | — | 1 / 0 | 1 / 2 |
| Rejected frames: deferred / inline / pending max | — | 0 / 0 / 0 | 1,331–1,341 / 0 / 5.5 MB |
| lightNVR CPU (% of one core, avg) | 148.9–155.5 | 154.1–158.0 | 151.0–154.9 |
| RSS high-water | 873–888 MB | 891–892 MB | 882–929 MB |
| Disk write rate | 2.06 MB/s | 3.29–3.32 MB/s | 2.87 MB/s |
| Evidence retained per camera (budget 64 MiB) | — | 44.8–45.0 MB | 16.7–24.0 MB |

**Gate decision.**

- `db_probe_ms` p95 fails its own 2% repeatability. Across disabled pairs it
  varied by 2.61%, 5.03% and 2.42%. Following C4's rule it is
  `--advisory`: reported, not gating.
- With it advisory, all four gates pass:
  - disabled-run1 against enabled-defaults and enabled-saturated;
  - disabled-run3 against enabled-defaults2 and enabled-saturated2.
- Without `--advisory`, the second pair still passes (+4.4% and +2.2%), but
  the first fails (+13.4% and +8.8%). That pair ran during a host load spike
  (load average about 7).
- **The pooled +9.9% / +6.5% DB-probe tail is a recorded finding**, not
  noise to ignore.
  - The median is unchanged (8.1–8.4 ms).
  - The likely cause is the evidence worker's extra WAL traffic. It causes
    more frequent SQLite autocheckpoints, and those run inside commits that
    hold the global DB mutex.
  - Shrinking admission cannot remove it, because drops still insert an
    `unavailable` row. The saturated mode accepted 56% fewer frames and only
    moved +9.9% to +6.5%.
  - Recording itself, measured by segment interval and gaps, is unaffected.
  - Follow-up: run checkpoints outside the global mutex, or batch the
    worker's commits.

**Finding fixed in `9ed02bb4`.** In the first saturated smoke runs, about 8%
of submissions (and 10 of 120 in the window) took 1–4.4 ms. The cause was
`free()` of the rejected 2.7 MB frame on the detection thread, which is an
munmap. Rejected frames are now released by the worker.

**Not exercised.** Media eviction: the clip's JPEGs are about 100 KB, and a
camera retained at most 45 MB against the 64 MiB minimum budget.
`budget_exhausted` needs protected assets. Both are covered by the L4 and
L11 tests.
