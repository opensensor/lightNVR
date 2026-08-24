# PRD — Scoped Authorization & Audit

**Status**: In progress — user grants, selector scopes, scoped tokens, protected-route enforcement, legacy migration, and sensitive audit outcomes implemented; local groups, grant schedules, and the direct-go2rtc audio boundary remain
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 2 — access-control foundation
**Scope**: Action-level authorization, resource selectors, scoped API tokens,
consistent endpoint enforcement, and a durable audit trail. SSO is explicitly a
deferred, customer-triggered phase.

---

## 1. Problem

The original admin/user/viewer roles and allowed-tag filtering provided a useful
baseline. Upgrades now translate that state into selector-backed policy grants,
but institutional deployments still need to distinguish viewing video from
listening, talking, exporting, operating PTZ, deleting evidence, or changing a
camera. A per-camera checkbox matrix would become unmanageable at hundreds of
cameras and would drift as cameras move or are added.

The same decision must be enforced consistently by every UI and API endpoint,
and sensitive actions need an auditable record.

## 2. Goals

- Authorize explicit actions over dynamic resource scopes.
- Reuse Fleet 01 selectors so permissions follow locations and tags.
- Keep roles understandable: roles bundle actions; grants bind a role to a scope.
- Apply the same authorization rules to sessions and API tokens.
- Default-deny newly introduced privileged actions while preserving sensible
  behavior for existing users during migration.
- Audit access-sensitive and state-changing operations.
- Leave a clean integration seam for a future organizational identity provider.

## 3. Non-goals

- OIDC, SAML, LDAP, or automatic identity-provider discovery in the initial
  implementation.
- Multi-tenant cloud identity federation.
- Per-frame watermarking or DRM.
- Building organization-specific compliance reports before a customer defines
  the applicable policy.

## 4. Authorization model

`principal/group -> grant -> role(actions) + resource selector + optional schedule`

User grants and selector/collection scopes are implemented. Local groups and
grant schedules remain schema/product gaps and must not be represented as
enforced capabilities yet.

Initial actions:

- `live.view`, `audio.listen`, `audio.talk`
- `recordings.replay`, `recordings.export`, `snapshot.create`
- `ptz.control`
- `evidence.protect`, `recording.delete`
- `camera.configure`, `fleet.execute_job`
- `storage.configure`, `events.configure`
- `users.manage`, `system.admin`

Roles are named reusable action bundles. A grant binds a user or local group to
a role and an all-fleet scope, a durable shared-collection reference, or an
inline Fleet 01 selector. Collection grants follow current static or smart
membership and cannot reference private personal collections. A schedule may
limit a grant by local time and day. Explicit deny rules are out of scope for
v1; absence of an allow is deny.

## 5. Requirements

### 5.1 Policy storage and evaluation

- Persist roles, actions, local groups, grants, schedules, and policy version.
- Compile and cache selectors without caching past policy or camera changes.
- Expose a single server-side authorization function used by all handlers.
- Return `403` without revealing resource metadata when a principal lacks access.
- List endpoints filter unauthorized resources and compute totals after filtering.
- All new actions begin denied except for the built-in administrator role.
- Provide a policy simulation endpoint for administrators: principal + action +
  resource returns allow/deny and the matching grant, without executing the action.

### 5.2 Migration and built-in roles

- Map existing admins to every action over all cameras.
- Map existing viewers to safe read-only actions within their legacy
  `allowed_tags` scope during the one-way upgrade migration.
- Map existing users to a documented compatibility role within their current
  scope; call out any newly restricted destructive action during upgrade.
- Keep built-in roles immutable but cloneable.
- Migrate legacy `allowed_tags` labels into normalized tag-UUID selector grants.
  The column remains only as an upgrade tombstone: runtime structs/evaluation,
  public APIs, and the administration UI no longer accept or use it.

### 5.3 API tokens

- Tokens inherit or narrow the issuing principal's grants; they never widen them.
- Token creation requires an explicit expiry, description, and displayed-once
  secret.
- Persist only a strong token hash and token identifier.
- Audit creation, use, revocation, expiry, and denied privileged calls.

### 5.4 Audit trail

- Record timestamp, request/correlation ID, principal, authentication method,
  action, target UUID, outcome, remote address, and safe structured details.
- Required events include login outcomes, policy changes, camera configuration,
  PTZ control, audio talk activation, export, protect/unprotect, deletion,
  storage policy changes, event route changes, and backup restore.
- Never log passwords, tokens, camera credentials, or raw authorization headers.
- Provide filtered, paginated export with a configurable retention period.
- Audit records are append-only through supported APIs.

### 5.5 Administration UI

- Role editor presents action descriptions and warns on destructive bundles.
- Grant editor uses location, tag, collection, or explicit-camera selectors.
- Scope preview shows matched count and a sample before save.
- “Test access as user” uses the simulation endpoint and is itself audited.
- Every denied UI action remains hidden or disabled, but server enforcement is
  always authoritative.

### 5.6 Deferred SSO phase

OIDC/SSO is **not part of the initial implementation**. It activates when a
committed organizational deployment—such as an SJC-scale customer—requires it
and supplies a real IdP configuration and administrative testing partner.

When triggered, the phase should add:

- Authorization Code flow with PKCE, issuer discovery, key rotation, logout, and
  configurable local-admin break-glass access.
- IdP group/claim mapping to existing lightNVR roles and grants.
- Just-in-time user provisioning with safe default scope.
- Audit entries that preserve external subject and issuer.

The core authorization schema must not depend on OIDC, so delaying SSO does not
defer useful access-control work.

## 6. Phasing

| Phase | Scope | Trigger |
| --- | --- | --- |
| P0 | Action vocabulary, policy evaluator, endpoint inventory | Immediate |
| P1 | Selector grants, migration, scoped API tokens | After Fleet 01 selector API |
| P2 | Audit log, policy administration UI, and sensitive-operation outcomes | After P0/P1 |
| P3 | OIDC/SSO and group mapping | Committed organizational customer only |

Current implementation notes:

- Protected camera, recording, taxonomy, configuration, storage, event, user,
  settings, and system routes use centralized action evaluation. List totals,
  facets, collection expansion, and job status are filtered or principal-bound.
- High-volume HLS media authorization is cached for 30 seconds and does not
  append a durable success decision for every segment; denials and errors remain
  auditable. This bounds database load and revocation latency.
- Required camera configuration and evidence operation families emit redacted
  success/failure/error audit outcomes with stable camera/target identity.
- `audio.listen` and `audio.talk` are still reported unenforced because browser
  WebRTC signaling can connect directly to go2rtc. A tokenized or lightNVR-
  mediated signaling boundary is required before those actions are trustworthy.
- Local groups and grant schedules are not implemented. `fleet.execute_job`
  remains reserved for the future bulk-job surface.
- P3 OIDC/SSO remains deliberately deferred as described above.

## 7. Acceptance criteria

- Endpoint tests prove every protected action denies an unauthorized user even
  when the HTTP request bypasses the UI.
- A grant for `tag:parking` automatically includes a newly tagged camera and
  excludes it when the tag is removed.
- A user may replay but not export recordings when those actions differ.
- A camera-scoped token cannot query or mutate cameras outside its grants.
- Every required sensitive operation produces a redacted audit record with a
  correlation ID.
- Upgrading preserves administrator access and produces a migration report.
- No OIDC dependency is required to ship P0–P2.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Missed endpoint creates an authorization gap | Route inventory plus mandatory handler tests and centralized middleware/helper |
| Dynamic selectors create surprising access | Preview, simulation, audit, and visible matching-grant explanation |
| Existing users lose required access | Compatibility mapping and upgrade report |
| SSO work becomes speculative | Gate P3 on a committed customer and actual IdP |

## 9. Dependencies

- Fleet 01 for stable camera UUIDs and selectors.
- Fleet 03 should reuse audit correlation IDs and may publish security events, but
  durable audit storage must not depend on MQTT delivery.
