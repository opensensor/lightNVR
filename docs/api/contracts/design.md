# Permission-aware API contract design

Execution baseline: `dea6094a731c86debb03cc1c75d0251ab3b96cec` (2026-11-05).
The difference from the PRD's `8455d005` is the 0.46.3 version bump only.
This document freezes the initial E1–E3 interfaces. Later changes require a
coordinator handoff to all consumers; it is not evidence of implementation.

## Pilot operations

| operationId | Method/path | Eligibility and resource facts |
| --- | --- | --- |
| listStreams | GET /api/streams | Authenticated real principal; list remains discoverable with zero matching cameras. Responses filter on `live.view`, and optionally the LPR surface, before totals. Summary/compatibility variants are separate schemas. |
| getStreamFull | GET /api/streams/{name}/full | At least one camera satisfies `live.view` using raw DB facts. Sensitive response variant additionally requires `camera.configure` on that same camera using runtime-enriched facts. |
| updateStream | PUT /api/streams/{name} | At least one camera satisfies `camera.configure` using raw DB facts. This does not imply read permission. Input/private-mode variants preserve the existing handler. |
| getOpenApi | GET /api/openapi.json | Authenticated real principal. No camera permission required. |

Trace: registrations are in `src/web/libuv_api_handlers.c`; list/full serialize
in `api_handlers_streams_get.c`; update uses `check_stream_write_access` in
`api_handlers_streams_modify.c`. `httpd_authorize_stream_action_with_context`
in `httpd_utils.c` resolves DB camera facts before evaluating authorization.
`authorization_evaluate_in_context` in `src/core/authorization.c` is authoritative.

The list handler enriches runtime and go2rtc health before authorization. Full
read authorizes `live.view` before runtime enrichment, and checks configure
permission after runtime enrichment. PUT checks raw DB facts. This existing
inconsistency is documented, not changed by discovery. A health-dependent
grant can therefore permit a list entry while forbidding a direct full read.
Tests must model those operation-specific observations explicitly.

## Canonical metadata, version 1

Every covered operation has `x-lightnvr-authorization`:

```json
{
  "version": 1,
  "kind": "camera",
  "requirements": [
    {"action": "live.view", "resource": "raw"}
  ]
}
```

Kinds are `authenticated`, `camera`, `global`, `public`, and
`non_scoped_identity` (T22 additive handoff) and `identity_management` (F1d). `camera` means
there exists one current camera satisfying **every** requirement, including
token restrictions. Different resources in requirements select raw/runtime/live
observations of that same camera. `global` evaluates all requirements with NULL
camera. `authenticated` needs no action and permits useful empty-list discovery.
`public` describes public operations when added to the canonical contract; it
does not make discovery itself public. Requirements are mandatory/nonempty for
camera/global and absent for authenticated/public/non_scoped_identity/identity_management. Unknown keys, kinds, versions,
action names, or resource values are errors, not permissive defaults.

Resource names are `raw` (DB), `runtime` (one captured metrics enrichment), and
`live` (runtime plus one bounded go2rtc activity enrichment). A global requirement
uses `raw` as a placeholder but evaluates NULL. The projector must not infer
scope from role numbers/names or a union of action bits.

Response alternatives may carry `x-lightnvr-visibility` with the same predicate
shape. Prefer annotations on `anyOf` alternatives referencing reusable models.
An alternative includes the operation's read requirements plus any additional
configure requirements, so field access on camera B cannot be combined with
read access on camera A. Redacted models remain available when full models are
eligible, because different rows/resources can have different permissions.
Use `anyOf` rather than overlapping `oneOf` variants unless exclusivity is proven.
Full schemas must describe actual redaction (empty strings/absence), not imply
secret values are returned to every privileged reader.

Projection removes ineligible operations/alternatives, then traverses surviving
local references to retain needed components. Decode JSON Pointers including
`~0`/`~1`, handle cycles, and handle security-scheme name references and supported
discriminator/link references. Reject unsupported dynamic/external references.
An unexpectedly empty required alternative is an error; never widen a schema
to `{}` to make projection succeed. Remove unused tags/examples/links together
with components. Strip internal predicate metadata from served output; retain
only public conditional-scope descriptions and coverage metadata.

Top-level `x-lightnvr-coverage` reports `partial: true`, the canonical revision,
covered operation IDs, and the fact that absence is not a statement about
unspecified APIs. Do not include user/token identifiers, camera names, selectors,
grant IDs, raw URLs, live examples or credential values. Keep stable operation
IDs across personalized views. Every resulting document must independently lint.

## Principal and deployment policy

Use `httpd_check_action_access`, preserving session-before-Basic-before-key/token
resolution and CIDR/password-change restrictions. There is no requested-user
override. Return 401 for no valid principal. Return 403 for synthetic
`authentication_method` values `auth_disabled` or `demo`; these modes still have
their existing API behavior. An authenticated deployment with no accessible
cameras returns a valid partial document with list/discovery operations.
For a real authenticated user, demo mode does not introduce a blanket Streams
mutation restriction: `check_stream_write_access` still evaluates
`camera.configure`. The demo-specific branches in `httpd_utils.c` provide an
anonymous viewer and bypass the forced-password-change gate; they do not deny
authenticated camera configuration. Preserve the real user's action decisions
in discovery. T16 must confirm this over HTTP rather than treating every demo
deployment as read-only.

Return JSON errors and `Cache-Control: private, no-store` for discovery successes
and failures. No application/shared cache or ETag in the pilot. Invalidated
tokens/users are rejected on the next request. A previously downloaded spec is
never authorization to perform an operation. An evaluation/serialization error
returns 500 with a generic message, never the canonical document.

The canonical server URL is `../`, relative to the served `/api/openapi.json`,
so a proxy-mounted `/prefix/api/openapi.json` resolves to `/prefix/`. The explorer
fetches relative to its own page directory and resolves that relative server URL
against the actual spec URL before passing an object to Swagger. It does not
trust arbitrary forwarded host/prefix headers. Prefix behavior of unrelated
legacy pages is not expanded by this pilot.

## Snapshot implementation seam

Use one connection returned by `db_open_readonly_connection` with a bounded
busy timeout, explicitly `BEGIN`, then force the initial read. Load principal
activation/mode, policy version, grants/role actions, token scope/expiry,
collections/members and fleet/location/tag facts through **that connection**.
Existing functions open independent connections or take the global DB mutex;
calling them unchanged does not satisfy this requirement.

Introduce narrowly scoped `_on_connection` read helpers by extracting existing
queries. Ordinary APIs retain current wrappers/locking. The evaluator context
gets an opt-in borrowed read connection and fixed observation time; its grant,
collection and token loads use those helpers, and its decision algorithm remains
unchanged. No new role/scope interpreter. The snapshot owns connection/context,
and closes the read transaction before final destruction. An eager materialized
context may close earlier only after all required data has been captured.

Inventory has raw, runtime and live views captured once. Runtime sampling occurs
without the global DB/metrics lock held and without per-camera network calls.
The SQLite read transaction may remain open, but no write transaction/global
mutex spans external activity. Time-bound external collection and record the
sampling interval internally. This is a coherent DB observation plus bounded
runtime observations, not atomic state across physical devices. Unavailable
health stays UNKNOWN according to existing helpers; do not manufacture UP.

Refresh `is_active`, role and authorization mode from the same DB observation;
carry only authenticated credential identity from the HTTP principal. Check
token ownership/revocation/expiration against the fixed observation time. Do not
load password hashes/TOTP secrets into the snapshot. A small principal SELECT
in the snapshot loader is sufficient; do not reuse a broad credential SELECT.

Proposed stable native interface (exact C names to be retained by T5/T6):

```c
typedef struct authorization_snapshot authorization_snapshot_t;
/* 0 success, -2 principal no longer valid, -1 internal failure. */
int authorization_snapshot_create(const user_t *, authorization_snapshot_t **);
void authorization_snapshot_free(authorization_snapshot_t *);
int authorization_snapshot_camera_count(const authorization_snapshot_t *);
/* camera_index=-1 for global; resource is raw/runtime/live. */
int authorization_snapshot_evaluate(authorization_snapshot_t *,
    authorization_action_t, int camera_index, const char *resource,
    authorization_evaluation_t *);
/* Predicate callback returns 1 eligible, 0 ineligible, -1 error. */
typedef int (*openapi_access_fn)(void *, const cJSON *predicate);
int openapi_project(const cJSON *canonical, openapi_access_fn, void *, cJSON **out);
```

T6 owns metadata validation/projection and `openapi_snapshot_access`, the small
adapter combining snapshot evaluations for same-camera conjunctions. T7 passes
its request-owned snapshot through that callback. T5 tests evaluator consistency
independently; T6 uses controlled callbacks and exports **actual C output** for
schema validation. T7/T16 prove their composition against actual HTTP calls.

The native projector supports component schemas, responses, parameters, examples,
request bodies, headers, security schemes and links. Named schema maps, including
`definitions` and `$defs`, and schema-valued 2020-12 keywords are traversed;
literal examples/defaults/enum values and Link parameter/body payloads are data.
Unknown schema keywords/dialects, dynamic/external references, path-item refs,
callbacks and webhooks fail closed. JSON Pointers may traverse named objects,
including escaped property names, but may not index arrays: filtering a union
would otherwise silently change what an array index denotes. Both the complete
canonical input and final pruned result are checked. Actual permission metadata
is removed while literal properties with the same names remain intact.

The required extraction is bounded to existing grant, collection, token and
fleet read queries. If it needs a general DB routing layer, global thread-local
DB replacement, or rewrite of all handlers, stop T5 and split the work. Never
call public lock-taking DB functions while holding their mutex recursively.

## Tool and frontend boundaries

`contracts:generate`/`contracts:check` take `--target=native|web` or both by default.
Native output: `include/web/openapi_document_generated.h`, C byte array plus
length/revision, deterministic Python generation. Web output:
`web/generated/api/schema.d.ts`, `validators.js`, `validators.d.ts`; validator
exports keyed by named component schemas, accepting unknown and returning a
boolean type guard with sanitized diagnostics. Production validators fail fast
on invalid payloads (`allErrors: false`); diagnostics are not an exhaustive error
report. Union branches may still yield multiple errors. Validation never coerces
or mutates the response. T3 may refine filenames only
with an explicit handoff before T8. No handwritten files in generated directory.
Types alone do not validate JSON. Do not relax `web/js` escape restrictions.

T8 supplies list/full/update adapters in `web/js/api/streams.ts`, preserving
abort signals, error status and current body semantics. Caller form state stays
separate. Validators preserve additional properties, reject invalid required
fields, and do not coerce values/default missing values or log payloads.

The explorer is session-only/read-only. Hide credential-entry controls, disable
all submission methods, remote config/ref loading and external validation.
Discard an earlier identity's in-flight response using an incrementing request
generation plus AbortController. Clear the document on identity/lifecycle change
before refetch. No spec in persistent browser storage or static HTML.

## Qualification boundaries

Synthetic JSON fixture startup deadline: 30 s; request deadline: 10 s; graceful
owned-process stop: 15 s then owned PID termination, retaining diagnostics.
Media tests use 60 s startup/reconnect, 30 s shutdown and fixture GOP-dependent
tolerances declared in each result. These are test harness bounds, not production
SLOs. Any relaxation requires recorded failed evidence and a reason.
Do not truncate media or delete protected files to satisfy a deadline. Record
source HEAD plus tested-file/diff digests because implementation is uncommitted.

Test scope is in `principal-matrix.json`. Contract validity, permission behavior,
side effects and resource measurements are independent checks. Unknown input,
DB errors and failed snapshots fail closed. Physical performance and general
Rust compatibility are not claimed by these tests.

## API explorer entry

Open `/api-docs.html` directly, or `/prefix/api-docs.html` when the application is
mounted under a reverse-proxy prefix. The page resolves verification, discovery,
locale catalogs and its standalone Swagger UI 5.34.1 assets under its own page
directory. It does not trust forwarded host/prefix headers or accept a spec URL
from the query string. Normal Live, Streams and recordings entries do not load
the explorer vendor assets.

The explorer uses the current browser session only. It has no credential-entry,
Authorize or Try-it-out controls, no request submission, remote configuration,
remote schema references, external validator or persistent spec/authorization
storage. It displays partial-coverage metadata and explains that unspecified
APIs are outside this reference, rather than declaring them unauthorized.

The rendered document is cleared before revalidation and on logout, hiding or
leaving the page. Focus, pageshow and account-storage changes reload permissions.
Nonpersistent `lightnvr-auth` BroadcastChannel messages clear other tabs on
`clearAuthState`; visible pages also revalidate every 30 seconds to cover
cookie-only logout/account changes without a browser event. A monotonic request
generation, abort disposal and identity checks on both sides of spec loading
prevent an earlier account's delayed response from rendering. Each refresh is
bounded to 10 seconds; a network or validation failure leaves no prior spec.

## T22 identity/token extension

`non_scoped_identity` is a simple version-1 predicate with exactly `version` and
`kind`, evaluated from the snapshot's authenticated scoped-token flag. It reveals
no identity, token or owner ID and does not infer permission from role. Real
session, Basic and legacy-key identities qualify; scoped tokens do not. This
matches `/api/auth/verify`'s non-scoped resolver and token management's explicit
scoped-token rejection. Every non-scoped real principal can target their own
tokens; another owner's tokens still require `users.manage` in the handler.
Visibility does not authorize arbitrary path IDs or requested token authority.

The eight added operation IDs are `getLoginConfig`, `login`, `logout`,
`verifyIdentity`, `getClientConfig`, `listUserTokens`, `createUserToken`, and
`revokeUserToken`. Login/config/logout use `public`; client-config uses
`authenticated`; verify and token lifecycle use `non_scoped_identity`. Public
operations in a projected contract do not change discovery's real-principal
authentication requirement. Token secrets exist only in successful creation;
ordinary token metadata/list validators explicitly reject secret/hash fields.
Separate legacy key security schemes prevent implying scoped-token support on
verify/token-management operations. Existing session precedence is unchanged.

## T23 recording reads and captured identity

The nine recording-read operations cover list/detail, tag/detection-label
facets, per-recording tags, timeline segments/by-IDs, playback and thumbnails.
Filtered collections remain discoverable for authenticated principals with no
replay cameras; direct recording/timeline/media operations require a current
camera satisfying `recordings.replay` using raw facts. The complete operation
list remains in the canonical contract and coverage map.

A recording's nonempty captured `camera_uuid` is authoritative. Reusing its
historical stream name cannot transfer access to another camera. Current-name
filtering selects authorized camera UUIDs from the same loaded fleet and carries
them into list/count/facet queries. Renamed authorized captures remain visible.
Only NULL/empty captured UUIDs use current-name fallback; no historical UUID is
guessed for such legacy rows. Direct access to a deleted captured camera returns
404. Count-cache identity includes the current UUID/name mapping for legacy rows.

T23a applies the same identity resolution to detection enrichment, all public
timeline branches and related protection/deletion/tag/export/file boundaries.
Otherwise-valid fixed batches containing unauthorized members fail before any
mutation or job creation. Existing early operational 404/409 aborts can precede
authorization of later members when the whole batch has no effects. Filtered
delete workers receive server-captured UUID scope; client JSON is not authority.
SQLite errors propagate instead of becoming partial rows or successful empty
jobs. Migration 0092 indexes the NULL/empty-UUID label fallback separately from
modern captured detections.

JSON schemas do not validate playable bytes. Real-server tests use generated
H264 MP4/JPEG, ffprobe/decode, byte comparisons and reassembled ranges. Current
range behavior, including duplicate `Accept-Ranges` values, is documented as
observed compatibility behavior. Archive/HEVC preparing branches are modeled
from source; the local H264 fixture does not qualify those remote/transcode paths.

## T24 recording actions and jobs

Twelve operations extend coverage to protection, retention overrides, tags,
deletion, direct downloads and batch export/status/result. Protection/tag/retention
operations require `evidence.protect`; deletion requires `recording.delete`;
downloads/export creation require `recordings.export`, using raw camera facts.
Status/result operations are discoverable for authenticated principals because
an existing job can outlive the current camera scope. Their handlers separately
enforce creator identity and exact scoped-token binding; an owner's ordinary
session or another token cannot substitute for the token that created a job.

The source-defined states are pending/running/complete/error, with the deletion
fallback `unknown`. Tests observe running/not-ready, complete and explicit errors;
they do not force a pending-state observation. These routes expose no cancel
operation. A filtered delete database failure produces an error job; zero matches
produce successful empty completion. Unauthorized mixed fixed batches create no
job or mutation. Otherwise authorized jobs can still report partial operational
failures; this is not a guarantee that every error is atomic.

Request schemas preserve existing numeric coercions and IDs-versus-filter
precedence. Batch-tag counters count successful per-recording SQL operations,
including ignored inserts and deletes of absent tags, rather than changed rows.
Tests also compare the actual final tags and recording metadata.

Real HTTP tests validate ZIP entries with Python's independent CRC/content
checks and decode the extracted video. Missing-source/member-count failures are
detected, but the current writer has unchecked read/write/close paths. A complete
status is not proof against every filesystem fault. The follow-on plan's F16
requires a separate reproduction and focused repair; this contract does not
claim that limitation is fixed or qualified by the successful archive fixture.


## Session and trusted-device self-service extension

The four list/revoke operations use `non_scoped_identity`, matching the actual
handler's session/Basic/legacy-key authentication rather than administrative role
names or a scoped token's users.manage permission. Real owners manage only their
own credentials; synthetic identities are rejected. Their unreachable schemas
are pruned from scoped-token discovery. The partial inventory is 37 operations,
34 paths and 203 residual registered operations.

Lists retain a documented maximum of 32 entries. Revocation uses an atomic,
owner-filtered delete that identifies the removed cookie token, independent of
list position or expiry. Pending MFA rows are excluded. Strict decimal safe-
integer IDs keep native routing and generated JavaScript callers consistent.
The Settings consumer validates redacted lists and exact success acknowledgments,
never automatically retries DELETE, and refetches after ambiguous failures.
See the [identity acceptance receipt](../../internal/plans/engineering-identity-contracts-plan.md)
for validation, credential-class decisions and remaining MFA/password scope.


### MFA/password identity management (F1d)

`identity_management` has exactly `version: 1` and `kind`. It permits discovery
for a non-scoped personal principal, or a principal with effective global
`users.manage`, including scoped owner/action/scope intersection. Native projection
uses the same authorization snapshot evaluation as other global actions; failures
abort projection. This does not grant arbitrary target-user access: handlers
restrict self-service to the current account and require `users.manage` otherwise.
The password request's administrative reset alternative is visible only to an
effective manager. Forced first-login self changes still require the old password.
Basic and legacy-key compatibility is preserved.

Seven added operations cover MFA completion, four enrollment/status operations,
password change and legacy GET logout. Logout documents negotiated JSON and empty
302 redirects with cookie clearing. Login's challenge and success variants are
mutually exclusive. Setup alone exposes the secret; status/acknowledgments reject
extra fields. Typed browser adapters validate every success, redact failures and
never automatically retry mutations. Unavailable status cannot imply disabled MFA.


## Fleet reads and selectors (F2)

Thirteen added operations bring the partial inventory to 57 operations across
54 paths, with 183 registered operations remaining. Fleet queries, selector
previews, saved views, collection reads and location/tag catalogs are discoverable
by authenticated principals, including empty camera scopes. Camera tag assignments
require raw `live.view` discovery; each concrete handler checks the target camera.
Assignment writes require both `camera.configure` and `live.view` on that camera
before mutation because their success response includes the resulting assignments.
Discovery does not promise a particular row or grant access through stored selectors.

Rows, facets, counts and organization trees use the caller's visible inventory.
Conjunctions apply to one camera, and token authority intersects its owner. Shared
collections expose only visible members/counts; smart selectors are redacted from
non-owners. Private and empty foreign collections are hidden. Saved views have
owner/shared visibility, while applying a saved selector still filters current
camera authority. Queue counts describe the authorized inventory before search
filters; they are not page-local totals. Runtime selector depth/node/byte bounds
remain enforced by the native evaluator.

Generated guards validate all thirteen adapter responses before consumers render
them. Query/preview POSTs are read-only, bounded and retryable; this does not change
mutation retry policy. Errors omit raw bodies, and catalog/query failures offer
Retry. The backend also fixes hidden-camera totals in tag-assignment GET/PUT
responses and a SQLite iteration guard that returned only the first saved view.
