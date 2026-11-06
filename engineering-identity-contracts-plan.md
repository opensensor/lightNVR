# F1: Identity self-service and MFA contracts

Status: F1a boundary review in progress; the session/MFA security correction is
implemented in [PR #808](https://github.com/opensensor/lightNVR/pull/808), but the
11-operation contract expansion is not accepted. Baseline main `98c7a9ac`,
following merged PRs #805/#806/#807.
See [expansion plan](engineering-api-expansion-plan.md) and
[PRD](docs/prd/ENG_01_Safety_Contracts_TypeScript.md).

## Bounded scope

Add contracts and actual consumers for these 11 registered operations, preserving
the existing 33-operation contract. The inventory target is 44 accepted operations
and 196 residual operations if route registration remains unchanged.

| Family | Method and registered route | Handler source |
| --- | --- | --- |
| Sessions | GET /api/auth/sessions; DELETE /api/auth/sessions/# | src/web/api_handlers_auth_backend_agnostic.c |
| Trusted devices | GET /api/auth/trusted-devices; DELETE /api/auth/trusted-devices/# | src/web/api_handlers_auth_backend_agnostic.c |
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
