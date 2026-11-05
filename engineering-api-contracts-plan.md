# Plan: Engineering Hardening and Permission-Aware API Contracts

**Generated**: 2026-11-05
**Mode**: Swarm-ready dependency plan
**Baseline**: `main` at `8455d00577e122b82bd128427b3ed402da66b3e9`
**Source**: [ENG 01 PRD](docs/prd/ENG_01_Safety_Contracts_TypeScript.md)
**Execution**: Started 2026-11-05 at `dea6094a` (only a version bump since the planning baseline). No commits, pushes, or PRs are required by this plan.

## Overview

Deliver a canonical OpenAPI JSON contract, an authenticated projection of that
contract for the current principal, generated frontend types/runtime validators,
and a separate Swagger-style explorer. Begin with Streams list/full/update plus
discovery. Preserve the current C backend and Preact application.

The first independently shippable milestone is **T21**, covering PRD E1–E3 and
the directly related Streams/session correctness work. HLS recovery, settings
zero handling and full root-test typing remain independent hardening milestones. T22–T24 extend E4 into authentication
and recordings. T25–T28 establish bounded E5 recovery evidence and can proceed
without waiting for full API coverage. T29 reconciles those milestones and
produces the next bounded expansion plan for storage/fleet/events/ONVIF/system.
This is deliberately not a promise to cover every route in one change.

TS/TSX application conversion is already done. Rust, automatic translation or
synchronization, required repository status checks, the deferred CodeQL backlog,
SSO, a new authorization model, and physical-device qualification are excluded.

## Assumptions

- The existing uncommitted PRD/index edits are input. Preserve them and all other
  unrelated working-tree changes. Do not recover implementation from #693.
- Keep one author-maintained `docs/api/openapi.json`. Generated TS, validators,
  and an embedded C document derive from it. No independent handwritten YAML.
- `/api/openapi.json` and `/api-docs.html` are proposed new surfaces, not existing
  endpoints. The JSON endpoint supports authenticated sessions and the supported
  non-session credential methods, including scoped tokens.
- The initial explorer is **read-only and session-based**. It has no token-entry
  form, no saved credentials, and no Try-it-out requests. Scoped-token JSON
  discovery is still fully implemented and tested with cookie-free API clients.
  Interactive requests and in-browser token switching require a later slice.
- For discovery only, reject synthetic `auth_disabled` and anonymous `demo`
  principals. Real authenticated demo users remain subject to actual mutation
  restrictions. Do not change existing auth-disabled API behavior. T1 records
  precise errors and operator messaging for these cases.
- Potential grants with no matching cameras are not advertised as usable
  camera capabilities. An authenticated principal can still fetch an empty
  pilot projection with coverage metadata. A list operation that legitimately
  returns an empty filtered list needs its own documented eligibility rule.
- OpenAPI visibility is descriptive. Public source already contains the full
  contract; every API call must keep its existing authorization enforcement.
- Use one immutable authorization/resource observation for a projection: coherent
  SQLite policy data plus bounded captured runtime facts used by each operation.
  Existing `authorization_context_t` memoization is not a transaction snapshot;
  a SQLite snapshot also does not freeze live camera health.
- No numeric production performance budgets are invented. T2 records a
  reproducible baseline; T20 measures deltas and proposes justified limits.

## Prerequisites

- Node from `.nvmrc`, root and web lockfiles, Python 3, CMake/compiler and the
  native development dependencies in `docs/BUILD.md`. Initialize the pinned FAAC
  submodule; go2rtc/FFmpeg executables are also needed for T25/T27 real media.
- Fresh task-owned build directories and loopback ports. Do not use prebuilt
  `build/bin/lightnvr` as current-source evidence.
- No production credentials, cameras or recordings. Test principals, tokens,
  storage and media are synthetic and created in private temporary directories.
- Browser checks need the pinned Playwright Chromium installation. Core API
  contract tests must run without an external camera or managed go2rtc.
- A named prior-release source revision is needed for T28. Default candidate:
  the latest available 0.45.x tag preceding this baseline, resolved and recorded
  before fixture generation; never silently substitute current migrations.

### Inspected implementation seams

| Concern | Existing implementation | Planning consequence |
| --- | --- | --- |
| Route registration | `src/web/libuv_api_handlers.c` | Compare actual method/path/handler mappings; normalize `#` wildcards explicitly |
| Principal acquisition | `src/web/httpd_utils.c`, `include/web/httpd_utils.h` | `httpd_check_action_access` accepts scoped tokens but also synthetic demo/auth-disabled users; cookies are checked before API credentials |
| Action evaluation | `src/core/authorization.c`, `include/core/authorization.h` | Reuse actual evaluator; a union of action bits cannot prove same-camera conjunctions |
| Policy data | `src/database/db_authorization.c`, `db_api_tokens.c`, fleet/collection helpers | Existing helpers take locks and load at different times; do not wrap them in a recursive global lock |
| Streams | `api_handlers_streams_get.c`, `api_handlers_streams_modify.c`, `StreamsView.tsx` | Summary and compatibility responses differ; full reads have permission-dependent redaction |
| JSON trust | `web/js/fetch-utils.ts`, `web/js/query-client.ts` | Generic types currently assert JSON shape; preserve cancellation and HTTP errors when adding validation |
| Native tests | `tests/unit/CMakeLists.txt`, `tests/cmake/CiSuites.cmake` | Use existing Unity helpers and explicit CI labels; serialize edits to shared registration |
| Browser harness | `tests/integration/global-setup.ts`, `global-teardown.ts` | Existing ports are hard-coded; do not run parallel worker servers through it |
| Assets | `web/vite.config.js`, `web/js/pages/*`, `web/*.html` | New docs entry must be separately loaded; source glob changes require CMake reconfiguration |

### Tool direction and primary references

T3 pins compatible exact versions after a small local smoke test; it must not
upgrade unrelated dependencies. Use OpenAPI 3.1 JSON, an OpenAPI document linter,
`openapi-typescript` for erased types, and Ajv standalone generation for selected
JSON Schema validators. Generated runtime code must be measured and compatible
with the existing strict TS/build rules; no browser schema compiler or network
reference resolution. Serve a standalone Swagger UI bundle only on the docs page,
outside the Preact runtime aliases. Its bundled runtime is an added docs-only
dependency, not a replacement of Preact or a free asset-size change.

Primary documentation consulted on 2026-11-05:

- [OpenAPI 3.1.1 specification](https://spec.openapis.org/oas/v3.1.1.html): security schemes and specification extensions; pin the compatible 3.1 revision in T3.
- [openapi-typescript](https://openapi-ts.dev/introduction): OpenAPI 3.1 support and runtime-free generated types.
- [Ajv standalone generation](https://ajv.js.org/standalone.html): compile validators at build time; handle the 2020-12 dialect deliberately.
- [Swagger UI configuration](https://swagger.io/docs/open-source-tools/swagger-ui/usage/configuration/): `supportedSubmitMethods: []`, `persistAuthorization: false`, disabled external validation and query configuration.
- [Redocly CLI lint](https://redocly.com/docs/cli/commands/lint): candidate document validation; separate schema validity from independent behavior assertions.

## Execution conventions

- Workers are not alone in the codebase. Edit only assigned files directly in
  the workspace, report paths changed, and never overwrite/revert unrelated work.
  No `git add -A`, stash/reset, commits, dependency installs outside ownership, or
  silent expansion into another worker's files.
- New filenames below are planned outputs. Commands referencing them become
  available only after their producer task completes. T3 must expose the exact
  script interface below, or the coordinator updates every consumer before work
  resumes. Do not claim a check passed when its script/target does not exist.
- `$ENG_BUILD` below means the literal task-owned path `build/eng-api-T<ID>`.
  Replace it with the current task ID. Configure from repo root:
  `cmake -S . -B "$ENG_BUILD" -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON -DENABLE_SOD=OFF -DENABLE_LITERT=OFF -DENABLE_GO2RTC=OFF -DENABLE_MQTT=OFF -DENABLE_SSL=ON`.
  Do not reuse a concurrently built tree. Reconfigure after adding source files.
- Native worker checks below first build their named target, then run
  `ctest --test-dir "$ENG_BUILD" --output-on-failure --no-tests=error -R '<anchored expression>'`.
  Record flags, tool versions, HEAD and a source-tree evidence digest with results. Because commits are not required, HEAD alone is insufficient: hash the tracked diff plus relevant untracked implementation/generated files (exclude secrets, build output and unrelated user files), and retain the manifest.
- Execution-host correction: `/usr/local` has stale FFmpeg 61 headers/pkg-config
  files but the linker uses system FFmpeg 62. For every qualifying build on this
  host, use `cmake --fresh`,
  `PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig`,
  an empty `PKG_CONFIG_PATH` (or only the T25 task-local libyaml pkg-config path),
  and both `-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu` and
  `-DCMAKE_CXX_FLAGS=-isystem /usr/include/x86_64-linux-gnu` as quoted arguments.
  A compiled runtime probe confirmed matching avcodec 62/62, avformat 62/62 and
  avutil 60/60. T2's JSON/asset baseline remains recorded evidence; its binary is
  not accepted for media qualification. This is an environment override, not a
  portable product build requirement or a reason to change application code.
- T25/T27 exercise **managed go2rtc integration**, not an external source-only
  substitute. Build the pinned `go2rtc/go2rtc` binary first as documented in
  `docs/BUILD.md`, then use a separate task-owned `ENG_MEDIA_BUILD` configured
  with the same flags above except `-DENABLE_GO2RTC=ON` and
  `-DGO2RTC_BINARY_PATH="$PWD/go2rtc/go2rtc"`; include libyaml development headers.
  Build `lightnvr`, run scripts with `--binary "$ENG_MEDIA_BUILD/bin/lightnvr"`,
  and let each owned recorder instance supervise only its own go2rtc child.
  Source disconnect is injected at the synthetic source, not by killing another
  worker's go2rtc. The JSON contract build remains go2rtc-disabled.
- Contract commands use a T2-owned process fixture with dynamically allocated
  ports, temporary config/DB and owned-PID cleanup; `--binary` always points to a
  fresh build. Never kill arbitrary processes or attach to a production URL.
- If blocked, report the failed assumption, paths involved and smallest producer
  change needed. Mark the task blocked in its log, not complete. Pause dependent
  tasks; independent work can continue. A test exposing an unrelated production
  bug becomes a focused repair task with explicit ownership/dependencies.
- Only the coordinator updates this plan's status/log fields. Worker reports
  are evidence, not permission to call an incomplete milestone accepted.
- Package installation, Vite output, and CMake's shared `generate_version_js`
  source output are exclusive resources. The coordinator serializes those write
  steps against consumers, even where tasks have otherwise disjoint ownership.

## Dependency Graph

```text
T1 -> T2 -> T3 -> T4 --------> T6 -> T7 -> T16
       └-> T5 ---------------> T6
                   └-> T8 -------------------> T14
T3 -> T11 -> T12 -----------------------------> T14
T3 -> T13 -----------------------------------> T14, T15
T3,T7 ---------------------------------------> T15
T2,T14,T15,T16 -> T17 -> T19 -> T21 -> T18
T2,T14,T15,T16 -> T20 ----------> T21
T3 -> T9,T10                          (independent hardening)
T21 -> T22 -> T23 -> T24
T2 -> T25a -> T25; T1,T2 -> T26; T25,T26 -> T27 -> T28
T9,T10,T18,T21,T24,T25,T26,T27,T28 -> T29
```

The `depends_on` fields are authoritative; the graph groups some edges for readability.

## Tasks

### T1: Freeze pilot semantics, coverage and interfaces
- **depends_on**: []
- **agent_type**: local
- **ownership**: `docs/api/contracts/design.md`, `docs/api/contracts/coverage.json`, `docs/api/contracts/principal-matrix.json` (new); product code read-only.
- **description**: Inventory registered methods/routes and mark pilot/later/public/protocol-only coverage. Define stable operation IDs, `x-lightnvr-authorization`, scope predicates, conditional response variants, projection metadata, denied/error semantics and exact module interfaces for T5–T8. Inspect how all policy contributors change, including collections/tags, camera membership, user activation and token expiry, plus runtime-health selectors and operation-specific resource preparation. Resolve auth-disabled/demo, empty scope, credential precedence, base-path behavior and schema content before parallel work.
- **acceptance_criteria**:
  - Pilot covers Streams list (summary and compatibility), full read, PUT, and discovery. Public/bootstrap descriptions are explicit without pretending the full API is specified.
  - Matrix includes anonymous, admin, scoped viewer, configure-only, mixed read/configure scope, split grants whose actions never overlap on one camera, empty scope, read-only token owned by admin, two tokens of one owner, expired/revoked token, inactive owner, demo and auth-disabled deployments.
  - Separate operation eligibility, per-resource conditions and response variant eligibility. Read permission cannot be inferred from configure permission. List-empty semantics are explicit.
  - Define snapshot lifetime/ownership and a concrete loader/evaluator strategy before T5; no action-mask shortcut or pretend snapshot. Record which handlers authorize before/after runtime/go2rtc health enrichment, preserve those semantics explicitly, and report pre-existing inconsistencies instead of silently fixing them in discovery. Include health grants in the matrix. Define finite test deadlines before fault-test implementation.
- **validation**: `python3 -m json.tool docs/api/contracts/coverage.json`; `python3 -m json.tool docs/api/contracts/principal-matrix.json`; manually trace each pilot route through handler and evaluator and record file/function references.
- **status**: Completed
- **log**: Inspected actual list/full/update authentication, raw/runtime/live resource preparation, grant/token/collection loaders and readonly-connection seams. Froze version-1 metadata, snapshot/evaluator/projector interfaces, principal/mode policy and fixture deadlines. Both JSON artifacts pass `python3 -m json.tool`; route inventory compares literal registrations to source. Execution HEAD differs from planning HEAD only by 0.46.3 version bump. Product code unchanged by T1.
- **files edited/created**: `docs/api/contracts/design.md`, `docs/api/contracts/coverage.json`, `docs/api/contracts/principal-matrix.json`.

### T2: Build isolated real-server fixtures and capture baseline behavior
- **depends_on**: [T1]
- **agent_type**: worker
- **ownership**: `tests/contracts/fixture-server.mjs`, `tests/contracts/fixture-server.test.mjs`, `tests/contracts/fixtures/`, `tests/contracts/playwright.config.ts`, `tests/contracts/baseline.spec.ts`, `scripts/measure-contract-assets.mjs`, `docs/api/contracts/baseline.md` (new).
- **description**: Create a small fixture around the current C binary, reusing existing test conventions without importing hard-coded global setup. Each test instance owns its temp directory, PID and ports. Seed disabled cameras and test policies through supported interfaces or a documented offline fixture step before startup. Provide API and UI projects, retained failure artifacts, a mode for two simultaneous instances, and repeatable asset accounting. API project discovery explicitly includes `baseline.spec.ts` as well as `*.api.spec.ts`; the UI project includes only `*.ui.spec.ts`. The root Playwright project does not discover this separate directory.
- **acceptance_criteria**:
  - Two fixtures coexist; one cleanup cannot stop/delete the other. Bind collisions fail clearly or retry boundedly; no broad `pkill`. Shutdown/crash paths retain useful logs.
  - Independent baseline assertions record actual success/denial/redaction and config side effects, not schema-generated expected objects. No real secrets in fixture artifacts.
  - Baseline report records commit, build flags, default/modern installed and per-entry bytes, and environment. Helpers for later tests have stable signatures and no hidden persistent global state.
- **validation**: `node --test tests/contracts/fixture-server.test.mjs`; build fresh `lightnvr`; `LIGHTNVR_CONTRACT_BIN="$ENG_BUILD/bin/lightnvr" npx playwright test -c tests/contracts/playwright.config.ts baseline.spec.ts --project=api --workers=1`; `npm --prefix web run build`, then `node scripts/measure-contract-assets.mjs --dist web/dist --profile default --out /tmp/lightnvr-eng-baseline-default.json`; repeat with `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build` and profile/output `modern` / `/tmp/lightnvr-eng-baseline-modern.json`, before T3.
- **status**: Completed
- **log**: Fresh Debug ASan/UBSan server; fixture lifecycle 4/4 and real API baseline 1/1 passed. Two concurrent servers, owned-port/PID/bind evidence, retained failures and signal/timeout failure handling verified. Chromium session smoke passed. Captured 12 actual synthetic response cases. Default/modern installed assets 11,727,012 / 8,120,386 bytes; exact commands, source/binary hashes, per-entry measurements and raw report paths recorded in baseline.md. Reviewed ownership and fixture code; Node/web build resources released.
- **files edited/created**: `tests/contracts/fixture-server.mjs`, `tests/contracts/fixture-server.test.mjs`, `tests/contracts/fixtures/cameras.json`, `tests/contracts/fixtures/baseline-payloads.json`, `tests/contracts/playwright.config.ts`, `tests/contracts/baseline.spec.ts`, `scripts/measure-contract-assets.mjs`, `docs/api/contracts/baseline.md`.

### T3: Pin contract tooling and deterministic generation
- **depends_on**: [T1, T2]
- **agent_type**: worker
- **ownership**: root `package.json`, `package-lock.json`; `web/package.json`, `web/package-lock.json`; `scripts/contracts/`, `tests/contracts/tooling/`, `tests/contracts/tsconfig.json`, `docs/api/contracts/toolchain.md`, `redocly.yaml` (new); `web/VITE.md` for generated-code conventions.
- **description**: Pin exact compatible validator/type-generation tools and the docs-only Swagger distribution. Implement root commands `contracts:lint`, `contracts:generate`, `contracts:check`, `contracts:test-tools`, and `contracts:typecheck`; generation/check support `--target=native`, `--target=web`, and no argument for both. `contracts:lint -- --projections DIR` validates every actual emitted OpenAPI document in DIR and fails if none exist. `contracts:typecheck` checks new contract TS/config code using a dedicated `tests/contracts/tsconfig.json`, independently of the later full-root task. Native output is a committed C byte-array header generated by Python so native builds do not require Node. Web output includes declarations and standalone validators. `--check` generates into temporary paths and compares without modifying tracked artifacts.
- **acceptance_criteria**:
  - Tool smoke fixtures exercise JSON Schema 2020-12, unions, null/absent fields, formats, local references and generated strict TS consumption. No remote `$ref` resolution at runtime; reject unsupported schema features explicitly.
  - Runtime validation neither coerces values nor inserts defaults/removes additional fields. Additive fields remain compatible. Unsupported authorization metadata fails generation rather than defaulting to public visibility.
  - Deterministic output and negative drift detection demonstrated. Put generated validator JS and adjacent declarations in `web/generated/api/`, outside handwritten `web/js`; preserve its two-file JS allowlist and strict application rules. Document that this runtime JS is generated and not checked application TS: trust comes from generator/drift checks and positive/negative validator fixtures, not a declaration wrapper. Generated declarations accept `unknown` and are checked by a separate strict tooling fixture with library checking enabled. Generation owns the entire directory; no handwritten helpers or broad exclusions there.
  - No unrelated dependency upgrades; docs runtime isolated from Preact aliases; schema/compiler toolchain remains development-only.
- **validation**: `npm ci`; `npm --prefix web ci`; `npm run contracts:test-tools`; `npm --prefix web run typecheck`; repeat fixture generation twice and compare bytes; deliberately corrupt one generated fixture and prove check fails, then remove the temporary change.
- **status**: Completed
- **log**: Exact tooling pins installed without changing pre-existing locked versions. Both npm ci commands, 5 tooling test groups, strict contract and web typechecks passed. Deterministic native/web generation, negative drift, nonempty projection lint and TS 5.9/6.0 strict consumer fixtures verified. Coordinator review caught schema traversal confusing literal property names with keywords; corrected in both JS conversion and Python preflight with explicit regressions before acceptance. Approved narrow T2 handoff for fixture declarations/type-only baseline annotations. All Node/install leases released. Native symbols/validator names and build-only x-lightnvr-validators stripping are documented in toolchain.md.
- **files edited/created**: `package.json`, `package-lock.json`, `web/package.json`, `web/package-lock.json`, `scripts/contracts/generate.mjs`, `scripts/contracts/generate_native.py`, `scripts/contracts/lint.mjs`, `tests/contracts/tooling/generator.test.mjs`, `tests/contracts/tooling/openapi.json`, `tests/contracts/tsconfig.json`, `tests/contracts/fixture-server.d.mts`, `tests/contracts/baseline.spec.ts`, `docs/api/contracts/toolchain.md`, `redocly.yaml`, `web/VITE.md`.

### T4: Author canonical Streams/discovery contract
- **depends_on**: [T1, T2, T3]
- **agent_type**: worker
- **ownership**: `docs/api/openapi.json`, `include/web/openapi_document_generated.h`, `tests/contracts/fixtures/schema/` (new); coverage/design edits only by handoff to T1 coordinator.
- **description**: Describe current list/full/update wire behavior and planned discovery response. Add permission metadata and response variants, examples with synthetic values, limits/errors, compatible additional properties and coverage/version metadata. Generate only native output; T8 owns web output.
- **acceptance_criteria**:
  - Reconcile actual summary/full data, query variants, encoded names, nullable/default/zero values, retention settings, credential redaction and response wrappers. Do not change C behavior to fit a convenient schema.
  - Response alternatives remain structurally honest when redacted fields are empty strings or omitted; avoid overlapping `oneOf` alternatives that reject valid payloads. Examples contain no environment-derived values.
  - All metadata needed by the projector comes from the canonical contract; no second manually maintained operation permissions table.
- **validation**: `npm run contracts:lint`; `npm run contracts:generate -- --target=native`; `npm run contracts:check -- --target=native`; validate T2 baseline payloads plus intentionally malformed fixtures using tooling from T3.
- **status**: Completed
- **log**: Canonical OpenAPI 3.1.1 covers four pilot operations, summary/compatibility/full redaction variants, async normal update versus synchronous privacy response and discovery envelope. Lint/native generation/drift plus five independent fixture groups pass, including all 12 actual baseline payloads and malformed/zero/null/surface cases. Embedded SHA c501bfd5f678f8ed26d0880fc5bb89af10071f741c565dd4fe252b00abc17499. Surface-specific guards prevent minimal LPR summaries accepting malformed admin rows. Source-confirmed privacy 502 may occur after the flag is saved; documented rather than changed. T16 must assert no side effects for denied/invalid requests, not all errors. Exact T8/T6 handoff in fixtures/schema/README.md; native/schema ownership released.
- **files edited/created**: `docs/api/openapi.json`, `include/web/openapi_document_generated.h`, `tests/contracts/fixtures/schema/validate.mjs`, `tests/contracts/fixtures/schema/README.md`.

### T5: Provide consistent authorization snapshots for discovery
- **depends_on**: [T1, T2]
- **agent_type**: local
- **ownership**: `include/core/authorization.h`, `src/core/authorization.c`; new `include/database/db_authorization_snapshot.h`, `src/database/db_authorization_snapshot.c`; necessary snapshot seams in `src/database/db_authorization.c`, `db_api_tokens.c`, `db_fleet_query.c`, `db_camera_collections.c`, `src/core/camera_collection_filter.c`, `src/web/fleet_health.c` and corresponding headers; `tests/unit/test_authorization_snapshot.c`; `tests/unit/CMakeLists.txt`.
- **description**: Add an opt-in immutable snapshot input for the existing evaluator, containing required principal/token/grants/role actions, collection-selector data and credential-free camera inventory. Preserve normal request callers. Acquire a coherent read under a proven database snapshot/lock boundary, then evaluate without holding the database lock. Capture bounded runtime/metrics/go2rtc health observations without DB/metrics lock inversion or per-camera network calls, retaining raw/enriched resource variants where existing handlers differ. T1 must establish the exact strategy and demonstrate a minimal loader/evaluator seam before broad edits. Existing helpers that open independent connections do not form one snapshot; never call public lock-taking helpers under their own global mutex. If preserving shared evaluator semantics needs a broad rewrite, stop and split/re-plan this task.
- **acceptance_criteria**:
  - The same evaluator implements decisions; snapshot loading is data acquisition, not a duplicate policy interpreter. No private credential material reaches discovery outputs.
  - Controlled concurrent policy, collection/camera-membership, token and user-state changes cannot produce mixed old/new permissions. Time-based token expiry is defined at a fixed snapshot time, and the next request observes expiry/revocation.
  - Health-based grants are tested against operation-specific captured facts, including unavailable/stale metrics and managed go2rtc disabled. Runtime observations have explicit sampling times and failure semantics; do not claim global atomicity across database and external device state. Snapshot owns copied data, has bounded allocation/error cleanup, is request/thread local and leaves ordinary authorization behavior unchanged. Authentication and snapshot activation state are reconciled rather than trusting a stale user struct.
- **validation**: build `test_authorization_snapshot` and `test_authorization`; run anchored CTest selection for both under ASan/UBSan. Include forced allocation/DB errors and deterministic mutation interleaving, with no arbitrary timing sleeps.
- **status**: Completed
- **log**: Generic snapshot uses one private SQLite transaction, refreshed credential-free principal, fixed token time and captured raw/runtime/live inventory; existing evaluator and ordinary wrappers retained. Narrow connection readers cover grant/action, token, shared collection/membership and fleet facts. Eight snapshot cases cover deterministic writer interleaving, membership/inventory changes, deactivation, expiry/revocation, health variants, empty fleet, allocation/DB failures and cleanup. Existing authorization suite and snapshot suite both pass under ASan/UBSan on corrected coherent-FFmpeg build (2/2 CTests, 3.50s). Earlier fixture FK cleanup ordering corrected before acceptance. Source digest 3624a44807d9cc5725641472417ecf4be02c15173ecac91bd78ffb17c13a5b98 and binary hashes in build/eng-api-T5/evidence.json; logs /tmp/lightnvr-eng-T5-*-system-ffmpeg.log. Dependency refined to T1/T2 because this generic implementation consumes no canonical OpenAPI artifact; T6 still waits for T4.
- **files edited/created**: `include/core/authorization.h`, `src/core/authorization.c`, `include/database/db_authorization_snapshot.h`, `src/database/db_authorization_snapshot.c`, `include/database/db_authorization.h`, `src/database/db_authorization.c`, `include/database/db_api_tokens.h`, `src/database/db_api_tokens.c`, `include/database/db_fleet_query.h`, `src/database/db_fleet_query.c`, `include/database/db_camera_collections.h`, `src/database/db_camera_collections.c`, `include/core/camera_collection_filter.h`, `src/core/camera_collection_filter.c`, `include/web/fleet_health.h`, `src/web/fleet_health.c`, `tests/unit/test_authorization_snapshot.c`, `tests/unit/CMakeLists.txt`.

### T6: Implement operation eligibility and valid document projection
- **depends_on**: [T4, T5]
- **agent_type**: worker
- **ownership**: new `src/web/openapi_projection.c`, `include/web/openapi_projection.h`, `tests/unit/test_openapi_projection.c`; `tests/unit/CMakeLists.txt` after T5.
- **description**: Consume canonical metadata and T5 snapshot to select operations and response variants, then prune unreachable components/metadata. Use local-reference traversal with cycle handling, JSON Pointer escaping, security/discriminator/link references and a supported-feature allowlist. Keep original document immutable and return a request-owned result.
- **acceptance_criteria**:
  - Same-camera conjunctions, operation-specific health observations and owner/token intersection hold. Empty lists/global/self-service rules follow T1. Unknown predicates, failed evaluation or incomplete snapshot fail closed.
  - Mixed-permission responses are described conditionally without leaking camera identities, selector internals, hidden examples or sensitive component definitions.
  - Every projected document is valid, including an empty projection. No dangling references, hidden operation links or security schemes accidentally interpreted as anonymous access.
  - Unit tests serialize the actual C projector output for independent OpenAPI validation; tests of a JS reimplementation do not count.
- **validation**: build/run `test_openapi_projection` under sanitizers; validate emitted documents with `npm run contracts:lint -- --projections <task-owned-output-directory>` (T3 interface); include cyclic refs, escaped pointers and malformed metadata negatives.
- **status**: Completed
- **log**: Request-owned C projection validates complete input, selects operations/same-camera variants, follows cyclic/local named references, prunes definitions/links/security/tags and checks final references. Unsupported keywords/dialects and array-index pointers fail closed; literal payload/map names remain data. Coordinator implementation plus independent review fixed named-map traversal and root/path metadata cases before acceptance. Sixteen native cases pass under ASan/UBSan, including allocation-failure cleanup, immutable input, malformed hidden metadata, same-camera adapter and empty output. Six actual C documents independently linted; only valid single-branch anyOf style warnings. Source/binary manifest build/eng-api-T6/evidence.json; logs /tmp/lightnvr-T6-review-{build,ctest,lint}.log. Adapter ownership refinement documented in design.md; CMake registration released to T7.
- **files edited/created**: `src/web/openapi_projection.c`, `include/web/openapi_projection.h`, `tests/unit/test_openapi_projection.c`, `tests/unit/CMakeLists.txt`, `docs/api/contracts/design.md`.

### T7: Expose authenticated discovery through the real HTTP stack
- **depends_on**: [T5, T6]
- **agent_type**: local
- **ownership**: new `src/web/api_handlers_openapi.c`, `include/web/api_handlers_openapi.h`, `tests/unit/test_api_handlers_openapi.c`; `src/web/libuv_api_handlers.c`; `tests/unit/CMakeLists.txt` after T6; minimal `CMakeLists.txt` packaging checks only if needed.
- **description**: Register `GET /api/openapi.json`, acquire principal with existing credential precedence, reject synthetic principals according to T1, construct snapshot/project/serialize and set `application/json` plus `Cache-Control: private, no-store` on success and relevant errors. Use embedded generated document, not files in a deployment's working directory.
- **acceptance_criteria**:
  - No `user_id`/role query override, broad admin fallback, public canonical-spec route or cached shared response. Memory/DB failure never returns the unfiltered document.
  - Session, Basic, legacy API key and scoped-token behavior matches supported existing auth methods; mixed-cookie/token requests follow documented precedence. Restricted password-change/CIDR behavior is preserved.
  - Runtime modes and enabled features reflected accurately; API route ordering cannot fall through to static assets or wildcard stream handlers. Release/native/container builds include the same spec revision.
- **validation**: fresh configure/build `lightnvr` and `test_api_handlers_openapi`; anchored CTest; T2 fixture smoke fetch with session and cookie-free token; anonymous/mode/error responses and headers asserted over HTTP.
- **status**: Completed
- **log**: Registered real GET route using existing credential precedence and request-owned snapshot/projector. Synthetic modes return403, invalid identity401, internal failures500; all carry private,no-store JSON without canonical fallback. Seven native sanitizer cases pass, including failure injection, refreshed permissions, query override refusal and preserved password-gate distinction (Basic/session restricted; existing key/token exemption retained). Nine real HTTP smoke cases pass with Basic, session, cookie-free token, mixed-cookie precedence, demo and auth-disabled modes; actual HTTP output independently linted. Source digest43851821490cfccf14f2ccee4c6444993856ef6af2e9ef3d427fab758743b548 and binary hashes in build/eng-api-T7/evidence.json; smoke report build/eng-api-T7/http-smoke.json, logs /tmp/lightnvr-T7-{test,http}.log. T16 expands full principal matrix.
- **files edited/created**: `src/web/api_handlers_openapi.c`, `include/web/api_handlers_openapi.h`, `tests/unit/test_api_handlers_openapi.c`, `src/web/libuv_api_handlers.c`, `tests/unit/CMakeLists.txt`.

### T8: Generate typed models and small validated Streams adapters
- **depends_on**: [T3, T4]
- **agent_type**: worker
- **ownership**: new `web/generated/api/`, `web/js/api/streams.ts`, `web/js/api/contractErrors.ts`, `web/tests/contracts/`; no shared fetch/query or page edits.
- **description**: Generate canonical request/response types and standalone validators; implement small Streams functions using existing fetch transport. Keep editable form state separate from wire DTOs and preserve abort/status information. No global migration of all fetch callsites.
- **acceptance_criteria**:
  - Unknown JSON is validated before typed use. Invalid required fields fail with useful non-sensitive error; legitimate extra fields survive. Redacted/full variants and empty arrays handled without unsafe casts.
  - No schema compiler bundled; generated types create no runtime dependency. Typed update DTO preserves current sent fields and omission semantics.
  - Invalid fixtures and a negative type fixture prove the boundary checks; errors do not log URL credentials, request bodies or privileged response data.
- **validation**: `npm run contracts:generate -- --target=web`; `npm run contracts:check -- --target=web`; `npm --prefix web run typecheck`; `npm --prefix web test -- --runInBand --testPathPatterns='contracts/'`.
- **status**: Completed
- **log**: Generated three web artifacts and surface-specific unknown-to-validated adapters; no runtime schema compiler. Request/status/abort errors remain sanitized and ambiguous writes are identified. Approved opt-in sensitive transport logging seam, with default transport behavior retained. Coordinator caught full-read URL missing /full; fixed against canonical/native route and both encoded-name fixture assertions. Final contracts suite 26/26 and web typecheck pass; generation/drift pass. Ownership released.
- **files edited/created**: `web/generated/api/schema.d.ts`, `web/generated/api/validators.js`, `web/generated/api/validators.d.ts`, `web/js/api/streams.ts`, `web/js/api/contractErrors.ts`, `web/js/fetch-utils.ts`, `web/tests/contracts/README.md`, `web/tests/contracts/streams.spec.ts`, `web/tests/contracts/streams.type-fixture.ts`.

### T9: Reproduce and repair HLS recovery scope errors
- **depends_on**: [T1, T3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/HLSVideoCell.tsx`, new `web/tests/e2e/hlsRecovery.browser.spec.js`.
- **description**: Trigger each of the three out-of-scope `go2rtcEnabled` reads, then make the smallest lifecycle-correct repair. Preserve transport preference and cancellation behavior.
- **acceptance_criteria**: Browser regression fails before the fix and passes after; no `ReferenceError`; expected retry/fallback and unmount cleanup occur; the three defect-pinning suppressions are removed. If already fixed on execution HEAD, verify and record rather than reapply.
- **validation**: `npm --prefix web run test:e2e -- --testPathPatterns='hlsRecovery.browser.spec.js$'`; `npm --prefix web run typecheck`.
- **status**: Completed
- **log**: Reproduced ReferenceError in all three recovery callbacks with enabled and auto-detected go2rtc. Hoisted initialization-local setting into callback scope and removed three defect suppressions. Chromium 10/10, typecheck, targeted ESLint and diff check pass; tests cover fallback/retry, force-native preference, pending settings/preflight cancellation and post-cleanup timer guards. Logs /tmp/lightnvr-T9-{before,after,typecheck,eslint}.log. Ownership released.
- **files edited/created**: `web/js/components/preact/HLSVideoCell.tsx`, `web/tests/e2e/hlsRecovery.browser.spec.js`.

### T10: Preserve zero-valued detection settings
- **depends_on**: [T1, T3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/SettingsView.tsx`, new `web/tests/e2e/settingsZero.browser.spec.js`.
- **description**: Reproduce zero becoming 50 and fix defaulting only for absent values, preserving server-supported zero semantics.
- **acceptance_criteria**: Save/load/save retains zero; missing value still receives the intended default; no unrelated settings refactor.
- **validation**: `npm --prefix web run test:e2e -- --testPathPatterns='settingsZero.browser.spec.js$'`; `npm --prefix web run typecheck`.
- **status**: Completed
- **log**: Real Chromium SettingsView regression demonstrated zero loading/refetching as 50 before the fix (2 failures, absent-default case passed). Replaced only the threshold fallback with nullish defaulting. Browser suite now 3/3 passes: initial zero, absent default 50, and save/refetch/second-save retains zero. Refetch completion is observed through a changed response field, not a timing sleep. Web typecheck passes. Logs /tmp/lightnvr-T10-{before,after,typecheck}.log.
- **files edited/created**: `web/js/components/preact/SettingsView.tsx`, `web/tests/e2e/settingsZero.browser.spec.js`.

### T11: Separate camera edit state from ONVIF creation
- **depends_on**: [T1, T3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamsView.tsx`, new `web/tests/e2e/streamCreateAfterEdit.browser.spec.js`.
- **description**: Reproduce edit/close/discover/add carrying stale edit mode; repair state transitions without changing API shape.
- **acceptance_criteria**: The sequence creates through POST, editing still uses PUT, cancel/reopen/clone paths behave correctly and asynchronous discovery cannot resurrect stale selection.
- **validation**: `npm --prefix web run test:e2e -- --testPathPatterns='streamCreateAfterEdit.browser.spec.js$'`; `npm --prefix web run typecheck`.
- **status**: Completed
- **log**: Chromium reproduction showed edit/cancel then ONVIF add sending PUT, late inventory reopening discovery, and stale device/profile selection. Explicit per-save editing intent and editor/discovery/selection generations repair those transitions. Browser 5/5, web typecheck, targeted ESLint and diff check pass; pre-fix/final logs /tmp/lightnvr-T11-*.log. Wire DTOs unchanged. Mutation variables now {streamData, editing, editorGeneration?}; T12/T14 must preserve this captured intent. Source ownership released.
- **files edited/created**: `web/js/components/preact/StreamsView.tsx`, `web/tests/e2e/streamCreateAfterEdit.browser.spec.js`.

### T12: Use supported mutation pending state
- **depends_on**: [T11]
- **agent_type**: worker
- **ownership**: `web/js/query-client.ts`, `StreamsView.tsx`, `UsersView.tsx`, `settings/WorkspacesTab.tsx` under `web/js/components/preact/`; new `web/tests/e2e/mutationPending.browser.spec.js`.
- **description**: Replace demonstrated unsupported `isPending` reads with the installed query library's actual state and remove the type shim preserving those broken reads. Inspect all consumers before editing. Serialize after T11 because both touch StreamsView.
- **acceptance_criteria**: Pending/disabled state starts and clears on success/error; duplicate submits are prevented in affected flows; no query-library upgrade or unrelated placeholder/cancellation refactor.
- **validation**: `npm --prefix web run test:e2e -- --testPathPatterns='mutationPending.browser.spec.js$'`; `npm --prefix web run typecheck`; `rg -n 'isPending' web/js` and explain intentional remaining occurrences.
- **status**: Completed
- **log**: Installed query-core 4 mutation state is isLoading. Replaced unsupported reads and removed shim, with approved one-line cameraFavorites handoff. Browser reproduction also showed same-tick duplicates in Users/Workspaces; small in-flight refs reset onSettled prevent them. Eight cases fail before and pass after: progress/disabled, success/error/reset/retry and duplicate prevention. Web typecheck, targeted ESLint and diff check pass; no isPending reads remain in web/js. T11 intent/generation guards preserved. Logs /tmp/lightnvr-T12-*.log.
- **files edited/created**: `web/js/query-client.ts`, `web/js/components/preact/StreamsView.tsx`, `web/js/components/preact/UsersView.tsx`, `web/js/components/preact/settings/WorkspacesTab.tsx`, `web/js/components/preact/live/cameraFavorites.ts`, `web/tests/e2e/mutationPending.browser.spec.js`.

### T13: Distinguish session rejection from transport failure
- **depends_on**: [T1, T3]
- **agent_type**: worker
- **ownership**: `web/js/utils/auth-utils.ts`, `web/js/types/core.ts`, `web/js/components/preact/AuthGate.tsx`, new `web/tests/authSessionFailure.spec.ts`, `web/tests/e2e/authSessionFailure.browser.spec.js`.
- **description**: Reproduce logout on transient validation errors, then model an unavailable validation result separately from definite invalid credentials. Update dependent rendering only as needed; retain permission denial and real logout behavior.
- **acceptance_criteria**: Network timeout/5xx does not erase a valid session; definitive rejection/logout clears it; unknown status never elevates permissions or unlocks an initially protected page; retry and unmount are bounded. Scope changes outside ownership require coordinator reassignment first.
- **validation**: `npm --prefix web test -- --runInBand --testPathPatterns='authSessionFailure.spec.ts$'`; `npm --prefix web run test:e2e -- --testPathPatterns='authSessionFailure.browser.spec.js$'`; `npm --prefix web run typecheck`.
- **status**: Completed
- **log**: Reproduced transient 503 erasing auth. Explicit valid/invalid/unavailable results preserve verified sessions during outages while initially unknown pages remain locked with retry. Definite rejection clears state; generation/abort guards fence stale replies. Ten-second body-inclusive deadline, periodic single-flight checks, HttpOnly cookie handling and page lifecycle cancellation covered. Unit 20/20, Chromium 7/7, web typecheck and targeted ESLint pass. Approved one English retry-state locale key; ownership released.
- **files edited/created**: `web/js/utils/auth-utils.ts`, `web/js/types/core.ts`, `web/js/components/preact/AuthGate.tsx`, `web/tests/authSessionFailure.spec.ts`, `web/tests/e2e/authSessionFailure.browser.spec.js`, `web/public/locales/en.json`.

### T14: Adopt validated Streams contracts in the existing UI
- **depends_on**: [T8, T11, T12, T13]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamsView.tsx`, `StreamCard.tsx`, `StreamConfigModal.tsx`; `web/js/types/streams.ts`; `web/tests/streamSummaries.spec.js`; new `web/tests/contracts/streamsConsumer.spec.ts`.
- **description**: Use T8 adapters for list/full/update and replace duplicate wire models with generated aliases where appropriate. Keep create/test/ONVIF operations outside the pilot clearly typed and working. Preserve layouts, localization keys, Preact and browser preferences.
- **acceptance_criteria**: Summary/full/permission variants render correctly; loading/error/empty states and malformed payload behavior are explicit; form state is not blindly cast into DTOs; cancellation/status information survives the adapter.
- **validation**: `npm --prefix web run typecheck`; `npm --prefix web test -- --runInBand --testPathPatterns='streamSummaries|streamsConsumer|typeEscapes'`; `npm --prefix web run build`.
- **status**: Completed
- **log**: Streams list/full/update use validated adapters and generated wire aliases, explicit zero-preserving form mappings, per-camera control/redaction checks, safe retry/error states and truthful202queued feedback. Create/ONVIF and T11/T12 captured intent/pending behavior preserved. Targeted33/33, expanded locale51/51, existing Chromium13/13, typecheck and productionbuild pass. Approved narrow T11/T12 fixture and all18locale handoffs. Real Chromium consumer tests require root dependencies and pinned browser before any discovering gate; T19 must sequence CI accordingly, documented in contracts README. Source/build ownership released.
- **files edited/created**: `web/js/components/preact/StreamsView.tsx`, `web/js/components/preact/StreamCard.tsx`, `web/js/types/streams.ts`, `web/tests/contracts/streamsConsumer.spec.ts`, `web/tests/contracts/README.md`, `web/tests/e2e/streamCreateAfterEdit.browser.spec.js`, `web/tests/e2e/mutationPending.browser.spec.js`, `web/public/locales/{ar,de,en,es,fil,fr,it,ja,ko,nl,pl,pt-BR,pt-PT,ru,tr,uk,zh-TW,zh}.json`.

### T15: Add separately loaded read-only API explorer
- **depends_on**: [T3, T7, T13]
- **agent_type**: worker
- **ownership**: new `web/api-docs.html`, `web/js/pages/api-docs-page.ts`, `web/js/api/explorerSession.ts`, `web/css/api-docs.css`, `web/tests/apiExplorerSession.spec.ts`; `web/vite.config.js`; docs-entry-specific locale additions under `web/public/locales/`.
- **description**: Serve a session-based Swagger-style document viewer using the pinned standalone bundle. Fetch spec with existing-origin credentials and pass the object directly; no external validation, remote config/spec URLs, Try-it-out or credential persistence. Start with a documented direct URL rather than broad navigation edits.
- **acceptance_criteria**: No docs bundle loaded on Live/Streams/recordings pages; no vendor runtime passed through Preact aliases; loading/denied/partial/empty states usable and localized. Clear DOM/spec on logout, page lifecycle identity changes and before revalidation; reload after account change rather than retaining admin document. Use a monotonic identity/request generation and abort disposal so an old admin response completing after logout/account change cannot render. Hide Swagger credential-entry/Authorize controls as well as disabling submit methods.
  - Relative base resolution works for direct deployment and a fixture proxy prefix without trusting arbitrary forwarded headers. Set `supportedSubmitMethods: []`, `persistAuthorization: false`, `queryConfigEnabled: false`, and disable the external validator using the pinned version's documented configuration; assert no validation network call. Third-party requests and schema remote references are disabled. Static page contains no embedded personalized spec.
- **validation**: `npm --prefix web run typecheck`; `npm --prefix web test -- --runInBand --testPathPatterns='apiExplorerSession.spec.ts$'`; both `npm --prefix web run build` and `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build`; inspect normal-page network chunks.
- **status**: Completed
- **log**: Separate pinned byte-identical Swagger bundle, read-only controls/request interceptor/CSP, relative direct/prefix URLs, partial/error/empty states and18catalog translations. Generation/abort plus identity verification before/after fetch fences stale results; logout broadcast and30s visible checks clear rendered/store state, hidden pages/disposal cancel. Approved narrow auth-utils verifyURL/cache:no-store and credential-free BroadcastChannel seam. Final explorer19/19,typecheck,defaultbuild pass; auth/locale/typeEscapes59/59 and both profile/network checks passed before final verify cache option. T20 must rebuild both final profiles; detailed source/profile caveat /tmp/lightnvr-T15-handoff.json. Finaldefault installed14,467,751bytes; no vendor files/requests on Live/Streams/recordings. Shared normal closure increase roughly1.45KBraw is explicitly attributed in /tmp/lightnvr-T15-attribution.json, not claimed zero. T20 reviews cost. Defaultdist restored and lease released.
- **files edited/created**: `web/api-docs.html`, `web/js/pages/api-docs-page.ts`, `web/js/api/explorerSession.ts`, `web/css/api-docs.css`, `web/tests/apiExplorerSession.spec.ts`, `web/vite.config.js`, `web/js/utils/auth-utils.ts`, `web/public/locales/{ar,de,en,es,fil,fr,it,ja,ko,nl,pl,pt-BR,pt-PT,ru,tr,uk,zh-TW,zh}.json`, `docs/api/contracts/design.md`.

### T16: Exercise real-server contracts and principal isolation
- **depends_on**: [T2, T4, T7]
- **agent_type**: worker
- **ownership**: new `tests/contracts/streams.api.spec.ts`, `tests/contracts/discovery.api.spec.ts`, `tests/contracts/permissionCases.ts`.
- **description**: Run canonical/projection validation and independent policy assertions through the real HTTP server. Use separate contexts for sessions and cookie-free token requests; add an explicit mixed-credential precedence case. Do not run a shared mutation fixture concurrently.
- **acceptance_criteria**: Full T1 matrix covered; spec visibility matches direct calls and conditional fields. Forbidden/invalid edits leave DB-backed configuration unchanged. Revocation, expiration, membership changes and deactivation affect next requests. No personalized schema crosses user/token contexts, including success/error caching headers. Actual response shape/status/content type is tested, not just HTTP 200.
- **validation**: fresh native build; `LIGHTNVR_CONTRACT_BIN="$ENG_BUILD/bin/lightnvr" npx playwright test -c tests/contracts/playwright.config.ts --project=api --workers=1`; zero retries for acceptance, retained diagnostics on failure.
- **status**: Completed
- **log**: Fresh final T16 ASan/UBSan binary passes24/24 API tests in2.1m with zero retries; all22 T1 scenarios mapped to passing tests. Forty-six actual served documents independently linted and retained in build/eng-api-T16/http-projections. Direct wire/schema checks, denied/invalid write stability, empty/split/health scopes, token/owner isolation, expiry/revocation, membership, modes, CIDR/password gates, cookies and concurrent policy observations covered. Contract typecheck and canonical lint pass. Development fixture assumptions corrected against actual APIs; no product change. Exact test-source hashes/commands/results build/eng-api-T16/api-results.json, native source/binary evidence.json (source1082fea98da15bf840c9ec27ae15bad2ac86b34541f61719ad219e375c9393ca). Ownership released.
- **files edited/created**: `tests/contracts/permissionCases.ts`, `tests/contracts/streams.api.spec.ts`, `tests/contracts/discovery.api.spec.ts`.

### T14a: Honor per-camera policy in Streams row controls
- **depends_on**: [T14, T16]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamsView.tsx`, `web/js/components/preact/StreamCard.tsx`; regression in T17-owned browser tests.
- **description**: Repair the real-server regression where a policy user with legacy viewer role and camera.configure on A receives can_configure=true and successful PUT A, but cannot see Edit. Keep global creation authority separate.
- **acceptance_criteria**: Row controls follow server permission facts; fresh full-response checks remain enforced; camera B remains denied; unrelated global operations do not gain authority.
- **validation**: Scoped legacy-viewer regression, existing Streams component checks, full T17 browser suite.
- **status**: Completed
- **log**: Edit/enable/disable and row credential visibility use server camera permission; clone/delete/bulk/create retain separate existing gates. Real legacy-role 1 and 2 sessions save A through UI while B is unchanged/denied. Web typecheck, 33 consumer checks, 13 T11/T12 browser regressions and all 13 T17 browser tests pass; final default build passed.
- **files edited/created**: `web/js/components/preact/StreamsView.tsx`, `web/js/components/preact/StreamCard.tsx`, `tests/contracts/streams.ui.spec.ts`.

### T17: Verify the typed workflow and explorer in a browser
- **depends_on**: [T2, T14, T14a, T15, T16]
- **agent_type**: worker
- **ownership**: new `tests/contracts/streams.ui.spec.ts`, `tests/contracts/explorer.ui.spec.ts`; `tests/contracts/proxy-fixture.mjs`.
- **description**: Use the isolated real backend for Streams and personalized documentation. Add a private loopback reverse-proxy fixture for a prefix deployment. Intercept only deliberately malformed network responses; do not mock authorization success.
- **acceptance_criteria**: Viewer/configurator/admin sessions show correct operations and conditional models; logout/login as another user, back navigation, tab restoration and 401/5xx cannot retain another account's spec. Delay an admin response across logout/account switch and verify it is discarded, not briefly displayed. Explorer causes no mutation or external network requests. Streams controls retain form data on transient failure and reject invalid payloads usefully.
- **validation**: `npx playwright install chromium`; `LIGHTNVR_CONTRACT_BIN="$ENG_BUILD/bin/lightnvr" npx playwright test -c tests/contracts/playwright.config.ts --project=ui --workers=1`; keyboard navigation and narrow viewport smoke within the browser suite.
- **status**: Completed
- **log**: Full 13/13 real-server browser suite passes without retries on T16 native binary and immutable final asset snapshot /tmp/lightnvr-T17-final-assets-5dbqsk2a/dist, manifest build/eng-api-T17/final-assets.json digest90bb7bdf686f0e19d52842f97efa3840b58d7df007dd4ad876df4eca25cfc0ab. Covers direct/prefix docs, account switching, held responses, back/tab restoration, real revoked-session401, transient errors, malformed payloads, keyboard/narrow viewport and scoped role2 repair. Contract typecheck passed. Raw traces remain local because they can contain fixture credentials; T21 reruns after native memory repair.
- **files edited/created**: `tests/contracts/streams.ui.spec.ts`, `tests/contracts/explorer.ui.spec.ts`, `tests/contracts/proxy-fixture.mjs`, `tests/contracts/proxy-fixture.d.mts`, `tests/contracts/fixture-server.mjs`, `tests/contracts/fixture-server.d.mts`.

### T18: Type-check root integration and load tooling
- **depends_on**: [T21]
- **agent_type**: worker
- **ownership**: root `package.json`, `package-lock.json`, new `tsconfig.tests.json`; `.github/workflows/static-analysis.yml` after the pilot; `playwright.config.ts`; type-only repairs to a manifest of TS files under `tests/integration/`, `tests/load/`, `tests/contracts/` frozen at T21. New files produced by T22–T24 remain owned by those producers.
- **description**: After pilot acceptance, add root `test:typecheck` and its CI step, covering actual integration/load/config code separately from web TS. The pilot already checks its own contract tests through T3; this broader cleanup must not delay it. Preserve harness runtime behavior and installed tooling. Do not blanket-cast newly exposed bugs; record focused follow-ups if necessary.
- **acceptance_criteria**: Strict root check actually includes intended files, has a negative type fixture, and does not relax web settings. No test disappears from discovery. Any runtime repair is separated with a regression and coordinator-owned scope.
- **validation**: `npm run test:typecheck`; `npx playwright test --list`; `npx playwright test -c tests/contracts/playwright.config.ts --list`; `npm --prefix web run typecheck`.
- **status**: Completed
- **log**: Strict root test:typecheck covers all55 frozen files, future contract files and root config without relaxing web settings. Six negative fixtures produce six actual diagnostics when expect-error directives are removed. Root/web typechecks pass; discovery preserved213root/37contract before T22 expansion. Five existing TS modules erase to identical normalized JavaScript; approved URL pagination type uses only fields actually read, and two Playwright-only diagnostic Error assertions are narrowly scoped. Evidence build/eng-api-T18/typecheck-evidence.json. No package installs/upgrades, contract-test edits or runtime behavior changes.
- **files edited/created**: `package.json`, `tsconfig.tests.json`, `.github/workflows/static-analysis.yml`, `tests/typecheck/root-negative.ts`, `tests/integration/pages/StreamsPage.ts`, `tests/integration/specs/investigation.ui.spec.ts`, `tests/integration/specs/recordings.ui.spec.ts`, `tests/integration/specs/streams.ui.spec.ts`, approved `web/js/components/preact/recordings/urlUtils.ts`.

### T19: Run explicit contract jobs in CI
- **depends_on**: [T3, T7, T14, T17]
- **agent_type**: local
- **ownership**: `.github/workflows/integration-test.yml`, `.github/workflows/static-analysis.yml`, `.github/workflows/sanitizer.yml`; script interface changes are handed back to T3 rather than concurrently editing root package files.
- **description**: Add deterministic contract lint/drift/tooling and contract-test type checks and fresh-server API/browser contract execution. Reuse build variants and existing native labels rather than implying UI shards run every API spec. Preserve existing suites. No GitHub branch-protection or required-check changes.
- **acceptance_criteria**: Missing test/report or tool failure fails the job; wrong schema and generated drift demonstrated to fail; artifacts retain diagnostics without credentials. No camera/go2rtc dependency for JSON pilot. Sanitizer selection includes snapshot/projection/handler tests. Contract project cannot run twice via accidental broad glob discovery.
- **validation**: Run every added command locally on the integrated tree; inspect workflow discovery/build flags and script exit propagation. Use CI when available without claiming hosted execution merely from YAML validation.
- **status**: Completed
- **log**: Dedicated fresh-server contract job added with explicit API/UI projects, zero retries/forbid-only, native projection lint, fixture ownership checks and allowlisted credential-free status artifacts. Root tooling installed before web typecheck/browser-dependent unit tests. YAML parses; discovery shows37 contract tests separately from213 root integration tests. Local root/web npmci, Chromium install, all added contract commands and schema-negative/drift tool checks pass. Current Release fixture ownership4, API24, browser13, native ci-unit82 and independent projected-document lint pass; Debug ci-sanitizer57 passes. Web784tests/type/lint and20 related component browser checks pass. Artifact allowlist drops secret sentinels from errors/stdout/attachments and missing reports fail; real sanitized summaries in build/eng-api-T21/status. Debian host package provisioning and hosted Actions execution are not claimed locally; no repository required-check changes.
- **files edited/created**: `.github/workflows/integration-test.yml`, `.github/workflows/static-analysis.yml`, `.github/workflows/sanitizer.yml`.

### T20a: Reduce authorization snapshot duplication
- **depends_on**: [T5, T6, T16]
- **agent_type**: worker
- **ownership**: `src/database/db_authorization_snapshot.c`, `tests/unit/test_authorization_snapshot.c`.
- **description**: T20 baseline found three full camera arrays consume 66 MiB per 1024-camera request. Retain one inventory and compact observation facts while preserving distinct raw/runtime/live authorization inputs.
- **acceptance_criteria**: Observation semantics and memoization remain unchanged; allocation failures fail closed; memory improvement is measured on a fresh Release binary.
- **validation**: Snapshot/projection/handler native regressions, full contract matrix at integration, repeated T20 benchmark.
- **status**: Completed
- **log**: One inventory plus 96 bytes/camera of raw/runtime facts replaces three full inventories; source-derived storage drops 67,560 to 22,616 bytes/camera. Independent three-view reference compares 24 evaluation inputs/results after clock/source changes. Snapshot9/projector16/handler7/authorization45 cases pass under ASan/UBSan with leak detection. Coordinator reviewed all seven mutated fields and absence of camera-pointer caching. Fresh Release full/repeat benchmark passes with unchanged operation sets and SQL statement/row counts; 1024-camera whole-process high water drops 476.72 to264.22MiB. Runtime metrics allocation unchanged; full HTTP matrix reruns at T21.
- **files edited/created**: `src/database/db_authorization_snapshot.c`, `tests/unit/test_authorization_snapshot.c`, coordinator-owned wrapper registration in `tests/unit/CMakeLists.txt`.

### T20: Measure contract and explorer costs
- **depends_on**: [T2, T14, T14a, T15, T16, T20a]
- **agent_type**: worker
- **ownership**: new `tests/contracts/projection-benchmark.mjs`, `docs/api/contracts/performance.md`; T2 measurement-script extensions by serialized handoff.
- **description**: Compare baseline/default/modern assets, generated client/validator bytes, separate docs assets and installed total. Measure discovery latency, memory and query growth for declared synthetic fleet/principal sizes and concurrent requests. Include empty scope and large mixed scopes.
- **acceptance_criteria**: Report repeatable setup, sample distributions, source revision and raw artifacts; normal pages load zero explorer-only/vendor assets, with shared utility/rechunk byte changes explicitly reported and reviewed; identify disproportionate per-camera DB query growth. State measured deltas and proposed limits without calling a workstation result embedded-device qualification. Material unexplained cost blocks pilot acceptance until corrected or explicitly documented as an unresolved decision.
- **validation**: Build each profile and run `node scripts/measure-contract-assets.mjs --dist web/dist --profile default --out /tmp/lightnvr-eng-current-default.json` (repeat with modern/profile-specific output); `node tests/contracts/projection-benchmark.mjs --binary "$ENG_BUILD/bin/lightnvr"`; reproduce one run from the written commands.
- **status**: Completed
- **log**: Full 0/16/128/512/1024 matrix and focused16/1024 repeat passed before/after T20a. Final artifacts /tmp/lightnvr-projection-benchmark-X0XdYp/result.json and -YuKeSb/result.json identify Release binary08323d6a/source68a80fb0 in build/eng-api-T20a/evidence.json; query counts remain flat for nonempty fleets. Final T14a default/modern builds and real Chromium asset captures pass; zero explorer vendor loading on normal pages, explicitly accepted shared utility/rechunk cost. Final installed default14,467,958/modern10,685,341bytes. Default restored byte-identically; web-only source manifest22fb7b73 identifies files despite concurrent native work. Post-repair memory cost reviewed:22.09MiB camera data/request at1024,264.22MiB measured whole-process high water; unchanged metrics allocation and physical-device envelope remain explicit follow-ons, not unexplained pilot blockers. See performance.md for sample distributions, source hashes and interpretation limits.
- **files edited/created**: `tests/contracts/projection-benchmark.mjs`, `docs/api/contracts/performance.md`, approved bounded `maxStreams` option/declaration in `tests/contracts/fixture-server.mjs`, `tests/contracts/fixture-server.d.mts`.

### T21: Integrate and accept the permission-aware pilot
- **depends_on**: [T12, T13, T14, T17, T19, T20]
- **agent_type**: local
- **ownership**: `docs/API.md`, `docs/api/README.md`, `docs/internal/AUTHORIZATION_ENDPOINT_INVENTORY.md`, `docs/prd/ENG_01_Safety_Contracts_TypeScript.md`, `docs/prd/README.md`; integration-only edits to preceding outputs after owner handoff.
- **description**: Reconcile all pilot branches/slices, confirm E1–E3 acceptance, document new routes, mode exclusions, read-only explorer and partial coverage. Review snapshot and projection security boundaries locally. Do not use an old worker test result as evidence for the integrated source.
- **acceptance_criteria**: Fresh Release and sanitizer builds pass applicable tests; schema/type/generated outputs agree; entire principal/browser matrix passes without retries; original unit/UI coverage preserved; measured costs reviewed. Report blockers honestly and do not proceed with a claimed accepted pilot while they remain.
- **validation**: `npm run contracts:lint`; `npm run contracts:check`; `npm run contracts:test-tools`; `npm run contracts:typecheck`; `npm --prefix web run typecheck`; `npm --prefix web run lint`; `npm --prefix web test -- --runInBand`; all contract API/UI and T11/T12/T13 component browser suites (T9/T10 are independently validated, not pilot prerequisites); fresh CTest `ci-unit`/`ci-sanitizer` appropriate to configured variants; default/modern production builds; `git diff --check`.
- **status**: Completed
- **log**: E1–E3 pilot accepted locally on fresh Release08323d6a/source68a80fb0 and current Debug sanitizer targets. Release82/82 CTests, sanitizer57/57, API24/24, browser13/13, web68suites/784tests, focused component browser20/20 all pass without retries; lint/type/schema/generation/tool negatives and both web profiles pass. Snapshot/projector/authentication boundaries reviewed; T14a/T20a evidence included, costs/remaining device limits explicitly reviewed. Sanitized API/UI status and source/test/log hashes in build/eng-api-T21/acceptance.json; frozen T18 file list in root-types-manifest.json. PRD/API/inventory reconciled, relative links and diff-check pass. No hosted result or full-API/physical qualification claim; no commits.
- **files edited/created**: `docs/API.md`, `docs/api/README.md`, `docs/internal/AUTHORIZATION_ENDPOINT_INVENTORY.md`, `docs/prd/ENG_01_Safety_Contracts_TypeScript.md`; local acceptance and typing manifests in `build/eng-api-T21/`.

### T22: Expand contracts to identity and token lifecycle
- **depends_on**: [T21]
- **agent_type**: worker
- **ownership**: canonical/generated contract artifacts, `docs/api/contracts/coverage.json`; new `tests/contracts/identity.api.spec.ts`; narrow projector predicate additions only after T21 handoff.
- **description**: First E4 slice: actual login/verify/logout, safe client bootstrap and scoped-token lifecycle operations used by the UI. Enumerate exact methods in coverage before coding; keep user administration/TOTP extensions explicitly later if they exceed this slice. Model identity/self-service restrictions separately from camera actions.
- **acceptance_criteria**: Token owners cannot mint broader authority or rewrite policy through restricted tokens; one-time secrets never appear in ordinary responses/spec examples; documented auth alternatives match real routes. Public bootstrap is explicit in canonical schema without weakening authenticated discovery.
- **validation**: lint/generate/check both targets; fresh-server `identity.api.spec.ts` plus all existing contract API tests; `npm run contracts:typecheck` and web typecheck. Invalid and denied operations assert no side effects. Full-root typing runs at T29 after T18.
- **status**: Completed
- **log**: Eight exact identity/token routes added, total12operations/11paths. Reviewed simple non_scoped_identity metadata reuses snapshot credential classification; no new policy model or identity values emitted. Verify excludes scoped credentials; public logout negotiatesJSON/302 and does not revoke tokens; safe bootstrap accepts scoped credentials. Seven new real-server cases prove self-service/owner/requester containment, cross-user denial, scoped policy/token rejection, one-time secret exclusion, invalid-write/session state preservation and actual session lifecycle. Corrected fresh SSL-on Releasea6f26551/canonicalc7ec4f1a passes31API/82ci-unit; focused3ASan/UBSan tests and7fresh explorer browser cases pass. Tool6/schemafixture5/type/lint/generation checks pass; 0/16 benchmark smoke validates expanded independent operation assertions, not full-fleet qualification. Receipt build/eng-api-T22/acceptance.json; MFA completion/user administration/session-device extensions remain explicit later coverage.
- **files edited/created**: canonical/generated contract artifacts; `docs/api/contracts/coverage.json`, `docs/api/contracts/design.md`; `tests/contracts/identity.api.spec.ts`, `tests/contracts/permissionCases.ts`, `tests/contracts/discovery.api.spec.ts`, `tests/contracts/streams.api.spec.ts`, `tests/contracts/explorer.ui.spec.ts`, `tests/contracts/projection-benchmark.mjs`; `include/database/db_authorization_snapshot.h`, `src/database/db_authorization_snapshot.c`, `src/web/openapi_projection.c`, `scripts/contracts/generate_native.py`, `tests/contracts/tooling/generator.test.mjs`, native snapshot/projector/handler tests and coordinator-owned CMake wrappers.

### T23a: Authorize recording access by captured camera identity
- **depends_on**: [T22]
- **agent_type**: worker
- **ownership**: native recording list/count/facet/timeline query and detection-enrichment boundaries, demonstrated protection/deletion authorization boundaries, and focused native regressions, with coordinator-reviewed interface changes; T23 owns real HTTP regressions.
- **description**: T23 reproduced historical name reuse exposing a hidden camera's recording in list/count while direct lookup denies it. Carry authorized camera UUIDs into query predicates; use name fallback only for rows without captured UUIDs.
- **acceptance_criteria**: Hidden captured identities cannot leak through counts, rows, tags, detection labels or timeline name routes, or receive unauthorized protection/deletion. Renamed authorized captures remain visible under documented filters. Preserve explicit stream/collection intersection, empty scopes, legacy rows and bounded SQL batching; bind-budget failures remain closed.
- **validation**: Native regression tests plus T23 real-server name-reuse/rename cases and full contract matrix; sanitizer tests for affected native boundaries.
- **status**: Completed
- **log**: Isolated fixture with recording stream_name=A/camera_uuid=B returned total1 and hidden B metadata to an A-scoped viewer; direct recording GET returned403. A second scoped actor with only protect/delete on A received200 on hidden B protection and deletion, with actual durable row/file effects (sanitized proof build/eng-api-T23/capture-identity-write-repro.json). Approved UUID-carrying query variants, same-fleet collection intersection, detection identity joins and shared mutation preflight repair. Existing camera-label index covers captured UUIDs; measured legacy fallback still scanned 20,000 modern rows (140,051 SQLite VM steps), so migration0092 adds a partial legacy-label index (isolated query:65 steps). Actual migrated-DB regression and fresh upgrade qualification are required. No native edits by schema owner; dedicated worker owns repair and schema owner owns HTTP regressions.
- **accepted evidence**: Independent authorization/index review passed. Six affected Debug targets passed119 cases with ASan/UBSan/leak detection; seven new scope cases cover captured identity, legacy fallback, low bind budgets, partial-step errors and the migrated20k-row facet walk. Fresh Release passed all nine T23 HTTP/media cases, including denied historical-identity writes and mixed-batch no-effects, plus83 ci-unit CTests. Logs: `/tmp/lightnvr-T23a-final-{configure,build,tests}.log`; T23 retains its full API evidence separately.
- **files edited/created**: `src/database/db_{recordings,recording_tags,detections,fleet_query}.c` and matching `include/database/` headers; `src/web/httpd_utils.c`, `include/web/httpd_utils.h`; `src/web/api_handlers_{recordings_list,recording_tags,retention,timeline,recordings_backend_agnostic,recordings_batch_download,recordings_files_backend_agnostic}.c`; `tests/unit/test_recording_camera_scope.c`, `tests/unit/test_authorization.c`, `tests/unit/CMakeLists.txt`; `db/migrations/0092_index_legacy_detection_labels.sql`, `include/database/db_embedded_migrations.h`.

### T23: Expand contracts to recording reads and playback
- **depends_on**: [T22, T23a]
- **agent_type**: worker
- **ownership**: canonical/generated artifacts and coverage after T22; new `tests/contracts/recordingsRead.api.spec.ts`, recording contract fixtures under `tests/contracts/fixtures/`.
- **description**: Cover UI-consumed recording list/detail/timeline and playback metadata/routes, filtered counts, interrupted-media explanations and permission variants. Document byte-range behavior as protocol tests, not JSON validation.
- **acceptance_criteria**: Hidden cameras do not leak through counts/facets/timeline; stable recording IDs survive polling/retention; valid ranges return usable media and invalid ranges/errors match the contract; interrupted/unplayable status is not misrepresented as playable footage.
- **validation**: lint/generate/check both targets; fresh-server `recordingsRead.api.spec.ts` plus Streams/discovery regressions; FFmpeg/ffprobe verification of synthetic playback fixture; `npm run contracts:typecheck` and web typecheck.
- **status**: Completed
- **log**: Nine UI GET operations accepted: canonical now21 operations/20 paths. Fresh coherent Release/TLS build passes40 API cases (9 new recording cases),83 ci-unit tests and7 fresh-web explorer tests; root/contract/web typing, lint and drift pass. Actual H264/JPEG fixture, full/206/416 byte checks, reassembly/decode, historical UUID/legacy rename/count and denied mutation regressions pass. Receipt build/eng-api-T23/acceptance.json names native8429b0d3/source513488ea and exact artifacts/logs. Archive/HEVC variants are source-modeled, not remote/HEVC qualification; polling/sibling deletion is not retention-engine qualification. Unused manifest/play/files-check/protected-list schemas remain later. Benchmark0/16 smoke is not final full-fleet evidence.
- **files edited/created**: `docs/api/openapi.json`, `docs/api/contracts/coverage.json`, generated native/web contract artifacts; `tests/contracts/recordingsRead.api.spec.ts`, `tests/contracts/fixtures/recordings/{seed.ts,README.md}`, `tests/contracts/{permissionCases.ts,discovery.api.spec.ts,identity.api.spec.ts,explorer.ui.spec.ts,projection-benchmark.mjs}`, `tests/unit/test_api_handlers_openapi.c`.

### T24: Expand contracts to protection, deletion and export jobs
- **depends_on**: [T23]
- **agent_type**: worker
- **ownership**: canonical worker owns canonical/generated artifacts, coverage and independent projection expectations after T23; separate recording-boundary worker owns new `tests/contracts/recordingsActions.api.spec.ts`. Coordinate guard interfaces and serialize build/test leases through the parent.
- **description**: Add protection/retention override, deletion and export/job lifecycle operations consumed by the UI. Use independent policies and synthetic files to exercise mutation semantics and creator/token-bound job access.
- **acceptance_criteria**: Mixed unauthorized batches cannot partially mutate; progress/result access respects exact owner/token binding; protected media remains protected; actual pending/running/error/complete jobs and not-ready results have distinct documented states. These recording jobs expose no cancel route or cancelled state; document cancellation as unsupported rather than inventing it. Download/media behavior validated independently of JSON schemas.
- **validation**: lint/generate/check both targets; fresh-server recording action/read and full contract API suites; compare fixture file hashes/existence and metadata before/after denied operations; `npm run contracts:typecheck` and web typecheck.
- **status**: Completed
- **log**: Twelve action operations accepted, bringing partial canonical coverage to33 operations/30 paths. Final fresh Release fadd75c4 passes50 API cases,13 browser cases against immutable web assets and83 ci-unit CTests. Nine action cases verify actual ZIP CRC/content/decode, protected files, exact creator/token binding, mixed unauthorized no-effects, filtered-query failure versus empty success and genuine running/not-ready/error/complete states. Six schema groups retain10 reviewed request-boundary assertions; six tooling groups, lint, drift and root/contract/web types pass. No cancel route/state is invented. Counter semantics and ID/filter precedence are documented; unchecked ZIP read/write/close failures remain an explicit F16 follow-on, not a universal atomicity claim. Receipt: build/eng-api-T24/acceptance.json; pre-review failures/binary retained separately. No product C changes in this slice.
- **files edited/created**: `docs/api/openapi.json`, `docs/api/contracts/coverage.json`, generated native/web artifacts; `tests/contracts/recordingsActions.api.spec.ts`, `tests/contracts/{permissionCases.ts,discovery.api.spec.ts,explorer.ui.spec.ts,projection-benchmark.mjs}`, `tests/contracts/fixtures/schema/validate.mjs`, `tests/unit/test_api_handlers_openapi.c`.

### T25a: Repair recovery of recent recordings left by a crashed process
- **depends_on**: [T2]
- **agent_type**: worker
- **ownership**: `src/storage/storage_manager.c`, `include/storage/storage_manager.h`, `src/database/db_recordings.c`, `include/database/db_recordings.h`, `tests/unit/test_storage_reconcile_incomplete.c`; T25 harness changes remain with the same worker by handoff.
- **description**: T25 reproduced a fresh crashed row staying incomplete after restart. The only reconciliation call occurs at startup and excludes rows newer than one hour, so elapsed time alone cannot recover the row. Capture a recording-ID upper bound synchronously before this process starts recording/HTTP writers, then asynchronously reconcile only that preexisting set without the age exclusion. Preserve the ordinary stale/manual API and all media/protection/transfer/deletion guards. Verify IDs cannot be reused below the boundary; a failed boundary capture must not become an unrestricted scan. Do not lower the age cutoff while active writers run or capture the boundary inside the background scan.
- **acceptance_criteria**: Recent pre-boundary incomplete rows recover with honest playable/unplayable status; every post-boundary row remains untouched, even with an old start time. An empty boundary excludes all later rows. Pagination progresses through retained rows. Missing volumes, protected content and idempotence retain current guarantees. Startup ordering and failure behavior are documented and deterministically tested.
- **validation**: Coherent-FFmpeg Debug/ASan/UBSan build; existing and new `test_storage_reconcile_incomplete` cases; rerun T25 fresh crash/restart qualification without aging the crashed row. Preserve the age-only run as diagnostic history, not acceptance evidence.
- **status**: Completed
- **log**: Captured synchronous startup ID boundary before worker creation; explicit bounded paging never recaptures it, zero stays empty, capture failure skips scan. Ordinary manual stale API and existing retention/protection guards preserved. Startup ordering and AUTOINCREMENT supported-insert non-reuse traced and tested. Reconciliation 25/25 Unity cases (19 existing + 6 new) pass under ASan/UBSan, CTest 1/1. Fresh T25 crash recovery passes without timestamp aging. Reviewed implementation/ownership and evidence in tests/reliability/README.md. Separate existing pathname-collision/clock-rollback limitation recorded for follow-on scope.
- **files edited/created**: `src/storage/storage_manager.c`, `include/storage/storage_manager.h`, `src/database/db_recordings.c`, `include/database/db_recordings.h`, `tests/unit/test_storage_reconcile_incomplete.c`.

### T25: Qualify interrupted recording reconciliation end to end
- **depends_on**: [T2, T25a]
- **agent_type**: worker
- **ownership**: new `tests/reliability/interrupted-recording.py`, `tests/reliability/README.md`, `tests/reliability/results-schema.json`; may read existing storage reconciliation fixtures; product code read-only until a repair task is assigned.
- **description**: Use owned recorder/source processes and synthetic video, interrupt recording, restart and inspect files, DB and playback results together. Distinguish genuinely playable survivors from quarantined/explained failures; reuse #802 semantics.
- **acceptance_criteria**: Declared bounded restart/reconciliation completion; decodable surviving footage and timestamp evidence; unplayable files are visible with accurate explanation; protected content preserved; repeated restart is idempotent. Never claim byte-identical media or delete evidence to get green.
- **validation**: `python3 tests/reliability/interrupted-recording.py --binary "$ENG_MEDIA_BUILD/bin/lightnvr" --go2rtc ./go2rtc/go2rtc`; preserve source/recorder logs, ffprobe output and result JSON; run relevant existing reconciliation CTest suite.
- **status**: Completed
- **log**: Full fresh managed-go2rtc crash/restart scenario passed with no age injection at /tmp/lightnvr-recovery-b0yo0x2v/result.json: two decodable survivors, protected invalid_media crash file accurately returns 422, protected IDs/hashes and catalog survive repeated restart, integrity ok. Reconciliation 25/25 cases pass. Corrected coherent-FFmpeg/YAML Debug ASan/UBSan build manifest build/eng-api-T25/build-evidence-T25a.json; source digest 8b99f58f2f01d73f9415f06206415ed1b1e39864ac56ef9760ebe3d3423b284f. Original 60s blocked reproduction and age-only diagnostics retained separately. Fixture bounds, timing/GOP checks, owned-process cleanup and reusable interfaces documented; no physical-device or pathname-collision qualification claimed.
- **files edited/created**: `tests/reliability/interrupted-recording.py`, `tests/reliability/README.md`, `tests/reliability/results-schema.json`.

### T26: Qualify storage fault and protection behavior
- **depends_on**: [T1, T2]
- **agent_type**: worker
- **ownership**: new `tests/reliability/storage-faults.py`, `tests/reliability/storage-fixtures/`; unique report `docs/api/contracts/storage-fault-evidence.md`; no production storage mutations.
- **description**: Test full/read-only/unavailable storage using an isolated unprivileged container or test-local fault boundary; document which failures are kernel/filesystem-real versus injected. Never fill the host filesystem or remount user storage. Establish correct permission/protection outcomes before any lifecycle refactor.
- **acceptance_criteria**: Fault is actually observed (root bypass of chmod is not a read-only test); pressure/degraded state reported; no deletion of protected footage; recovery does not corrupt catalog/file identity. All fault restoration and cleanup occur even on timeout.
- **validation**: `python3 tests/reliability/storage-faults.py --binary "$ENG_BUILD/bin/lightnvr" --self-test`; execute declared isolated fault matrix with retained artifacts; existing storage retention/publication/reconciliation tests pass. Missing isolation backend is a blocked qualification, never a silent skip/pass.
- **status**: Completed
- **log**: Isolated four-fault server matrix and fault-boundary self-tests passed; injected ENOSPC/EROFS explicitly distinguished from kernel EACCES/missing-directory failures. Per-target degraded/pressure state, emergency fallback, protected file/catalog identity, recovery/restart/decode verified. Four storage CTests pass on corrected coherent-system-FFmpeg Debug ASan/UBSan build; stale host header/library mismatch diagnosed before acceptance. Exact flags, source/binary hashes and retained artifact paths in storage-fault-evidence.md. Cached aggregate health limitation documented; all fault restoration and owned-process cleanup completed. Product code unchanged by T26.
- **files edited/created**: `tests/reliability/storage-faults.py`, `tests/reliability/storage-fixtures/fault_boundary.c`, `tests/reliability/storage-fixtures/probe_boundary.c`, `tests/reliability/storage-fixtures/ffmpeg_abi_probe.c`, `tests/reliability/storage-fixtures/README.md`, `docs/api/contracts/storage-fault-evidence.md`.

### T27a: Close rejected go2rtc HTTP source responses
- **depends_on**: [T25, T26]
- **agent_type**: worker
- **ownership**: `go2rtc/internal/http/http.go`, focused test under `go2rtc/internal/http/`; T27 worker owns evidence and binary rebuild.
- **description**: Repair the demonstrated unclosed response body on non-200 synthetic source replies. Preserve the pinned submodule revision and record its working-tree patch separately; no commit or submodule pointer change is authorized.
- **acceptance_criteria**: Deterministic regression fails before and passes after the minimal cleanup; unchanged lifecycle descriptor gate passes with rebuilt go2rtc. Evidence records base SHA and dirty-source digest.
- **validation**: Focused Go HTTP package test and T27 reconnect qualification.
- **status**: Completed
- **log**: Three deterministic rejected-response cases failed before tcp.Close(res), then Go package race test passed. Unchanged real reconnect gate now keeps go2rtc descriptors24/24/24/24. Exact source/test patch and hashes retained in T27 evidence. No submodule commit/pointer change: delivering this repair later requires including its dirty submodule files, not only the parent diff.
- **files edited/created**: `go2rtc/internal/http/http.go`, `go2rtc/internal/http/http_test.go`.

### T27b: Preserve HTTP worker ownership during disconnect/shutdown
- **depends_on**: [T25, T26]
- **agent_type**: worker
- **ownership**: `include/web/libuv_server.h`, `src/web/libuv_connection.c`, `src/web/libuv_server.c`, new `tests/unit/test_libuv_handler_lifecycle.c`; coordinator registers tests in `tests/unit/CMakeLists.txt`.
- **description**: Fix demonstrated shutdown use-after-free while a handler still owns its connection. Defer destruction until worker completion and drain outstanding work before server/loop/TLS/global teardown. Preserve the three-second diagnostic warning without freeing a busy loop.
- **acceptance_criteria**: Deterministic disconnect and delayed-handler shutdown regressions pass under ASan, including work exceeding the old three-second drain window. Original T27 lifecycle workload meets its declared thirty-second bound. No claim that arbitrary blocking handlers have a universal termination deadline.
- **validation**: Focused native lifecycle regression and unchanged T27 process/media qualification.
- **status**: Completed
- **log**: Deterministic ASan reproduction confirmed connection freed before queued worker finished. Loop-owned close-completed state defers release; after-work avoids closed sends; stop drains before global teardown and destroy refuses busy-loop free. Reviewer caught never-started destruction cleanup, added and tested. Three native cases pass4.32s including delayed handler beyond3s warning; unchanged T27 overlap exits0 in4.12–4.47s. Original delegated worker was stopped by automated possible-cybersecurity-risk flag; coordinator completed repair locally and obtained independent review. Logs /tmp/lightnvr-T27b-{red,final}-test.log; final source/binary/test manifest build/eng-api-T27/evidence-final.json.
- **files edited/created**: `include/web/libuv_server.h`, `src/web/libuv_connection.c`, `src/web/libuv_server.c`, `tests/unit/test_libuv_handler_lifecycle.c`, `tests/unit/CMakeLists.txt`.

### T27: Qualify reconnect and bounded lifecycle
- **depends_on**: [T25, T26, T27a, T27b]
- **agent_type**: worker
- **ownership**: new `tests/reliability/lifecycle.py`, `docs/api/contracts/recovery-evidence.md`; reuse T25 fixture interfaces by handoff.
- **description**: Exercise disconnect/reconnect and concurrent stop/restart with synthetic input. Record selected ownership invariants where failures justify a follow-up C refactor; retain isolated fixture interfaces for the upgrade task.
- **acceptance_criteria**: Recovery/shutdown deadlines and allowed media-gap/timestamp tolerances are declared before runs; no dangling owned processes or sustained descriptor/thread growth across cycles. Repeated cycles preserve catalog/file identity and protected/playable media. No production databases or processes are touched.
- **validation**: `python3 tests/reliability/lifecycle.py --binary "$ENG_MEDIA_BUILD/bin/lightnvr" --go2rtc ./go2rtc/go2rtc`; native shutdown/stream lifecycle tests; preserve process/resource and media evidence.
- **status**: Completed
- **log**: Full three-phase/three-cycle qualification passed in /tmp/lightnvr-lifecycle-fj8enmir/result.json: completed-media recovery6.48–6.59s, flat descriptor/thread trends, manual request races, shutdown4.12–4.47s with exit0/no surviving owned children, protected identity/hash/media and DB integrity. JSON result schema passed; six related native CTests passed across recorded runs. Actual failures produced focused T27a/T27b repairs without relaxing bounds. Shared T25 fixture fresh-crash regression also passed at /tmp/lightnvr-recovery-ww9ab9j6/result.json. Exact build distinctions and final startup-cleanup regression documented in recovery-evidence.md. No arbitrary-handler deadline, clock-rollback/same-path or device qualification claim.
- **files edited/created**: `tests/reliability/lifecycle.py`, `tests/reliability/interrupted-recording.py`, `tests/reliability/README.md`, `docs/api/contracts/recovery-evidence.md`.

### T28: Qualify upgrade from a real prior release
- **depends_on**: [T2, T27]
- **agent_type**: worker
- **ownership**: new `tests/reliability/upgrade.py`, `tests/reliability/upgrade-fixture.json`, `docs/api/contracts/upgrade-evidence.md`.
- **description**: Resolve the prior release defined in prerequisites, build it in a task-owned checkout, create synthetic DB/config/media with that binary, and upgrade a copy using current source. Record source and binary hashes, schema versions and configuration with the fixture. Reuse lifecycle fixtures by handoff; never fabricate an old database by deleting columns from a current one.
- **acceptance_criteria**: Actual migration path completes, API/authorization semantics and protected/playable recordings remain correct, repeated current-version restart is idempotent. Failed upgrade leaves an untouched original fixture for recovery inspection. No downgrade compatibility claim. An unavailable prior toolchain/build is a blocked qualification with retained diagnosis, not an automatic skip.
- **validation**: `python3 tests/reliability/upgrade.py --binary "$ENG_BUILD/bin/lightnvr" --fixture-manifest tests/reliability/upgrade-fixture.json`; record the manifest-resolved prior build command; retain before/after DB integrity, API, config and media evidence.
- **status**: Completed
- **log**: Actual0.45.3 c82d6ed17c7e8ec9038bf4ebcdaa96d3b8cd8163 built in isolated checkout with exact historical Go/FAAC pins; genuine migration0089 installation recorded/protected synthetic media and scoped principals. Copy upgraded to0091, preserving playable media hashes/ID/protection, viewer/token allow/deny and denied-write state, integrity/FKs, and repeat-start persistent schema/history/config/rows. Entire original tree remained unchanged after cleanup. Coordinator caught report-schema fields missing; corrected and reran whole experiment. Final /tmp/lightnvr-upgrade-zdegojsx/result.json passes independent Ajv2020 result validation with harnessSHAecedb31b8a17821c05a4094f0aed40cc2467bcf6e12a665abd67a41012874db8. Old report-format failures retained unchanged. Known startup CREATE/DROP write-probe changes SQLite schema cookie by2; test asserts that separately, not arbitrary drift. WAL preserved/read, never ignored. Python/JSON/diff checks pass; no product changes.
- **files edited/created**: `tests/reliability/upgrade.py`, `tests/reliability/upgrade-fixture.json`, `docs/api/contracts/upgrade-evidence.md`.

### T27c: Deliver the go2rtc repair from a clean parent checkout
- **depends_on**: [T27a]
- **agent_type**: local
- **ownership**: new parent-tracked go2rtc patch and application script; Docker and existing CI go2rtc build steps; recovery reproduction documentation.
- **description**: A dirty submodule alone does not deliver T27a through a parent checkout. Preserve a reviewed patch plus regression and apply it before supported source builds without commits or submodule pointer changes.
- **acceptance_criteria**: Clean pinned source receives exactly the tested fix; already-applied source is unchanged; incompatible source fails before mutation; unrelated submodule edits remain untouched.
- **validation**: Clean isolated checkout patch/idempotence/mismatch checks, Go race regression, build-hook review.
- **status**: Completed
- **log**: Parent-tracked patch1d60c408 carries exact tested source/test; Docker and both existing CI Go build steps apply it with explicit patch dependency. Three delivery regressions verify actual application on metadata-free pristine source, idempotence/unrelated-edit preservation, and mismatch/partial-application failure before writes. The pristine test caught GNU patch reverse auto-detection; forced reverse dry-run fixes that false-positive. Fresh isolated pinned Go checkout matches original repaired files exactly, passes full HTTP race tests and all503/404/500 focused subtests, and builds production-style binary. Evidence build/eng-api-T27c-patch-evidence.json; shell/YAML/diff checks pass. No submodule commit/pointer change; no hosted/container-build claim.
- **files edited/created**: `patches/go2rtc/0001-close-rejected-http-responses.patch`, `patches/go2rtc/README.md`, `scripts/apply-go2rtc-patches.sh`, `tests/reliability/test_go2rtc_patch.py`, `Dockerfile`, `.github/workflows/integration-test.yml`, `docs/api/contracts/recovery-evidence.md`.

### T28a: Make recovery qualification portable to CI
- **depends_on**: [T27, T28]
- **agent_type**: worker
- **ownership**: new prior-build preparation script and manual CI workflow; reliability README/upgrade evidence by serialized handoff.
- **description**: Replace reliance on the workstation-specific prepared upgrade paths/hashes with an explicit pinned-source preparation command that records fresh build provenance, and provide opt-in CI execution of bounded recovery scenarios.
- **acceptance_criteria**: Genuine prior release/submodule pins remain enforced; only owned build/temp data is used; failed/missing reports fail execution; artifacts publish credential-free status; no fabricated old schemas or hosted-run claims.
- **validation**: Prepare/reuse a verified isolated historical checkout, run current upgrade with generated manifest, validate workflow commands and report handling locally.
- **status**: Completed
- **log**: New preparer verifies explicit reuse or clones/builds exact prior tag/gitlinks and records fresh provenance without fabricating schemas or overwriting dated evidence. Both paths and their real upgrade experiments pass; clean manifest build/eng-api-T28a-clean/upgrade-manifest.json, report /tmp/lightnvr-T28a-clean-upgrade/result.json independently validates. Historical Go stays unpatched; current T28 managed binaries remain hash-unchanged for these dated runs. Manual-only Debian workflow builds patched current/unpatched prior, runs four bounded scenarios unprivileged under private TMPDIR, rejects missing/failed reports, and uploads only allowlisted status/provenance/counts. Four report formats, five negative cases, secret-sentinel omission, Python/shell syntax, actionlint1.7.7 and diff checks pass. Hosted execution unobserved; T29 reruns final current source.
- **files edited/created**: `tests/reliability/prepare-upgrade.py`, `.github/workflows/reliability-qualification.yml`, `tests/reliability/README.md`, appendix in `docs/api/contracts/upgrade-evidence.md`.

### T29: Final integration and bounded follow-on plan
- **depends_on**: [T9, T10, T18, T21, T24, T25, T26, T27, T27c, T28, T28a]
- **agent_type**: local
- **ownership**: PRD/index, API documentation, authorization/coverage maps, integration workflows for accepted expanded tests; new `engineering-api-expansion-plan.md`; integration fixes only after task handoff.
- **description**: Re-run whole delivered system on integrated source, publish evidence and reconcile implemented versus accepted scope. Add appropriate CI/release execution for accepted identity/recording/recovery scenarios without branch restrictions. Break remaining storage/fleet/events/ONVIF/settings/system coverage into a new plan using actual pilot effort and remaining route inventory; keep Rust explicitly deferred.
- **acceptance_criteria**: T21 checks plus all new API/protocol/recovery tests pass on named build variants; no silent mocked substitutes, skipped qualifications, leaked credentials or new undocumented wire behavior. Remaining routes and field/device evidence are visible and not called complete. Any C boundary refactor requires its own focused plan backed by a demonstrated need.
- **validation**: Repeat T21 integrated validation plus T9/T10, `npm run test:typecheck`, and T22–T28 checks; `git diff --check`; verify docs links, full route coverage dispositions, generated drift, and exact workflow execution. Record hosted checks as observed, not inferred.
- **status**: Completed
- **log**: Final Release fadd75c4 and managed Debug cb68e8b3 manifests match the working tree (846/554 files checked). Passed 50 API tests, 13 browser contracts, 83 ci-unit targets, 58 sanitizer targets, 68 web suites/784 tests and 33 focused browser cases, plus typing, tooling, schemas, lint and generated/migration drift. Four final recovery reports validate against the same managed binary, including actual 0.45.3 upgrade 0089→0092 with original data untouched. Full and repeat projection runs through 1,024 cameras pass; measured high water is 267.90/267.98 MiB, with broader-contract costs documented. Both asset profiles and browser vendor isolation pass; all 261 default files restore byte-identically. Coverage exactly matches 240 registrations (33 covered/207 later); documentation links and whitespace checks pass. Four workflows pass actionlint 1.7.7 with shellcheck disabled; hosted runs remain unobserved. Reconciled the PRD/API evidence and reviewed the 17-task proposed expansion plan, including ZIP I/O, pathname collision and target-capacity limits. Receipt: build/eng-api-T29/acceptance.json. No commits, deployment, branch restrictions or Rust work.
- **files edited/created**: `docs/prd/ENG_01_Safety_Contracts_TypeScript.md`, `docs/API.md`, `docs/api/README.md`, `docs/api/contracts/{design.md,performance.md}`, `docs/internal/AUTHORIZATION_ENDPOINT_INVENTORY.md`, `.github/workflows/integration-test.yml`, `tests/contracts/asset-isolation.mjs`, `engineering-api-expansion-plan.md`, this execution log and retained local T29 receipts.

## Parallel Execution Groups

With four available agent slots, these are ready sets, not instructions to spawn
every task at once. Keep high-risk native integration local; use at most three
workers beside the coordinator. Prioritize the T1→T4→T7 contract path.

| Wave | Ready tasks | Can start when / write constraints |
| --- | --- | --- |
| 1 | T1 | Immediately; decisions precede implementation |
| 2 | T2 | T1 complete; capture baseline before toolchain/dependency mutation |
| 3a | T3, T5, T25a, T26 | T2 complete; T5 stays with coordinator; T25 qualification follows its evidence-driven T25a repair; T3 package installation is exclusive against all Node/browser commands |
| 3b | T4, T9, T10, T11, T13 | T3 complete; choose up to capacity, disjoint source ownership |
| 3c | T12 | T11 complete; T3 installations finished |
| 4 | T8, T27 | T8 requires T4; T27 waits for T25/T26; T5 may already be running |
| 5 | T6, T14 | T6 after T5; T14 after T8/T11/T12/T13 |
| 6 | T7 | T6 complete; shared native registration remains serialized |
| 7 | T15, T16 | T7 ready; T15 also needs T3/T13 |
| 8 | T17, T20 | T17 needs T14/T15/T16; T20 same plus T2 |
| 9 | T19 | T17 and pilot code/test producers complete |
| 10 | T21 | Pilot work and T20 complete; T9/T10/full-root typing are not prerequisites |
| 11 | T18 | T21 complete; exclusive root install/type cleanup; may overlap native-only work |
| 12 | T22 → T23 → T24 | T21 complete; serialized canonical/generated ownership; avoid Node tests during T18 dependency installation |
| 13 | T28 | T27 complete; isolated prior-release checkout |
| 14 | T29 | T9/T10/T18, T24 and operational T25–T28 complete |

An earlier ready task may overlap later waves only if its explicit dependencies
and exclusive resource/file ownership permit it. T2/T20 production web builds
write `web/dist` and generated version output: schedule those serially with
other Vite/browser tests, or use task-owned workspaces/output roots.

## Integration Strategy

- Preserve the completed TS conversion and integrate against current main.
  Recheck source drift before executing; adjust findings already repaired and
  record the actual implementation baseline without replaying old patches.
- T3 is the sole initial dependency/lockfile owner after T2 baseline. T18 edits root tooling after pilot acceptance. Every `npm ci`/install holds an exclusive coordinator scheduling lock against all Node/typecheck/build/browser consumers of that node_modules tree, including during T18.
  T5→T6→T7 serialize `tests/unit/CMakeLists.txt`; T11→T12→T14 serialize StreamsView.
  T4→T22→T23→T24 serialize canonical schema writes; generated output is never
  resolved by hand after conflicts.
- T5 is a local security/concurrency review point. A snapshot design that requires
  a broad database/evaluator rewrite is a scope blocker to re-plan, not license
  for an unattended architectural replacement.
- Keep behavior fixes, schema/tooling, discovery and explorer as reviewable
  slices. A runtime contract mismatch is investigated as a product decision,
  not silently normalized away or hidden by relaxing schemas.
- Prefix/proxy support is limited to the new docs surface and tested fixtures;
  do not claim the entire legacy frontend supports arbitrary subpaths.
- Store sanitized evidence with HEAD, tested-source digest, dirty-file manifest and configuration. Transient logs and
  generated test data stay in task artifacts rather than tracked source.

## Testing Strategy

- Independent policy assertions plus schema validation: valid JSON alone cannot
  prove authorization, redaction, or lack of unauthorized side effects.
- Native: Unity plus real SQLite for snapshot/evaluator behavior; actual C
  projector outputs independently parsed/linted; ASan/UBSan cleanup/error paths.
- API: fresh server, temporary DB, cookies and scoped tokens in separate contexts,
  deterministic permission transitions, actual status/content-type and cache
  headers, protected data/mutation assertions, no retries concealing failure.
- Browser: new isolated real-backend suite plus focused component reproductions;
  session switches, stale async completions, back navigation and docs asset
  isolation. Mocks are limited to intentional malformed response injection.
- Tooling: deterministic generation, deliberate schema/type drift, bad metadata,
  negative type/validator fixtures, root integration/load check and existing lint.
- Operational: synthetic media, actual playability and timestamps, storage faults,
  protected files, owned process shutdown, upgrade provenance. Report unsupported
  physical/long-soak evidence separately, never as a local pass.

## Risks and Mitigations

- **Risk**: Documentation and server authorization diverge.
  **Mitigation**: Canonical operation metadata, existing evaluator, immutable
  snapshot, same-resource conjunction tests and direct-call comparisons.
- **Risk**: Projection leaks privileged schemas or loses reference validity.
  **Mitigation**: Explicit variant policy, complete supported-reference traversal,
  cycle/pointer tests and independent validation of C-generated documents.
- **Risk**: Admin cookie silently overrides a restricted token in tests/UI.
  **Mitigation**: Session-only initial explorer; cookie-free API token contexts and
  a separate precedence test matching current auth resolution.
- **Risk**: Snapshot changes introduce deadlocks or change normal authorization.
  **Mitigation**: T5 local ownership, no nested public lock-taking helpers, immutable
  copies, opt-in evaluator context and old authorization regressions.
- **Risk**: OpenAPI/validator dialect differences or generated escapes weaken types.
  **Mitigation**: Pinned toolchain, dialect smoke fixtures, no coercion/defaulting,
  negative tests and a narrow generated-code policy rather than global exceptions.
- **Risk**: Swagger assets inflate every page or leak specs to external services.
  **Mitigation**: Separate standalone entry, disabled remote configuration/validator,
  no Try-it-out, strict request tests and measured installed/entry bytes.
- **Risk**: Parallel tests overwrite data, ports, build output or generated files.
  **Mitigation**: Task-owned native builds, isolated server fixtures, serialized
  shared web builds and lockfile/schema owners, bounded cleanup of owned resources.
- **Risk**: PRD expands into another giant foundation branch.
  **Mitigation**: T21 pilot acceptance, focused fixes, serialized small domain
  expansions, and T29 follow-on planning based on demonstrated costs.

## Reviewer Notes

- One independent reviewer agent reviewed this plan and relevant source without
  editing or implementing. Actionable findings were incorporated:
  - T1/T5 now cover health-dependent selectors, operation-specific resource
    preparation, bounded runtime sampling, fleet/collection loader ownership and
    a concrete snapshot feasibility checkpoint. SQLite consistency alone is not
    described as complete authorization consistency.
  - T2 baseline precedes T3 installation; frontend workers wait for T3; T18 later
    installations and shared generated/build outputs have exclusive scheduling.
  - Generated validator representation is explicit, outside handwritten `web/js`,
    with generator/drift/behavior checks and no relaxation of its TS policy.
  - T15 uses the correct locale path and rejects stale in-flight spec responses;
    T17 exercises the account-switch race.
  - T9/T10 and broad root typing no longer block T21. New contract tests have
    their own typecheck before the pilot; full root typing follows acceptance.
  - Lifecycle and historical upgrade are separate tasks T27/T28, each with its
    own fixture and evidence requirements.
  - Managed-media build mode, projection-lint CLI, asset arguments, baseline
    test discovery and source-tree evidence digests are explicit.
- The planning audit checked task IDs, required fields, dependency existence/cycles,
  reachability to final integration and shared-file ordering. Implementation
  status and validation evidence now live in each task's execution log above.

## Execution summary

All 37 tasks, including the evidence-driven repair and delivery follow-ups,
are complete and locally accepted. The final contract covers 33 operations across
30 paths; the coverage map explicitly leaves 207 other registered operations for
later. The original main baseline is historical; validation was recorded against
the working tree before the subsequent user-requested commit and PR creation.

Final evidence is collected in `build/eng-api-T29/acceptance.json`. The PRD and
API guides distinguish implemented behavior, actual local qualification and
remaining limitations. The [follow-on plan](engineering-api-expansion-plan.md)
is proposed work, not an instruction to execute another wave. No blocked task,
hosted-run claim, physical-device claim or Rust conversion is hidden in this
completion record.
