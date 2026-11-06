# Engineering API expansion: next bounded slices

Status: proposed follow-on work; not authorized for execution by this file.

Continue from [the executed hardening plan](engineering-api-contracts-plan.md)
and [the PRD](docs/prd/ENG_01_Safety_Contracts_TypeScript.md). Baseline:
`dea6094a731c86debb03cc1c75d0251ab3b96cec` plus accepted working-tree changes
recorded there. TS/TSX conversion is complete. Automated Rust conversion,
synchronization with C and a second backend remain deferred.

## Ownership and acceptance

Use at most three domain workers beside the integration owner. Workers own
distinct handlers, domain tests and adapter modules. One integration owner
controls canonical OpenAPI, generated artifacts, coverage, projection metadata,
shared fixtures, CMake and package manifests. Workers propose bounded contract
fragments; integrate and qualify each slice before its dependents. Serialize
shared UsersView, SettingsView, StorageTab and StreamsView edits. Dependency
installation, Vite output and CMake's generated version.js are exclusive resources.

Every slice requires canonical/projected lint, generated drift, root/web typing,
fresh real-server principal tests, relevant native sanitizer checks and the
actual frontend consumer. Denied/invalid writes must leave durable state intact.
Existing unit tests provide fixtures, not automatic contract coverage. Use
owned loopback services and temporary data. Device changes require an identified
device and authorized scope. No branch-protection changes or broad C rewrite.

## Dependencies

```text
F0 -> F1 -> F3
   -> F2 -> F4 -> F5 -> F6
F1 -> F7 -------------> F8 (also F4)
   -> F9 -> F10
F2 -> F11
F0 -> F12
F0 -> F16
F0 -> F13 -> F14
F1..F14,F16 -> F15
```

### F0: Freeze remaining coverage
- **depends_on**: []
- **ownership**: Integration owner; route inventory, coverage and this plan.
- **description**: Start after T29 acceptance. Compare every literal registration, including multiline/conditional calls; assign every residual operation a slice below or explicit later disposition. Freeze credential classes, wire errors, feature flags and fixture limits. Before delegation, expand the broad F4/F10/F11 families into concrete task IDs, dependencies, exact files and bounded acceptance criteria.
- **validation**: No missing/duplicate/stale method-route entries; accepted counts agree with canonical and native tests.
- **status**: Not started

### F1: Identity self-service and MFA
- **depends_on**: [F0]
- **ownership**: Auth/TOTP handlers, password handling in `api_handlers_users_backend_agnostic.c` and tests; Login/AuthGate/TOTP modal and SettingsView self-service consumers.
- **description**: Sessions/trusted-devices list/revoke; login TOTP completion; user TOTP setup/verify/disable/status; password changes and legacy GET `/logout`. Distinguish legacy keys from scoped tokens.
- **validation**: Owner/other-user/admin/token matrix, challenge expiry/replay, actual revocation, secret exclusion and denied no-effects.
- **status**: Not started

### F2: Fleet reads and selectors
- **depends_on**: [F0]
- **ownership**: Fleet query/view/collection/location/tag read handlers and adapters.
- **description**: Fleet camera query, selector preview, views, locations, tags and collection list/detail/members; consistent loaded inventory and existing evaluator.
- **validation**: Rows/facets/counts, same-camera conjunctions, private/shared collections, empty/stale selectors and token intersections; include populated location/tag trees.
- **status**: Not started

### F3: Identity administration, policy and audit
- **depends_on**: [F1]
- **ownership**: User/authorization/audit handlers and tests; one UsersView owner.
- **description**: User CRUD, legacy-key issuance, password/login locks, actions/simulation, role CRUD, user-policy GET/PUT and audit events/export/settings.
- **validation**: Policy/revocation changes affect discovery and handlers; preserve self-lockout/revision behavior; no secret leaks or token privilege expansion; validate actual audit export bytes.
- **status**: Not started

### F4: Fleet organization and live workspace objects
- **depends_on**: [F2]
- **ownership**: Collections separate from location/tag handlers; saved views/layouts/plans/favorites as distinct sub-slices.
- **description**: Location CRUD/assignment, tag CRUD/merge/assignment, collection CRUD/membership/preview and saved views. Explicitly enumerate live layouts, floor plans and favorites.
- **validation**: Cross-owner/shared access, selector transitions, reference deletion, revision failures and unauthorized bulk no-effects; real uploaded image/content checks.
- **status**: Not started

### F5: Storage configuration and preview
- **depends_on**: [F4]
- **ownership**: Target/pool/policy handlers and tests; single StorageTab owner.
- **description**: Target CRUD/probe, pool CRUD, policy CRUD/preview, storage health/compliance. Use owned destinations for probes.
- **validation**: Revision conflicts, credential redaction, invalid destinations, protected references and scoped selectors. Separate local filesystem evidence from remote-provider qualification.
- **status**: Not started

### F6: Storage jobs and lifecycle actions
- **depends_on**: [F5]
- **ownership**: Migration/archive/cleanup handlers and tests; shared recovery harness coordinated centrally.
- **description**: Policy apply; migration list/create/detail/cancel/retry; archive list/retry; storage cleanup. Reuse recording mutation test techniques while documenting each storage handler's actual policy. Migration get/cancel/retry currently require global `storage.configure`, without recording-job owner/token matching; changing that requires a separate behavior decision.
- **validation**: Denials create no jobs/files; retry/cancel preserve protection and catalog/file consistency. Reuse publication/deletion-ledger tests and real injected full/read-only/unavailable storage faults; distinguish acknowledgement from completion.
- **status**: Not started

### F7: Settings, system and public bootstrap
- **depends_on**: [F1]
- **ownership**: Settings/system/workspace/health handlers; one SettingsView/SystemView owner.
- **description**: Settings GET/POST, go2rtc validation, workspace GET/PUT, system info/status/health/incidents/logs, effective-config/override-status. Separate restart/shutdown/log-clear/backup from reads; explicitly classify health, metrics, telemetry and setup/public routes.
- **validation**: Zero values, credentials, revisions/errors; isolated process-control acknowledgements/restarts and real backup contents. HTTP success alone does not establish completion.
- **status**: Not started

### F8: Event routing
- **depends_on**: [F4, F7]
- **ownership**: Event catalog/destination/route handlers, EventRoutingView and owned local sinks.
- **description**: Catalog, destination CRUD, route CRUD, preview and POST `/api/motion/trigger`, using accepted selector and health-setting shapes.
- **validation**: Secret redaction, stale revisions, private collection references, suppression/retry and denied no-delivery effects; observe actual local deliveries separately from mocks.
- **status**: Not started

### F9: Stream lifecycle and ONVIF discovery
- **depends_on**: [F0]
- **ownership**: Stream/ONVIF discovery handlers; sole StreamsView owner.
- **description**: Stream single read/create/test/delete/refresh/recording controls; zones GET/POST/DELETE, retention GET/PUT and detection-engines GET/PUT; seven ONVIF discovery/device operations. Preserve captured UUIDs, create-after-edit intent and pending-state guards.
- **validation**: Owned SOAP/discovery fixtures, invalid/denied configuration no-effects; multicast/vendor behavior remains unqualified until observed on identified hardware.
- **status**: Not started

### F10: Device control and streaming protocols
- **depends_on**: [F9]
- **ownership**: PTZ/imaging and HLS/go2rtc boundaries as separately reviewed sub-slices.
- **description**: PTZ capabilities/presets/movement, imaging/daynight, GET `/api/ice-servers`, HLS and direct signaling/proxy routes. Address the documented direct go2rtc audio authorization gap before claiming action isolation.
- **validation**: SOAP fixtures plus authorized device checks; media bytes, signaling permissions and reconnect behavior alongside JSON schemas.
- **status**: Not started

### F11: Investigations, detections and residual recording routes
- **depends_on**: [F2]
- **ownership**: Investigation/bookmark, detection/model/snapshot and LPR sub-slices; residual recording work coordinated with storage.
- **description**: Investigation timeline/segment-at/search/preview/thumbnails/bookmarks; detection results/models/snapshots; LPR search/export/delete; recording files/check/delete, protected-list/sync and timeline manifest/play excluded from the current UI contract slice.
- **validation**: Captured UUID/name-reuse/legacy fallback through every aggregate/enrichment route; hidden counts/facets remain hidden. Independently validate actual images, ZIP/media bytes and ranges.
- **status**: Not started

### F12: Recording pathname collision safety
- **depends_on**: [F0]
- **ownership**: Recording path creation/writer boundary and focused native/media tests.
- **description**: Reproduce same-second filenames, restart reuse and wall-clock rollback. Current second-resolution paths can reuse filenames; startup ID reconciliation does not establish file identity. Choose the smallest proven writer fix after reproducing overwrite.
- **validation**: Forced identical timestamps/rollback never alter existing or protected file hashes; new rows/files remain paired through crash/failure cleanup. Document close/publication/ownership before lifecycle changes.
- **status**: Not started

### F13: Target capacity and active-media measurements
- **depends_on**: [F0]
- **ownership**: Performance harnesses and named target evidence; optimization requires a separately reviewed measured need.
- **description**: Extend disabled-camera measurements to populated tags/locations/collections, realistic policies and active recording on representative hardware. Measure the remaining configured-max metrics buffer (about 11.9 MiB at 1,024 streams), snapshot/JSON memory, latency, descriptors and assets. T24 grows the modern validator chunk from 136,439 to 358,973 raw bytes while only Streams/explorer currently consume it; measure exclusion or splitting of unused guards before broadening coverage further.
- **validation**: Named hardware/binaries/flags, repeated serial/concurrent samples and controls. Derive budgets from target headroom; workstation numbers are not embedded capacity or production SLOs.
- **status**: Not started

### F14: Longer recovery, storage and upgrade qualification
- **depends_on**: [F13]
- **ownership**: Reliability harnesses/release evidence; device/network/storage fixtures separately owned.
- **description**: Extend bounded crash/reconnect/storage/upgrade runs to declared longer durations, network/object-store faults and selected hardware. Preserve exact prior-release/submodule provenance; upgrade a copied installation.
- **validation**: Decode/DTS/gaps, protected hashes, integrity, resources and cleanup; predeclare durations/tolerances and retain failures. Distinguish hosted observations from local checks.
- **status**: Not started

### F15: Reconcile coverage and the next decision
- **depends_on**: [F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F16]
- **ownership**: Integration owner; canonical, coverage, PRD and receipts.
- **description**: Accept slices individually, then reconcile all residual route/device/protocol limits and measured costs. Rust requires a separate explicit future decision.
- **validation**: Fresh integrated contract/native/frontend/recovery checks, exact drift/coverage and named builds; no unobserved hosted/device claims.
- **status**: Not started

### F16: Recording export I/O failure handling
- **depends_on**: [F0]
- **ownership**: ZIP writer in `src/web/api_handlers_recordings_batch_download.c` and isolated failure regressions; serialize against other recording job edits.
- **description**: T24 source review found unchecked ZIP output writes/close and source-read failures. Existing tests prove explicit missing-source/member-count errors, successful ZIP bytes and owner binding; they do not establish failure atomicity for all filesystem faults. Reproduce output ENOSPC/EIO, close failure and source truncation/read races before choosing the smallest repair.
- **validation**: Process-local fault injection into owned fixtures; failed archives never become complete or downloadable, temporary files are cleaned safely, source/protected hashes remain intact, and successful archives pass independent CRC/content/decode checks. Define completion versus durability explicitly; do not imply power-loss guarantees from successful buffered writes.
- **status**: Not started
