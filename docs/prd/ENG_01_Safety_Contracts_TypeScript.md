# PRD — Security, Quality, and Typed Frontend Foundation

**Status**: Draft — scoped for implementation; phase acceptance remains open
**Created**: 2026-10-31
**Baseline**: `main` at `1b8d4332`, after PR #691 merged the Live views
**Owner**: LightNVR maintainers; assign an accountable owner to each delivery slice
**Priority**: Product reliability and maintainability
**Scope**: Security and quality remediation, enforceable CI checks, Preact
TypeScript/TSX adoption, executable API contracts, operational regression
coverage, embedded resource budgets, and targeted C boundary improvements.

## 1. Problem and intended outcome

LightNVR needs a stronger foundation for routine development and larger future
refactoring. Existing analysis, tests, and documentation provide a substantial
starting point, but several analysis jobs only report findings, frontend application
code has no dedicated type-check gate, and the API contract is primarily prose.
These gaps make it easier for changes to introduce mismatched data assumptions,
repeat resolved defects, or increase resource costs unnoticed.

The next investment is to improve the current C backend and Preact frontend.
Every phase must provide independent value to the released product. A future
Rust implementation may reuse the resulting contracts and tests, but this PRD
does not depend on a language migration.

Preact remains the frontend runtime. TSX adds TypeScript to JSX; it does not
require replacing Preact with React. Type annotations are removed during the
build. Preserving the embedded footprint still requires measuring the emitted
assets, dependencies, and runtime behavior.

## 2. Current baseline

The following describes the inspected revision, not completed acceptance for
this program. Current scanner results and GitHub alert counts must be collected
in P0; the presence of a workflow does not establish a clean security backlog.

| Area | Present on main | Preparation needed |
| --- | --- | --- |
| Live view consolidation | PR #691 replaced the separate WebRTC view with the shared [LiveView](../../web/js/components/preact/LiveView.jsx), supported by [transport URL helpers](../../web/js/utils/live-view-url.js) and [compatibility tests](../../web/tests/liveViewTransportTabs.spec.js) | Preserve this completed refactor and its behavior during TSX adoption |
| Frontend build | [Vite](../../web/vite.config.js) targets Preact directly; application files use JS/JSX; [package scripts](../../web/package.json) have no dedicated type-check command | Add incremental TypeScript configuration, test support, and a required check |
| Frontend lint | [ESLint](../../web/eslint.config.js) targets JS/JSX, reports accessibility rules as warnings, disables hooks rules, and enforces an existing UI-string baseline | Extend coverage to TS/TSX and progressively enforce resolved categories |
| Native analysis | [clang-tidy and cppcheck](../../.github/workflows/static-analysis.yml) intentionally report without blocking; [CodeQL](../../.github/workflows/codeql-analysis.yml) analyzes C/C++ | Triage findings and establish explicit regression and release policies; cover frontend security analysis too |
| Dependency/container analysis | [Trivy](../../.github/workflows/container-scan.yml) reports findings with a zero finding exit code | Inventory shipped dependencies and make actionable findings enforceable |
| API contract | [API documentation](../API.md) describes routes, role-dependent responses, and compatibility behavior; no first-party OpenAPI definition was found in the scoped review | Establish a machine-readable contract and test it against C |
| Tests | C/SQLite tests, [ASan/UBSan](../../.github/workflows/sanitizer.yml), frontend unit tests, and [Playwright API/UI/go2rtc projects](../../playwright.config.ts) exist; the main browser CI step explicitly runs UI shards | Map actual execution to risks; make API contracts and relevant failure scenarios explicit CI jobs |
| Deployment | [Container release matrix](../../.github/workflows/docker-publish.yml) covers amd64, arm64, and arm/v7; Vite has default and modern-only browser build paths | Establish measured budgets for supported profiles and identify any additional embedded targets that remain required |

## 3. Goals and scope boundaries

### Goals

- Resolve actionable security and quality findings and prevent their recurrence.
- Catch incorrect frontend props, API usage, state handling, and null assumptions
  before release, with strict checking throughout migrated code.
- Make API compatibility executable and reusable by clients and future backends.
- Preserve recording, playback, authorization, recovery, and upgrade behavior.
- Keep resource costs visible on constrained NVRs and viewing devices.
- Clarify ownership and lifecycle boundaries in C where evidence justifies work.

### Out of scope

- A Rust backend, transpilation pipeline, or automatically generated release code.
- Replacing Preact, redesigning the UI, or changing the public API for typing convenience.
- Rewriting FFmpeg, go2rtc, SQLite, or other native dependencies.
- A simultaneous database/configuration format migration or broad framework upgrade.
- Treating scanner silence, a successful build, or a coverage percentage as proof
  of security or behavioral equivalence.

Necessary bug fixes can change incorrect behavior, but must be identified as
such and documented separately from mechanical typing or refactoring changes.

## 4. Requirements

### 4.1 Findings inventory, remediation, and enforcement

**Q1 — Establish one findings inventory.** Collect current CodeQL, clang-tidy,
cppcheck, sanitizer, frontend lint, dependency, and container results. Reconcile
known security and quality issues with those results. Identify production,
optional-feature, build-only, and third-party findings without silently excluding
any category. Keep sensitive vulnerability details in the appropriate private
security channel; public tracking uses sanitized references.

Each finding records its stable ID or fingerprint, source/tool version, affected
revision and configuration, severity, exposure, owner, disposition, and evidence.
Dispositions are: open, fixed, verified false positive, or time-bounded exception.
An exception requires a reason, owner, mitigation where applicable, and review
date. Expired exceptions fail the policy check. Baseline updates require an
explicit reviewed change and cannot automatically absorb new findings.

**Q2 — Fix by product impact.** Prioritize reachable security defects and data
loss, then memory/lifetime problems, correctness, accessibility, and maintainability.
Review authorization, credential handling, path/file operations, external input,
and dependency exposure as relevant to actual findings. Use focused regression
tests for behavioral defects. Separate dependency upgrades from unrelated type
conversion and avoid indiscriminate forced upgrades.

**Q3 — Make progress durable.** Introduce required checks that reject new
actionable findings, and promote cleaned rule categories to blocking. Baseline
comparison must use finding identity rather than total warning counts: removing
one old warning cannot conceal a new one. Scanner crashes, configuration errors,
missing expected reports, and accidentally empty scans are failures, not clean
results. Pin or record tool versions and scanner databases so changes are
explainable without indefinitely freezing security intelligence.

The release gate requires no unresolved first-party security defects or known
data-loss defects. Remaining upstream/unfixable findings require documented,
current risk acceptance; all other actionable quality findings must be resolved
before final program acceptance. A suppression without supporting evidence is
not resolution. Apply checks to relevant feature configurations, including
production combinations omitted by lean analysis builds.

**Acceptance:** Every collected finding has an owner and disposition; fixed
categories cannot regress unnoticed; deliberate sample violations and a failed
scanner demonstrably fail the gate; required checks are configured in repository
rules as well as workflow YAML. Capture this enforcement evidence once when
introducing each gate rather than adding tests that merely restate configuration.

### 4.2 Incremental TypeScript and TSX

**T1 — Add build-time checks without changing the runtime.** Add a frontend
`tsconfig` using `jsx: react-jsx`, `jsxImportSource: preact`, `strict`,
`noUncheckedIndexedAccess`, `isolatedModules`, and no emitted output from the
type checker. Configure module
resolution and compatibility aliases consistently with Vite. Introduce a
required `typecheck` command running `tsc --noEmit`; Vite's transpilation alone
does not check types.

Allow JS/JSX alongside TS/TSX during migration. Use targeted JSDoc/checking or
narrow typed adapters at remaining JS boundaries. A migration inventory must
show what is still unchecked; `allowJs` alone does not mean that JS is checked.
New application modules use TS/TSX after the foundation lands. Existing modules
move in coherent feature slices, with their meaningful inputs and outputs typed.

**T2 — Preserve all existing checks.** Update ESLint parser/configuration, file
globs, UI-string enforcement, hooks rules, Jest discovery and transforms, and
the relevant build/test entrypoints for TS/TSX. Run type checking separately for
first-party TypeScript integration/load tests as well; execution or type stripping
is insufficient. Keep Preact compatibility types aligned with runtime aliases.
Do not adopt a new test runner as a prerequisite.

**T3 — Make types useful.** Type request helpers and query/mutation wrappers
before broad component conversion. Distinguish API data from editable form state
and persisted browser state. Model pending/success/error states and transport
choices explicitly. Handle optional/redacted values, cancellation, errors,
DOM refs, events, and storage parsing. Restrict explicit `any`, unchecked casts,
non-null assertions, and suppression comments; unavoidable third-party adapters
must be narrow, explained, and tracked. Do not use `@ts-nocheck` to mark a file
as migrated. Enable hooks correctness rules for migrated scopes and expand their
coverage as findings are resolved.

**Acceptance:** Migrated files and their public boundaries pass strict checking;
incorrect props and response-field use fail the required check; migrated tests
are discovered and run; accessibility and localization checks still apply; no
React runtime is introduced; asset reports satisfy the applicable budgets.
Final scope is all first-party browser application code under `web/js`, with
generated/vendor files identified explicitly. Build configuration may remain JS
if checked, and does not count as an application migration exception.

### 4.3 Executable API contracts

**A1 — Create a canonical contract.** Add a first-party OpenAPI 3.1 definition
at the proposed path `docs/api/openapi.yaml`. Select and pin compatible validation
and generation tools in P2. Inventory method/path pairs from actual routing and
frontend usage, then reconcile them with the existing API documentation. Track
coverage explicitly; a partial initial schema must not imply whole-API coverage.

For each covered operation, specify parameters, content types, request/response
schemas, status/error behavior, authentication, and authorization expectations.
Record null versus absent fields, defaults, enum values, units, timestamp formats,
identifier encoding, numeric limits, and pagination. Model summary/full and
role-dependent response variants separately. Preserve current wire behavior,
including tri-state retention values and redacted camera credentials. Do not
silently change integer representations or add privileged fields to make types
simpler.

**A2 — Generate shared types and verify behavior.** Generate deterministic TS
request/response types from the schema and consume them through a small client
layer. Generated files are not hand-edited; CI regenerates and checks for drift.
Keep readable documentation aligned with the canonical contract. Run contract
tests against the actual C server, exercising successful, malformed, unauthorized,
forbidden, empty, and role-redacted cases where applicable. Tests assert policy
and side effects as well as response shape. Do not generate expected behavior
solely from the same implementation under test.

**A3 — Validate external data deliberately.** Treat network/storage input as
untrusted until checked. A cast such as `as Stream` does not validate JSON.
Provide runtime validation at selected client boundaries, beginning with the
pilot's request results and persisted state. Measure the validator's emitted
cost; do not ship an entire schema/compiler runtime merely for type generation.
Server-side input validation and authorization remain authoritative. Define
compatible handling of additional fields and unknown values without hiding
missing required data or parsing errors.

**Acceptance:** The pilot contract matches real C responses and permissions;
invalid fixtures fail validation; a schema/client mismatch fails CI; compatible
additive changes are supported by policy. Final coverage includes all operations
consumed by the first-party UI, plus an inventory of other supported public API
operations and their schema or explicit protocol-test coverage. Streaming/media
responses need protocol tests rather than pretending their bytes are JSON.

### 4.4 Operational and compatibility regression coverage

**R1 — Map risks to executed checks.** Reuse existing C, SQLite, frontend,
Playwright, media, and sanitizer fixtures. Record which job actually executes
each important scenario. Add explicit API-contract execution; UI shards alone
do not establish API-project coverage. Distinguish mocks from real server/media
tests and mark hardware-dependent evidence separately.

| Scenario | Required observable outcome | Execution lane |
| --- | --- | --- |
| Authentication, permissions, and redaction | Each role receives permitted data/actions; denial produces no unauthorized mutation or credential disclosure | PR contract/API tests |
| Stream edit and async actions | Invalid input leaves configuration intact; pending/error UI is correct; repeated clicks do not duplicate a write | PR API/UI tests |
| Unified Live view | Existing index/HLS URLs, mode overrides, transport flags, filters, variant storage keys, fullscreen and substream behavior remain compatible | PR unit/UI; media smoke tests |
| Disconnect and audio/video recovery | Recording resumes within the fixture's declared deadline; resulting media is playable and timestamps remain within documented expectations | Targeted PR tests and scheduled media runs |
| Segment boundaries and retained recordings | Correct duration/order/metadata; retention protects held files and respects inherited, unlimited, and finite policies | PR C/SQLite tests and scheduled media runs |
| Full/read-only/unavailable storage | Failure is reported, cleanup respects policy, and recovery does not erase protected recordings or corrupt metadata | Isolated fault-injection tests |
| Shutdown, forced termination, and restart | Shutdown respects its deadline; restart reconciles incomplete work; database integrity and committed recordings remain recoverable | PR targeted tests and scheduled recovery runs |
| Upgrade and persisted state | A previous supported release's database, configuration, and browser preferences remain usable; incompatible changes are detected | Upgrade fixtures/release qualification |
| Repeated lifecycle activity | No sanitizer errors, deadlocks, unexplained recording gaps, or sustained unbounded growth in memory, threads, or file descriptors | Scheduled soak on declared target/workload |

Each failure fixture specifies injected fault, expected state transitions,
deadlines/tolerances, and evidence. Establish numeric limits before using a
scenario as an acceptance gate. Compare playback and media metadata where
appropriate; byte-identical recordings are not required. Use isolated temporary
storage and test streams, never production recordings. Preserve existing
ASan/UBSan coverage and assess a focused race-detection job where dependencies
and platform support permit it.

**Acceptance:** Required scenarios have passing execution evidence, independent
assertions, and retained diagnostics. Quarantined tests have an owner and expiry;
a mandatory acceptance scenario cannot be counted as passed while quarantined.
Retries expose flakes rather than erasing their history. The recent audio and
recorder recovery tests remain part of the applicable coverage.

### 4.5 Embedded resource budgets

**B1 — Establish reproducible baselines before migration.** Record commit,
lockfiles, build flags, architecture, browser/device, stream workload, and cache
state. Cover default and modern-only web builds, the supported release matrix,
and at least one named constrained deployment. Decide whether MIPS or additional
non-container targets remain release requirements and document their real build
and validation path.

Measure installed web assets (including shipped compressed copies, locales,
fonts, and CSS), compressed first-load JS/CSS by page, lazy-loaded bytes, and
dependencies. Separately measure browser startup/interaction and memory, and
NVR binary size, idle/recording RSS, CPU, file descriptors, and recording gaps.
The NVR serves frontend assets; its memory budget is distinct from the viewing
browser's memory budget.

**B2 — Enforce agreed budgets.** Check in a machine-readable baseline/budget
manifest with numeric limits for each required profile. Asset-size checks run on
PRs; noisy runtime measurements use a controlled runner, repeated samples, and a
documented comparison method. P0 must fill in the limits and name the target;
later acceptance cannot pass against placeholder budgets. No unreviewed budget
increase is allowed. Intentional increases require measured cost, product reason,
and an explicit manifest change. Type-only conversion is expected to preserve
runtime behavior and should not require a larger dependency footprint.

**Acceptance:** CI publishes comparable before/after asset measurements; an
oversized change fails the gate; pilot and later migrations remain within the
agreed budgets. Emulation/build success is not recorded as physical-device
performance evidence.

### 4.6 Targeted C boundary improvements

**C1 — Document and simplify a bounded responsibility.** Identify ownership,
allocation/free responsibilities, thread affinity, callback lifetime, lock order,
cancellation, and shutdown behavior for selected interfaces. Separate pure
policy decisions from filesystem, database, and media side effects where useful.
Represent valid states/transitions and error outcomes explicitly. Use the
[stream-state interface](../../include/video/stream_state.h) and
[request/response abstraction](../../include/web/request_response.h) as existing
boundaries to assess, not justification for a broad rewrite.

Choose the first refactor from a demonstrated finding or a heavily changed
responsibility, record its invariants, and add missing behavioral coverage before
changing ownership. Keep schema changes, public API changes, and structural
refactoring in distinct reviewable changes. Pure helpers such as the existing
Linux `/proc` parsers are candidates for bounded cleanup; no Rust port is part
of this phase.

**Acceptance:** At least one selected boundary has documented invariants,
clear ownership, passing failure-path tests, and evidence that the targeted
problem was removed or the dependency surface reduced. Existing recording and
resource gates pass. Completion is based on this bounded improvement, not an
unmeasurable claim that the entire backend is clean.

## 5. Delivery phases and dependencies

Phases express dependencies rather than calendar promises. Security fixes start
as soon as findings are confirmed. TypeScript/contracts can progress while the
existing backlog is remediated; migration does not wait for every style warning.

| Phase | Concrete work | Depends on | Exit gate |
| --- | --- | --- | --- |
| P0 — Inventory and baseline | Collect findings; map tests/jobs and API operations; record supported profiles and resource limits; preserve the merged Live baseline | Current main | Owned findings inventory, coverage map, measured budget manifest, explicit target list |
| P1 — Remediation and CI policy | Fix security/data-loss defects first; introduce identity-based finding checks; promote cleaned categories; add frontend security coverage and required-check rules | P0 | New violations fail; tooling failures fail; release-blocking findings resolved; remaining work owned |
| P2 — Type and contract foundation | Add strict incremental TS, test/lint support, API schema/toolchain, generated types, and explicit API-contract CI | P0; P1 policy applied to changed scope | Required checks demonstrably reject type/contract drift and preserve existing tests |
| P3 — Streams pilot | Type fetch/query boundaries and one Streams list/edit flow; add schemas, validation, C contract tests, UI states, and asset comparison | P2 and P0 budgets | Complete pilot acceptance below; review actual migration effort before expanding |
| P4 — Frontend rollout | Migrate remaining UI by feature; expand API contracts, hooks/accessibility enforcement, test typing, and localization coverage | P3 | All first-party browser application code typed; required endpoint coverage complete; budgets hold |
| P5 — Reliability and C boundaries | Fill operational coverage gaps; run recovery/upgrade/soak scenarios; implement one justified bounded C refactor | P0 coverage map; applicable P1 gates; contract/behavior tests before affected refactors | Operational matrix accepted; bounded refactor meets C1; supported targets qualified |
| P6 — Readiness review | Reconcile findings, contracts, migration inventory, CI enforcement, resources, and compatibility evidence | P1, P4, P5 | Program acceptance below; any future Rust experiment requires its own scoped decision |

P5 test infrastructure may begin alongside P1/P2. A C refactor waits for its
relevant behavioral tests, not necessarily the entire frontend rollout. P1
remediation continues until the final findings gate is satisfied.

### 5.1 First end-to-end pilot

Use the existing Streams list and edit workflow. Cover the actual operations
used by [StreamsView](../../web/js/components/preact/StreamsView.jsx),
[StreamCard](../../web/js/components/preact/StreamCard.jsx), and
[StreamConfigModal](../../web/js/components/preact/StreamConfigModal.jsx):
`GET /api/streams`, the detail/full variant used for editing, and
`PUT /api/streams/{name}`. Record the exact variants during P2 routing review.
Do not expand this pilot to deletion, bulk actions, or recording architecture.

1. Capture existing requests, role-dependent results, edit semantics, and errors
   in independent fixtures and tests; resolve inconsistencies explicitly.
2. Define the contract and generate request/response types. Keep redacted list
   data separate from privileged edit data and editable form values.
3. Type the relevant fetch/query adapters and async-action interfaces; validate
   external data and persist useful error/cancellation information.
4. Convert the pilot components and supporting code to TS/TSX with strict checks.
   Keep the route, layout, localization, and Preact runtime behavior stable.
5. Verify empty/loading/error/success states, missing optional fields, malformed
   responses, permitted/denied edits, invalid input, and duplicate-submit behavior.
6. Run actual C contract tests, focused UI/unit tests, applicable native tests,
   and before/after asset measurements. Record implementation and review effort.

**Pilot exit:** One real feature works from schema through C response and typed
UI, with no untracked type escapes, policy regressions, or budget overruns.
This validates the migration approach, not recording concurrency or whole-API
compatibility. The rollout must adjust if the pilot exposes oversized generated
clients, ineffective types, unstable tooling, or excessive review cost.

### 5.2 Suggested reviewable PR sequence

| Slice | Change | Evidence needed |
| --- | --- | --- |
| 1 | Findings/coverage inventories and resource measurement tooling | Reproducible baseline artifacts and named owners/profiles |
| 2 | Required findings gates and initial security/correctness fixes, split by root cause | Finding disposition, behavioral regression evidence, failed-gate demonstration |
| 3 | TS/TSX infrastructure and lint/test support, with a small existing helper migrated | Working strict check, discovered tests, unchanged runtime output or explained difference |
| 4 | Pilot OpenAPI contract, generation, and tests against C | Operation coverage, negative cases, deterministic generation |
| 5 | Typed fetch/query boundaries and Streams pilot | Full pilot acceptance and resource report |
| 6 onward | Separate feature migrations: auth/shared UI; unified Live; recordings/timeline; settings/system; remaining fleet/event/privacy surfaces | Feature contracts, typed tests, applicable compatibility and budget checks |
| Ongoing | Operational fixtures and bounded C improvements | Scenario evidence before/after each affected refactor |
| Final | Acceptance reconciliation and contributor documentation | All P6 gates met; outstanding exceptions explicitly reviewed |

The order of feature slices may follow active development, but shared contracts
and adapters precede their consumers. Preserve PR #691's unified component;
do not recreate a second Live implementation to simplify migration.

## 6. Program acceptance

- Every finding in the P0 inventory is fixed, a verified false positive, or an
  approved current upstream/unfixable exception; no actionable first-party
  security, data-loss, or quality finding remains open.
- Required analysis, type, lint, generation-drift, contract, and relevant
  regression checks prevent merges on failure; changed CI paths cannot silently
  bypass them. Scheduled/release gates have named owners and retained evidence.
- The application migration inventory is complete. Strict type checks cover
  first-party browser application code; integration tests have their own check;
  generated/vendor exclusions and necessary boundary adapters are explicit.
- UI-consumed API operations have canonical contracts tested against C;
  remaining public operations have documented coverage status and appropriate
  schema/protocol checks. Authorization and sensitive-field behavior are tested.
- Unified Live compatibility, recording/recovery, retention, persistence, and
  upgrade scenarios meet the recorded limits. No required acceptance case is
  satisfied solely by mocks, quarantine, or a retry concealing a known defect.
- Default/constrained frontend profiles and supported deployment targets meet
  recorded asset/runtime budgets, with physical evidence where performance is
  claimed. Required measurements and limits are populated, not deferred.
- The selected C boundary improvement passes its invariants and regression
  checks, and contributors can reproduce the applicable commands locally.

Successful completion does not commit LightNVR to a second backend. A later Rust
proposal must identify a specific responsibility, expected ownership benefit,
compatibility evidence, target constraints, and maintenance cost. These decisions
remain separate from accepting improvements to the current product.

## 7. Rollout, risks, and decisions

| Risk or decision | Required handling |
| --- | --- |
| Old findings make every PR red | Establish the reviewed baseline first; block new findings immediately and remove baseline entries as fixed |
| Green CI means a scanner did not actually run | Check report presence/scope and tool errors; verify repository required-check behavior, including path filters |
| TSX becomes cosmetic through `any` or assertions | Review typed boundaries, restrict escapes, and require negative type-check evidence for the pilot |
| Generated types disagree with C | Test real responses and policy; change contracts only with reviewed compatibility intent |
| Validators or compatibility packages inflate the UI | Measure per-page and installed assets; use small generation/validation outputs and retain Preact |
| Existing tooling drops migrated tests or lint rules | Update discovery/transforms/scopes before feature conversion; retain localization and accessibility checks |
| Refactoring changes Live preferences or recording behavior | Preserve the merged compatibility fixtures and add operational assertions before structural changes |
| Noisy performance or flaky media tests block useful work | Use controlled profiles, declared tolerances, retained diagnostics, and owned expiry-bound quarantine; keep acceptance evidence explicit |
| Scope expands into a rewrite | Deliver bounded PRs and the pilot first; exclude language/backend replacement from this program |

P0 must settle the accountable owners, exact constrained devices, required
architectures/browser profiles, numeric budgets, CI cadence/runner allocation,
and findings exception policy. P2 must settle generation/validation tooling and
the API compatibility policy. These are implementation decisions with named
phase deadlines, not reasons to defer inventory or confirmed security fixes.

During rollout, existing JS and new TS may coexist. Revert a problematic feature
slice without disabling established security or type gates. Preserve current
database/configuration formats and browser storage keys; any necessary
compatibility change receives its own migration and rollback assessment.

## 8. Related work and references

- [Mobile-first foundation](UXD_01_MobileFirstFoundation.md) and
  [Live view ergonomics](UXD_02_LiveViewErgonomics.md): preserve shared async,
  accessibility, mobile, and playback behavior.
- [Scoped authorization and audit](FLEET_02_Scoped_Authorization_Audit.md): reuse
  action/scope semantics in contracts and negative tests.
- [Storage lifecycle](STORAGE_01_Multi_Target_Lifecycle.md) and
  [retention policies](PRD_Recording_Retention_Policies.md): define protected
  data and cleanup behavior for operational tests.
- [Host and hardware health](OPS_03_Host_Hardware_Health.md): reuse existing
  resource signals and fault fixtures.
- [Preact TypeScript guide](https://preactjs.com/guide/v10/typescript/),
  [Vite TypeScript checking](https://vite.dev/guide/features.html#typescript),
  [TypeScript incremental JS support](https://www.typescriptlang.org/tsconfig/allowJs.html),
  and [erased types](https://www.typescriptlang.org/docs/handbook/2/basic-types.html#erased-types).
