# F3 identity administration, policy and audit evidence

Base: main `84ed0174`; isolated branch `codex/hardening-identity-admin`.

## Accepted route inventory

This slice adds exactly 20 operations (77 total on the isolated baseline, 163 registrations remaining). Coverage and principal expectations are recorded independently in `coverage.json`, `principal-matrix.json` and the API test's explicit operation lists.

| Route | Operation | Discovery predicate |
|---|---|---|
| GET `/api/auth/users` | `listAdminUsers` | `global` |
| POST `/api/auth/users` | `createAdminUser` | `non_scoped_global` |
| GET `/api/auth/users/{id}` | `getAdminUser` | `global` |
| PUT `/api/auth/users/{id}` | `updateAdminUser` | `identity_management` |
| DELETE `/api/auth/users/{id}` | `deleteAdminUser` | `global` |
| POST `/api/auth/users/{id}/api-key` | `createLegacyApiKey` | `non_scoped_identity` |
| PUT `/api/auth/users/{id}/password-lock` | `setUserPasswordLock` | `global` |
| POST `/api/auth/users/{id}/login-lockout/clear` | `clearUserLoginLockout` | `global` |
| GET `/api/authorization/actions` | `listAuthorizationActions` | `global` |
| POST `/api/authorization/simulate` | `simulateAuthorization` | `global` |
| GET `/api/authorization/roles` | `listAuthorizationRoles` | `non_scoped_global` |
| POST `/api/authorization/roles` | `createAuthorizationRole` | `non_scoped_global` |
| PUT `/api/authorization/roles/{uuid}` | `updateAuthorizationRole` | `non_scoped_global` |
| DELETE `/api/authorization/roles/{uuid}` | `deleteAuthorizationRole` | `non_scoped_global` |
| GET `/api/authorization/users/{id}` | `getUserAuthorizationPolicy` | `non_scoped_global` |
| PUT `/api/authorization/users/{id}` | `setUserAuthorizationPolicy` | `non_scoped_global` |
| GET `/api/audit/events` | `listAuditEvents` | `global` |
| GET `/api/audit/events/export` | `exportAuditEvents` | `global` |
| GET `/api/audit/settings` | `getAuditSettings` | `global` |
| PUT `/api/audit/settings` | `updateAuditSettings` | `global` |

Global requirements are `users.manage` except audit's `system.admin`. `non_scoped_global` also excludes scoped tokens; role/policy reads have this restriction too. Self update and legacy key discovery never authorize an arbitrary other account. Direct handlers always reauthorize the concrete target. Scoped management tokens retain existing CRUD/locks/simulation compatibility but cannot mint unrestricted legacy keys or manage policies. The new `non_scoped_camera` primitive is independently tested for the parallel fleet workspace slice.

## Changes and reproduced failures

- **Scoped credential escape, RED → GREEN:** on the baseline native server, a `users.manage` token received HTTP 200 from legacy-key issuance where the regression expected 403. The handler previously returned early for users.manage before checking credential class. It now rejects scoped tokens first. The real-server test verifies that denial does not rotate an existing working legacy key. Basic and legacy-key owner compatibility remains covered.
- **Password lock form, RED → GREEN:** a real Chromium UsersView interaction checked “Lock Password Changes”, created the account and received a success toast while the stored field remained false. CRUD and lock writes are separate native APIs. The consumer now calls the password-lock operation after a validated CRUD response when needed, validates its acknowledgment and reports partial application if lock confirmation fails. It refreshes the list on error and never retries either mutation automatically. Legacy-role edits also omit the numeric role field, which is rejected by the native update handler; policy grants have a separate editor.
- All twenty operations have explicit native wire schemas, generated declarations and runtime guards. User, role, policy, simulation and audit records require their actual fields. User records reject credential fields. API-key material is allowed only in the one-time issuance response. Audit details retain their intentional free-form object shape.
- UsersView, AccessPolicyModal, RoleManagerModal, ApiKeyModal's catalog/key actions, AuditHistoryModal and AuditDecisionModes use a closed route-to-validator adapter. Request guards run before writes; target IDs must match validated user/policy replies so a wrong-user acknowledgment cannot redirect a follow-up lock; malformed payloads/status/media types cannot claim success; messages exclude wire payloads and credentials; all mutations force zero retries. Draft models remain local while wire types alias canonical components.
- CSV export checks status, content type, attachment name, total-count header and actual CSV header before download. The API test checks real paginated quoted bytes, embedded quotes/newlines and spreadsheet-formula neutralization. The browser test downloads a real exported artifact.
- Prior discovery assertions remain exact within explicit previously accepted families. New F3 assertions independently cover the twenty new IDs; the canonical document is not used as the authorization oracle.
- Generator bundling preserves symlinked dependency paths for isolated worktrees and explicitly allows Ajv's `fast-deep-equal` runtime needed by unique action arrays. No runtime dependency or lockfile changes.

## Validation

Commands run from the isolated worktree; fixtures use ephemeral loopback ports and unique `/tmp/lightnvr-contract-*` directories. Native suites were serialized against other workers because some tests use fixed temporary database paths.

- **6/6 PASS:** Release native build, then six focused tests: projection, OpenAPI handler, authorization snapshot, users handlers, audit log and audit summary.
- **6/6 PASS:** Separate Debug build: the same six tests under ASan/UBSan with leak detection and halt-on-error.
- **24/24 PASS** in the cross-family regression; **6/6 PASS** in the final F3 run after metadata finalization. Real-server API regression: `identity.api.spec.ts`, `identityAdmin.api.spec.ts`, `identityHardening.api.spec.ts`. Tests cover all twenty direct unauthorized routes, durable no-effects, self-lockout, stale policy revisions, delegation limits, token/owner intersection changes and revocation, Basic/key compatibility, user CRUD/locks, simulation and real CSV.
- **4/4 PASS:** Actual Chromium UsersView/admin modals: locked account creation, legacy administrator edit, invalid list/write acknowledgment privacy and one-write behavior, role creation, real audit CSV download, policy edit failure then valid revised policy.
- **147/147 PASS** across nine frontend contract and administration helper unit suites; frontend and contract TypeScript checks; full frontend ESLint; production Vite build.
- **PASS:** Canonical/projected OpenAPI lint (four existing path-ambiguity warnings), generator drift check, twelve tooling tests including optimized/unmodified Ajv equivalence over 9,988 valid/invalid cases.

The native CRUD-plus-lock sequence is deliberately not claimed atomic. A lock failure after a successful save returns an explicit partial-result message and refreshes current users. Audit retention/pruning/mode changes likewise retain the native documented partial-failure behavior. API/UI tests use owned fixtures; no external cameras or production users were mutated.

## Personal credential transport follow-up

The older session/trusted-device adapter now forces `redirect: error`,
`credentials: same-origin` and `cache: no-store` after caller options, and requires
JSON response content type before exposing session metadata or acknowledging
revocation. Authentication semantics, external abort propagation and zero mutation
retries are unchanged. The administration adapter already enforced the redirect
and credential invariants and needed no change.

The shared owned-loopback redirect harness exercises real 307/308 responses for
both session and trusted-device revocations with a synthetic API-key header. Each
mutation reaches only its intended application origin once; the redirect target
receives no request, even when caller options request following redirects,
including credentials, caching and seven retries. Invalid MIME checks cover both
session reads and successful-looking revocation bodies. Five new cases failed
before the fix; all 26 focused identity cases now pass, with TypeScript and
adapter ESLint clean. Evidence: ignored
`build/evidence-contracts/identity-transport-{red,green,typecheck,eslint}.log`.
