# F1: Identity self-service and MFA contracts

Status: F1a's MFA/session correction merged in PR #808. F1b and the four-route
part of F1d merged together in PR #809: session/trusted-device list and
revoke, permission-aware discovery, generated validation and Settings consumption.
The contract has 37 operations / 34 paths, with 203 residual routes. The remaining
seven identity operations still target 44 accepted operations when qualified.
F1c behavior is implemented for review against main `4a0dfda2`, following
merged PRs #812–#814. The earlier F1b baseline was `7aa09680`.
See [expansion plan](engineering-api-expansion-plan.md) and
[PRD](docs/prd/ENG_01_Safety_Contracts_TypeScript.md).

## Bounded scope

The eleven-operation identity increment is split into a reviewed four-operation
session/device slice and seven remaining operations. The inventory target after
all eleven remains 44 accepted operations and 196 residual operations if route
registration remains unchanged. The session/device slice raises coverage from
33 to 37; it does not claim acceptance of MFA configuration, password or logout.

| Family | Method and registered route | Handler source |
| --- | --- | --- |
| Sessions (implemented) | GET /api/auth/sessions; DELETE /api/auth/sessions/# | src/web/api_handlers_auth_backend_agnostic.c |
| Trusted devices (implemented) | GET /api/auth/trusted-devices; DELETE /api/auth/trusted-devices/# | src/web/api_handlers_auth_backend_agnostic.c |
| MFA completion | POST /api/auth/login/totp | src/web/api_handlers_totp.c |
| MFA configuration | POST /api/auth/users/#/totp/setup, /verify, /disable; GET /api/auth/users/#/totp/status | src/web/api_handlers_totp.c |
| Password | PUT /api/auth/users/#/password | src/web/api_handlers_users_backend_agnostic.c |
| Legacy logout | GET /logout | src/web/api_handlers_auth_backend_agnostic.c |

Canonical path parameters replace registered wildcards. Keep login challenge
responses distinct from authenticated success. A secret may appear only in an
explicitly authorized setup response; session/trusted-device lists, status,
discovery, logs and ordinary errors must never contain credential material.
Cookies, redirects and revocation side effects are part of acceptance, beyond JSON.

## Dependencies and ownership

One integration owner serializes canonical schema, projection metadata, generated
artifacts and shared fixtures. F1a precedes F1b and F1c; both precede F1d; F1e is
the integration acceptance step. Do not split concurrent edits to shared auth
handlers or SettingsView. Each task can be reviewed as a bounded commit.

| Task | Depends on | Files and work | Acceptance |
| --- | --- | --- | --- |
| F1a: Freeze principal and wire behavior | Current F0 checkpoint | Inspect registration, auth/TOTP/password handlers, central authorization, auth DB helpers and current principal matrix. Record every success/error/redirect/cookie shape and credential class. | Explicit session, Basic, legacy-key, scoped-token and anonymous/challenge expectations per operation; no role-name-only projection policy. |
| F1b: Session and trusted-device lifecycle | F1a | Auth handlers and DB helpers; real-server contract fixtures/tests. Bound ID parsing and list limits; owner-scoped revoke behavior. | Own versus another user's IDs, current-cookie clearing, session reuse after revoke, trusted-device removal requiring MFA again, invalid IDs and denied durable no-effects. |
| F1c: MFA and password boundaries | F1a | TOTP and password handlers, existing TOTP/auth DB tests, real-server challenge fixtures. | Setup/verify/disable/status, challenge expiry and replay, wrong codes, configured attempt limits, trusted-device opt-in, forced first-password change and actual old/new password validity. |
| F1d: Contract integration and UI consumption | F1b, F1c | Canonical OpenAPI, projection metadata, coverage/principal matrix, generated types/guards; LoginView, AuthGate, TotpSetupModal and SettingsView with a typed identity adapter. | Lint and drift pass; runtime validation for the real consumers; malformed responses cannot falsely log in, revoke or disable MFA. Preserve post-login history handling. |
| F1e: Integrated acceptance | F1d | Native CI selection, API/browser contract suites and evidence/docs. | Fresh Release and sanitizer checks, all accepted contract tests, focused frontend tests/typecheck/lint, bounded coverage report and no secret-bearing artifacts. |

F1a must resolve existing ambiguities with tests before widening discovery:
session/trusted-device handlers currently obtain an authenticated user and scope
DB operations to that user's ID, whereas TOTP/password handlers evaluate
users.manage and distinguish scoped tokens from self-service. A token issued by
an admin is not automatically an unrestricted owner session. Test effective
permissions and token intersection; fix any reproduced mismatch in the same slice.
The legacy logout route needs HTTP redirect/cookie tests rather than a fabricated
JSON response schema. User administration beyond these operations remains F3.

## Acceptance matrix and evidence

Use two isolated users, an effective users.manage principal, a principal without
that action, scoped tokens with and without it, a legacy key, Basic credentials,
a cookie session and anonymous requests. Include bootstrap/password-change and
pending-MFA states. For each mutation, compare DB state and then attempt the
protected operation with both the retained and revoked credential. Test permission
projection separately from actual handler enforcement.

Use deterministic clock/DB-expiry fixtures instead of long sleeps. Consume a
challenge once, reject its replay and reject expiry; verify that failures do not
create a session or trusted device. Check whether TOTP time-step reuse is already
prevented and explicitly decide its contract from the implementation and security
requirements; do not assume challenge single-use proves code single-use.

Retain only sanitized receipts. Measure new validator bundle cost against the
existing expansion budget before adding broad consumers. Passing tests establish
this 11-operation scope, not all identity administration or device authorization.

## Following decisions

F2 fleet reads follows F1 acceptance. Plan S1 audio/signaling authorization as a
separate security slice ahead of broad F10 device controls. Target hardware,
longer recovery qualification and Rust remain separate decisions.


## F1a security preflight: pending MFA sessions (2026-11-06)

The owned real-server baseline reproduced a password-verified `totp_token`
authenticating as a normal session before completing MFA. The legacy form-login
branch also issued a session without the JSON MFA challenge. These are behavioral
fixes to land before publishing the wider identity contract.

The correction stores pending challenges separately from authenticated sessions
using an explicit state in the sessions table. Normal session validation and
listing exclude pending rows. A challenge expires after five minutes, allows
five code checks, is bound to the issuing IP/user agent, and must be atomically
consumed before a full session or trusted-device cookie can be issued. Recheck
active/MFA state and the account's current source restriction at completion.
Regular session tokens cannot be used as challenges. Password changes revoke
pending challenges along with the user's sessions. MFA-enabled form login
redirects to the JSON login UI without issuing credentials.

Migration 0093 requires existing MFA-enabled users to sign in again once: their
old pending and authenticated rows cannot be distinguished reliably. Other users
retain sessions. Downgrade deletes pending rows before removing their state
columns, so a challenge cannot become a regular session on rollback. Existing
trusted-device policy and Basic/legacy-key API authentication remain unchanged;
this slice hardens session issuance, not every supported credential class.

Regression acceptance includes pending cookies on protected endpoints, full-token
rejection at the completion endpoint, replay, simultaneous completion with only
one session/device, numeric wrong-code exhaustion, expiry, client-context and
source restrictions, account/MFA disablement, password revocation, trusted-device
revocation, form-login behavior, cross-owner consumption, and actual migration
upgrade/rollback. Run the native unit and sanitizer labels plus the complete
real-server API suite, not only the new cases. No canonical operations or UI
consumers are counted as accepted by this preflight.

Remaining F1 decisions: complete the principal matrix and lifecycle/parser work,
then publish the 11-operation schema and typed consumers. The five-check limit is
per challenge, not an account-wide TOTP throttle. The existing TOTP time window
permits a valid code on separate challenges; challenge single-use does not imply
time-step replay prevention. Account-wide throttling/time-step policy and Basic
or legacy-key MFA semantics must be explicit before claiming broader MFA coverage.


Local preflight validation: all 86 native ci-unit and all 61 ASan/UBSan
ci-sanitizer tests pass, including database fault/rollback and missing-discriminator
regressions. All 61 real-server API tests pass with zero retries (six new MFA
cases); contract/root test type checks, canonical generation drift, migration
embedding and documentation links pass. Optimized Release gcov/LCOV intersects
92/93 changed executable C lines (98.9%) across the auth database and two handlers.
This is a local changed-line measurement, not branch coverage or a hosted Codecov
result. Receipts are under build/f1-qualified-*, build/f1-fault-* and build/f1-pr/.


After PR #807 merged, PR #808 targets main and contains only the identity
correction and plan updates. Hosted validation at `72c0fd02` passed
[Integration Tests](https://github.com/opensensor/lightNVR/actions/runs/42233280978),
[Sanitizer Build](https://github.com/opensensor/lightNVR/actions/runs/42233285534),
[Static Analysis](https://github.com/opensensor/lightNVR/actions/runs/42233289267)
and [CodeQL](https://github.com/opensensor/lightNVR/actions/runs/42233293791).
The subsequent main merge and status reconciliation do not alter native or web
application sources; normal main-targeted PR checks rerun on the updated head.


## F1b session/device vertical slice (2026-11-06)

The four session/trusted-device list/revoke operations now use canonical OpenAPI
and `non_scoped_identity` discovery, generated TS types and runtime guards, and
an actual Settings adapter. Session cookies, Basic and legacy keys resolve the
owner; scoped tokens (including users.manage) do not. Cookie precedence is
preserved. An administrator cannot revoke another account's credential through
these self-service endpoints. Synthetic auth-disabled principals receive 403.

IDs are canonical positive decimal integers up to 9007199254740991, matching
exact JSON/JavaScript representation. Prefixes, signs, leading zeros, encoded
aliases and overflow fail without revocation. The router rejects extra path
segments with 404; malformed single-segment IDs receive 400. Missing/other-owner
records return indistinguishable 404, and database failures return 500.

Lists retain their existing 32-entry cap, now specified in the schema, ordered
by recent activity and descending ID for ties. There is no pagination in this
slice. A single owner-filtered DELETE RETURNING identifies the removed token
and clears its cookie independently of list order or expiry; only the completed
delete may report success. Pending MFA rows cannot be revoked as full sessions.
This does not cancel requests that were already authorized before revocation.

Settings validates every list and acknowledgment, rejects secret-bearing or
malformed payloads, avoids automatic retries of DELETE, and refreshes after an
ambiguous failure without showing success. Credential query caches expire on
unmount; requests receive cancellation signals. The current-session revoke
causes the normal authenticated refresh to return the browser to login. All
responses include private/no-store directives; JSON helpers also append their
existing no-cache policy.

This vertical delivery intentionally completes the four-route portion of F1d
with F1b, rather than waiting to combine all eleven operations. Next: F1c's MFA
configuration/password boundary and replay/throttling decisions, then the seven
remaining schemas and Login/TOTP consumers. F2 fleet reads and S1 audio/signaling
retain their previous order and scope. Rust remains deferred.


F1b local qualification: 86 native ci-unit tests, 61 ASan/UBSan ci-sanitizer
tests and 68 real-server API cases pass (seven new lifecycle cases). The 16
browser cases all pass, including three real Settings scenarios.
All 805 frontend tests across 69 suites pass, including 21 new adapter cases.
Root/web type checks, web lint, canonical/projected lint, all six generator-tool
tests, generated drift and documentation links pass. Local optimized Release
coverage intersects all 75 changed executable C lines; this is not branch coverage
or a hosted Codecov measurement. Build/test receipts are retained in build/f1b/.

Production asset isolation passes for Live, Streams, Recordings, Settings and the
API explorer; only the explorer requests Swagger vendor assets. The modern-script
validator chunk in the default build is 328181 raw bytes versus the previous
358973 (-30792, about 8.6%). Vite reports 36.78 kB gzip versus 30.87 kB (+5.91 kB).
Runtime generation now fails fast on invalid responses, while retaining
non-mutating validation and sanitized diagnostics. It did not eliminate the
generated-code maintenance findings; see the expansion plan's post-merge report
checkpoint. The gzip cost remains a measured tradeoff.
Settings now consumes this shared validator chunk; unused guard splitting and
target-hardware capacity remain separate follow-ups. No new discovery latency,
whole-process memory or device qualification claim is made by this slice.

The full branch C/C++ CodeQL scan at 99ffe05e (run 42252922921, analysis
2113509966) retains exactly the 22 existing findings from the recording-hardening
baseline, with no additions or removals by rule, file and diagnostic. Hosted patch
coverage is assessed after all instrumented jobs upload, rather than from the
initial unit-only report.

PR #809 subsequently merged with all hosted checks and patch coverage passing.
The expansion plan records the remaining quality/security report inventory and
its distinction from this slice's completed behavioral qualification.

## F1c password and TOTP behavior (2026-11-06)

Implemented on main baseline `4a0dfda2`, following merged quality PRs #812–#814.
This increment qualifies security behavior before F1d publishes the seven
remaining identity operations and adds typed Login/AuthGate/TOTP consumers.
The canonical count remains 37 operations / 34 paths, with 203 residual routes.

Basic authentication and legacy API keys retain their existing password/TOTP
management access, as explicitly requested. Self-service and effective
`users.manage` checks remain distinct; scoped tokens use their owner's effective
permissions intersected with token actions. This does not require an MFA-verified
browser session for management, revoke API keys on password changes, or encrypt
stored camera credentials. Password hashing migration is a separate follow-up.

The corrected boundaries are:

- A successful password change atomically verifies the required old password,
  writes the new credential, and revokes all sessions, pending MFA challenges and
  trusted devices. Administrative resets retain their existing policy. Login
  issuance rechecks the current password and MFA state inside its transaction.
- TOTP setup cannot overwrite an enabled secret. Pending setup expires after ten
  minutes; the authorized secret response is private/no-store and URI labels are
  escaped. IDs must be complete positive decimal int64 values with the expected
  route suffix. Status contains only configuration/enabled booleans.
- Enrollment and disable atomically change MFA state and revoke other sessions,
  all challenges and remembered devices. The initiating owner's cookie session
  is retained; an administrator managing another account retains none of that
  account's sessions. A failed revocation rolls back the state change.
- A successful OTP consumes its time step once across enrollment, inline login
  (including force-MFA mode) and challenge completion. The accepted ±1-step window
  remains; a consumed step or any older step cannot authenticate again. Reuse is
  rejected even when the configurable attempt limiter is disabled.
- Incorrect/replayed codes share a durable account attempt window across new
  challenges and inline login. It uses the existing login limiter enabled/max/
  window settings (default five attempts per 300 seconds). A new challenge,
  password success or repeated setup cannot reset it; successful OTP validation
  or window expiry resets failures. The existing five-check challenge cap remains.
- Challenge consumption, OTP state, session creation and optional remembered-device
  creation commit together. Failure issues no credentials and leaves the code
  available for retry after a database error. Verification still checks current
  active-account/source policy and challenge context/expiry.

Migration 0094 preserves existing enabled secrets and authenticated sessions.
Old unfinished enrollments have no expiry and must restart setup. Downgrade
removes pending challenges before dropping replay/attempt columns. Database
reopen tests verify attempt/replay persistence; missing security columns fail
closed. Restoring an older database also restores its older replay state, so this
is not protection against administrative database rollback.

The single-use decision follows [RFC 6238 section 5.2](https://www.rfc-editor.org/rfc/rfc6238#section-5.2).
The shared retry bucket follows the account-level rate-limit requirement for
short OTP outputs in [NIST SP 800-63B](https://pages.nist.gov/800-63-4/sp800-63b.html).
These references justify these controls, not a claim of overall standards compliance.

Next: F1d's seven schemas and typed consumers, including actionable expired-setup,
replayed-code and cooldown errors, then F1e integrated acceptance. F2 fleet reads
and S1 signaling authorization keep their previous order. Encryption remains an
opt-in design/migration decision; Rust remains deferred. Code finding closure
requires post-change analysis and is not inferred from passing behavior tests.

F1c local qualification: 91 native ci-unit suites and 67 ASan/UBSan
ci-sanitizer suites pass. The complete API run passed 78 cases; the final focused
run passed all 11 new hardening cases including the subsequently added force-MFA
case (79 distinct API cases overall). All 16 browser cases pass. Final focused
native/sanitizer checks, test type checks, canonical generation drift, embedded
migration consistency and documentation links pass. Six initial security
regressions were first reproduced as failures on main.

Optimized Release gcov/LCOV intersects 366/385 changed executable C lines (95.1%),
including the extracted TOTP helpers. This is local line coverage, not branch
coverage or a hosted Codecov result. Receipts are in `build/f1c/`. No scanner
finding is marked closed by this local qualification.
