# F11 evidence: investigations, detections, LPR and remaining media protocols

Branch: `codex/hardening-evidence-contracts`, based on F3 `ab12d410`.

## Accepted route inventory (23)

| Method | Native route | Contract operation |
| --- | --- | --- |
| GET | `/api/detection/results/#` | `getEvidenceDetections` |
| GET | `/api/detection/models` | `listEvidenceModels` |
| POST | `/api/lpr/search` | `searchEvidenceLpr` |
| POST | `/api/lpr/export` | `exportEvidenceLpr` |
| DELETE | `/api/lpr/reads/#` | `deleteEvidenceLpr` |
| GET | `/api/snapshots/#/#` | `getEvidenceSnapshot` |
| GET | `/api/recordings/files/check` | `checkEvidenceRecordingFile` |
| DELETE | `/api/recordings/files` | `deleteEvidenceRecordingFile` |
| GET | `/api/recordings/protected` | `countEvidenceProtected` |
| POST | `/api/recordings/sync` | `syncEvidenceRecordings` |
| GET | `/api/timeline/manifest` | `getEvidenceManifest` |
| GET | `/api/timeline/play` | `redirectEvidencePlayback` |
| POST | `/api/investigations/timeline` | `queryEvidenceTimeline` |
| POST | `/api/investigations/segment-at` | `resolveEvidenceSegment` |
| POST | `/api/investigations/search` | `searchEvidence` |
| POST | `/api/investigations/recordings/preview` | `previewEvidenceRecordings` |
| POST | `/api/investigations/thumbnail-samples` | `sampleEvidenceThumbnails` |
| GET | `/api/investigations/thumbnail/#/#` | `getEvidenceThumbnail` |
| GET, POST | `/api/investigation-bookmarks` | `listEvidenceBookmarks`, `createEvidenceBookmark` |
| GET, PUT, DELETE | `/api/investigation-bookmarks/#` | `getEvidenceBookmark`, `updateEvidenceBookmark`, `deleteEvidenceBookmark` |

Stream recording and detection-engine configuration routes belong to the separately implemented camera/media family. This inventory does not count those routes twice.

## Findings and changes

**Reproduced history disclosure:** a detection captured with camera B's UUID and historical name A appeared in an A-authorized viewer's `/api/detection/results/A` response. The regression initially received `historical-hidden-label`; it now excludes that label and includes camera A's history even when its recorded name was B. The new private SQLite query binds the authorized immutable UUID in the same SELECT as timestamps and detections. Only rows whose capture UUID is NULL/empty retain legacy name fallback. Native tests cover name reuse, both legacy forms, time limits, committed motion intervals, a busy writer, missing table errors and no partially returned results.

**Snapshot identity:** new MQTT snapshots are stored under the captured camera UUID instead of the sanitized display name. The endpoint resolves UUID namespaces as UUIDs and serves that literal namespace, with no fallback to a current or historical name directory. Real API tests cover exact JPEG bytes, missing UUID-namespace file despite an existing legacy file, rename, name reuse and denied camera access. Existing name-only snapshot URLs remain supported: those historical files have no recoverable capture UUID and continue to authorize the current named camera. This compatibility boundary is explicit; the change does not claim to backfill historical identity. Tests exercise the serving path with owned JPEG files; the changed MQTT producer passes an explicit `ENABLE_MQTT=1` compiler check (the isolated API binary disables MQTT), not claimed as a live camera integration test.

Canonical schemas represent native response fields, bookmark ownership/revisions, per-camera search/facets, JSON attachment exports, JPEG/ranges, M3U playlists and empty playback redirects. The `global_or_camera` discovery predicate represents the actual detection-model handler: global configurators work with zero cameras; a camera-scoped configurator also qualifies. Native and generator tests reject malformed metadata; independent real API projections test administrator/viewer/read-only LPR token classes and preserve same-camera conjunctions.

The closed `evidenceJSON` adapter validates requests and responses and classifies query POSTs separately from mutations. Failed writes never retry or claim success. Actual InvestigationView, InvestigationActions, InvestigationBookmarks, live event navigator, detection polling, recording detection overlays, timeline detection overlays and model selection consume guards. The existing LPR transport keeps its injectable fetch/status outcomes and validates canonical search/delete data and real JSON export attachment metadata before display/download.

Basic authentication, legacy keys and scoped-token owner intersections remain compatible. Bookmarks continue to be owned by user ID, including scoped tokens, while every saved camera needs current replay access. Bookmarks never grant permission or hold media against retention.

## Validation and reproduction

Run from this worktree with an isolated built server:

```sh
LIGHTNVR_CONTRACT_BIN="$PWD/build/identity-admin/native/bin/lightnvr" npx playwright test -c tests/contracts/playwright.config.ts --project=api evidence.api.spec.ts
LIGHTNVR_CONTRACT_BIN="$PWD/build/identity-admin/native/bin/lightnvr" npx playwright test -c tests/contracts/playwright.config.ts --project=ui evidence.ui.spec.ts
npm --prefix web test -- --runInBand tests/contracts/evidence.spec.ts tests/lprSearch.spec.js tests/detectionPolling.spec.js tests/investigationUtils.spec.js
npm run contracts:test-tools
npm run contracts:check
npm run contracts:typecheck
npm --prefix web run typecheck
npm --prefix web run build
```

- Eight real API cases use owned loopback servers, synthetic H.264/JPEG media and isolated databases. They check hidden captured identities/facets, mixed-camera rejection, actual bookmarks, revision conflict and denied durable no-effects, file checks/delete/sync, native thumbnails, playlist/redirect/MP4 ranges and encrypted LPR rows decrypted by the native server. The LPR key is random, explicitly fixture-only and never inherited from the operator's environment; ciphertext is independently generated with Node AES-GCM and bound camera/timestamp/source AAD. Export bytes are parsed and compared with search results; audit output must exclude the plate filter/text.
- Three real Chromium cases exercise built InvestigationView load/save/delete, malformed bookmark list/write acknowledgments, and malformed plate-search responses. Private malformed values never render and failed acknowledgments do not close the save dialog or claim success.
- Six focused Release and six ASan/UBSan suites passed: `test_api_read_isolation`, `test_openapi_projection`, `test_api_handlers_investigations`, `test_api_handlers_investigation_bookmarks`, `test_api_handlers_lpr`, `test_db_lpr_reads`.
- 108 web tests and 13 contract tooling tests passed. Canonical optimized guards matched unmodified Ajv across 13,834 cases over 138 schema exports.
- Type checks, changed-consumer ESLint, Vite production build and canonical lint passed. Lint retains the legitimate no-2xx warning for the playback operation whose successful response is 302.

Local evidence is under `build/evidence-contracts/`: `red.log` / `green.log` establish the capture regression; `final-api.log`, `ui.log`, `native.log`, `sanitize.log`, `webtests.log`, `tooling.log`, `typecheck.log`, `contracts-typecheck.log`, `eslint.log` and `webbuild.log` and `mqtt-compile.log` record validation. Transient fixture state and build output are not committed. The existing contracts CI automatically discovers the added API/browser files.
