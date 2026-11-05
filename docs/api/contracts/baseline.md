# Real-server contract baseline

Captured 2026-11-05 before T3 dependency/toolchain changes, at source HEAD
`dea6094a731c86debb03cc1c75d0251ab3b96cec` (LightNVR 0.46.3).
This is a workstation baseline, not embedded-device qualification.

## Source and build evidence

- Linux x64, kernel `7.0.0-38-generic`, Node `v24.22.1`, FFmpeg libavformat `61.7.100`.
- Fresh executable: `build/eng-api-T2/bin/lightnvr`, SHA-256
  `b02db6c8d4503f9305aa78c768daff3806e9cec5c30286f4987338a53aa1aefd`.
- Native source digest (534 sorted tracked and relevant untracked files under
  `src`, `include`, `CMakeLists.txt`):
  `bd94a464df621c8b6186c775fdeb4c8bddbc8f0ffc4f9d48717e67e364aae26a`.
  Digest input is each UTF-8 path, NUL, file bytes, NUL, in sorted path order.
- Asset measurement source digest (918 files; native, web, package manifests,
  measurement script and contract fixture sources):
  `748f831a592a58d9611e8856920d3633f326e30d014f9ecc2887206650032e67`.
  Each raw report includes the complete filename manifest and per-asset hashes.
  Final fixture-only signal/bind ownership guards followed this asset snapshot;
  they do not change the native binary or web assets.
- Relevant tracked native/web diff was empty. Existing unrelated working-tree
  changes were retained. No dependency installation occurred during T2.

```sh
cmake -S . -B build/eng-api-T2 -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON \
  -DENABLE_SOD=OFF -DENABLE_LITERT=OFF -DENABLE_GO2RTC=OFF \
  -DENABLE_MQTT=OFF -DENABLE_SSL=ON
cmake --build build/eng-api-T2 --target lightnvr -j6
```

Debug enables ASan/UBSan in this project; neither was disabled for these runs.
Existing compiler warnings were present. Build logs:
`/tmp/lightnvr-eng-configure.log`, `/tmp/lightnvr-eng-build-T2.log`.

## Observed HTTP behavior

The fixture creates disabled, non-recording A/B cameras through the supported
API, changes the bootstrap admin password through its password endpoint, marks
setup complete, creates a viewer with a camera-A selector grant, and creates an
admin-owned `live.view` token restricted to A. No offline DB mutation or real
camera/credential is required. Disabled cameras have runtime entries and support
the full endpoint. Credentials in camera fixtures are synthetic; the POST
handler ignores ONVIF credentials when `isOnvif` is false, so the populated
`admin_url` is the independent sensitive-field redaction assertion.

- Anonymous list: 401.
- Admin compatibility list: A and B, both disabled and recording false.
- Viewer-A and scoped read-A token: list contains only A; summary contains only
  A with `total=1`. Summary is selected with `summary=true`.
- Full A is wrapped in `{stream: ...}`. Viewer/token responses redact `admin_url`
  and ONVIF password to empty strings and report `can_control_privacy=false`.
  The admin A response includes the seeded administrative URL.
- Viewer/token direct full B: 403. Their privacy writes on A: 403, with the
  complete admin-readable configuration unchanged.
- Admin mixed privacy/update payload: 400 with unchanged configuration.
- Admin standalone privacy update: 200; A privacy becomes true and B remains
  false. The disabled stream returns `stream_registered=false`.

[Captured payloads](../../../tests/contracts/fixtures/baseline-payloads.json)
contain 12 actual success/error cases from the C binary, including original
synthetic camera UUIDs and request descriptions. They are independent evidence
for T4 schemas, not generated expectations. Bearer secrets and credentials are
not captured. Later health-dependent selectors, configure-only/mixed scopes,
revocation and discovery cases remain T7/T16 work.

## Asset baseline

Installed bytes include all files in `web/dist`, including `.gz` copies.
Per-entry values below count each reachable local asset once, including lazy
imports and legacy/modern scripts; they are **not initial transfer sizes**.
Navigation links to other HTML pages are excluded. Runtime media, API responses,
and computed URLs are excluded. Gzip is the sum of level-9 gzip sizes for the
same closure. The script records the exact closure filenames for review.

| Profile | Installed bytes | Files |
| --- | ---: | ---: |
| default | 11,727,012 | 229 |
| modern | 8,120,386 | 169 |

| Entry | Default bytes | Default gzip | Modern bytes | Modern gzip |
| --- | ---: | ---: | ---: | ---: |
| hls.html | 3,482,393 | 917,344 | 1,509,735 | 437,736 |
| index.html | 3,497,663 | 921,878 | 1,517,327 | 440,014 |
| investigation.html | 1,680,334 | 375,801 | 611,399 | 164,547 |
| login.html | 1,442,220 | 307,779 | 496,402 | 130,519 |
| plates.html | 1,195,222 | 223,996 | 372,958 | 88,371 |
| recordings.html | 1,786,332 | 408,876 | 667,456 | 181,749 |
| settings.html | 1,883,585 | 402,130 | 717,492 | 177,932 |
| streams.html | 2,015,536 | 443,211 | 783,625 | 198,767 |
| system.html | 1,597,824 | 350,945 | 574,588 | 152,246 |
| timeline.html | 1,810,906 | 424,308 | 681,244 | 189,874 |
| users.html | 1,716,576 | 389,484 | 634,135 | 172,398 |

Raw reports: `/tmp/lightnvr-eng-baseline-default.json` and
`/tmp/lightnvr-eng-baseline-modern.json`. These local evidence files contain
source/environment metadata, file byte counts/hashes and per-entry closures.
Repeat before changing dependencies to compare the same baseline:

```sh
npm --prefix web run build
node scripts/measure-contract-assets.mjs --dist web/dist --profile default --out /tmp/lightnvr-eng-baseline-default.json
LIGHTNVR_WEB_LEGACY=false npm --prefix web run build
node scripts/measure-contract-assets.mjs --dist web/dist --profile modern --out /tmp/lightnvr-eng-baseline-modern.json
```

Both profiles built successfully. Existing large-chunk/bundler timing warnings
remain. Final `web/dist` is the default profile. Build logs are
`/tmp/lightnvr-contract-build-default.log` and
`/tmp/lightnvr-contract-build-modern.log`.

## Fixture interface and validation

Run from the repository root, or set `LIGHTNVR_CONTRACT_ROOT` explicitly.
`startFixture({binary?, port?, startupTimeoutMs?, seed?, retain?})` defaults to
`LIGHTNVR_CONTRACT_BIN` or `build/eng-api-T2/bin/lightnvr`. It returns its owned
`directory`, `pid`, `port`, `baseURL`, `logPath`, seeded `users`, `cameras`, and
in-memory `tokens`. Request helpers:

- `request(route, {principal, method, body, headers})` returns a Fetch Response;
  `json(route, options)` checks success and parses JSON.
- Principals: `admin`, `viewer`, `readA`, or `anonymous` (no credentials).
- `apiContext(playwright.request, principal)` creates a separate cookie-free
  Playwright API context. Caller disposes it.
- `browserContext(browser, principal)` creates a browser context and logs in
  password principals through the real session endpoint. Caller closes it.
- `stop({failed})` is idempotent, terminates only the owned child and deletes
  only its own successful fixture directory. Failures retain the entire tree.

Startup is bounded to 30 seconds, requests to 10 seconds and graceful shutdown
to 15 seconds before killing the owned PID. The port is reserved on loopback;
collision fails rather than attaching to another server. Readiness requires
both the owned PID file and a successful bind message in its own server log.
A bind race fails closed within the startup deadline. There is no `pkill` or
shared test directory. Unexpected native signals, nonzero exits, sanitizer
findings and forced shutdown timeouts fail cleanup and retain diagnostics.

```sh
node --test tests/contracts/fixture-server.test.mjs
LIGHTNVR_CONTRACT_BIN=build/eng-api-T2/bin/lightnvr npx playwright test -c tests/contracts/playwright.config.ts baseline.spec.ts --project=api --workers=1
```

Validation: fixture suite **4 passed**; API baseline **1 passed**. The lifecycle
suite proves two simultaneous real servers coexist, stopping one preserves the
other's files/API, occupied ports reject safely, startup failure logs remain,
and unexpected SIGKILL is reported as failure. Intentional failure fixtures
emit retained-directory messages. No spontaneous sanitizer failure occurred.
A Chromium smoke additionally verified a real viewer session cookie, one-row
scoped API result and `streams.html` HTTP 200.

Logs: `/tmp/lightnvr-contract-fixture-tests.log` and
`/tmp/lightnvr-contract-baseline-tests.log`. Playwright keeps traces/screenshots
on failure; baseline failures attach process logs and retain the server tree.
The API project explicitly discovers `baseline.spec.ts` and `*.api.spec.ts`;
the UI project discovers `*.ui.spec.ts`. Existing root Playwright discovery is
unchanged and remains under `tests/integration`.

T2 acceptance is complete. Exclusive Node dependency and `web/dist` ownership
is released to T3; subsequent phases can reuse the fixture and raw evidence.
