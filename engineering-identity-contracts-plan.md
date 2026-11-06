# F1: Identity self-service and MFA contracts

Status: planned, not implemented. Baseline main `b9d3ac61`, following merged
PR #804. This is the next API slice after recording-integrity acceptance.
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
