# T29 process lifecycle qualification

Run only while holding the shared native/server/media lease. The harness itself
acquires T13's exclusive runtime lock and refuses occupied reserved ports.

```sh
python3 scripts/quality/run-native.py --task T29.0 --regex '^test_shutdown_coordinator$' --jobs 4
python3 scripts/quality/run-native.py --task T29-server --profile integration --sanitizer none --build-only --jobs 4
node tests/operational/lifecycle/run.mjs build/eng-native/T29-server/integration-none/bin/lightnvr
```

The process harness reuses T13's token-owned run contexts, config hashing, PID
identity records, pidfd signaling/descendant cleanup and artifact archiving.
It never uses a preexisting server or runtime directory. The deliberate crash
uses a pidfd verified against the recorded start time and executable. That
injected SIGKILL and any descendant cleanup are recorded separately and never
counted as graceful shutdown. Every normal stop has an independent60-second
fixture deadline and must need no forced process or child cleanup. This includes
the production recorder's15-second outer join and up-to10-second inner writer
stop, plus server/database/media cleanup; it matches T13's normal grace window.

The scenario starts current code on a source-derived synthetic0.45.0 database
and config, probes preserved playable footage, checks stale valid/empty/missing/
truncated and fresh incomplete rows, and verifies browser preferences in real
Chromium. It then records actual synthetic RTSP video through managed go2rtc,
checks playable committed segments, stops and restarts, intentionally kills an
active recorder, and checks SQLite integrity, committed-byte preservation and
resumed recording. Media readiness has a45-second fixture deadline; these host
functional deadlines are not physical-device/performance qualifications.

All individual assertions are written to `checks.json`. A failure remains a
nonzero run even if cleanup succeeds. Failed production assertions require a
bounded follow-up; they must not be weakened to describe the observed defect as
successful recovery. Artifacts and isolated runtime state are retained for
review, including phase logs, config, schema/source hashes and stop reports.
