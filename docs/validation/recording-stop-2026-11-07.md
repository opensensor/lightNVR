# Recording stop qualification — 2026-11-07

Base: `f7d391e7c67aadadd9c8e227e0c3b0e15b5376e7` (merged #853).
The active-recording harness found a corrupt final H.264 frame when stopping a
high-resolution HTTP MPEG-TS recording. FFmpeg's demuxer can return a successful
read containing a partial packet after the I/O interrupt flag is set. The
recorder previously appended that packet before noticing the stop.

The recorder now drops newly read packets once the interrupt flag is observed,
ends input reuse, and follows the existing final-segment cleanup/audio drain.
A previously carried boundary keyframe remains valid and is not discarded by
this check. No new API or stop deadline is introduced.

## Reproduction and regression

- Original binary SHA-256: `6681d8836956262ca1c9af30458fe59ed81becfaa75926ccddda174a5a461c66`.
  One 1080p/15fps stream, ten seconds, then manual stop failed full strict decode
  with `Invalid level prefix` / corrupt macroblock 98,43. Retained evidence:
  `/tmp/lightnvr-mobile-experience/build/active-recording-strict/results.json` and
  `/tmp/lightnvr-contract-17cVQh`.
- Same workload with the packet guard passed. The new media lifecycle contract
  case passed under Debug ASan/UBSan (including leak detection); no sanitizer
  diagnostic was found in the retained process/server logs.
- All seven media lifecycle API cases passed against the Release build.
- Existing interrupted-recording recovery passed with the Release binary and
  pinned go2rtc: `/tmp/lightnvr-recovery-714tkhun`. This is the bounded crash and
  immediate-reconciliation scenario, not extended storage or power-loss testing.

## Reproduction environment

AMD Ryzen 9 7950X, 32 logical CPUs, 131,701,190,656 bytes RAM; local Linux storage.
CMake Release (`-O3 -DNDEBUG`) and Debug (ASan/UBSan), SSL/go2rtc on,
LiteRT/SOD/Sentry/MQTT off. System FFmpeg headers/libraries were selected with
`PKG_CONFIG_LIBDIR` and `-isystem /usr/include/x86_64-linux-gnu` because the host
also has unrelated `/usr/local` headers. Full compiler flags and hashes are in
the measurement JSON. This workstation is not a declared 2024+ target NVR.

The [harness instructions](../../tests/performance/active-recording.md) describe
reproduction and measurement limits. F13 target capacity, F14 extended
storage/upgrade qualification and F15 production acceptance remain open.

## Serial eight-stream observation

Release binary SHA-256: `db5a8b5699c26a041470b53382b2d592a8705bdf1ad1fe549230ebcd9da7cf44`.
60 samples over the declared 60-second interval; all 15 finalized
files from all eight streams passed strict full decode. List latency median
12.85 ms, observed p95 14.06 ms and maximum 15.07 ms.
Native RSS range 86.2–172.8 MiB;
peak high-water RSS 172.8 MiB; descriptor range
28–44.

This final run was serial, without other agent builds/tests. It is a bounded
continuity observation, not evidence of sustainable capacity or a latency SLO.
Raw receipt: `/tmp/lightnvr-recording-stop/build/active-eight-serial/results.json`
(SHA-256 `3711254e487eb24e57df99d7aa22a226bcbacbc570216eb6913991e9889c8e40`).
