# Post-merge recovery qualification — 2026-11-07

Fresh Release build from `6b346f40` after #854–#857, using the pinned go2rtc
`ba99c7e76365b4b69cbbc155925616ac867c9760`. All four report validators passed,
and all reports identify the same native binary. The [machine-readable receipt](recovery-qualification-2026-11-07.json)
contains build flags, hashes, declared bounds, resource series and retained raw
report locations. No production configuration, camera or recording was used.

## Observed results

Lifecycle used one synthetic 320×180 H.264 source at 10 fps, a one-second GOP
and six-second recording segments. This establishes no multi-camera capacity.

- **Lifecycle, ten cycles per phase:** ten five-second source outages, ten
  overlapping manual stop/stop/start races and ten process shutdown/restart
  overlaps. Completed playable media returned within 6.54 seconds of source
  resume (60-second bound); largest catalog gap was seven seconds (72.2-second
  bound). Graceful shutdown took at most 4.53 seconds (30-second bound).
- Recorder reconnect samples remained at 21 descriptors and 83 threads.
  Go2rtc descriptors remained at 24; threads stayed within the declared jitter.
  Manual-mode and restart resources also passed their bounds. All 43 final
  catalog rows were checked; media requiring an interrupted-media explanation
  remained distinguishable from playable clips. Decodable media passed strict
  FFmpeg decoding and monotonic-DTS checks. Protected hashes and database
  integrity passed; no managed child outlived checked graceful shutdowns.
- **Storage faults:** all four cases passed: injected full/read-only write
  failures, actual kernel permission denial and an unavailable directory.
  Boundary self-tests confirmed fault isolation; protected media/catalog identity
  survived cleanup. This is not physical disk exhaustion or power-loss testing.
- **Interrupted recording:** fresh crash recovery and immediate reconciliation
  passed with protected-file checks and explicit playable/unplayable outcomes.
- **Historical upgrade:** a real 0.45.3 installation at `c82d6ed1`, with its own
  historical dependency pins, was copied and upgraded. Both current launches,
  authentication, media, schema/integrity and original-file preservation passed.
  The prior binaries were reused only after their source/dependency/executable
  hashes and clean checkout were verified; they were not recompiled this run.

## Reproduce and interpret

Use the [reliability harness instructions](../../tests/reliability/README.md).
Capture current provenance immediately after building, pass that manifest to
interrupted/lifecycle/upgrade runs, use `--cycles 10` for the lifecycle run, and
validate each report with `prepare-upgrade.py validate-report`. This build used
Release, managed go2rtc and SSL; LiteRT/SOD/MQTT/Sentry were off. System FFmpeg
headers/libraries were selected explicitly because this workstation also has
unrelated `/usr/local` headers. Exact flags are in the receipt.

These runs took place on the development workstation alongside UI testing.
Their timings establish the declared local test bounds, not production latency
or capacity budgets. F14's local recovery/storage/upgrade gates have fresh
passing evidence; multi-day operation, named 2024+ NVR/camera/phone hardware,
provider faults and physical network/storage behavior remain unqualified.
Supporting go2rtc PR #4 is still open although its commit is pinned and tested.
F15 production acceptance remains open.
