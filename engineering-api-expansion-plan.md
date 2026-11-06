# Engineering API expansion: next bounded slices

Status: F0 checkpoint, F12 and F16 merged in PR #805; recording security
follow-up is under review in PR #807. F1 boundary review is in progress; its contract
expansion is not accepted. Other slices remain proposed.

Continue from [the executed hardening plan](engineering-api-contracts-plan.md)
and [the PRD](docs/prd/ENG_01_Safety_Contracts_TypeScript.md). Baseline:
`6ddf77764e1f5553a839ff4c19ad456c3a334480` on main (2026-11-06), after
PR #805 recording integrity and PR #806 minimatch remediation. TS/TSX conversion is complete. Automated Rust conversion,
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
F0 -> F9 -> F10
F0 -> S1 (audio/signaling authorization; before F10 acceptance)
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
- **status**: Current-wave checkpoint complete: 33 accepted operations / 30 paths,
  207 residual registered operations (240 total); no contract expansion in F12/F16.
  Full residual assignment and F4/F10/F11 decomposition remain required before
  those families execute. F1 is bounded in the linked plan below.

### F1: Identity self-service and MFA
- **depends_on**: [F0]
- **ownership**: Auth/TOTP handlers, password handling in `api_handlers_users_backend_agnostic.c` and tests; Login/AuthGate/TOTP modal and SettingsView self-service consumers.
- **description**: Sessions/trusted-devices list/revoke; login TOTP completion; user TOTP setup/verify/disable/status; password changes and legacy GET `/logout`. Distinguish legacy keys from scoped tokens.
- **validation**: Owner/other-user/admin/token matrix, challenge expiry/replay, actual revocation, secret exclusion and denied no-effects.
- **status**: F1a security preflight implemented for review: pending MFA tokens
  are distinct from sessions, and completion consumes a bounded challenge once.
  Full principal/lifecycle coverage and the 11-operation contract remain pending. See
  [identity contracts plan](engineering-identity-contracts-plan.md).

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
- **description**: Reproduce same-second filenames, restart reuse and wall-clock rollback. Use random UUID suffixes, exclusive creation by both MP4 writers, and attempt-driven rotation independent of clock movement. Preserve writer/file ownership and existing catalog finalization order.
- **validation**: Forced identical timestamps/rollback never alter existing or protected file hashes; new rows/files remain paired through crash/failure cleanup. Document close/publication/ownership before lifecycle changes.
- **status**: Merged in PR #805; validation receipt below. Recording file/path security
  is being qualified separately in PR #807.

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
- **description**: T24 source review found unchecked ZIP output writes/close and source-read failures. Existing tests prove explicit missing-source/member-count errors, successful ZIP bytes and owner binding; they do not establish failure atomicity for all filesystem faults. Copy from one source descriptor, derive CRC from copied bytes, detect size/metadata changes, check all ZIP writes/seeks/source and archive closes, and publish only after successful close.
- **validation**: Process-local fault injection into owned fixtures; failed archives never become complete or downloadable, temporary files are cleaned safely, source/protected hashes remain intact, and successful archives pass independent CRC/content/decode checks. Define completion versus durability explicitly; do not imply power-loss guarantees from successful buffered writes.
- **status**: Merged in PR #805; validation receipt below. Recording file/path security
  is being qualified separately in PR #807.


### S1: Audio and signaling authorization boundary
- **depends_on**: [F0]; independent of F9, required before F10 action-isolation acceptance.
- **ownership**: Go2rtc proxy/signaling and audio consumers; separate security PR.
- **description**: Inventory browser-direct go2rtc WebRTC/audio paths, then choose
  a LightNVR-mediated or narrowly scoped expiring authorization boundary using
  the existing evaluator. Specify listen versus talk permissions, stream identity,
  token scope, expiry/revocation and what deployment exposure remains possible.
- **validation**: Denied listen/talk cannot negotiate or receive media through
  alternate direct/proxy paths. Test owner/token/camera intersections and revoked
  access with an owned go2rtc fixture; browser hiding alone is not enforcement.
- **status**: Prioritized for a separate bounded plan; not implemented here.

## Recording integrity execution receipt (2026-11-06)

This wave leaves the accepted API inventory unchanged. F12 owns recording path,
exclusive AVIO file creation and segment retry/rotation. F16 owns the batch ZIP
worker and its isolated fault tests. Neither changes authentication, JSON schemas,
Rust scope or branch protection. The next API implementation is F1, followed by
F2 fleet reads; S1 can proceed as a separately reviewed security slice.

F12 keeps filename timestamp prefixes and adds random UUIDs. Path preparation
creates directories only; the writer atomically owns a new file with O_EXCL.
An existing file or symlink is refused, never deleted, reopened or truncated.
The owned descriptor stays attached to custom AVIO until flush/close. Failures
leave only the new attempt's possible partial file for existing reconciliation;
no cleanup is permitted to unlink an unowned collision. Continuous-recording rows are still created at first keyframe and finalized
before switching to the next pathname. Detection clips reserve and retain their
file descriptor before their early catalog row, then transfer it to the muxer
at first keyframe (or close it if the clip stops before initialization).
Every attempted segment rotates even if the wall clock goes backwards.

F16 publishes a temporary archive only when every member, header, directory,
seek and close succeeds. CRC and size describe bytes read from one descriptor;
size/mtime/ctime checks reject observed concurrent source changes. It is not a
filesystem snapshot. A successful close establishes a completed archive, not
fsync or power-loss durability. Failed jobs retain no downloadable path and
remove only their own mkstemp file. Existing job ownership checks remain in force.

Validation on the integrated branch:

- Release: all 85 selected ci-unit tests passed; updated audio/export fixtures
  also passed their focused reruns. Debug: all 60 ci-sanitizer tests passed with
  ASan/UBSan and leak detection. Both new suites are selected in both CI lanes.
- 55 real-server API contract tests passed with zero retries. Successful ZIPs
  were independently checked by Python zipfile and decoded with FFmpeg; missing
  source jobs returned error and result 409. Existing owner/token isolation passed.
- Native regressions cover concurrent exclusive opens, symlinks, same timestamps,
  another process, rollback, both real MP4 writers and faststart on/off, detection
  collision without a new catalog row, retry row/file pairing, every archive
  write position (zero/short writes), ENOSPC/EIO, source read/close/change/size
  failures, archive close/seek failures and empty-source CRC/size.
- Fresh managed Debug binary plus fork go2rtc passed the owned SIGKILL/restart
  scenario: immediate reconciliation, protected hashes/IDs, playback/decode,
  idempotent second restart and SQLite integrity. Receipt is retained under
  build/recording-integrity-recovery/result.json; native source manifest SHA-256
  is `664b12abfc60b822f6b52ad1f086599f8913a8ed85d97747911487c53cb1c90c`.
- Local Release gcov/LCOV intersection with changed executable C lines is
  137/147 (93.2%). This optimized local measurement is not a hosted Codecov result.
- Contract generation/drift, documentation links and diff whitespace checks pass.

Reproduce native checks with BUILD_TESTS enabled, SOD/LiteRT/MQTT disabled,
SSL enabled, and coherent system FFmpeg headers/libraries. Use Release coverage
flags --coverage and -fprofile-update=atomic; Debug enables sanitizers. Run
`ctest -L ci-unit --output-on-failure` and the Debug ci-sanitizer label. The
[recovery harness](tests/reliability/README.md) documents managed go2rtc setup;
this run uses the fork pin's freshly built binary. Logs and build trees are under
build/recording-integrity*. Hosted results belong to the PR checks, separately
from this local receipt. No hardware, long-duration soak or power-loss
qualification is claimed.


## Security follow-up and next acceptance (2026-11-06)

[PR #806](https://github.com/opensensor/lightNVR/pull/806) is merged. The web
lockfile resolves affected minimatch 7–10 dependencies to 11.0.5 through a
scoped override; the unaffected callable 3.x dependency remains compatible.
Clean installation, dependency-tree validation, web lint/typecheck, 784 Jest
cases and production build passed. This does not clear unrelated npm advisories.

[PR #807](https://github.com/opensensor/lightNVR/pull/807) addresses new recording
CodeQL alerts #346–#350: explicit owner-only creation permissions, fixed-alphabet
UUID encoding and rejection of path syntax before constructing either recording
filename. Native tests exercise umask zero, both entropy sources and invalid
components in both recording modes. All 86 ci-unit tests pass; the initial fix
passed all 61 sanitizer tests and managed crash/restart qualification, and the
added component guards pass focused sanitizer tests. C/C++ security-extended
CodeQL now runs on PRs as well as main. Acceptance must inspect a full branch
analysis for these existing alerts; a PR scan reporting zero new findings alone
is not evidence that main's findings are fixed. Hosted results are recorded on
the PR, without dismissals or suppression changes.

Resume F1 after this recording correction: first freeze and test pending-MFA
versus authenticated-session boundaries, then session/trusted-device lifecycle,
then the 11-operation canonical contract and actual UI consumers. Keep behavioral
security fixes separately reviewable from generated schema changes. F2 fleet
reads follows F1 acceptance; S1 audio/signaling authorization remains a separate
security slice. Rust remains deferred.
