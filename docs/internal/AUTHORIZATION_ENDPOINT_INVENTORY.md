# Authorization endpoint inventory

This is the Fleet 02 P0 coverage map for the libuv HTTP routes registered in
`src/web/libuv_api_handlers.c`. It assigns every route family to the action that
the centralized policy evaluator must enforce. “Existing” means the handler
still uses its pre-Fleet-02 authentication or role check; follow-up enforcement
PRs move those routes to `httpd_authorize_action()` without changing the action
contract below.

## Public and identity lifecycle routes

These routes do not receive a resource action because they establish identity or
must be reachable before a policy can be evaluated.

| Routes | Treatment |
| --- | --- |
| `GET/POST /api/setup/status` | Setup bootstrap guard; public only until setup completes |
| `GET /api/health`, `GET /api/health/hls` | Deployment health probe; intentionally non-sensitive |
| `GET /api/metrics` | Operational metrics; keep public only when deployment-level network controls protect the endpoint, otherwise require `system.admin` |
| `GET /api/auth/login/config`, `POST /api/auth/login`, `POST /api/auth/login/totp` | Public authentication flow with throttling |
| `POST /api/auth/logout`, `GET /logout`, `GET /api/auth/verify` | Current-principal session lifecycle |
| `GET/DELETE /api/auth/sessions`, `GET/DELETE /api/auth/trusted-devices` | Self-service only; cross-user management maps to `users.manage` |
| TOTP setup, verify, disable, and status routes | Self-service with password/TOTP proof; cross-user administration maps to `users.manage` |
| `POST /api/telemetry/player` | Authenticated telemetry ingestion; cannot mutate camera or policy state |

## Camera and live operations

| Routes | Required action | Current enforcement |
| --- | --- | --- |
| `GET /api/streams`, `GET /api/streams/#`, `GET /api/streams/#/full` | `live.view` per returned camera | Existing viewer access plus allowed tags |
| `GET /api/fleet/*`, collection reads/previews, camera tag/location reads | `live.view` per returned camera | Existing viewer access plus allowed tags |
| `GET /hls/#/#`, go2rtc HLS/stream proxy reads | `live.view` | Existing authentication/proxy checks |
| Camera audio playback | `audio.listen` | Transport-specific existing checks |
| Camera backchannel/talk activation | `audio.talk` | Transport gap: browser WebRTC connects directly to unauthenticated go2rtc; enforce with a tokenized or lightNVR-proxied signaling path before enabling policy-mode talk |
| PTZ capability/preset reads | `live.view` | Centralized action policy with server-resolved camera scope |
| PTZ move, stop, home, absolute, relative, and preset writes | `ptz.control` | Centralized action policy with server-resolved camera scope; authorization and device-operation outcomes are audited |
| Imaging/day-night reads | `live.view` | Existing stream access checks |
| Imaging/day-night writes | `camera.configure` | Existing stream write checks |
| `POST /api/motion/trigger` | `camera.configure` | Existing non-viewer check |
| Detection results and event snapshots | `live.view` | Existing viewer access and stream scope |
| Detection model catalog | `camera.configure` | Existing endpoint check; model availability is camera configuration metadata |

List handlers must filter unauthorized cameras before calculating totals,
facets, or collection membership. A request for one unauthorized camera returns
`403` without disclosing camera metadata.

`authorization_filter_visible_cameras()` implements that rule for every handler
that loads the fleet inventory: `POST /api/fleet/cameras/query`,
`POST /api/fleet/selectors/preview`, the collection reads and previews, and the
collection-to-stream-name resolution used by recording filters. It evaluates
`live.view` per camera through one shared evaluation context, so the whole page
costs a single grant load. Legacy-mode principals keep their allowed-tags
behaviour because the evaluator applies that restriction for them.

Rows still marked "Existing" below have not moved to the centralized evaluator.
Their actions are reported with `"enforced": false` by
`GET /api/authorization/actions` so an operator authoring a policy can see which
boundaries the daemon does not apply yet.

Scoped API tokens authenticate only on routes that call
`httpd_check_action_access()`. A route still using `httpd_check_viewer_access()`
rejects a scoped token with `401` rather than ignoring its scope, which is the
fail-closed behaviour but also means a token carrying an unenforced action
cannot be used at all. The token UI therefore offers only actions that are both
enforced and reachable with a scoped token.

## Camera administration

| Routes | Required action | Current enforcement |
| --- | --- | --- |
| Stream create/test, update, delete, refresh | `camera.configure` | Existing non-viewer and allowed-tag checks |
| Location create/update/delete and camera assignment | `camera.configure` | Existing administrator checks |
| Tag create/update/merge/delete and camera assignment | `camera.configure` | Existing administrator checks for mutation |
| Collection create/update/delete and static membership changes | `camera.configure` | Existing owner/administrator checks |
| Zone create/delete | `camera.configure` | Existing stream write checks |
| Stream retention and recording-schedule writes | `camera.configure` | Existing non-viewer checks |
| Manual recording start/stop | `camera.configure` | Existing non-viewer checks |
| ONVIF discovery, test, profile reads, and add | `camera.configure` | Existing endpoint checks; discovery results must not expose credentials |
| Future template/bulk job preview | `fleet.execute_job` | Not implemented |
| Future bulk job confirmation/retry/cancel | `fleet.execute_job` | Not implemented |

## Recordings and evidence

| Routes | Required action | Current enforcement |
| --- | --- | --- |
| Recording lists/details, timeline segments/manifest/play, thumbnails | `recordings.replay` | Existing viewer access plus stream tags |
| Recording playback and HLS media | `recordings.replay` | Existing viewer access plus stream tags |
| Recording download and batch-download lifecycle | `recordings.export` | Centralized action policy; fixed batches are authorized in full, job status/results are bound to the creating user, and archive/transfer outcomes are audited |
| Snapshot creation/download | `snapshot.create` | Existing viewer access plus stream tags |
| go2rtc frame capture proxy | `snapshot.create` | Existing proxy checks |
| Protect/unprotect and retention overrides | `evidence.protect` | Centralized action policy; batch members are resolved and evaluated individually, with mutation outcomes audited |
| Single/file/batch recording deletion | `recording.delete` | Centralized action policy; fixed batches are filtered member-by-member, open-ended filters require sufficient server-resolved scope, and completion outcomes are audited |
| Recording tag reads | `recordings.replay` | Existing viewer access |
| Recording tag writes | `evidence.protect` | Existing endpoint checks |
| Recording database/file sync | `storage.configure` | Existing administrative behavior |

Every recording ID must resolve its camera UUID before evaluation. Batch actions
must authorize every fixed member and report unauthorized members as failures;
they may not trust a client-supplied collection expansion.

## System administration

| Routes | Required action | Current enforcement |
| --- | --- | --- |
| User list/create/get/update/delete, API-key generation, password lock and lockout clear | `users.manage` | Existing administrator/self-service rules |
| Authorization action catalog, roles, user grants, and policy simulation | `users.manage` | Central evaluator with optimistic policy-version checks |
| Scoped API token list/create/revoke | Self-service or `users.manage` | Hashed, expiring tokens; token-authenticated token chaining is rejected |
| Audit event list/CSV export and retention settings | `system.admin` | Central evaluator; access decisions and retention changes are themselves audited |
| Settings reads containing secrets | `system.admin` | Existing administrator/redaction behavior |
| Settings writes and go2rtc configuration validation | `system.admin` | Existing administrator checks |
| System information/status/log reads | `system.admin` | Existing endpoint checks |
| Restart, shutdown, clear logs, backup | `system.admin` | Existing administrator checks |
| go2rtc reload and administrative proxy operations | `system.admin` | Existing proxy checks |
| ICE server configuration reads | `system.admin` | Existing endpoint checks |
| Storage health | `storage.configure` for sensitive target details; non-sensitive summary may remain operational read | Existing endpoint checks |
| Storage cleanup | `storage.configure` | Existing administrator checks |
| Event catalog and route CRUD/preview | `events.configure` | Central evaluator; mutation outcomes are audited and writes use optimistic revisions |
| Future event destination profiles, runtime retry, and delivery controls | `events.configure` | Not implemented |

Shared static and smart collections may be referenced directly by grants. The
evaluator resolves their current membership server-side; private collections
are rejected, and referenced collections cannot be made private or deleted.
Collection membership or selector changes increment the policy version.

The durable audit store records every decision made through
`httpd_authorize_action()` / `httpd_authorize_stream_action()`, login outcomes,
policy and role changes, authorization simulation, scoped-token lifecycle, and
audit-retention changes. PTZ writes, evidence protection/retention, recording
deletion, recording export, and event route mutations now add a separate
redacted operation-outcome event; asynchronous jobs preserve only the minimum
principal and correlation context needed to record their eventual result.
Remaining handler migrations must follow the same pattern because an `allowed`
decision proves authorization, not that downstream work succeeded.

Scoped API tokens are accepted only through `httpd_check_action_access()` and
must be followed by central action evaluation for every affected resource.
Handlers still marked “Existing” use `httpd_check_viewer_access()` or legacy
role checks and intentionally reject scoped tokens until migrated. Multi-step
batch-download jobs additionally bind their status/result to the exact scoped
token that created them, not only to the owning user.

## Enforcement rollout order

1. Recording deletion/protection/export and PTZ/audio talk.
2. Stream configuration, ONVIF mutation, locations, tags, and collections.
3. Camera-scoped read/list filtering, including totals and facets.
4. Users, settings, system, and storage administration.
5. New Fleet jobs and event routes enter with action enforcement from their first
   implementation rather than using legacy checks.

An endpoint is not considered migrated until its direct-handler tests prove an
unauthorized request is denied without relying on the web UI.
