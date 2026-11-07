# Four-PR hardening execution

Requested 2026-11-07 using parallel-task. Baseline main `84ed0174`.
This executes items 1–4 of the remaining-work review, not encryption, Rust,
capacity benchmarking or physical-device qualification. Basic/key compatibility,
HTTP-only cameras and `/etc/lightnvr` defaults remain supported. No merges.

## Integration rules

Use isolated worktrees and at most three workers plus the orchestrator. Commit
only owned changes. Workers never push; the orchestrator reviews, integrates,
pushes and opens four parent-repository PRs. A maintained-go2rtc fork change may
require a supporting fork PR/pin. Shared canonical/generated changes are developed
in isolation and reconciled serially; regenerate rather than concatenate outputs.
PR 1 is independent. PRs 2–4 may be stacked to preserve canonical-schema history.
No operation is accepted merely by adding permissive schemas: capture real native
behavior, principal denial/no-effects, concrete protocol bytes and UI consumption.
Keep unsupported hardware/vendor/remote-provider evidence explicitly unqualified.
Tool sandbox policy is full access/approval never: use exec_command as provided;
do not pass unsupported skill-template sandbox parameters.

### P1: Audio and established-media authority (S1)
- **depends_on**: []
- **ownership**: media-authority worktree; go2rtc fork/proxy, native HLS, audio consumers, real-media fixtures and relevant authorization metadata.
- **description**: Complete S1 after merged S1a. Reproduce audio negotiation by live-only/listen/talk/combined principals and scoped tokens. Enforce actual directions for HTTP/WebSocket WebRTC, MSE and HLS; bind camera/substream identity and established peers to bounded authority, including expiry/revocation. Close alternate paths using the parser/fork rather than untrusted browser flags. Preserve video-only and authorized audio playback.
- **acceptance_criteria**: Real owned media proves denied receive/transmit and no alternate negotiation bypass; established peers cease access within a documented bound after session/token/policy revocation. No unenforced action is advertised as enforced without all-path evidence. Record external-go2rtc deployment limits.
- **validation**: RED behavioral evidence followed by GREEN media/protocol tests, relevant native sanitizer suites, fork Go tests/race checks if changed, and browser compatibility tests.
- **status**: In progress

### P2: Identity administration, policy and audit contracts (F3)
- **depends_on**: []
- **ownership**: identity-admin worktree; user/authorization/audit handlers, UsersView and adapters, isolated canonical/generated artifacts and domain fixtures.
- **description**: Cover the 20 residual user CRUD/key/password-lock/login-lockout, action/simulation/role/user-policy and audit list/export/settings operations. Read concrete handler semantics; validate response/request variants, token restrictions, revisions/self-lockout and actual audit export bytes. Integrate generated contracts into real consumers.
- **acceptance_criteria**: Effective policy changes/revocations affect discovery and requests, denied mutations have no durable effects, credentials only appear in explicitly authorized issuance responses, no token privilege expansion. Canonical and projected specs agree with independent native responses.
- **validation**: RED -> GREEN domain/API/browser regressions; lint, generation/drift, root/web typing, unit and sanitizer checks. Record exact accepted route inventory and fixture limits.
- **status**: Complete locally — `ab12d410`; publication pending.
- **log**: 20 operations; scoped-token legacy-key escalation blocked; actual user password-lock form fixed; API/browser, native optimized/sanitizer and generated contract validation pass. See identity-admin evidence receipt.

### P3: Fleet organization and live workspace contracts (F4)
- **depends_on**: []
- **ownership**: fleet-writes worktree; locations/tags, collections, saved views, layouts, floor plans/backgrounds, favorites and corresponding UI consumers; isolated canonical/generated artifacts.
- **description**: Complete all residual fleet organization and live workspace registrations. Subtasks: P3a location/tag CRUD/merge/assignment; P3b collection CRUD/membership and saved-view mutations; P3c live layouts/favorites; P3d floor-plan CRUD and binary backgrounds. Reuse accepted F2 reads.
- **acceptance_criteria**: Cross-owner/shared access, scoped counts, selector transitions, deleted references, revisions and denied bulk no-effects are proven. Validate actual uploaded/downloaded image bytes and cleanup, not JSON acknowledgments alone. Real consumers validate responses and preserve error/retry behavior.
- **validation**: RED -> GREEN domain/API/browser tests, canonical/projected lint and drift, frontend typing/tests, relevant native sanitizers.
- **status**: Complete locally — `b96db492`; publication pending.
- **log**: 31 operations; exact PNG/JPEG protocol bytes; owner/revision/principal API and browser checks; 864 web tests; native optimized and sanitizer projection validation. See `docs/api/contracts/fleet-writes-evidence.md`.

### P4a: Settings, system and bootstrap contracts (F7)
- **depends_on**: []
- **ownership**: remaining-api worktree; settings/system/workspace/health/telemetry/setup contracts, adapters and fixtures.
- **description**: Enumerate every corresponding residual route. Separate reads from backup/process-control/log-clear writes, public/bootstrap behavior from authenticated policy, and acknowledgement from completion. Exercise process control in isolated child processes.
- **acceptance_criteria**: Preserve zeros/credentials/revisions and deployment feature variants; validate backup bytes, process outcomes and consumer failure handling. No tests affect an operator instance.
- **validation**: Real-server principal matrix, owned backup/process fixtures, native regressions and schema/consumer checks.
- **status**: Complete locally; publication pending.
- **log**: 23 operations; actual restart/re-exec and shutdown, isolated backup bytes, 23-operation credential matrix, settings/system browser rejection and recovery, snapshot-bound bootstrap discovery, optimized+ASan snapshot/projection tests. See `docs/api/contracts/system-evidence.md`.

### P4b: Stream lifecycle, ONVIF and device/protocol contracts (F9/F10)
- **depends_on**: [P1]
- **ownership**: remaining-api worktree; stream/zones/retention/detection-engine/ONVIF/PTZ/imaging/daynight/ICE and media protocol surfaces, StreamsView.
- **description**: Enumerate all residual stream/device/media registrations. Use owned SOAP/media fixtures for create/test/delete/refresh/recording controls, discovery/device operations, PTZ/imaging/daynight and protocol response variants. Preserve captured identities and create/edit intent.
- **acceptance_criteria**: Invalid/denied requests have no configuration or device effects; actual protocol bodies/ranges/signaling align with the contract. Physical multicast/vendor interoperability remains separately qualified.
- **validation**: Independent SOAP/media bytes, real-server negative mutations, typed consumer tests, native/sanitizer checks.
- **status**: In progress

### P4c: Storage configuration and lifecycle contracts (F5/F6)
- **depends_on**: [P3]
- **ownership**: remaining-api worktree; storage targets/pools/policies/preview/health/compliance, migration/archive/cleanup and StorageTab.
- **description**: Cover every residual storage registration using owned destinations. Preserve current global storage.configure policy for migration operations; do not invent job-owner/cancellation semantics. Test asynchronous completion independently of acknowledgements.
- **acceptance_criteria**: Credential redaction, revisions, reference integrity, protected media, denial no-job/no-file effects and retry/cancel catalog consistency. Remote-provider qualification remains explicit.
- **validation**: Real-server schemas/principals, owned local storage fault/recovery fixtures, UI adapters, native sanitizers.
- **status**: Complete locally — `fe32298a`; integration/publication pending.
- **log**: 28 operations; cleanup rejects malformed/trailing JSON before scheduling; owned target/policy/migration/archive bytes and lifecycle, browser and sanitizer checks pass. See storage evidence receipt.

### P4d: Event routing contracts (F8)
- **depends_on**: [P3, P4a]
- **ownership**: remaining-api worktree; event catalog/destinations/routes/preview/motion trigger, EventRoutingView and local sinks.
- **description**: Cover all residual event operations. Reuse selector and settings semantics; observe actual delivery with an owned local sink.
- **acceptance_criteria**: Secret redaction, stale revisions/private collection references, suppression/retry and denied no-delivery behavior; validate actual consumer boundaries.
- **validation**: Real delivery and negative-effect fixtures, schema projection, domain/native/browser tests.
- **status**: Complete locally; publication pending.
- **log**: 13 operations; real MQTT bytes/PUBACK retry/cooldown; native no-effects and browser malformed-ack/revision-preview cases; 12 optimized +3 sanitizer suites. See events evidence receipt.

### P4e: Investigations and residual recordings/detections (F11)
- **depends_on**: []
- **ownership**: remaining-api worktree; investigation/search/timeline/bookmarks, detection/model/snapshot, LPR, recording file/protected/sync/timeline consumers and fixtures.
- **description**: Enumerate each residual operation; preserve captured UUID and legacy fallback across aggregation/enrichment. Validate concrete media/image/export bytes and ranges beside JSON.
- **acceptance_criteria**: Hidden rows/counts/facets remain hidden, name reuse cannot widen authority, invalid/denied writes have no durable effects, file and streaming responses are independently verified.
- **validation**: Real-server principal/no-effects tests, media/image/export fixtures, typed consumers, native sanitizers.
- **status**: In progress

### P4f: Final integration and four-PR publication
- **depends_on**: [P1, P2, P3, P4a, P4b, P4c, P4d, P4e]
- **ownership**: Orchestrator; shared canonical/generated files, route disposition, CI selection, plan/PRD receipts and PR metadata.
- **description**: Reconcile the current registered operations (240 at baseline, plus any new internal registrations), including explicit protocol/unsupported dispositions instead of blanket coverage. Integrate shared changes, run fresh full validation, and publish four reviewable PRs with base relationships and evidence. Reconcile stale planning headers.
- **acceptance_criteria**: Four workstreams published separately, no missing/duplicate/stale inventory entries, drift and full suites pass, evidence distinguishes local/hosted/device limits, no PRs merged.
- **validation**: Canonical/projected lint, generation/drift, native optimized/sanitizer, frontend/root/contract typing, API/browser/asset checks and relevant recovery/media suites. Inspect hosted failures and correct reproducible issues.
- **status**: Pending
