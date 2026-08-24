# Authorization endpoint inventory

This is the Fleet 02 coverage map for routes registered in
`src/web/libuv_api_handlers.c`. “Centralized” means the handler evaluates the
effective action for the server-resolved resource; list responses filter before
computing totals or facets. “Intentional” identifies bootstrap or health routes
that do not carry a policy action. Rows called out as gaps are not advertised as
enforced by `GET /api/authorization/actions`.

## Public, bootstrap, and identity routes

| Routes | Treatment |
| --- | --- |
| `GET /api/setup/status` | Intentional public setup state |
| `POST /api/setup/status` | Public only until setup completes; afterward one centralized `system.admin` decision and audited outcome |
| `GET /api/health`, `GET /api/health/hls` | Intentional deployment health probes |
| `GET /api/metrics` | `system.admin`; stream-labelled operational metrics are not an anonymous probe |
| Login configuration/login/TOTP-login | Public authentication flow with throttling and login-outcome audit |
| Logout/verify/session/trusted-device routes | Authenticated identity lifecycle; self-service identity checks, with cross-user administration mapped to `users.manage` |
| TOTP setup/verify/disable/status | Session/Basic/legacy-key self-service, or effective `users.manage`; a scoped token cannot use owner identity as a bypass |
| `POST /api/telemetry/player` | Authenticated and centrally evaluated as `live.view` for the submitted stream |

## Camera reads and live media

| Routes | Action | Enforcement |
| --- | --- | --- |
| Stream list/single/full reads | `live.view`; sensitive configuration fields additionally require `camera.configure` | Centralized, filtered, one fleet load and shared evaluation contexts |
| Fleet queries/previews | `live.view` | Centralized; filters before totals, pages, and facets |
| Location/tag reads | `live.view` | Centralized hierarchy/dictionary filtering; global viewers/configurators retain empty administrative nodes |
| Collection reads/members/previews | `live.view` | Centralized camera filtering; scoped users do not receive empty shared collection metadata, unfiltered counts, or smart selectors; owner/global configurator behavior is preserved |
| Direct HLS and go2rtc HLS session creation | `live.view` | Centralized with a bounded 30-second principal/camera/session cache; follow-up segments do not create per-segment durable success audit rows |
| PTZ capability/preset reads | `live.view` | Centralized per camera |
| Detection results, event snapshots, detection images | `live.view` | Centralized per stream/camera |
| Detection model catalog | `camera.configure` | Allowed for a principal that can configure at least one fleet camera; the catalog itself does not expose a camera inventory |
| ICE server configuration | authenticated plus `live.view` on at least one camera | Prevents anonymous TURN credential disclosure while preserving scoped playback |
| Camera audio playback | `audio.listen` | **Gap:** direct go2rtc/WebRTC signaling is not yet a trustworthy action boundary |
| Camera talk/backchannel | `audio.talk` | **Gap:** requires tokenized or lightNVR-mediated signaling before enforcement can be claimed |

A direct request for one unauthorized camera returns `403` without resource
metadata. High-volume HLS success decisions are the documented audit exception:
session creation is authorized, cache revocation is bounded to 30 seconds, and
deny/error outcomes remain durable without writing on every media segment.

## Camera configuration

| Routes | Action | Enforcement |
| --- | --- | --- |
| Stream create/update/delete/test/refresh | `camera.configure` | Centralized; redacted success/failure/error operation outcomes include stable camera identity |
| Manual recording and recording-schedule changes | `camera.configure` | Centralized per camera with outcome audit |
| Stream retention GET/PUT | `camera.configure` | Centralized per camera; PUT outcome audited |
| Zones, imaging, day/night, and motion trigger | `camera.configure` | Centralized per camera with mutation outcome audit |
| ONVIF discovery/test/profile/add | `camera.configure` | Centralized; credentials/URLs are excluded from audit details and created cameras retain stable identity where available |
| Location/tag/collection mutations and camera assignments | `camera.configure` | Centralized global configuration authority with redacted outcomes; numeric legacy role is not an access bypass |
| Future bulk job preview/execute/retry/cancel | `fleet.execute_job` | **Gap/not implemented** |

## Recordings and evidence

| Routes | Action | Enforcement |
| --- | --- | --- |
| Lists/details/timeline/manifest/playback/thumbnails | `recordings.replay` | Centralized and filtered by replay-authorized cameras; explicit unauthorized stream is `403`; collection and stream predicates intersect |
| Recording tag/detection-label facets and protected count | `recordings.replay` | Derived only from authorized cameras; large fleets use chunked SQLite queries below variable limits |
| `GET /api/recordings/files/check` | `recordings.replay` | Recording metadata is resolved first, then camera authorization precedes filesystem stat |
| Playback media | `recordings.replay` | Centralized recording-to-camera resolution |
| Download and batch-download lifecycle | `recordings.export` | Centralized; fixed batches authorize every member and jobs bind status/result to creator and exact scoped token |
| Snapshot creation/download and go2rtc frame capture | `snapshot.create` | Centralized per camera |
| Protect/unprotect, retention override, and recording-tag writes | `evidence.protect` | Centralized; fixed batches preflight all members rather than partially mutating; outcomes audited |
| Single/file/batch deletion | `recording.delete` | Centralized; open-ended filters require an all-fleet grant; progress is bound to creator user and exact scoped token; outcomes audited |
| `GET /api/recordings/batch-delete/progress/#` | creator binding | Authenticated; other users/tokens receive non-disclosing `404` |
| `POST /api/recordings/sync` | `storage.configure` | Centralized with outcome audit |

Every recording ID/path resolves to recording metadata and a camera before a
resource action is evaluated. Client-supplied collection expansion is never an
authorization boundary.

## System, users, storage, and events

| Routes | Action | Enforcement |
| --- | --- | --- |
| User list/create/get/update/delete, legacy-key generation, password lock/lockout clear | `users.manage` | Centralized for cross-user operations; supported self-service is session/Basic/legacy-key only |
| Password and TOTP cross-user operations | `users.manage` | Centralized effective action, not numeric role |
| Authorization roles, policy, simulation, scoped-token lifecycle | `users.manage` or documented session/Basic/legacy-key self-service | Centralized with optimistic versions and audit; delegation proves action-and-scope containment, scoped tokens cannot rewrite policy or mint tokens, and policy updates accept `mode: policy` only |
| Audit list/CSV/retention | `system.admin` | Centralized and audited |
| `GET /api/settings`, settings writes, go2rtc validation | `system.admin` | Centralized; full settings are never a viewer bootstrap document |
| `GET /api/client-config` | authenticated runtime bootstrap | Safe subset only: auth/demo flags, player availability/flags/timeouts, and thumbnail controls |
| System information/status/logs and restart/shutdown/clear-log/backup operations | `system.admin` | Centralized; sensitive mutations produce outcomes |
| Administrative go2rtc APIs/reload/stream inventory | `system.admin` | Centralized; viewer readiness uses `/api/client-config` instead |
| Storage target/policy/cleanup/write-probe routes | `storage.configure` | Centralized with optimistic revisions and mutation/probe outcomes |
| Event route/destination/catalog administration | `events.configure` | Centralized with optimistic revisions and mutation outcomes |
| Future outbox retry/runtime controls | `events.configure` | **Gap/not implemented** |

## Legacy retirement, audit, and remaining product gaps

At startup, each legacy principal is transactionally replaced with exactly the
built-in compatibility role and its current scope. Legacy `allowed_tags` labels
become normalized tag-UUID `tag_any` selectors. Stale grants are replaced, the
tombstoned column is cleared only on commit, and the migration is idempotent.
The runtime user structure/evaluator and public user/policy APIs no longer read,
write, or accept `allowed_tags` or `mode: legacy`; new users are created inactive,
receive their CIDR restriction and policy, and are activated last.

The durable audit store records centralized decisions and separate redacted
operation outcomes for camera configuration, PTZ, evidence, export/deletion,
taxonomy/collection, storage, events, and sensitive system changes. Target UUIDs
are captured before destructive mutations or derived from successful create
responses. Scoped-token use is audited at a bounded 60-second interval; denied
use distinguishes expired, revoked, inactive-owner, and unknown credentials.
Passwords, token secrets/hashes, authorization headers, credentials, and raw
URLs must never enter details.

Fleet 02 is not fully complete: local authorization groups and grant schedules
are not implemented, `fleet.execute_job` has no product surface, and
`audio.listen`/`audio.talk` need a secure media-signaling boundary. P3 OIDC/SSO
remains deliberately deferred until a committed deployment supplies a real IdP
and testing partner.
