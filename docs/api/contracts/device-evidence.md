# P4b device contracts receipt

Base: P1 `5c336457`, isolated `codex/hardening-device-contracts`. This slice owns
22 registrations, alongside the separate stream lifecycle/media slice. Basic,
session, legacy-key, scoped-token and HTTP-camera compatibility remain supported.
No physical cameras, user instances, or LAN multicast scans were contacted.

## Exact registration coverage

`{name}` is the URL-encoded camera name; native registration uses `#`.

| Method | Route | Operation |
|---|---|---|
| GET | /api/streams/{name}/ptz/capabilities | getPtzCapabilities |
| GET | /api/streams/{name}/ptz/presets | getPtzPresets |
| POST | /api/streams/{name}/ptz/move | movePtz |
| POST | /api/streams/{name}/ptz/stop | stopPtz |
| POST | /api/streams/{name}/ptz/absolute | absolutePtz |
| POST | /api/streams/{name}/ptz/relative | relativePtz |
| POST | /api/streams/{name}/ptz/home | homePtz |
| POST | /api/streams/{name}/ptz/set-home | setPtzHome |
| POST | /api/streams/{name}/ptz/goto-preset | gotoPtzPreset |
| PUT | /api/streams/{name}/ptz/preset | setPtzPreset |
| GET | /api/streams/{name}/imaging/settings | getImagingSettings |
| PUT | /api/streams/{name}/imaging/settings | setImagingSettings |
| GET | /api/streams/{name}/imaging/options | getImagingOptions |
| GET | /api/streams/{name}/daynight | getDaynight |
| PUT | /api/streams/{name}/daynight | setDaynight |
| GET | /api/onvif/discovery/status | getOnvifDiscoveryStatus |
| GET | /api/onvif/devices | getOnvifDevices |
| POST | /api/onvif/discovery/discover | discoverOnvifDevices |
| GET | /api/onvif/device/profiles | getOnvifProfiles |
| POST | /api/onvif/device/add | addOnvifDevice |
| POST | /api/onvif/device/claim | claimOnvifDevice |
| POST | /api/onvif/device/test | testOnvifDevice |

Camera reads require `live.view`; PTZ mutations require `ptz.control`; imaging
writes require `camera.configure`. Global ONVIF endpoints require global
`camera.configure`. A camera-only configure grant does not expose discovery or
permit arbitrary device URLs. Schemas and projection reuse those native rules;
there are no new authorization predicate kinds.

The canonical document adds precise request/response schemas, generated C bytes,
TypeScript types and runtime validators, and 22 coverage dispositions. Existing
schemas and operations are semantically unchanged. Response schemas reject
unknown properties and malformed nested rows. Camera-dependent imaging fields
are explicitly optional, not unconstrained objects. Numeric requests must be
finite and fit the native float representation; actual position and imaging
ranges remain device-dependent. Capabilities expose the device ranges and the
`onvif_queried` flag distinguishing returned data from native fallback defaults.
Legacy imaging/daynight aliases remain accepted. ONVIF `insecure` remains a
legacy accepted boolean hint; it does not change the native transport policy.

## Production fixes and RED → GREEN

- A PTZ move with `{"pan":"fast"}` returned 200 and reached SOAP discovery.
  The real-server regression expected 400. Request parsing now rejects wrong
  types, non-object bodies, unknown/duplicate fields, trailing data, nonfinite
  or unrepresentable numbers, and oversized preset values before device I/O.
  Partial invalid imaging updates also fail before resolving the source token,
  rather than silently applying the remaining values. Invalid optional ONVIF
  credentials and discovery fields fail before device/discovery effects.
- A multi-service `GetServices` result without the supported Media namespace
  entered an infinite native loop: end-of-list traversal restarted at the
  second service. The original owned SOAP case timed out and the native process
  required termination. Removing the restart yields a bounded fallback. The
  same fixture remains a regression case; it intentionally advertises a
  nonmatching Media namespace alongside PTZ and Imaging services.
- The fallback generated `/onvif/onvif/media_service`. An exact-path assertion
  failed before the prefix correction and passes with `/onvif/media_service`.
- ONVIF add used the input JSON's `stream_name` after freeing that JSON. A
  controlled regression run restored only the old pointer statement in an
  isolated build and ASan reported **heap-use-after-free** in the success
  response. Restoring the bounded owned copy passes the same test and sanitizer.
  The real-server add test also verifies the exact returned name and persisted
  camera UUID, followed by an inventory claim bound to that UUID.
- A SOAP Fault from GetPresets returned an empty 200 response. The real-server
  fault test failed (expected 502, received 200); the native parser now returns
  the handler's documented device error. Preset names/tokens are XML-escaped;
  actual outgoing bytes preserve `Owned & safe` as text (`Owned &amp; safe`).
- The actual Streams ONVIF discovery UI accepted malformed device rows. The
  browser regression against the prior compiled consumer failed to show a
  contract error; the updated build rejects the rows, does not render the
  private wire marker, performs one request, and recovers on a valid empty list.

## Runtime consumers

`web/js/api/device.ts` maps each closed operation to its precise route, method,
request and response validator. It validates status, JSON content type, parsed
body and nested shape; preserves cancellation; excludes wire payloads and device
credentials from errors; and never retries device commands automatically. Failed
write acknowledgments carry `mayHaveApplied` so a failure is not mistaken for
proof that the device stayed unchanged.

The adapter is used by the existing `ptzActions.ts` control functions,
`DayNightControl.tsx`, and StreamsView's inventory/discovery/profiles/test/claim
flows. Existing captured ONVIF selection/generation checks remain in place.
ONVIF-created stream protocol/priority are now numeric, matching the native
create contract. Imaging APIs without an existing UI consumer are covered by the
same typed adapter rather than adding speculative UI.

## Independent evidence

`tests/contracts/device.api.spec.ts` uses the actual isolated LightNVR server and
`device-fixture.ts`, an owned HTTP/SOAP listener on an ephemeral loopback port.
Expected permissions and operation sets are handwritten independently of schemas.
The matrix checks admin Basic, authenticated session, legacy key, read-only user,
camera-scoped operator, camera-scoped token, all-camera token, empty-policy user
and anonymous callers. Rejected PTZ/imaging/ONVIF calls produce zero SOAP
requests; denied add/claim also preserve durable inventory/configuration. Camera
A tokens cannot control B. Camera-only authority cannot use global ONVIF APIs.

All 15 PTZ/imaging/daynight operations exercise actual SOAP exchanges and validate
canonical and projected success responses. Tests inspect emitted actions,
coordinates, persistence flags, escaped text and source-token binding. They cover
real advertised position ranges, optional imaging settings/options, fallback
capabilities, SOAP faults, empty profiles, missing claim targets, connection
success/failure, new camera creation and inventory claims. ONVIF profile access
uses the real `X-Device-URL`, `X-Username`, `X-Password` header protocol.

Discovery success is intentionally tested through a native linker-wrapped
transport returning a synthetic device (`test_api_handlers_device.c`). The real
handler serializes the full inventory envelope; invalid discovery requests never
enter that transport. This is **not** multicast discovery qualification. Native
unit fixtures use unique `mkstemp` database paths; browser/API fixtures use their
own temporary directories, credentials, cameras and listeners.

## Validation

- Release real-server device API suite passed (initial five cases); final six
  API cases pass under Debug ASan/UBSan, including real add/claim and exact paths.
- Real Chromium ONVIF consumer regression passes after its recorded RED run.
- Native three-case device suite passes Release and ASan/UBSan; the isolated
  pointer regression independently produces the recorded ASan UAF diagnostic.
- Full web suite: **72 suites, 866 tests passed**. Focused adapter/PTZ suite:
  21 cases covering invalid requests, malformed acknowledgments, wrong status or
  content type, cancellation, exact captured routes, failed-refresh preservation,
  sanitized errors and no retries.
- Web/contract/root test type checks, production web build, contract generation
  drift and OpenAPI lint pass. Lint reports the existing four ambiguous-path
  warnings; no new warning category was added. Contract tooling tests pass.

Commands (from the isolated worktree):

```sh
LIGHTNVR_CONTRACT_BIN=build/device-asan/bin/lightnvr \
  npx playwright test -c tests/contracts/playwright.config.ts device.api.spec.ts --project=api
LIGHTNVR_CONTRACT_BIN=build/device/bin/lightnvr \
  npx playwright test -c tests/contracts/playwright.config.ts device.ui.spec.ts --project=ui
ctest --test-dir build/device-asan --output-on-failure -R '^test_api_handlers_device$'
npm test --prefix web -- --runInBand
npm run typecheck --prefix web
npm run contracts:typecheck
npm run test:typecheck
npm run build --prefix web
npm run contracts:check
npm run contracts:lint
npm run contracts:test-tools
```

Physical multicast discovery, vendor-specific coordinate spaces and real camera
motor/imaging behavior remain qualification work. The local evidence proves
native authorization, request validation, SOAP bytes, response contracts and UI
handling; it does not claim that every vendor accepts every advertised command.

## Redirect transport follow-up

The device adapter now forces `redirect: 'error'` and `credentials: 'same-origin'`
after caller options. Owned two-origin HTTP tests reproduced ONVIF username and
password forwarding through both 307 and 308 before the correction. Afterward,
the source sees one request and the redirect destination receives none, even
with explicit caller `redirect: 'follow'`/`credentials: 'include'` options. The
shared `redirectHarness.ts` runs real HTTP without contacting any external host;
`deviceRedirect.spec.ts` supplies the regressions. Focused device/PTZ tests:
23 passed; frontend type checks passed.
