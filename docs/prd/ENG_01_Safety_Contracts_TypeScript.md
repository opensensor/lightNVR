# PRD — Engineering Hardening and Permission-Aware API Contracts

**Status**: TS/TSX migration and the first permission-aware API hardening wave
are merged in [PR #804](https://github.com/opensensor/lightNVR/pull/804).
Recording pathname/export integrity is merged in [PR #805](https://github.com/opensensor/lightNVR/pull/805),
and minimatch remediation in [PR #806](https://github.com/opensensor/lightNVR/pull/806).
The recording CodeQL correction is merged in [PR #807](https://github.com/opensensor/lightNVR/pull/807).
The identity/MFA boundary correction merged in [PR #808](https://github.com/opensensor/lightNVR/pull/808).
The session/trusted-device contract and Settings slice is implemented for review;
the remaining MFA/password/logout contract work follows. Rust remains deferred.
**Created**: 2026-10-31
**Reconciled**: 2026-11-06 against main at `7aa0968045cc427e1a1eb018290c1706a83873ef`.
**Owner**: LightNVR maintainers; name an owner when starting each implementation slice.
**Scope**: API contracts, permission-aware discovery, runtime boundary validation,
focused correctness fixes, operational regression coverage, and measured resource costs.

**Original execution baseline**: `dea6094a731c86debb03cc1c75d0251ab3b96cec` (0.46.3 version
bump after the reconciliation baseline). The authoritative task/evidence log is
[engineering-api-contracts-plan.md](../../engineering-api-contracts-plan.md).

**Current follow-up**: [engineering API expansion](../../engineering-api-expansion-plan.md)
and [bounded identity contracts plan](../../engineering-identity-contracts-plan.md).
PR #809 merged the session/device follow-up. Main now contains 37 accepted
operations / 34 paths and 203 residual registered
operations; its bounded acceptance receipt is in the identity plan. Main's forked go2rtc pins `6c311990` including rejected
HTTP response cleanup; the temporary parent patch delivery has been removed.
Main at `b9d3ac61` has successful hosted [Integration Tests](https://github.com/opensensor/lightNVR/actions/runs/42215794378),
[Sanitizer Build](https://github.com/opensensor/lightNVR/actions/runs/42215794273),
and [Static Analysis](https://github.com/opensensor/lightNVR/actions/runs/42215794248).
These historical results predate PR #805. See the expansion plan's security
follow-up receipt for the current correction and its acceptance requirements.

## 1. History and current direction

This PRD originally lived in the unmerged engineering-foundation work at
`e37e791e3ecbeb5849cf48885d12ea3e317e2156`, under this same path.
[PR #693](https://github.com/opensensor/lightNVR/pull/693) was closed as
superseded. Its implementation tracker, generated inventories, and validation
claims are not the current plan or evidence that work landed on `main`.
This revision restores the PRD to `main`'s documentation and replaces its stale
migration sequence with the state of the released code and the next work.

The TypeScript conversion is complete as an application migration. The next
large investment is to make the C API's behavior executable and discoverable:
one canonical OpenAPI contract, an authenticated JSON projection for the current
user or scoped token, and a Swagger-style explorer using that projection.
This serves operators, integrations, and the existing Preact UI immediately.

Delivery remains small, focused PRs based on current `main`. There is no plan to
revive the foundation branch, impose required status checks/branch-protection
rules, or make release progress depend on clearing the deferred CodeQL backlog.
Existing checks continue to run; proposed checks should report meaningful
failures without introducing repository-level merge restrictions.

A possible automated Rust implementation that follows C and reuses the web UX
is **postponed**. Contracts and compatibility tests should be reusable if that
work resumes, but no Rust code, translator, synchronization service, or second
release pipeline is part of the active phases.

## 2. Historical main baseline before PR #804

“Landed” below means present in the inspected source, not comprehensive product
acceptance or physical-device qualification.

| Area | Evidence on main | Remaining work |
| --- | --- | --- |
| Preact TS/TSX application | [#765](https://github.com/opensensor/lightNVR/pull/765), [strict configuration](../../web/tsconfig.base.json), [typecheck command](../../web/package.json), TS-aware lint/Jest and existing localization checks | Runtime contract validation, selected type escapes, root integration/load-test type checking; no second application conversion |
| Current frontend checks | [Static Analysis](../../.github/workflows/static-analysis.yml) runs typecheck and lint; [Integration Tests](../../.github/workflows/integration-test.yml) runs frontend unit tests and three UI shards | Hooks rules remain disabled in [ESLint](../../web/eslint.config.js); typed code does not prove async behavior |
| Native safety fixes | [#694](https://github.com/opensensor/lightNVR/pull/694) health startup; [#695](https://github.com/opensensor/lightNVR/pull/695) migration boundaries; [#698](https://github.com/opensensor/lightNVR/pull/698) backup identity; [#699](https://github.com/opensensor/lightNVR/pull/699) storage publication; [#700](https://github.com/opensensor/lightNVR/pull/700) floor-plan publication; [#702](https://github.com/opensensor/lightNVR/pull/702), [#703](https://github.com/opensensor/lightNVR/pull/703), [#704](https://github.com/opensensor/lightNVR/pull/704) launch/shutdown/disposal | Keep their regression tests; these fixes do not establish whole-process lifetime or recovery correctness |
| Recording recovery and input limits | [#792](https://github.com/opensensor/lightNVR/pull/792), [#802](https://github.com/opensensor/lightNVR/pull/802), [reconciliation tests](../../tests/unit/test_storage_reconcile_incomplete.c); zone validation/editor follow-ups through [#803](https://github.com/opensensor/lightNVR/pull/803) | Qualify interrupted-media handling together with retention, playback, restart, and real media workloads |
| TLS and credential response fixes | [#762](https://github.com/opensensor/lightNVR/pull/762) HTTPS/TLS; [#770](https://github.com/opensensor/lightNVR/pull/770) legacy-key disclosure fix and subsequent user-handler fixes | Model actual supported auth methods and redacted responses in contracts; do not reopen these as missing implementation |
| Authorization foundation | [Central evaluator](../../include/core/authorization.h), action catalog, scoped tokens, and [endpoint inventory](../internal/AUTHORIZATION_ENDPOINT_INVENTORY.md) | Reuse effective action/resource decisions; do not introduce a separate role-name policy engine for documentation |
| CI test selection | [#701](https://github.com/opensensor/lightNVR/pull/701), [shared CTest registration](../../tests/cmake/CiSuites.cmake), explicit `ci-unit`/`ci-sanitizer` labels | Distinguish selected suites from all registered tests; close execution gaps deliberately |
| System health/resource work | [Health PRD](OPS_03_Host_Hardware_Health.md), deterministic replays, real-broker presence tests, and health-specific CPU/heap checks in CI | These are subsystem budgets, not a whole-NVR or browser performance baseline; physical calibration remains separate |
| API specification | [Prose API](../API.md), authorization inventory, manually authored [frontend types](../../web/js/types/streams.ts) | No first-party OpenAPI contract, permission-aware spec endpoint, or generated contract client is present at this baseline |

The application retains two deliberate JavaScript exceptions: generated
`web/js/version.js` and build-time-loaded
[theme-init.js](../../web/js/utils/theme-init.js). `allowJs` is enabled,
`checkJs` is disabled, and `skipLibCheck` remains enabled. The web test tsconfig
covers its TS/TSX tests; it does not cover root `tests/integration`, `tests/load`,
or all existing JavaScript specs.

The latest recording change lists interrupted files with playback explanations;
it must not be summarized only as “hide unplayable recordings,” which describes
an earlier iteration of #802. Normal retention/protection rules still apply.

## 3. Lead deliverable: permission-aware OpenAPI

E1–E3 are implemented and locally accepted by T21. The following requirements
describe the delivered pilot and its expansion rules; the main-baseline table
above remains historical evidence. The accepted T24 contract covers 33
operations across 30 paths; 207 remaining registered operations are tracked in the
[coverage map](../api/contracts/coverage.json).

### 3.1 One canonical contract, personalized discovery

Source path: `docs/api/openapi.json`, using OpenAPI 3.1.1. Exact generator,
validator, linter and renderer versions are pinned in the
[toolchain record](../api/contracts/toolchain.md).
JSON is the served interchange format; avoid maintaining separate handwritten
JSON, YAML, TypeScript, and prose definitions of the same operations.

The canonical contract describes the supported API and all documented response
variants. Generate shared TypeScript types from this full contract at build
time. Do not generate a different client bundle for each user.

Runtime route: authenticated `GET /api/openapi.json`, returning a
valid OpenAPI document projected for the **current request principal**, including
scoped-token restrictions and supported deployment behavior. `/api-docs.html`
loads the projection in a separate read-only session-based explorer.

Examples of intended behavior:

| Principal | Discovery behavior | Request-time behavior |
| --- | --- | --- |
| Viewer of camera A | Read/live operations and applicable redacted response variants; no camera-configuration or system-administration operations | A remains visible; direct requests for B are denied according to existing route semantics |
| Configurator of A only | Camera-edit operation is discoverable with an explicit resource-scope condition | Editing B remains denied; discovery is not an all-camera grant |
| Broadly privileged user with a read-only scoped token | Only capabilities available through that exact token | Owner permissions cannot bypass the token's action/scope limits |
| Administrator session | Supported administrative and camera operations permitted to that session | Normal authorization, input validation, and audit still apply |

This is capability-aware documentation, not a security boundary. A person can
read the repository's full canonical contract. Hiding an operation in the UI or
spec never substitutes for enforcement in its handler.

### 3.2 Share authorization semantics with the C server

Map each covered method/path to a stable `operationId`, handler, authentication
requirements, effective action requirements, resource resolution, response
variants, and tests. Start from
[libuv route registration](../../src/web/libuv_api_handlers.c) and the existing
authorization endpoint inventory, then reconcile frontend consumers.

Document standard authentication mechanisms with OpenAPI security schemes.
LightNVR's camera selectors, action combinations, self-service rules, and token
restrictions need explicit metadata, provisionally `x-lightnvr-authorization`.
OpenAPI supports specification extensions; this extension is a LightNVR
contract, not something Swagger automatically enforces. See the
[OpenAPI specification](https://spec.openapis.org/oas/v3.1.1.html).

Requirements for the projection:

- Reuse the existing policy evaluator and request principal. Do not filter only
  by numeric role, role name, or an unscoped action union. Never accept a client
  supplied user ID as the identity whose schema is served.
- Model global, camera-scoped, authenticated self-service, and public/bootstrap
  operations separately. For multiple required actions, demonstrate that the
  conjunction is satisfiable on the **same resource**, with token restrictions.
- Define discovery for selectors with no current camera matches. Distinguish a
  potential scoped capability from one currently usable; do not enumerate hidden
  cameras, selectors, collection contents, or credentials in the spec.
- Field permissions can differ per camera for the same user. Preserve explicit
  redacted/full or conditional variants; never promise sensitive fields for all
  results because the user can configure one camera. Omit variants impossible
  for the principal and test mixed-permission lists.
- Remove unreachable schemas, examples, tags, links, and other metadata belonging
  only to filtered operations, while preserving all needed references. Every
  projected document must validate without broken `$ref` links.
- Generate from consistent request-local policy state, fail closed on evaluation
  errors, and return `Cache-Control: private, no-store` initially. Do not share
  personalized responses through public caches or store token secrets in examples.
  A later cache requires explicit revocation and isolation tests.
- Reflect policy changes, user deactivation, token expiration/revocation, and
  feature availability on subsequent discovery requests. An already loaded spec
  grants nothing; the API reauthorizes every operation.
- Keep documented enforcement gaps explicit. In particular, discovery must not
  advertise `audio.listen`/`audio.talk` isolation as solved while the direct
  go2rtc signaling boundary remains open in Fleet 02.

### 3.3 Streams pilot and contract hardening

Start with the existing list/edit workflow:

- `GET /api/streams`, including the summary query actually used by the UI.
- `GET /api/streams/{name}/full`, including permitted/redacted variants.
- `PUT /api/streams/{name}`, including validation, denied writes, and outcomes.

Also specify the discovery endpoint itself and its authentication/error behavior.
Explicitly label pilot coverage; an empty or partial personalized spec must not
imply that all other APIs are unavailable. Keep public/bootstrap behavior
documented in the canonical contract; personalized discovery initially requires
authentication. Auth-disabled/demo deployment behavior needs a deliberate,
tested policy, not accidental anonymous access to the administrative projection.

Capture request and response behavior before introducing generated consumers:
absent versus null, defaults and zero values, enum/number limits, camera names
and encoding, pagination/totals, retention inheritance, error bodies, and
authorization/redaction. Use the current server as evidence, but independently
review expected semantics rather than generating both implementation and test
expectations from the same C code.

Add small runtime validators at the pilot's network boundary. A generic
`fetchJSON<T>` or `as Stream` is not validation. Distinguish malformed data,
authorization denial, transient transport failure, and cancellation. Preserve
compatible additive fields and show a useful error for invalid required data.
Do not change wire formats just to simplify the schema.

Run tests against a freshly built C server and isolated fixture data. Cover
success, malformed input/output fixtures, invalid identifiers, empty results,
unauthenticated/forbidden access, mixed camera scope, token restrictions, and
negative side effects: a denied or invalid edit must leave configuration intact.

**Pilot exit:** Valid canonical and personalized JSON; viewer/configurator/admin
and restricted-token cases agree with actual API behavior; generated TS types
are consumed by the Streams flow; malformed fixtures fail runtime validation;
schema/type drift is detected; the UI and wire behavior remain compatible.

### 3.4 Swagger-style explorer and expansion

Serve the explorer from the same deployment, loading `/api/openapi.json` through
the same authenticated context. Keep its assets off normal Live/recording page
loads and measure installed asset cost. A session-bound document must not be
presented as a token's capabilities after switching credentials; refetch on
identity changes and discard prior privileged data on logout.

If interactive requests are offered, use the real authorization, request-origin,
and CSRF behavior required by the API. Do not persist bearer secrets in browser
storage or introduce a privileged proxy to make the explorer work. Support the
deployment's reverse-proxy base path, and use synthetic examples rather than
live camera URLs or credentials. The JSON endpoint must be useful without the
explorer.

Expand by product risk: authentication/client bootstrap and tokens;
recordings/timeline/protection/deletion/export; storage; then fleet, events,
ONVIF, and remaining settings/system operations. Keep an operation coverage map
with explicit unsupported/deferred entries. Media ranges, HLS and signaling
need protocol tests alongside the JSON schemas.

## 4. Focused correctness work alongside the pilot

The type conversion deliberately preserved behavior. It exposed candidate bugs
that must be reproduced and fixed separately, not hidden with more type casts.
The following table records baseline source findings. T9–T14a subsequently
reproduced and repaired them with focused browser regressions; it is not a list
of defects still present in the accepted working tree.

| Priority | Current evidence | Focused acceptance |
| --- | --- | --- |
| Recording/live recovery | [HLSVideoCell](../../web/js/components/preact/HLSVideoCell.tsx) has three `@ts-expect-error` sites reading `go2rtcEnabled` outside its scope | Trigger each relevant error path; recovery/fallback works without a reference error; remove those suppressions |
| Configuration integrity | [SettingsView](../../web/js/components/preact/SettingsView.tsx) uses `default_detection_threshold || 50` | Save/reload/save a valid zero without converting it to 50 |
| Correct write operation | [StreamsView](../../web/js/components/preact/StreamsView.tsx) retains `isEditing` across close and ONVIF discovery | Edit, close, then onboard a camera; verify create rather than stale edit behavior |
| Transient auth failures | [auth-utils](../../web/js/utils/auth-utils.ts) maps fetch errors to invalid sessions | Distinguish temporary network/5xx failure from authentication rejection; preserve secure denial behavior |
| Async state | [query-client](../../web/js/query-client.ts) explicitly types unavailable `isPending` as undefined | Callers use supported pending state; duplicate actions and failure cleanup are exercised |

Start with the smallest reproducible cases and retain focused regressions.
The legacy API-key response leak from the conversion notes was already addressed
by #770 and later follow-ups; do not treat the historical list as current truth.
Other candidates, including timeline identity across retention/polling and
batch-job failure/cancellation display, need their own current-source triage.

## 5. Operational confidence and C boundaries

Build on existing fixtures rather than inventing another broad test framework.
Map each scenario to the command/job that executes it and record build flags.
The current general Playwright lane runs `--project=ui`; dedicated health tests
exercise selected API specs, but this is not execution of the complete API
project. Add an explicit API-contract lane for the pilot and expand deliberately.

The operational scenarios covered by T25–T28 are interrupted recording reconciliation with
playback, full/read-only/unavailable storage, retention of protected media,
disconnect/reconnect, concurrent stop/restart, and upgrade from a declared prior
release. Their bounded workstation evidence is recorded in the execution plan;
physical-device and long-soak qualification remain separate. Assert playable media and metadata together, not merely HTTP success
or a database row marked complete. Preserve evidence for files classified as
interrupted/unplayable; do not silently discard footage to satisfy a check.

For each scenario specify injected fault, expected state transitions, bounded
completion/recovery time, media/timestamp tolerance, resource observations, and
retained logs before calling it accepted. Use temporary data and synthetic
streams. Qualify mocks, real C/media tests, and field-camera results separately.
Longer soaks and physical-device qualification remain separately scheduled work,
not an implicit prerequisite for the contract pilot.

Refactor C ownership only where failures or repeated changes justify it. Before
changing a selected boundary, record allocation/free responsibilities, callback
lifetime, thread affinity, lock ordering, cancellation and shutdown invariants.
Keep schema changes, lifecycle refactors and unrelated frontend work separate.

Measure default/modern frontend assets and the actual deployment profiles before
setting numeric budgets. Reuse health-specific benchmarks without claiming they
cover whole-NVR RSS, CPU, descriptors or recording gaps. The API pilot must report
its JSON projection cost, validator/client bytes, and separate explorer bytes;
physical hardware numbers must not be inferred from a container build.

## 6. Revised delivery sequence

The old P0–P6 dependency chain assumed TS had not started and required findings
gates had been approved. It is superseded by the sequence below.

| Slice | Deliverable | Dependency | Completion evidence |
| --- | --- | --- | --- |
| E0 — Baseline reconciliation | This restored PRD and current implementation/CI map | Current main | Source and merged-PR references; clear verification limits |
| E1 — Streams contract | Small canonical JSON contract, operation/action/resource mapping, independent C API tests | E0 | Real responses and negative mutations match schemas and policy; exact tool versions recorded |
| E2 — Permission-aware discovery | Authenticated JSON projection; scoped tokens, conditional fields, reference pruning, revocation/isolation tests | E1 | Valid documents across the principal matrix; direct denied calls remain denied; no hidden-resource data |
| E3 — Typed pilot and explorer | Deterministic generated TS types, runtime validation, Streams adoption, separate docs UI | E1; explorer needs E2 | Drift checks, UI error-state tests, identity switching, asset/projection measurements |
| E4 — API hardening expansion | Auth and recordings first; explicit contract execution; root test typing in a separate slice | Pilot accepted | Coverage map, real-server compatibility tests, stable wire behavior |
| E5 — Recording/recovery qualification | Fault/upgrade tests and justified C boundary changes | E0; behavior tests before refactoring | Playability, protection, recovery and resource evidence under declared faults |
| Deferred — Rust experiment | Separately scoped alternate implementation and synchronization evaluation | Explicit future decision | Shared UX/API compatibility, C revision provenance, divergence reporting and target cost evidence before any shipping claim |

Focused bug fixes from section 4 can land alongside E1–E3. E5 starts from current
fixtures and does not wait for the entire API to be specified. Each slice is
independently reviewable and valuable even if Rust never resumes.

## 7. Verification record and limits

### Final integrated acceptance (T29)

The executed plan accepts 33 operations across 30 paths, with all 240 literal
method/route/handler registrations reconciled and 207 explicitly left for later.
E1–E3, the bounded E4 identity/recording increment and E5 workstation scenarios
are implemented. This does not mean the entire API has contracts. The
[next plan](../../engineering-api-expansion-plan.md) separates remaining domains,
recording pathname collisions, export I/O handling and target capacity. Rust
remains deferred.

Final Release `fadd75c4…` passes 50 real-server API tests, 13 real-browser contract
tests and 83 `ci-unit` targets. The final managed Debug `cb68e8b3…` passes 58
`ci-sanitizer` targets with ASan/UBSan/leak detection. Web tests pass 68 suites /
784 tests; 33 focused browser cases cover HLS, zero settings, create-after-edit,
mutation pending and session failure behavior. Root/contract/web typing, lint,
schema fixtures, generator checks and drift pass. Exact hashes, build flags,
source/test manifests and local artifacts are in `build/eng-api-T29/acceptance.json`.

The recording tests exposed and repaired historical-name authorization errors:
captured UUIDs now govern reads, counts, facets, timeline enrichment and related
mutations; legacy NULL/empty UUIDs preserve current-name fallback. Mixed
unauthorized fixed batches fail without mutation/job creation, and database
failures cannot become successful empty jobs. Migration 0092 indexes the measured
legacy-label fallback. Independent review and real-file denial assertions verify
these changes; a personalized spec alone would not have repaired the handlers.

All four final managed-media reports pass: fresh crash/restart reconciliation
without timestamp aging; full/read-only/permission/unavailable storage; bounded
disconnect/reconnect and concurrent lifecycle work; and an actual 0.45.3 copied
installation upgraded from 0089 through 0092 with the original untouched. Media
decode, protected hashes, durable IDs and database integrity are checked together.
The go2rtc response-close repair was initially delivered as a parent patch;
main now pins the repair and regression in the maintained go2rtc fork; libuv shutdown keeps finite in-flight handlers alive until
their work completes. Arbitrarily blocked handlers and longer device soaks are
outside these bounded qualifications.

Final 0–1,024-camera projection measurements and the 16/1,024 repeat pass. Snapshot
statement counts stay flat with nonempty fleet size, while broader operations
increase fixed SQL work, JSON bytes, latency and validator bundle size. Process
high water is 267.90/267.98 MiB; default/modern installed assets are 14,946,302 /
10,925,296 bytes. Normal pages load no Swagger vendor files in either profile.
See [performance evidence](../api/contracts/performance.md) for distributions,
methodology and the remaining metrics-buffer/unused-validator costs.

The T29 review identified two follow-ups: second-resolution pathname reuse and
unchecked ZIP I/O failures. F12/F16 now address these in the active follow-up PR;
its acceptance and limits are tracked in the expansion plan. The T29 tests proved
detected job errors and successful archive integrity, not universal I/O failure
atomicity. Physical cameras, active-media capacity and remote archive/HEVC
qualification remain outstanding. Hosted results are recorded at the top of this PRD. Contract CI now
explicitly runs API/browser cases; a manual reliability workflow reproduces the
four operational scenarios and uploads only sanitized status. No repository
merge restrictions or CodeQL backlog gates were added.

### Working-tree pilot acceptance

T21 locally accepts the four-operation Streams/discovery pilot on the integrated
source. Fresh Release binary `08323d6a472f34bbcbdfcdf397d2ab0438378a1f4437e3c42c7fdb384f53e02b`
has native source manifest `68a80fb0bc4578c86d12e08ae399c3805e27c1f2aa6c4e8ca590e787a3f9ed68`
in `build/eng-api-T20a/evidence.json`. It passes 82 `ci-unit` CTests, 24 real-server
API tests and 13 real-browser contract tests with zero retries. The current Debug
targets pass 57 `ci-sanitizer` tests with ASan/UBSan and leak detection.

Frontend checks pass all 68 suites / 784 tests, typecheck and lint; 20 focused
Streams/session browser regressions also pass. Canonical/projected schema lint,
deterministic generation/drift and negative tooling checks pass. Both production
profiles were built and measured. The new CI lane explicitly executes contract
API and browser projects, with credential-free status artifacts; these are local
results, not an observation of a hosted run on the working-tree changes.

T9–T14a repaired the reproduced HLS scope, zero-valued setting, stale create/edit
state, mutation pending state, transient session handling and per-camera policy
control defects. The table in section 4 records their baseline symptoms.
The explorer loads no vendor assets on normal pages; shared utility/rechunk
increases are measured explicitly rather than called zero. T20a reduces camera
snapshot storage from 65.98 to 22.09 MiB at 1,024 cameras; full-process observed
high water drops 476.72 to 264.22 MiB in the declared workload. See
[performance evidence](../api/contracts/performance.md) for repeat distributions,
remaining metrics-buffer cost and device-sizing limits.

That T21 receipt accepts E1–E3 only. Later task receipts track E4/E5 expansion
and final integrated reruns. The proposed [follow-on plan](../../engineering-api-expansion-plan.md)
separates remaining API domains, pathname collision safety and target capacity.
Rust remains deferred.

### Original main reconciliation

For this reconciliation, `origin/main` was fetched and matched local HEAD at
`8455d005`. Local `npm --prefix web run typecheck` passed, and
`npm --prefix web test -- --runInBand` passed all 64 suites / 711 tests.
Relative documentation links and `git diff --check` also passed. Source inspection
covered the original PRD, merged history, current workflows, authorization
interfaces, typed fetch boundaries, and the named defect candidates.

At the final CI observation for the reviewed HEAD,
[Sanitizer Build](https://github.com/opensensor/lightNVR/actions/runs/42041589382),
[Static Analysis](https://github.com/opensensor/lightNVR/actions/runs/42041589419),
and both CodeQL runs had passed.
[Integration Tests](https://github.com/opensensor/lightNVR/actions/runs/42041589413)
and [Container Security Scan](https://github.com/opensensor/lightNVR/actions/runs/42041589516)
were still in progress. This is a dated observation, not a claim that all checks
passed or that hosted results covered these documentation edits.

clang-tidy, cppcheck and container vulnerability findings remain reporting-only
in the inspected workflows. A successful run is not proof of an empty findings
backlog. Deferred alerts were not re-triaged for this documentation update.
No native rebuild, camera soak, physical benchmark or future contract tests were
run as part of that original restoration; subsequent local execution is recorded
above and in the task log.

## 8. Related work

- [Fleet 02: Scoped Authorization and Audit](FLEET_02_Scoped_Authorization_Audit.md)
  and [endpoint inventory](../internal/AUTHORIZATION_ENDPOINT_INVENTORY.md).
- [API reference](../API.md) and [event contract](../EVENT_CONTRACT.md).
- [Storage lifecycle](STORAGE_01_Multi_Target_Lifecycle.md),
  [archive](STORAGE_02_Hot_Cold_Archive.md), and
  [backup/restore](OPS_01_Backup_Restore.md): consumers of operational contracts,
  not prerequisites for starting the Streams pilot.
- [Host health](OPS_03_Host_Hardware_Health.md) and
  [Live ergonomics](UXD_02_LiveViewErgonomics.md): preserve existing behavior and
  reuse diagnostics/fixtures.
- [Superseded foundation PR](https://github.com/opensensor/lightNVR/pull/693)
  and [landed TypeScript conversion](https://github.com/opensensor/lightNVR/pull/765).
