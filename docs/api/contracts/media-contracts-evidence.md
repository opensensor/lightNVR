# Stream lifecycle and media contract evidence

This P4b contribution accounts for **23 registrations**: 15 typed JSON operations
and eight explicitly classified protocol/compatibility surfaces. Device ONVIF,
PTZ, imaging and day/night contracts are owned by the separate device contribution.
This branch builds on the media-authority work and its follow-up fixes.

## Public JSON contract

| Family | Operation IDs | Authority |
| --- | --- | --- |
| Creation and probe | `createStream`, `probeStream` | Global `camera.configure` |
| Camera details | `getStream` | `live.view`; sensitive fields additionally require configuration authority on that same camera |
| Asynchronous lifecycle | `deleteStream`, `refreshStream` | Camera `camera.configure`; 202 acknowledges scheduling, not completion |
| Manual recording | `getStreamRecording`, `controlStreamRecording` | Read uses `live.view`; start/stop uses `camera.configure` |
| Detection zones | `getStreamZones`, `saveStreamZones`, `deleteStreamZones` | Read uses `live.view`; replacement/deletion uses `camera.configure` |
| Retention | `getStreamRetention`, `setStreamRetention` | Camera `camera.configure` |
| Detection engines | `getStreamDetectionEngines`, `setStreamDetectionEngines` | Camera `camera.configure` |
| ICE configuration | `getIceServers` | `live.view` on at least one camera; configured TURN credentials intentionally reach the authorized client |

The canonical document includes request/response schemas, errors, relevant query
parameters and personalized visibility predicates. Generated native document,
TypeScript declarations and standalone validators come from that document.
The inventory marks all 15 as `stream-lifecycle` and all eight protocol routes as
explicit exclusions with linked [request/status/payload semantics](media-protocols.md).
No owned registration remains `later`.

## Reproduced defects and corrections

- Retention accepted `max_storage_mb=-1` and returned roughly
  `18446744073709552000` after unsigned conversion. It now rejects non-object
  requests, wrong types, fractional/nonfinite/out-of-range values before any
  field is saved. Zero and inherited retention remain valid.
- Zone replacement accepted empty identifiers and malformed optional fields.
  Validation now rejects lossy/duplicate identifiers, invalid typed flags,
  invalid colors/confidence and oversized strings before replacing any zone.
  The polygon's existing finite normalized coordinate checks are retained.
- Detection engines silently truncated a 64-byte key into a different 63-byte
  key and accepted saturated integer inputs. Strings and original double values
  are now checked before narrowing/copying. Invalid replacement preserves the
  complete prior engine list.
- Real queued DELETE and refresh callbacks were held while the owned database
  and runtime replaced the named camera. On the baseline both reached backend
  effects for the replacement; DELETE removed it. Tasks now capture the UUID
  returned by authorization and compare it at worker execution. Refresh checks
  again after waiting for the shared go2rtc lifecycle. PUT also checks its
  captured authorized UUID before applying its queued configuration. Deterministic
  callbacks verify no replacement database/runtime change or backend operation.
  These guards cover queued work and the explicit refresh wait; they are not a
  new atomic coordinator for every possible concurrent camera mutation.
- Owned HTTP 307 and 308 sources forwarded credential-bearing create and update
  bodies to a second owned origin before adapter hardening. Both new lifecycle
  and existing stream adapters now force `redirect: error` and
  `credentials: same-origin` after caller options. The same tests now observe
  one request at the source and zero at the destination.

## Independent evidence

`tests/contracts/mediaLifecycle.api.spec.ts` runs six scenarios against a newly
started native process, owned SQLite database and owned media server:

1. Invalid retention writes leave all stored fields unchanged; zero/inheritance
   updates return and persist their expected values.
2. Invalid zone replacement is atomic; valid polygon save/read/delete succeeds.
3. Engine replacement preserves the old list for malformed identifiers, numeric
   overflow/range errors and invalid paths.
4. Basic admin, cookie session, legacy API key, camera-scoped read/configuration
   tokens, viewer, empty-policy user and anonymous requests match an independent
   permission oracle. Discovery agrees with actual handlers. Denied writes do
   not create cameras, recordings, zones or configuration changes; camera B and
   global creation remain inaccessible to an A-only configuration token.
5. A real FFmpeg-generated H.264 source is probed through owned HTTP and yields
   the expected width/height/fps/codec. Denied probes never contact that source.
   Create, refresh, permanent delete completion and same-name recreation exercise
   actual native handlers; the replacement receives a different UUID.
6. Manual recording returns 202, progresses to a real manual recording ID,
   rejects a duplicate start, stops with 200, and leaves a finalized owned MP4.
   Independent `ffprobe -count_packets` sees H.264 video and nonzero packet count.

The real Chromium consumer test opens and dismisses an existing edit, then
creates a new camera. A malformed 201 acknowledgment leaves the filled form open,
shows a bounded error and does not render private response fields. Retrying
through the native server creates the new camera with POST and never changes
camera A. StreamsView, zone loading/editing, manual recording, ICE configuration
and live-player refresh now use the validated adapters. Mutation adapters do not
silently retry side effects.

The eight media protocol dispositions reuse the independent SDP/SRTP, WebSocket,
MSE/HLS mux/demux, proxy-session and revocation evidence in
[the S1 receipt](media-authority-evidence.md). They do not invent a JSON shape for
streaming frames or claim a functioning upstream reload handler.

## Validation

```sh
npm run contracts:generate
npm run contracts:lint
npm run contracts:check
npm run contracts:typecheck
cmake --build build/media-authority --target lightnvr \
  test_delete_stream_launch_failure test_put_stream_launch_failure -j4
ctest --test-dir build/media-authority --output-on-failure \
  -R '^test_(delete|put)_stream_launch_failure$'
LIGHTNVR_CONTRACT_BIN=build/media-authority/bin/lightnvr \
  npx playwright test -c tests/contracts/playwright.config.ts \
  --project=api mediaLifecycle.api.spec.ts
npm --prefix web run typecheck
npm --prefix web test -- --runInBand
npm --prefix web run build
LIGHTNVR_CONTRACT_BIN=build/media-authority/bin/lightnvr \
  npx playwright test -c tests/contracts/playwright.config.ts \
  --project=ui mediaLifecycle.ui.spec.ts
```

Native Debug uses ASan/UBSan. Both launch/lifecycle suites pass (7 DELETE/refresh
cases and 19 PUT cases); all six native API scenarios and the browser scenario
pass. Frontend typing and all **72 suites / 862 tests** pass. Canonical lint passes
with the four existing ambiguous-recording-path warnings; generation drift is
clean. The redirect tests use the shared `redirectHarness.ts` contributed by the
identity/evidence worker and integrated with this change by the orchestrator.

No physical cameras, operator databases, multicast network discovery or external
media services were used. Hardware/vendor interoperability and hosted merged-CI
results remain separate from these local results. Final inventory/projection
oracle reconciliation across the parallel contract contributions belongs to the
integration PR.
