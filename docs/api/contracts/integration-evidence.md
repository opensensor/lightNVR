# Four-workstream integration evidence

Date: 2026-11-07. Main baseline: `84ed0174`. This receipt describes the
review stack, not released main or production hardware qualification.

## Review order

1. [Media authority #835](https://github.com/opensensor/lightNVR/pull/835),
   supported by [go2rtc #4](https://github.com/opensensor/go2rtc/pull/4).
2. [Identity administration #836](https://github.com/opensensor/lightNVR/pull/836).
3. [Fleet writes #837](https://github.com/opensensor/lightNVR/pull/837).
4. [Remaining API families #838](https://github.com/opensensor/lightNVR/pull/838).

Each parent PR is based on the preceding branch. No PR was merged by this work.
Domain receipts in this directory document the owned fixtures, RED/GREEN
regressions, exact behavior and limits for each family.

## Integrated inventory

The registration inventory accounts for all 241 native registrations: 232
OpenAPI operations across 168 paths, eight media-protocol dispositions and
one internal lease callback. The inventory test verifies source digests, lines,
handlers, unique identities and canonical method/path mappings. Those nine
explicit exclusions keep the discovery document's `partial` flag true.

## Local validation

- 137 real-server API contract cases passed with MQTT enabled, including
  permission intersections, denied/no-effect writes, actual SOAP/MQTT/media
  bytes, owned storage destinations and restart/shutdown child processes.
- All 44 real-browser contract cases passed, including the complete virtualized
  explorer, principal/session isolation, malformed acknowledgments, and both
  restart observation deadlines (HTTP 200 and 503 without an observed outage).
- 82 web unit suites / 956 tests and 12 component browser suites / 69 tests
  passed. Old mocks were updated to native envelopes; malformed responses still
  produce errors. Explorer tests walk the actual virtualized document to
  observe every administrator operation without enabling mutation controls.
- 98 Release native suites passed across the full run and focused rerun of
  two projection assertions updated for the new GET stream contract. Both
  continue to deny PUT/DELETE and privileged schemas to read-only principals.
  The final metadata rebuild and both projection/handler suites also passed.
- All 75 selected ASan/UBSan suites passed after the integrated production
  changes. These builds disable managed go2rtc; its separate evidence is below.
- Root, web and contract TypeScript checks, canonical lint, generation drift,
  route inventory and production web build passed. Fourteen tooling tests
  include 33,306 Ajv equivalence cases across 306 schemas.

The final API run used `LIGHTNVR_CONTRACT_MQTT=1` and
`LIGHTNVR_CONTRACT_BIN=build/quality-833/fallback/bin/lightnvr` with
`npx playwright test -c tests/contracts/playwright.config.ts --project=api --workers=2`.
The UI run uses the same binary/configuration with `--project=ui`. These
fixtures allocate isolated databases, ports and owned services; they do not
operate an installed user instance. Local logs are under ignored
`build/hardening-four/`.

## Media coverage follow-up

At #835 commit `2567b8a8`, the production patch has local LCOV coverage of
215/218 executable lines (98.62%). All 144 proxy, proxy-thread and WebSocket
lines in the original missing-coverage report are exercised. CI now includes
the proxy-query and WebSocket suites in the go2rtc-enabled job. Additional
regressions exercise lease registry capacity/reclamation, invalid principals
and cameras, entropy failure and missing upstream policy acknowledgments.
See [media authority evidence](media-authority-evidence.md) for commands,
protocol/race evidence and independently reproduced pre-existing Go failures.

Hosted results must be evaluated for each exact PR head. The preceding #835
revision passed the go2rtc-enabled job and coverage upload; its successor's
coverage aggregation was still pending when this receipt was written.
Local percentages are not claims about the final hosted Codecov report.

## Measured artifacts and remaining qualification

The final production build emits 1,344,968 bytes of canonical JSON and
6,082,236 bytes of readable generated validator JavaScript. Vite reports the
modern validator chunk at 2,240.15 kB raw / 218.55 kB gzip and the legacy chunk
at 2,230.06 kB raw / 181.69 kB gzip. These are artifact sizes, not measured
startup, parse cost or active-media performance on representative hardware.
F13 should measure those costs and prioritize splitting unused validators.

F13 capacity work and F14 extended recovery/storage/upgrade and physical-device
qualification remain outstanding; F15 production acceptance depends on them.
Owned loopback SOAP and media fixtures do not establish vendor-camera,
multicast or remote-provider interoperability. Legacy name-only history retains
its documented compatibility boundary, and identity rechecks do not introduce
a global transaction coordinator. At-rest encryption and Rust remain deferred.
Basic/legacy-key compatibility, HTTP cameras and `/etc/lightnvr` defaults are
preserved.
