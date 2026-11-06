# Engineering API expansion: next bounded slices

Status: F0 checkpoint, F12 and F16 merged in PR #805; recording security
follow-up merged in PR #807. F1 boundary correction merged in PR #808. The four-operation session/device
contract and Settings slice merged in PR #809; the other seven F1
operations remain pending. Other slices remain proposed.

Continue from [the executed hardening plan](engineering-api-contracts-plan.md)
and [the PRD](docs/prd/ENG_01_Safety_Contracts_TypeScript.md). Baseline:
`7aa0968045cc427e1a1eb018290c1706a83873ef` on main (2026-11-06), after
PR #805 recording integrity, PR #806 minimatch remediation, PR #807 recording security
PR #808 MFA/session separation and PR #809 session/device contracts. TS/TSX conversion is complete. Automated Rust conversion,
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
- **status**: Original-wave checkpoint complete: 33 accepted operations / 30 paths,
  207 residual registered operations (240 total); no contract expansion in F12/F16.
  The F1b follow-up raises this to 37 operations / 34 paths and 203 residual.
  Full residual assignment and F4/F10/F11 decomposition remain required before
  those families execute. F1 is bounded in the linked plan below.

### F1: Identity self-service and MFA
- **depends_on**: [F0]
- **ownership**: Auth/TOTP handlers, password handling in `api_handlers_users_backend_agnostic.c` and tests; Login/AuthGate/TOTP modal and SettingsView self-service consumers.
- **description**: Sessions/trusted-devices list/revoke; login TOTP completion; user TOTP setup/verify/disable/status; password changes and legacy GET `/logout`. Distinguish legacy keys from scoped tokens.
- **validation**: Owner/other-user/admin/token matrix, challenge expiry/replay, actual revocation, secret exclusion and denied no-effects.
- **status**: Complete and merged: #808 preflight, #809 session/device lifecycle,
  #816 password/TOTP behavior, #819 remaining identity contracts and browser
  consumers. Generated-validator quality cleanup merged in #818. F1 acceptance
  reached 44 operations / 41 paths, with 196 residual routes. See the
  [identity contracts plan](engineering-identity-contracts-plan.md).

### F2: Fleet reads and selectors
- **depends_on**: [F0]
- **ownership**: Fleet query/view/collection/location/tag read handlers and adapters.
- **description**: Fleet camera query, selector preview, views, locations, tags and collection list/detail/members; consistent loaded inventory and existing evaluator.
- **validation**: Rows/facets/counts, same-camera conjunctions, private/shared collections, empty/stale selectors and token intersections; include populated location/tag trees.
- **status**: Implemented for review: thirteen reads/queries, generated types and
  runtime guards, validated consumers and scoped disclosure regressions. Contract
  coverage is now 57 operations / 54 paths, with 183 residual routes. Includes
  fixes for camera-tag count disclosure and incomplete saved-view iteration.


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
  follow-up merged separately in PR #807.

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
  follow-up merged separately in PR #807.


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

[PR #807](https://github.com/opensensor/lightNVR/pull/807) is merged and addresses recording
CodeQL alerts #346–#350: explicit owner-only creation permissions, fixed-alphabet
UUID encoding and rejection of path syntax before constructing either recording
filename. Native tests exercise umask zero, both entropy sources and invalid
components in both recording modes. All 86 ci-unit tests pass; the initial fix
passed all 61 sanitizer tests and managed crash/restart qualification, and the
added component guards pass focused sanitizer tests. C/C++ security-extended
CodeQL now runs on PRs as well as main. Acceptance must inspect a full branch
analysis for these existing alerts; a PR scan reporting zero new findings alone
is not evidence that main's findings are fixed. The full branch [analysis](https://github.com/opensensor/lightNVR/actions/runs/42231014851)
confirmed all five findings absent, returning to the 22-finding prior baseline,
without dismissals or suppression changes. Main is being rescanned after merge.

F1 continues in [PR #808](https://github.com/opensensor/lightNVR/pull/808), now
based on main: first freeze and test pending-MFA versus authenticated-session
boundaries, then session/trusted-device lifecycle,
then the 11-operation canonical contract and actual UI consumers. Keep behavioral
security fixes separately reviewable from generated schema changes. F2 fleet
reads follows F1 acceptance; S1 audio/signaling authorization remains a separate
security slice. Rust remains deferred.


## Post-merge report checkpoint (2026-11-06)

PR #809 merged at `7aa09680`. All 18 PR checks, including real-server contracts,
three browser shards, sanitizers and hosted patch coverage, passed. Green checks
mean the jobs completed; they do not mean the report dashboards are empty.

| Report | Observed state | Disposition |
|---|---|---|
| Code Quality standard findings | 740 open, all in `web/generated/api/validators.js`: 453 constant conditions, 243 unused assignments, 37 defensive checks, 7 unreachable statements | Ajv-generated control flow; retain generator ownership and deterministic drift checks. Do not hand-edit generated output or equate these maintenance findings with 740 security vulnerabilities. |
| Code scanning | 18 open C/C++ security alerts, all predating #809 | Keep open pending the transport/storage review below. Main analysis `2113689691` contains 22 results, matching the previous full-branch scan by rule, file and diagnostic; result count includes more than the open-alert count. |
| Dependabot | No open alerts | No dependency change required by this checkpoint. |
| Secret scanning | Repository API reports this feature disabled | No secret-scanning clean-state claim; configuration unchanged. |
| AI findings | Existing [PR #810](https://github.com/opensensor/lightNVR/pull/810) exposes an upgrade-harness ancestry bug | Reproduced and regression-tested below. The complete AI dashboard was unavailable through the connected browser; this is not a claim that it contains only one finding. |

The prior fail-fast generator change did **not** eliminate the generated quality
backlog. CodeQL CLI 2.27.4 / JavaScript queries 2.5.0 (matching hosted analysis)
measured a compiler-only experiment at 730 results; changing reference sharing
and required-field generation still left 456. Those experiments were discarded:
changing runtime generation solely to improve a maintenance score does not fix
an application defect. No analysis exclusions, alert dismissals, query weakening,
or experimental dependency/artifact changes are included. GitHub's
[interpretation guidance](https://docs.github.com/en/code-security/how-tos/maintain-quality-code/interpret-results)
explicitly distinguishes generated-code maintenance results from source
maintainability. F13 remains the place for measured validator-size/performance work.

PR #810 checks ancestry in the current repository, which contains the execution
HEAD, instead of a reusable historical clone that may not contain that object.
Two real-Git regressions exercise `prepare()`: a valid reused clone succeeds and
records the correct provenance; an unrelated execution HEAD fails and emits no
success manifest. Build/tool probing is mocked; Git ancestry and manifest output
are real. The valid-reuse test fails against the old line. All eight qualification
cleanup/preparation tests pass locally, and the existing contract CI job runs them.
This is a harness correction, not a new end-to-end upgrade qualification claim.

Prioritize the remaining reports as bounded security work before broadening
fleet contracts:

1. **Transport evidence:** review #14/#264 (RTSP probes), #270/#271/#272 and
   #334–#337 (ONVIF), and #345 (loopback health). Trace actual destinations,
   credential-bearing bytes, redirects and TLS behavior; distinguish XML namespace
   URLs from transport URLs. Add protocol/credential-leak regressions. Preserve
   explicit compatibility with HTTP-only cameras and the health listener's actual
   transport. Dismissal requires evidence per finding; none is dismissed here.
2. **Credentials at rest:** review #64/#63/#267 (configuration) and
   #181/#183/#185/#307/#316 (stream/event-destination SQLite records). Document
   required recoverable secrets, file/backup permissions, key custody, migration,
   rotation and recovery before choosing encryption or a secret-store boundary.
   Hashing outbound camera/destination passwords cannot replace recoverable storage.
3. Continue F1c's MFA/password boundary and replay/throttling decisions, then the
   remaining seven identity contracts. Fleet reads follow that acceptance. Rust
   remains deferred.


## Credential transport and storage: compatibility-first pass

Baseline: main `2b2d8006` after #810. The user supports encryption but does not
want a disruptive rollout. This pass tightens permissions and transport boundaries;
it introduces no required key, encrypted-row migration, or HTTP-only camera ban.

### Implemented behavior

- Both native RTSP preflight callers use one bounded socket helper. It parses only
  the host/port and sends constant `OPTIONS *` bytes. URL userinfo, private paths,
  queries and fragments are never sent by this probe. An RTSP 401/404/405 confirms
  server availability; FFmpeg still performs resource/authentication validation.
  This replaces the previous unreliable interpretation of an OPTIONS 404 as a
  missing stream. The socket exchange has a two-second budget after DNS lookup;
  system hostname resolution itself is not deadline-bounded by this helper.
- ONVIF detection accepts HTTP(S) endpoints without userinfo/control characters,
  restricts curl protocols, explicitly disables redirects, and preserves TLS
  peer/hostname checks. HTTP compatibility remains. SOAP credentials must never
  follow a 307 to another destination. Trace output records header byte counts,
  not raw headers, request targets or curl notes. Advertised service/subscription
  endpoints are omitted from logs; shared URL logging drops query/fragment data.
  This is not a same-origin policy for camera-advertised endpoints or a complete
  TLS-only transport policy; those need camera compatibility fixtures.
- Config and generated go2rtc YAML rewrites check regular-file identity and a
  single link, establish 0600, then truncate/write. Permission failure stops the
  write and preserves old contents. Symlink/special-file targets are rejected;
  operators must configure a regular file rather than an alias for managed writes.
- Writable SQLite startup tightens the database and existing WAL/SHM/journal files
  to 0600 before opening connections. New databases and backup staging files are
  created privately rather than relying on the process umask. Backup-cycle
  directories are 0700. Read-only startup leaves permissions/content unchanged.
  Existing exported backups, externally managed read-only configs and filesystem
  snapshots are not rewritten or scrubbed; operators retain responsibility for
  their access controls. Services reading managed files must use the service
  owner or an explicit privileged export, rather than group/world readability.
- Plaintext credentials remain recoverable by the running service. Owner-only
  permissions do not encrypt disks, backups or snapshots and do not resolve the
  eight cleartext-storage alerts. No alert dismissal or query exclusion is part
  of this change. The loopback health URL matches the actual local listener and
  carries no credentials; HTTP namespaces in SOAP must not be replaced with HTTPS.

Regression coverage captures actual RTSP bytes and a real ONVIF 307 exchange,
checks forbidden endpoint schemes/userinfo and trace secrecy, exercises fresh and
legacy SQLite/WAL modes under umask 000, verifies read-only initialization and
backup source preservation, and injects go2rtc chmod failure. Existing full
native, sanitizer and contract suites remain the acceptance gates. A full branch
CodeQL result, not just an empty PR diff, determines which alerts are resolved.

### Opt-in encryption migration design (not enabled in this pass)

1. **Inventory and format.** Cover stream main/substream/detection/admin/publish
   URLs, ONVIF passwords, source overrides, event-destination credentials, global
   MQTT/TURN settings and DB-backed go2rtc override YAML. Treat whole URL/override
   values as secret-bearing; extracting only a password misses query tokens and
   embedded provider credentials. Trace every read/write/backup/config-generation
   path before declaring the inventory complete. Use explicit version/key metadata
   so a legacy plaintext password cannot be mistaken for an encrypted envelope.
2. **Operator-controlled key custody.** Require an explicit opt-in and an external,
   owner-readable key file or secret mount, separate from database/config backups.
   Do not silently generate a required key or reuse the LPR key. Persist only key
   identifiers with ciphertext. Document provisioning, container/service access,
   recovery and a successful restore rehearsal before activating encryption.
3. **Authenticated encryption.** Reuse maintained mbedTLS primitives with a reviewed
   versioned AEAD format, checked random nonces, and associated data binding the
   stable camera/destination identity, field and format version. Decrypt before
   application length checks; never truncate an envelope into existing fixed-size
   cleartext buffers. Wipe transient key/secret buffers where practical. Public
   DTOs and logs continue to exclude secrets; ciphertext is not a public credential.
4. **Migration and recovery.** Quiesce writers and migrate transactionally through
   an explicit tool/action. Preserve a verified recovery path; interruption must
   leave either the old or the encrypted version usable. Define WAL, journal,
   freelist and temporary-file handling. Old backups/snapshots can retain plaintext;
   do not silently delete them or claim forensic erasure. Missing/wrong keys or
   failed authentication must fail closed for secret consumers, without replacing
   ciphertext with empty/plaintext values or restarting an uncontrolled camera loop.
5. **Rotation and rollback.** Support identified current/previous keys during a
   resumable re-encryption operation. Backup manifests identify required key IDs
   without embedding key material. Restore tests cover absent/wrong keys, older
   plaintext installs, mixed migration interruption, and key rotation. Returning
   to plaintext requires an explicit authenticated operator action and working key;
   never silently downgrade on an error. The default unencrypted installation
   remains supported until a separately reviewed deprecation policy exists.

Gate the encryption implementation on fixtures for all consumers, including the
read-only go2rtc config generator and child YAML files, plus a documented recovery
exercise. Continue the remaining F1c MFA/password work after this bounded pass;
Rust remains deferred.

## Generated validator quality follow-up (2026-11-06)

Following merged PRs #815–#817, the requested standard-findings cleanup fixes
runtime generation itself. The earlier compiler-only experiments above remain
historical evidence; the implemented approach changes the repeated emission
patterns that caused the reports. Required fields no longer assign property names
inside conditions, validity temporaries are independent per property, and error
accumulators no longer need null-allocation guards. A pinned conservative optimizer
removes dead temporaries and unreachable statements. There are no query exclusions,
alert dismissals or manual edits to generated validators.

Full local CodeQL 2.27.4 / JavaScript queries 2.5.0 analysis of all 463 tracked
JS/TS/JSON source inputs reports zero standard quality findings. The previous
reported groups were 453 useless conditionals, 243 useless assignments, 37
unneeded defensive checks and seven unreachable statements. Hosted score/closure
is still a separate result after the new head is analyzed.

The unchanged Ajv compiler serves as an independent oracle for all 50 exported
guards: 5,109 comparisons (630 valid / 4,479 invalid) preserve acceptance, exact
sanitized diagnostics and input data. Eight generator/tooling tests and all 805
frontend tests pass. All 79 API cases, 16 browser cases and the production
asset-isolation check pass against the rebuilt native application and stable
production web assets. The final browser validator chunk is about 319.87 kB raw /
34.07 kB gzip versus the previous 328,181 raw bytes / 36.78 kB gzip. This is a
workstation asset measurement, not embedded CPU/memory qualification.

The go2rtc dependency follow-up is merged: fork PR #3 / lightNVR PR #817 pins
x/net v0.62.0 and the patched crypto revision. Module checks, the unchanged upstream
test-failure comparison, govulncheck and Trivy binary scans pass. A full production
container rebuild also reports zero OS or go2rtc vulnerabilities in Trivy 0.76.2.
GitHub alert state depends on the new main image scan.

Next work remains F1d's seven identity contracts and typed consumers, F1e
acceptance, then F2 fleet reads and the separately reviewed S1 signaling boundary.
Basic/key compatibility is preserved; Rust and encryption remain deferred.


## F1d identity contract implementation (2026-11-06)

The remaining seven identity operations now have canonical schemas, snapshot
projection, generated types/guards and validated Login/AuthGate/MFA consumers.
Self-service and scoped management remain distinct, and password-reset authority
is pruned from non-manager discovery. See the
[identity qualification receipt](engineering-identity-contracts-plan.md) for
native, sanitizer, browser, semantic-equivalence, coverage and asset evidence.
All 467 tracked JS/TS/JSON inputs retain zero standard quality findings in the
full local CodeQL suite. GitHub now marks both go2rtc alerts #351/#352 fixed.

F2 fleet reads and selectors follows this review; S1 signaling/audio authorization
will be delivered separately. No credential-encryption migration or Rust work is
included, and Basic/legacy-key compatibility is retained.


## S1 next implementation boundary (2026-11-06)

Inspection after #818/#819 changes the starting assumption: WebRTC and MSE now
use the same-origin `/go2rtc` proxy (`settings-utils.ts`), and generated go2rtc
HTTP configuration binds to loopback. The remaining gap is authorization within
that transport. `go2rtc_proxy_authorize_request` checks `live.view` for WebRTC,
WebSocket and HLS; the WebSocket relay forwards frames after its initial check.
Neither fact establishes separate `audio.listen` / `audio.talk` enforcement.
The generated RTSP listener still binds all interfaces, and administrator config
overrides can change exposure. These are explicit deployment boundaries to resolve.

Keep S1 separate from fleet schema changes and deliver it in this dependency order:

1. Build an owned audio/video source and go2rtc fixture. Reproduce listen/talk
   negotiation with live-only, listen-only, talk-only and combined authority,
   including scoped tokens, substreams and both HTTP/WebSocket paths. Inventory
   native HLS and direct RTSP as alternative media paths before choosing enforcement.
2. Enforce media directions at negotiation with the existing camera evaluator.
   Use the actual SDP/media parser or a constrained transport protocol; do not
   trust browser flags or query parameters alone. Unsupported alternate message
   types must not bypass the checked camera or requested audio direction.
3. Bound established-connection authority. Test session/token expiry, revocation
   and lost owner permissions against an already playing peer. A handshake-only
   check is insufficient; document and test any bounded revocation delay.
4. Resolve direct listener/config-override exposure and preserve normal video-only
   playback plus authorized audio. Keep Basic and legacy-key authentication.
   If enforcement needs the maintained fork, review its change and parent pin
   separately, with real media and reconnect evidence before claiming isolation.

S1 is not yet implemented or qualified. Keep the authorization action catalog's
unenforced status until the bypass and established-session tests pass. Fleet
organization writes (F4) and identity administration/audit (F3) remain the next
contract families after this security boundary.


## F2 local qualification receipt (2026-11-06)

Thirteen read/query operations, fifteen additional exported runtime guards and
validated consumers are implemented for review. Native regressions cover the
saved-view SQLite loop; scoped HTTP tests cover hidden-camera tag totals on GET
and PUT. All 93 native ci-unit tests and 69 ASan/UBSan ci-sanitizer tests pass.
Changed executable C lines are covered 13/13 (3 saved-view and 10 tag-handler
lines); this is line coverage, not exhaustive branch/fault coverage.

All 852 frontend tests pass. Ten generator/tooling tests pass, including 7,696
independent comparisons across 77 guards against unmodified Ajv. Type checks,
lint, generated drift checks, documentation links and production asset isolation
pass. The full local CodeQL quality suite reports zero findings across 471 tracked
JS/TS/JSON inputs. Default validator assets are 496,372 raw bytes / 52,937 gzip
bytes, up from F1's approximately 355.83 / 39.95 kB. Target hardware resource
qualification remains open; these are workstation measurements.

All 86 real-server API cases pass. The browser suite's existing 21 cases and
both new fleet cases pass. The fleet
cases reject malformed rows/catalogs, restore real data on Retry and keep hidden
camera searches empty. An initial single-letter search assertion was corrected
to use the hidden camera UUID because search also matches UUIDs and addresses.

Merged-main Integration Tests exposed a separate stale mock in the legacy login
UI suite: its three-field configuration was rejected by the now-complete guard.
The fixture now preserves the real native response and overrides only its MFA
scenario fields. The failure was reproduced locally before the correction; all
three login cases pass afterward. Runtime validation is unchanged.
