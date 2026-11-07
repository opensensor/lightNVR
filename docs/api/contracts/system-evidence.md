# Settings, system, workspace and bootstrap contract evidence (F7)

Baseline `84ed0174`. This slice accepts 23 registrations; it does not by itself
complete the remaining contract inventory. The canonical document is the source
for native discovery, TypeScript declarations and runtime guards.

## Accepted operations

- Settings read/update and go2rtc override validation.
- System information (both current and legacy paths), status, host health,
  incident history, logs, log clearing, backup, restart and shutdown.
- Effective go2rtc configuration and override diagnostics.
- Workspace preference read/update and setup status read/update.
- Public health (full, process-liveness and history variants), HLS health,
  administrator-only Prometheus metrics and best-effort player telemetry.

Settings have required native fields, retain numeric zero and explicitly mask
MQTT/TURN passwords. Updates are partial objects. YAML validation uses the actual
`override` field and its 65535-byte native cap; setup writes acknowledge only
`success`, not an invented resulting state. Log queries use `level`, `count` and
`last_ts`. Incident cursors remain opaque. Metrics are text/plain and telemetry
returns empty 204; neither is mislabeled as JSON.

Setup mutation discovery uses a dedicated `bootstrap_setup` predicate. It reads
setup state on the authorization snapshot's existing SQLite connection, allows
bootstrap while incomplete, and requires effective global `system.admin` once
complete. Read failures fail projection closed. Snapshot tests prove that a
concurrent setup-state write cannot change an already-captured policy view.
Workspace writes retain non-scoped owner semantics; visibility preferences never
change camera permissions.

## Real consumer changes

The new system adapter validates outbound structured writes and actual response
shapes, preserves cancellation and caller headers, sanitizes errors, and forces
zero retries for writes. Invalid acknowledgments explicitly indicate uncertainty
and advise refreshing state before retrying.

Actual consumers include SettingsView, SystemView, go2rtc settings diagnostics,
workspace preferences, host health/history/log polling, stream health/history,
setup wizard/reset/status, restart reconnection, and settings reads in streams
and event routing. Player telemetry validates the outbound payload while retaining
its one-way beacon semantics. Settings/system views hide invalid loaded data and
provide explicit Retry. The wizard now uses the real CPU field, waits for a
validated settings acknowledgment before advancing and no longer closes after a
failed completion request. Restart reconnection waits for an observed outage
before declaring the server back online.

## Evidence

RED: the first independent native system test failed against baseline because
`SystemSettings` was absent from the canonical schema. Subsequent fixture reads
also corrected candidate fields against actual native serializers; existing UI
interfaces were scaffolding only, not the acceptance oracle.

GREEN on isolated loopback servers with owned temporary databases/configuration:

- Nine API tests: explicit 23-operation Basic/session/legacy-key/scoped-token
  discovery expectations; effective custom-role authority and revocation;
  anonymous/viewer/token denied-write no-effects; zero retention and masked
  credentials; invalid format preflight; workspace owner isolation; public
  health camera filtering; telemetry 204/denial; bootstrap transitions; backup
  JSON bytes and 0600 mode in a separate owned web root; actual restart re-exec
  followed by shutdown/process exit. Container restarts are treated as the native
  orchestrator handoff rather than claiming an external orchestrator was tested.
- Two actual Chromium scenarios: malformed settings/system reads cannot render
  private rows or enable editing; Retry recovers from real native data; malformed
  save acknowledgment reports error, is sent once, and does not report success;
  a subsequent valid write refreshes the baseline.
- Seven adapter unit tests cover malformed reads/writes, invalid outbound fields,
  zeros, no retries, secret-safe transport errors, headers, encoded cursor and abort.
- Full frontend Jest run: 72 suites / 859 tests passed before the final additional
  health/telemetry wiring; final integrated validation is recorded separately.
- Web and root test TypeScript checks, ESLint, production build, canonical lint
  (four existing recording-route ambiguity warnings), generation/drift pass.
- Contract tooling: 10 tests, 12,182 validator equivalence cases passed.
- Optimized and Debug ASan/UBSan `test_openapi_projection` and
  `test_authorization_snapshot` pass, including bootstrap snapshot/error tests.

Commands and raw logs for this development run are in ignored
`build/hardening-four/system-*`. Final integrated evidence belongs to the four-PR
execution plan. No operator configuration, production storage or hardware camera
was used. Builds tested here have managed go2rtc, MQTT and optional ML disabled;
this receipt does not assert those device/provider integrations are qualified.

Independent integration review found two additional consumer regressions, both
corrected with browser evidence. Bootstrap now applies valid client-config flags
when the independent setup-status request fails (two actual-page cases). A lost
restart acknowledgment now uses structured `mayHaveApplied` errors to keep
observing outage/recovery without resubmitting the restart; an explicit 403
returns to confirmation. The lost-response case failed with zero status probes
before the fix; both restart browser cases pass after it.

Transport review follow-up: the system adapter now forces `redirect: error` and
`credentials: same-origin` after caller options, and rejects successful responses
without JSON content type before parsing. Two owned loopback HTTP origins
reproduced 307/308 forwarding of a synthetic MQTT password before the change;
non-JSON acknowledgments also incorrectly reported saved settings. The shared
`web/tests/contracts/redirectHarness.ts` exercises actual fetch redirects, not
mocked redirect responses. Both redirect cases now send exactly one request to
the application origin, no request/body to the second origin, and report an
ambiguous mutation without retry. Three MIME cases fail closed. Together with
event transport tests, 22 tests pass; existing abort, sanitized-error and no-retry
cases remain green. Web TypeScript checks and adapter ESLint pass. Raw evidence:
`build/evidence-contracts/transport-{red,green,typecheck,eslint}.log`.
