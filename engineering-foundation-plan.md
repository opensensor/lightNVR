# Plan: Security, Quality, and Typed Frontend Foundation

**Generated**: 2026-10-31
**Mode**: Swarm-ready dependency plan
**Source**: [Engineering foundation PRD](docs/prd/ENG_01_Safety_Contracts_TypeScript.md)
**Inspected baseline**: `main`, `1b8d4332` (PR #691's unified Live view)
**Execution status**: In progress; resumed under user authorization on 2026-10-31. T1–T4 and T6 were committed before resumption. Physical-device measurements in T5/T36 are deferred at the user's request; this is not passing qualification evidence.

## Overview

Strengthen the existing C/Preact product through enforceable findings checks,
strict TS/TSX, executable API contracts, a Streams list/edit pilot, operational
regressions, resource budgets, and one justified C boundary improvement.

The foundation and pilot have concrete tasks below. Three discovery gates expand
work that cannot honestly be enumerated yet: actual scanner findings (T7), the
remaining application rollout after measuring the pilot (T25), and the selected
C improvement (T31). These tasks produce **additional executable task records in
this plan**, not permission for a worker to undertake a whole backlog. Their
integration gates cannot run until the new dependencies have been inserted and
completed. P6 is not complete merely because the scaffolding or pilot passes.

Keep shared manifests, generated outputs, workflow integration, and repository
rules under a local integrator. Parallel workers own isolated implementations or
fixtures. Prefer a maximum of three active workers; the coordinator occupies the
fourth available slot. Do not parallelize tests that share ports or output paths.

## Assumptions

- Preserve C, Preact, Vite, Jest, existing public behavior, and stored data. Rust,
  React adoption, a new test runner, and a database redesign are outside scope.
- The dirty working tree contains the source PRD/index and unrelated documents,
  investigations, `homeassistant/`, and other local files. Workers are not alone:
  edit only assigned files directly, never reset/revert unrelated work, and report
  changed paths. No commits, pushes, PR publication, or production deployment are
  required unless separately requested.
- Confirm the working revision and changed paths before starting each task; adapt
  this plan if main advances. Do not reimplement PR #691 or restore WebRTCView.
- Every new path/command named below is a **planned deliverable**, not an existing
  feature. Its producer task is identified. Names may change only through a
  coordinated plan update covering every consumer.
- P0 records actual findings, targets, workload limits, and measurements. No alert
  count, fixed dependency version, or performance result is assumed here.
- Default target coverage starts with amd64, arm64, arm/v7 and both current web
  build modes. Additional embedded/MIPS requirements need an explicit target
  record. Missing hardware blocks its acceptance evidence, not unrelated work.
- Preserve `web/package.json`'s Node 24 range and installed major versions. Select
  compatible pinned additions; do not upgrade Preact/Vite/Babel/Jest incidentally.
- The pilot is list/edit only. `StreamsView.jsx` also handles create/delete,
  bulk operations, discovery, and ONVIF: extract a typed feature boundary when
  needed rather than making those flows unplanned pilot requirements.
- Existing tests sometimes read literal `.jsx` source paths or bundle modules
  with extension-based stubs. Rename planning includes those consumers and the
  path-keyed i18n baseline; a successful Vite build is insufficient evidence.

## Prerequisites

- Node from `.nvmrc`, npm lockfiles, Python 3, CMake, a C/C++ compiler, and the
  dependencies in the current workflows. In particular current CMake requires
  FFmpeg development libraries at the versions declared there, SQLite, libuv,
  llhttp, curl, TLS dependencies, and initialized FAAC/Unity sources.
- Go, FFmpeg/ffprobe, a built go2rtc, and Playwright Chromium for media/UI runs;
  optional feature matrices also need their actual dependencies. Lean builds
  cannot qualify omitted production features.
- Read access to GitHub findings/workflow artifacts and dependency/container
  reports for T1. Insufficient access is recorded explicitly; an empty accessible
  result is not proof that all private findings were inspected.
- Repository administration is needed only to configure/verify required checks
  in T38/T35. Prepare a concrete ruleset diff first; use existing authorization
  or request the missing authority before changing remote settings.
- A named constrained device and controlled workloads for T5/T36. Keep physical
  performance evidence distinct from cross-builds and emulation.
- All integration/upgrade/fault tests use disposable storage and synthetic
  streams. No production camera credentials or recordings enter fixtures.

### Shared ownership and validation conventions

- T8 initially owns both package manifests/lockfiles and shared web configs;
  T15, explicit local T25.* integration checkpoints, T26, then T34/T35 own
  subsequent coordinated changes. Other workers
  submit dependency/config needs to that owner instead of editing shared files.
- T10 owns lint rule compatibility and establishes extension-independent source
  identity before conversion. A JSX-to-TSX rename therefore retains the same
  allowance without waiting for a later integration task. Never regenerate a
  larger i18n allowance or let two implementations share duplicate allowances.
- T12 owns generation tooling and the initial generated output. Later schema
  workers edit assigned fragments only. Explicit local T25.* checkpoints
  aggregate/generate outputs before dependent consumer tasks begin; T26 performs
  final reconciliation after those tasks. Concurrent code generation is prohibited.
- Keep small checked JS re-export shims at old import paths during a migration
  wave when necessary. They contain no duplicate implementation, are inventoried,
  and T34 removes them or verifies narrowly justified interop exceptions.
- Each task writes evidence to its own `quality/evidence/T<n>.md`; raw reports
  go under ignored `build/eng-artifacts/T<n>/`. Evidence must omit secrets.
  Only the coordinator updates task statuses/logs and consolidated inventories.
- Run all commands from the repo root. T13 creates `run-native.py` and
  `run-integration.mjs` wrappers: they isolate build directories, configurations,
  ports, PIDs, database/storage paths, and report directories by task ID, fail
  on zero selected tests, and forward subprocess failure codes. Until that is
  proven, all server/media runs are serial under the coordinator.
- `npm ci`, Vite builds, code generation, and whole-tree checks need an exclusive
  validation lease in a shared checkout. Workers can run isolated tests while
  unrelated edits continue, but final evidence comes from a stable integrated
  snapshot. Do not attribute another worker's intermediate failure to your task.

### Mandatory expansion protocol

T7, T25, and T31 are bounded **planning/integration preparation** tasks. For each:

1. Append full task records in the format below with stable IDs: T7.1, T7.2, etc.;
   T25.1, etc.; T31.1, etc. Give every task exact paths, dependencies, assertions,
   and root-runnable validation. Split a fix by root cause and a migration by
   a small component/helper cluster; normally no more than six application
   implementation files per worker. A single large view may be its own task.
2. Assign related tests, schema fragments, and all extension-sensitive consumers.
   Put shared imports, schemas, generation, and manifests in explicit local
   integration tasks. Serialize overlaps with existing tasks, including pilot
   files and native operational tests.
3. Update the dependency graph and wave table. Add every leaf/integration task
   to the designated gate's `depends_on`: T33 for T7, T26 for T25, T32 for T31.
   Validate all IDs, acyclicity, ownership, artifact producers, and resource
   leases **before marking the expansion task completed**.
4. Execute the resulting tasks under the later executor's authorization. A gate
   refuses completion when expansion records are absent, unresolved, or merely
   marked skipped. Zero remediation tasks is valid only with complete evidence
   that no actionable findings remain. T25 must account for every remaining file.

This makes unknown work explicit without assigning an unbounded rewrite to one
agent or letting a nominally completed planning task satisfy implementation.
If T9 discovers substantial pre-existing test typing work, apply the same rules
to T9.* tasks and insert their terminal IDs into both T13 and T15 before marking
T9 complete. The initial producer T8 must already provide the test-check command.

## Dependency Graph

```text
T1 findings ──> T6 policy ──> T14 scanners ────────────────┐
T1,T2,T3,T4 ─> T7 expand fixes ──> [T7.*] ──> T33 ──────┤
T2 inventory ──> T11 contract ──┐                       │
T2,T4,T11 ───> T8 toolchain ───┼─> T12 generation ──┐   │
                        ├─> T9 test typing ──┐       │   │
                        └─> T10 lint ────────┤       │   │
T3 coverage ───────────────────> T13 harness ┤       │   │
T4 assets ──> T5 device budgets              └─> T15 foundation
T15 ──> T16 fetch ──> T19 query ──┬─> T20 form ──┐
    ├─> T17 async primitives ─────┤             ├─> T22 pilot wiring
    └─> T18 C contracts ──────────┴─> T21 card ──┘       │
                                               T23 UI ──> T24 pilot gate
T18 ──> T39 contract CI ──> T38 required-check enforcement ──> T24
T24 ──> T25 expand rollout ──> [T25.*] ──> T26 integration ──> T34 strict closure
T3,T13 ──> T27 media; T28 storage; T29 restart/upgrade ──> T30 soak
T3,T7 ───> T31 expand C boundary ──> [T31.*] ──> T32 integration
T5,T27,T28,T29 also feed T32; T38 also feeds T35
T30,T32,T33,T34 ──> T35 final CI ──> T36 target qualification ──> T37 acceptance
```

The task records below are authoritative where this summary omits a supporting
edge. Bracketed expansion families are not runnable placeholders; their full
records and gate dependencies must be added by the named producer.

## Tasks

### T1: Inventory current security and quality findings
- **depends_on**: []
- **agent_type**: explorer
- **ownership**: `quality/findings.json`, `quality/scanners.json`, `quality/evidence/T1.md`; production code read-only.
- **description**: Collect current analysis/artifacts and known issues; separate application, optional-feature, build-only, third-party, and container findings. Record scanner revision/scope, stable identity, severity/exposure, owner, disposition, evidence, and exception expiry. Mark inaccessible reports as missing. Define the expected scanner-run manifest for T6.
- **acceptance_criteria**:
  - No invented clean baseline; each collected item is traceable and each expected scanner has coverage/access status.
  - No secrets or private vulnerability details in public files; use sanitized IDs for private records.
- **validation**:
  - `python3 -m json.tool quality/findings.json`
  - `python3 -m json.tool quality/scanners.json`
  - Manually reconcile scanner counts/fingerprints and configurations with source reports.
- **status**: Completed (commit `ff6a7d60`)
- **log**: Non-testable inventory; verified by json.tool and an independent recount from raw reports (CodeQL 34 open+11 dismissed, clang-tidy 296, cppcheck 605, eslint 897, 743 unique fingerprints). 384 itemized findings + 359 buckets; 13 expected scanners with access status. Gaps: secret scanning disabled (404, recorded as not scanned), no JS/TS CodeQL, CodeQL not on PRs, all scanners non-blocking, no required checks, go2rtc/SOD excluded from C analysis, Trivy amd64 only, no CODEOWNERS (placeholder owners). Notable for T7: CodeQL `cpp/sql-injection` (sqlite_migrate.c), uncontrolled allocation (event_router.c), path injection, TOCTOU, cleartext credentials; cppcheck OOB in stream_protocol.c, overflow in config.c; 2 private high-severity advisory drafts need maintainer status. Gotchas: static reports are from `1b8d4332` (+7 line offset after SPDX headers; fingerprints are line-independent); npm audit exits 1 on any finding while C scanners/Trivy force exit 0; workflow artifacts expire 2026-11-30; proposed exception expiry 2027-04-30 needs maintainer confirmation. Raw reports in `build/eng-artifacts/T1/`.
- **files edited/created**: `quality/findings.json`, `quality/scanners.json`, `quality/evidence/T1.md`

### T2: Map application, API, and rename ownership
- **depends_on**: []
- **agent_type**: explorer
- **ownership**: `quality/application-inventory.json`, `quality/api-inventory.json`, `quality/evidence/T2.md`; code read-only.
- **description**: Inventory all first-party `web/js` files, imports, entrypoints, tests, source-reading assertions, schema consumers, and shared UI. Enumerate actual C method/path routing and UI usage. Identify Streams summary/full/role variants and list/edit dependencies, including client-config and detection-model options. Record the Live compatibility behavior from PR #691.
- **acceptance_criteria**:
  - Every browser file has a proposed domain owner and every UI operation maps to real routing or a recorded discrepancy.
  - Explicitly identify shared i18n, auth, toast, modal, ZoneEditor, query, and extension-sensitive test dependencies.
- **validation**:
  - `python3 -m json.tool quality/application-inventory.json`
  - `python3 -m json.tool quality/api-inventory.json`
  - `rg --files web/js web/tests tests/integration`
  - Compare inventory completeness to the file list and route registration, not documentation alone.
- **status**: Completed (commit `8d715d7f`)
- **log**: Non-testable inventory; verified by json.tool and a completeness check (302/302 `rg --files` paths, 239/239 C routes from `register_all_libuv_handlers()` in `src/web/libuv_api_handlers.c`, 297/297 UI operations matched via a port of the C matcher). 193 web/js files across 16 domains; 4 unreachable files (`Toast.jsx`, `common/Image.jsx`, `url-param-handler.js`, `utils/dom-utils.js`); 10 discrepancies D-01..D-10. Streams pilot detail in `api-inventory.streams_pilot`: `/full` needs only live-view permission and has configure vs live-only redaction variants; PUT checks permission before body and returns 202; UI sends ignored `is_deleted`, reads unsent `detection_zones`, and reads `error_message`/`playback_transport` absent from admin list; UI gates editing by role while server is per-camera. Extension-sensitive consumers for T8/T10: source-reading specs (streamAvailability→StreamsView.jsx, i18n→Footer.jsx, open-issues-regressions→InvestigationView.jsx), esbuild `.jsx` bundling specs (systemHealth*, liveSlideshowControl, platesView with 6 filename-matched stubs), extension-filtered web/js walkers (browserDialogs, locale-coverage), 48 spec files with explicit-extension imports, Jest `*.spec.js`-only matching, ESLint `.js/.jsx` only, i18n baseline path-keyed. Live (#691): variant/storage prefixes and query keys must stay stable. Gotchas: router is first-match, unmatched `/api/*` → 404 not 405; Playwright `go2rtc` project matches no files; edit form uses raw `fetch` for `/full` and zones; `web/js/version.js` is generated/ignored.
- **files edited/created**: `quality/application-inventory.json`, `quality/api-inventory.json`, `quality/evidence/T2.md`

### T3: Map operational coverage and reproduce native baseline
- **depends_on**: []
- **agent_type**: explorer
- **ownership**: `quality/operational-coverage.json`, `quality/native-profiles.json`, `quality/evidence/T3.md`; code read-only.
- **description**: Map PRD scenarios to actual executed tests/jobs. Inspect media recovery, storage, shutdown, upgrade, stream state, and `/proc` seams. Record missing cases, production feature combinations, numerical scenario limits requiring calibration, and candidate bounded C improvements. Archive baseline native binaries/configuration and hashes under `build/eng-baseline/1b8d4332/native/`, with toolchain, dynamic-library/container provenance, and feature flags; record which target each artifact can actually run on.
- **acceptance_criteria**:
  - Existing test names/build flags are captured; mocks, real media, and hardware evidence are distinguished.
  - Baseline failures are recorded before refactoring; omitted features are not counted as covered.
- **validation**:
  - `cmake -S . -B build/eng-T3 -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON -DENABLE_SOD=OFF -DENABLE_LITERT=OFF -DENABLE_GO2RTC=OFF -DENABLE_MQTT=OFF`
  - `cmake --build build/eng-T3 -j2`
  - `ctest --test-dir build/eng-T3 --output-on-failure --no-tests=error -R 'test_(system_health_proc|stream_state|shutdown_coordinator|mp4_audio_recovery|mp4_writer_watchdog|storage_retention_sqlite)$'`
- **status**: Completed (commit `561d235c`)
- **log**: Built HEAD `7db92e0d` (compiled sources identical to `8927bc8a`; diff from `1b8d4332` is SPDX comments only), archived under the plan's `1b8d4332` path. As-specified lean Debug: 146/153 pass, targeted 5/6 (`test_mp4_audio_recovery` fails). All 7 failures are a host defect: stale FFmpeg 7.1 headers/.pc in `/usr/local` with libs moved away, so builds compile against 7.1 and link 8.0.1 (avcodec_open2 -22, ASan in av_buffer_unref). Control build with system pkg-config/headers: 153/153 and 6/6 — no code baseline failures. Archives: `debug-lean-as-specified/` (failure repro only), `debug-lean-system-ffmpeg/` (functional/sanitizer baseline), `release-full-system-ffmpeg/` (SOD/LiteRT/go2rtc/MQTT; needs `LD_LIBRARY_PATH=bin/`; approximate T5 reference, not the shipped container). x86_64 only; no arm/MIPS/device evidence. Coverage: 59 registered lean tests never run in CI; go2rtc/SOD tests never run; Playwright CI runs only UI project; no scheduled workflows; recovery tests are mostly link-time mocks. Missing cases recorded for T27 (real disconnect deadlines, post-recovery playability/timestamps, video-only/AAC, retry-exhaustion cooldown, segment metadata), T28 (ENOSPC/EIO, read-only/unmounted, retention modes, protected-file real delete path), T29 (shutdown deadline vs 900 s watchdog, SIGKILL recovery — `storage_reconcile_incomplete_recordings()` untested, upgrade fixtures absent). T31 candidates: stream state transition table; system API `/proc` parsing onto existing collectors (lowest risk); go2rtc_process `/proc` seam; reconcile policy/I-O split; injectable disk-space check. Gotcha for T13: runner must pin FFmpeg paths or fail on header/library version mismatch.
- **files edited/created**: `quality/operational-coverage.json`, `quality/native-profiles.json`, `quality/evidence/T3.md`

### T4: Measure and gate web asset budgets
- **depends_on**: []
- **agent_type**: worker
- **ownership**: `scripts/quality/assets.mjs`, `quality/web-budgets.json`, `quality/evidence/T4.md`, `tests/quality/assets.test.mjs`; no manifest/Vite changes.
- **description**: Build the unchanged frontend under an exclusive lease. Implement a dependency-light reporter/checker for default and modern-only outputs: installed bytes including compressed copies/locales, per-entry compressed JS/CSS, shared/lazy chunks, and dependency composition. Archive both outputs, configuration, and hashes under separate `build/eng-baseline/1b8d4332/web-default/` and `web-modern/` directories before subsequent builds overwrite dist. Record exact commands, commit, flags, and justified numeric budgets.
- **acceptance_criteria**:
  - Baselines predate TS/toolchain changes; byte counting does not double-count shared chunks or confuse installed/transfer costs.
  - Checker rejects missing/stale artifacts and a deliberately oversized fixture; budgets cannot silently self-update.
- **validation**:
  - `npm ci --prefix web`
  - `node scripts/quality/assets.mjs --capture --profile default`
  - `node scripts/quality/assets.mjs --capture --profile modern`
  - `node --test tests/quality/assets.test.mjs`
- **status**: Completed (commit `f1ab93b5`)
- **log**: RED (module-not-found, `build/eng-artifacts/T4/red.log`) → GREEN 9/9 node:test (no double-counted shared chunks; missing/stale/wrong-profile/oversized/unknown-budget failures; budgets file unchanged). Real `--check` passes on fresh builds (default 66 budgets, modern 43) and fails on wrong profile, newer source, deleted chunk. Profiles: default = `npm run build` (modern+legacy, 18 locales); modern = `LIGHTNVR_WEB_LEGACY=false`; capture scrubs `LIGHTNVR_WEB_*`/Sentry/sourcemap env. web/ is identical to `1b8d4332` (default captured at `7db92e0d`, modern at `ff6a7d60`; capture refuses if web/ differs). Builds are byte-reproducible. Baselines: installed 11.27 MB default / 7.90 MB modern; served modern JS 604,760 / 584,455; Live first load 451,434 / 425,660. Budgets: totals +3% (min 4 KiB), per-entry +5% (min 2 KiB). Gotchas: no Vite manifest (checker parses HTML/JS refs); web/dist holds one profile at a time, so check right after building that profile and run `npm ci` before building (staleness); no lazy chunks (hls.js in shared Live chunk); ~118 KB unreferenced `css/*.css` copies installed; npm ci skips esbuild/core-js/unrs-resolver install scripts.
- **files edited/created**: `scripts/quality/assets.mjs`, `quality/web-budgets.json`, `quality/evidence/T4.md`, `tests/quality/assets.test.mjs`

### T5: Establish constrained targets and runtime budgets
- **depends_on**: [T3, T4]
- **agent_type**: local
- **ownership**: `quality/target-profiles.json`, `quality/runtime-budgets.json`, `quality/evidence/T5.md`.
- **description**: Name the constrained NVR and viewing browser, supported architecture/feature profiles, workload, stream fixture parameters, cold/warm cache states, and repeatable measurement method. Use immutable T3/T4 baseline artifacts, or build the pinned baseline in an isolated checkout for additional targets; never measure an evolving checkout or overwritten web/dist as the original baseline. Capture browser and NVR resource baselines separately; settle required MIPS/non-container coverage with available project context or one focused clarification.
- **acceptance_criteria**:
  - Numeric limits and tolerances are justified by measurements; no placeholder target or fictitious physical result.
  - Missing hardware is an explicit blocker for target acceptance; unrelated development continues.
- **validation**:
  - `python3 -m json.tool quality/target-profiles.json`
  - `python3 -m json.tool quality/runtime-budgets.json`
  - Repeat the recorded workload on the named device/browser and compare samples under the declared method.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T6: Implement finding identity and exception policy checks
- **depends_on**: [T1]
- **agent_type**: worker
- **ownership**: `scripts/quality/findings.py`, `tests/quality/test_findings.py`, `tests/quality/fixtures/findings/`, `quality/findings-policy.json`, `quality/evidence/T6.md`.
- **description**: Normalize the inventoried report formats and compare stable findings against reviewed dispositions. Distinguish a valid empty result from missing/crashed/empty-scope scans. Handle exception expiry, tool/database provenance, rename stability, and explicit baseline edits.
- **acceptance_criteria**:
  - A new finding fails even when another disappears; stale exceptions and missing expected reports fail.
  - Reported scanner failure cannot be converted into success by a pipeline exit-code mistake.
- **validation**:
  - `python3 -m unittest discover -s tests/quality -p test_findings.py`
  - `python3 scripts/quality/findings.py --check --reports build/eng-artifacts/T1 --inventory quality/findings.json`
- **status**: Completed (commits `29eef189`, `27e4d57e`)
- **log**: RED (`ModuleNotFoundError: findings`) → GREEN 58/58 unittest. Stdlib checker normalizes CodeQL/Trivy SARIF, clang-tidy, cppcheck XML, eslint JSON/job log, npm audit, GitHub Dependabot/Code Quality/advisory/secret-scanning API files, reproducing T1 fingerprints. Identity comparison (new fingerprint or bucket increase fails regardless of disappearances); per-format scan-failure detection (truncated clang-tidy, cppcheck critical errors, npm audit error/zero deps, saved API error bodies, empty scope); exit codes 0 pass / 1 policy / 2 usage / 3 internal (never pass); provenance changes printed; renames only via explicit `path_renames`; `--check` never writes (hash-tested), `--update-baseline --reason` is the sole writer; `--release` ignores gaps and fails needs-triage. Real T1 check exits 0 with all fingerprints matched and 4 accepted, owner/expiry-bound coverage gaps printed as NOT SCANNED (cppcheck partial analysis, secret scanning disabled until 2026-12-31, no JS/TS CodeQL until 2027-01-31, sanitizer log not downloaded until 2026-11-30); without gaps / `--today 2026-12-01` / `--release` it fails truthfully. Gotchas: added `fixed-verified` disposition for T33; gap owners/expiries are proposals needing maintainer confirmation; T14 should emit `run-manifest.json` (files scanned, tool/DB versions) and then set `require_run_manifest: true`; line-independent identity behaves like a per-file count for duplicate diagnostics; eslint log fixture force-added past `*.log` ignore. Raw outputs in `build/eng-artifacts/T6/`.
- **files edited/created**: `scripts/quality/findings.py`, `quality/findings-policy.json`, `tests/quality/test_findings.py`, `tests/quality/fixtures/findings/`, `quality/evidence/T6.md`

### T7: Materialize atomic remediation tasks
- **depends_on**: [T1, T2, T3, T4]
- **agent_type**: local
- **ownership**: this plan and `quality/remediation-map.json`; production code read-only.
- **description**: Apply the mandatory expansion protocol. Create one T7.* task per root cause, with exact affected files, reproducer, severity priority, and regression check. Separate dependency updates from code fixes. Serialize work overlapping pilot files, C tests, or shared configs. Add local merge/verification tasks and add their terminal IDs to T33.
- **acceptance_criteria**:
  - All actionable inventory entries map to executable bounded tasks or reviewed upstream exceptions; critical fixes need not wait for TS migration.
  - New tasks have no broad `src/**`/`web/**` ownership. T33 is fenced by explicit dependencies.
- **validation**:
  - `python3 -m json.tool quality/remediation-map.json`
  - Coordinator checks graph, ownership, evidence producers, and finding-to-task completeness before releasing fix workers.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T8: Establish strict TS, package, and Jest infrastructure
- **depends_on**: [T2, T4, T11]
- **agent_type**: local
- **ownership**: root/web package manifests and lockfiles; web tsconfigs, `web/babel.config.cjs`, `web/jest*.cjs`, `web/eslint.config.js`, `web/vite.config.js` only if needed; `web/tests/types/`; `quality/toolchain.json`.
- **description**: Pin compatible TypeScript, typed ESLint, Babel TS/JSX transforms, test typings, and generation/validation dependencies verified against T11's pilot schema. Put generator/tool dependencies in the root package and web transform/check dependencies in the web package as their import resolution requires. Configure Preact automatic JSX, strict/noUncheckedIndexedAccess/isolatedModules/noEmit and incremental JS coexistence. Register both `npm --prefix web run typecheck` and root `npm run typecheck:tests` now; the latter points to the test config delivered by T9. Add TS/TSX discovery/transforms and a real typed fixture. Preserve Node 24 and existing majors. No empty-project green check.
- **acceptance_criteria**:
  - Valid Preact TSX is checked and executed; wrong props fail `tsc`; runtime remains Preact.
  - Both Jest unit and Selenium E2E configs discover migrated test extensions; compiler, Babel, and lint peer compatibility is verified.
  - New additions are build-time dependencies unless measured runtime use is necessary; remaining JS is explicitly untyped or checked, not silently counted as migrated.
- **validation**:
  - `npm ci` and `npm ci --prefix web` (exclusive lease)
  - `npm --prefix web run typecheck`
  - `npm --prefix web test -- --runInBand`
  - `npm --prefix web run build`
  - Introduce/remove one temporary invalid prop in the fixture and verify a nonzero type-check exit.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T9: Type-check existing integration and load-test code
- **depends_on**: [T3, T8]
- **agent_type**: worker
- **ownership**: `tests/tsconfig.json`, `tests/load/`, existing `tests/integration/` TS files, `playwright.config.ts`; no packages or new contract specs.
- **description**: Supply the test tsconfig consumed by T8's already-registered root `typecheck:tests` target and inventory existing Playwright/load type errors. Repair a small cluster without broad ambient-any declarations. If repair exceeds that scope, append ordered T9.* tasks using the expansion protocol and add their terminal dependencies to both T13 and T15 rather than treating the entire suite as one worker rewrite.
- **acceptance_criteria**:
  - Test code is checked independently of Playwright transpilation and Node type stripping.
  - No test behavior or coverage is removed to satisfy types; task evidence lists discovered tests before/after.
- **validation**:
  - `npm run typecheck:tests` (script registered by T8)
  - `npx playwright test --list`
  - Review the load CLI's typed configuration/argument flow without sending load; compiler coverage is the acceptance check for that code here.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T10: Preserve lint, hooks, and i18n across extension changes
- **depends_on**: [T8]
- **agent_type**: worker
- **ownership**: `web/eslint.config.js`, `web/eslint-rules/`, `web/i18n-literal-baseline.json`, `tests/quality/lint-compat.test.mjs`, `quality/evidence/T10.md`.
- **description**: Make TS/TSX AST handling and scoped hooks rules work with existing JS. Establish extension-independent baseline identity before migrations; verified JSX-to-TSX renames retain the original counts without waiting for another task. Reject duplicate implementation identities and do not increase allowances. Validate accessibility and untranslated-string detection on TSX as well as JSX. Keep legacy cleanup governed by the findings inventory.
- **acceptance_criteria**:
  - A rename does not evade localization rules or produce a fresh larger baseline; typed expression wrappers cannot hide visible literals.
  - Migrated scopes enforce hooks correctness and type escape policy while old findings stay explicit.
- **validation**:
  - `node --test tests/quality/lint-compat.test.mjs`
  - `npm --prefix web run lint`
  - Demonstrate failures for one TSX visible string, invalid hooks use, and representative accessibility violation in a clean enforced scope.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T11: Specify the pilot contract from real behavior
- **depends_on**: [T2]
- **agent_type**: worker
- **ownership**: `docs/api/openapi.yaml`, `docs/api/contracts/streams.yaml`, `docs/api/contracts/common.yaml`, `tests/contracts/fixtures/streams/`, `quality/evidence/T11.md`.
- **description**: Define OpenAPI 3.1 operations for list/summary, full edit detail, PUT, and read-only option dependencies identified in T2. Model viewer redaction separately from admin edit data, current errors, nullability, numeric limits, encoded names, schedule values, and retention semantics. Keep out-of-pilot APIs inventoried. Use synthetic manually reviewed fixtures, not only snapshots of current code.
- **acceptance_criteria**:
  - Schema matches observed routing and policy; discrepancies become findings rather than undocumented wire changes.
  - Sensitive fields cannot be required in viewer models or accidentally accepted as evidence of correct redaction.
- **validation**:
  - Review each method/status/model against C handlers and the recorded T2 call graph.
  - Contract syntax and fixture validation run in T12 before this artifact becomes a usable generated contract.
- **status**: Completed (commit `a764b6ca`)
- **log**: Recovered abandoned specification and independently verified 178 references, 53 schemas, and 91 fixtures (39 valid/52 invalid). Corrected encoded path suffix boundaries and query parsing descriptions; 15 path cases. Observed C behavior is distinguished from stricter client validation. T12 must enforce byte limits and embedded JSON contentSchema explicitly. T11 findings feed T7.
- **files edited/created**: `docs/api/openapi.yaml`, `docs/api/contracts/{streams,common}.yaml`, `tests/contracts/fixtures/streams/`, `quality/evidence/T11.md`

### T12: Generate and validate contract artifacts deterministically
- **depends_on**: [T8, T11]
- **agent_type**: worker
- **ownership**: `scripts/quality/contracts.mjs`, `web/js/api/generated/`, `tests/contracts/generated/`, `tests/quality/contracts.test.mjs`, `quality/evidence/T12.md`; package requests go to integrator.
- **description**: Validate/bundle local schema refs; generate runtime-free TS types and compact standalone validators for selected external boundaries using packages already verified/installed by T8. Prefer openapi-typescript and a compatible JSON Schema 2020-12 validator; verify OpenAPI annotations/dialect, role variants, response headers and status handling instead of passing the entire OpenAPI object as a JSON Schema. If an additional package is unavoidable, the coordinator inserts a local package-owner prerequisite before this task; do not wait on downstream T15 or edit manifests concurrently.
- **acceptance_criteria**:
  - `--check` generates in a temporary directory and compares bytes, detecting stale/untracked generated outputs without mutating the checkout.
  - Known-invalid fixtures fail; permitted extra fields and missing required/redacted data follow explicit policy. Generator/compiler runtime is absent from shipped assets.
- **validation**:
  - `node scripts/quality/contracts.mjs --generate`
  - `node scripts/quality/contracts.mjs --check`
  - `node --test tests/quality/contracts.test.mjs`
  - `npm --prefix web run typecheck`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T13: Isolate native, API, and UI validation runs
- **depends_on**: [T3, T9]
- **agent_type**: local
- **ownership**: `scripts/quality/run-native.py`, `scripts/quality/run-integration.mjs`, `tests/integration/global-{setup,teardown}.ts`, shared integration fixtures/configuration, `playwright.config.ts`, `tests/unit/CMakeLists.txt`, `tests/quality/test_runners.py`.
- **description**: Create task-scoped test runners and configure ports/storage/PIDs/reports consistently. Replace current fixed-port assumptions and generated config mismatches, or retain explicit serialization where hardcoding cannot be safely removed. Protect cleanup with run ownership, loopback-only targets, and PID provenance. Add an `eng` test-fragment registration seam for later native tests. Runner builds/binary selection and optional go2rtc fixtures must be explicit.
- **acceptance_criteria**:
  - Tests cannot delete another run's files or stop an unrelated service; no server run follows a production LIGHTNVR_URL.
  - Concurrent runs pass only after isolation is proven; otherwise the runner enforces a serial lock. No selected tests is failure.
- **validation**:
  - `python3 -m unittest discover -s tests/quality -p test_runners.py`
  - `node scripts/quality/run-integration.mjs --task T13 --project api --grep @api`
  - `python3 scripts/quality/run-native.py --task T13 --regex '^test_system_health_proc$'`
  - Prove independent test-data/process ownership and failure propagation under the configured concurrency mode.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T14: Wire analysis reports and blocking policy into CI
- **depends_on**: [T6, T8, T10]
- **agent_type**: local
- **ownership**: `.github/workflows/static-analysis.yml`, `codeql-analysis.yml`, `container-scan.yml`, `quality/scanners.json` scope extensions, `quality/evidence/T14.md`.
- **description**: Preserve raw-report artifacts while applying the T6 policy. Add frontend JS/TS security coverage and explicit production/development dependency reports. Record scanner exit/report status independently; ensure changed tool/database versions remain visible. Do not mistake CodeQL upload success for an enforced no-new-alert gate; connect actual finding policy or verified repository code-scanning rules.
- **acceptance_criteria**:
  - New findings and scanner errors produce required failing statuses; artifact upload still occurs.
  - Optional-feature exclusions and production image/architecture scope are visible; valid no-finding scans remain distinguishable from no scan.
- **validation**:
  - `python3 -m unittest discover -s tests/quality -p test_findings.py`
  - Run CI on an authorized test branch or equivalent isolated runner; attach evidence for clean, new-finding, and scanner-error cases.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T15: Integrate foundation checks and required-status policy
- **depends_on**: [T4, T7, T9, T10, T12, T13, T14]
- **agent_type**: local
- **ownership**: shared manifests/lockfiles, `.github/workflows/integration-test.yml`, new engineering workflow if justified, `quality/required-checks.json`, `quality/evidence/T15.md`.
- **description**: Resolve infrastructure changes, finalize command registrations and dependency pins, and add local/CI type/lint/unit/asset/schema statuses with explicit path-filter behavior. T39 activates real pilot contract CI once T18 exists; never allow an empty future suite to report success. Produce the concrete required-status settings for T38, without making remote administrative access a prerequisite for local development.
- **acceptance_criteria**:
  - Foundation runs on a stable integrated snapshot, including existing Live compatibility tests and both web build modes.
  - Foundation code/CI passes independently of remote administration; T38 owns actual enforcement evidence and fences pilot/final acceptance.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm run typecheck:tests`
  - `npm --prefix web run lint`
  - `npm --prefix web test -- --runInBand`
  - `node scripts/quality/contracts.mjs --check`
  - `node scripts/quality/assets.mjs --check --profile default`
  - `node scripts/quality/assets.mjs --check --profile modern`
  - Review the proposed branch/ruleset diff against `quality/required-checks.json`; actual enforcement is validated by T38.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T38: Apply and verify pilot required-check enforcement
- **depends_on**: [T15, T39]
- **agent_type**: local
- **ownership**: reviewed remote required-status configuration, `quality/evidence/T38.md`; no application code.
- **description**: Read current repository rules, apply the concrete type/lint/analysis/assets/schema/real-contract required-check diff with appropriate authority, and verify the actual job identities and path-filter behavior. Preserve unrelated branch protections. Missing authority blocks this task and pilot acceptance while independent implementation can continue.
- **acceptance_criteria**:
  - An actual failing required status prevents merge, including the T39 real C contract lane; workflow YAML alone is insufficient evidence.
  - No security setting is weakened. T35 must reconcile this evidence again when final statuses are added.
- **validation**:
  - Read back branch/ruleset requirements and compare with `quality/required-checks.json`.
  - Demonstrate required-status enforcement on an authorized test PR or equivalent repository check evaluation; do not merge deliberately failing code.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T16: Type and validate fetch boundaries
- **depends_on**: [T15]
- **agent_type**: worker
- **ownership**: `web/js/fetch-utils.{js,ts}`, new `web/js/api/http.ts`, `web/tests/fetchUtils.spec.*`, `quality/evidence/T16.md`.
- **description**: Introduce typed HTTP/error/cancellation results and unknown JSON with explicit decoders. Preserve credentials, timeout/retry behavior, and mutations that may have succeeded despite a missing response. Keep a narrow compatibility facade for unconverted consumers; never assert arbitrary JSON as the caller's chosen generic type.
- **acceptance_criteria**:
  - Pilot fetches require validation; legacy paths are recorded and cannot silently count as checked.
  - Retry, abort listener cleanup, 401/403 handling, malformed bodies, and mutation timeout behavior retain regression coverage.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm --prefix web test -- --runInBand --testPathPatterns=fetchUtils`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T17: Type async UI primitives
- **depends_on**: [T15]
- **agent_type**: worker
- **ownership**: `web/js/hooks/useAsyncAction.{js,ts}`, `web/js/components/preact/AsyncButton.{jsx,tsx}`, `web/tests/useAsyncAction.spec.*`, `quality/evidence/T17.md`.
- **description**: Type action signatures, pending/error states, props and events while preserving double-submit protection, localization, and existing callers through checked compatibility exports. Use the canonical rename identity established by T10 so lint acceptance does not depend on later integration.
- **acceptance_criteria**:
  - Promise result/error handling is explicit; unmount/cancellation and repeated invocation are covered without new runtime dependencies.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm --prefix web test -- --runInBand --testPathPatterns=useAsyncAction`
  - `npm --prefix web run lint`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T18: Test pilot contracts against the real C backend
- **depends_on**: [T15]
- **agent_type**: worker
- **ownership**: `tests/integration/specs/streams-contract.api.spec.ts`, `tests/contracts/fixtures/streams/` test additions, `quality/evidence/T18.md`; C/schema fixes go through owners.
- **description**: Add @contract tests for admin/viewer/denied access, redaction, list/summary/full variants, encoded names, valid edit, rejected edit with unchanged state, and option dependencies. Verify sensitive-field absence independently of permissive schema validation. Use synthetic test accounts/data.
- **acceptance_criteria**:
  - Real C responses satisfy the contract and policy; a deliberately wrong fixture/expectation fails.
  - Any real defect produces a remediation task; tests/schema are not weakened merely to make it pass.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T18 --project api --grep @contract`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T39: Activate the real pilot contract CI lane
- **depends_on**: [T18]
- **agent_type**: local
- **ownership**: contract job in the T15 engineering/integration workflow, `quality/required-checks.json`, `quality/evidence/T39.md`.
- **description**: Wire T18's actual C contract suite into PR CI, using the isolated runner and explicit build/fixture prerequisites. Ensure contract, handler, schema, and relevant client changes trigger it; update the concrete required-status plan for T38. Keep report upload independent of test success.
- **acceptance_criteria**:
  - Real contract assertions and a wrong fixture fail the CI job; absent/zero selected tests cannot pass.
  - The lane is ready before pilot acceptance and broad rollout, with actual status names recorded.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T39 --project api --grep @contract`
  - Verify CI trigger coverage, output artifacts, nonzero failure propagation, and expected test count on an authorized test branch or isolated equivalent.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T19: Type query/mutation interfaces and pilot API client
- **depends_on**: [T16]
- **agent_type**: worker
- **ownership**: `web/js/query-client.{js,ts}`, `web/js/api/streams.ts`, `web/tests/queryClient.spec.ts`, `web/tests/streamsClient.spec.ts`.
- **description**: Preserve both existing query call forms and cache behavior. Type keys, decoder-derived results, mutation input/output, invalidation and cancellation. Use generated pilot types/validators from T12. Isolate any inadequate third-party typings behind a narrow tested adapter, not a module-wide any declaration.
- **acceptance_criteria**:
  - Client types distinguish redacted summary data from admin forms; network errors remain typed and observable.
  - Existing JS consumers still work; migrated consumers cannot get unchecked generic results.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm --prefix web test -- --runInBand --testPathPatterns='queryClient|streamsClient|fetchUtils'`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T20: Type the edit form and form model
- **depends_on**: [T17, T18, T19]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamConfigModal.{jsx,tsx}`, new `web/js/components/preact/streams/streamForm.ts`, `web/tests/streamForm.spec.ts`.
- **description**: Separate editable strings/checkboxes/schedules from wire models. Preserve full-detail loading, secret/redacted-field semantics, tri-state retention, optional values, and unchanged-field submission. Describe any ZoneEditor/legacy child boundary explicitly without assuming unknown props are safe.
- **acceptance_criteria**:
  - Validated form conversion handles malformed values and omission semantics; no implicit-any props or destructive defaulting of missing credentials.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm --prefix web test -- --runInBand --testPathPatterns=streamForm`
  - `npm --prefix web run lint`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T21: Type stream cards without changing snapshot behavior
- **depends_on**: [T17, T18, T19]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamCard.{jsx,tsx}`, `web/tests/streamCard.spec.ts`.
- **description**: Type redacted operational data, callbacks, and badge helpers. Preserve bounded snapshot concurrency, cleanup/release, async actions, and mobile markup; edit/delete/bulk behavior outside list/edit remains delegated through explicit callbacks.
- **acceptance_criteria**:
  - Missing optional data has defined display behavior and no privileged-field assumption; snapshot slot cancellation/release is tested where relevant.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm --prefix web test -- --runInBand --testPathPatterns=streamCard`
  - `npm --prefix web run lint`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T22: Integrate the typed Streams list/edit feature
- **depends_on**: [T20, T21]
- **agent_type**: local
- **ownership**: `web/js/components/preact/StreamsView.jsx`, new `web/js/components/preact/streams/StreamListEditor.tsx`, `web/js/pages/streams-page.jsx`, pilot import shims. T10's canonical i18n identity already covers renames.
- **description**: Wire the typed API/model/components into the existing page. Extract list/edit orchestration if needed so ONVIF/create/delete/bulk workflows remain separately inventoried. Give cross-boundary values explicit validated shapes; do not label the remaining legacy view fully migrated. Preserve filters, layout, auth, toasts, localization, and pending behavior.
- **acceptance_criteria**:
  - List/edit has typed data flow end to end; unrelated flows still function; no duplicate implementation or new public route.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm --prefix web test -- --runInBand`
  - `npm --prefix web run build`
  - `node scripts/quality/contracts.mjs --check`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T23: Exercise pilot UI and preserved Live behavior
- **depends_on**: [T22]
- **agent_type**: worker
- **ownership**: `tests/integration/specs/streams.ui.spec.ts`, new `tests/integration/specs/streams-typed.ui.spec.ts`, `quality/evidence/T23.md`.
- **description**: Verify loading/empty/error/invalid-data/success states, viewer redaction, denied/permitted editing, failed-edit persistence, cancellation, and duplicate-submit behavior. Use controlled failure responses for UI branches plus real C contract coverage. Run existing Live tests unchanged as a compatibility check.
- **acceptance_criteria**:
  - UI state assertions are behavioral; test count does not shrink; accessibility/localization behavior remains intact.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T23 --project ui --grep 'Streams|@streams-typed'`
  - `npm --prefix web test -- --runInBand --testPathPatterns='liveViewTransportTabs|liveViewUrl'`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T24: Accept the pilot and measure rollout cost
- **depends_on**: [T5, T23, T38]
- **agent_type**: local
- **ownership**: `quality/pilot-acceptance.json`, `quality/evidence/T24.md`, pilot-only integration corrections.
- **description**: Run the stable pilot against budget/contract/type/UI gates, inspect type escapes and generated size, and record conversion/review effort. If blockers arise, create bounded corrections and add dependencies; do not start broad migration on an unaccepted pilot.
- **acceptance_criteria**:
  - PRD pilot acceptance passes with actual C and agreed resource evidence; known unresolved findings affecting this feature block acceptance.
  - Document limitations: this says nothing by itself about recorder concurrency or complete API coverage.
- **validation**:
  - Repeat T15's frontend checks and T18's contract run on the integrated revision.
  - `node scripts/quality/assets.mjs --check --profile default`
  - `node scripts/quality/assets.mjs --check --profile modern`
  - Run the T5 profile's recorded browser/NVR workload and compare budgets.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T25: Materialize the remaining frontend and API rollout
- **depends_on**: [T2, T24]
- **agent_type**: local
- **ownership**: this plan, `quality/rollout-map.json`, consolidated application/API inventory updates; code read-only.
- **description**: Apply the mandatory expansion protocol using measured pilot cost. Cover shared auth/i18n/navigation/UI first; then Live transport helpers/cells and the shared Live view; recordings/timeline; settings tabs; system/health; users/audit; fleet; events; investigation/LPR; remaining stream/ONVIF/bulk flows; and every leftover utility/page/test. Split each area into small exact file clusters. Add its schema and real C contract tests before typed consumers. Include raw-source test paths, imports, legacy shims, and non-JSON media protocols. Insert explicit local T25.* schema-aggregation/code-generation/package checkpoints before each dependent consumer wave, with exclusive write ownership; those checkpoints must not depend on downstream T26. Attach all terminal tasks to T26.
- **acceptance_criteria**:
  - Every remaining file and UI API operation is covered exactly once, with no catch-all migration worker and no recreated WebRTCView.
  - Tasks preserve field/redaction semantics, run type/lint/domain tests and budgets, and serialize actual shared ownership.
- **validation**:
  - `python3 -m json.tool quality/rollout-map.json`
  - Compare the map against `rg --files web/js` and T2 API inventory; coordinator verifies graph and all shared-write constraints.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T26: Integrate the full typed application and contracts
- **depends_on**: [T25]
- **agent_type**: local
- **ownership**: central OpenAPI/common definitions, generated outputs, shared import/config/manifest integration, `quality/evidence/T26.md`.
- **description**: **Locked until T25 has inserted completed rollout dependencies.** Perform final reconciliation after the local T25.* checkpoints have supplied generated types and integration support to their consumers. Regenerate deterministically and merge any bounded final corrections without expanding worker ownership. Verify all UI operations and inventoried public APIs have contract/protocol coverage as required by the PRD; incomplete endpoints remain blockers rather than schema-less success.
- **acceptance_criteria**:
  - Every T25.* implementation/integration task is complete; full UI/API suites pass on one revision with no missing discovered tests or unchecked application files.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm run typecheck:tests`
  - `npm --prefix web test -- --runInBand`
  - `node scripts/quality/contracts.mjs --check`
  - `node scripts/quality/run-integration.mjs --task T26 --project api`
  - `node scripts/quality/run-integration.mjs --task T26-ui --project ui`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T27: Close media recovery and segment regression gaps
- **depends_on**: [T3, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_mp4_audio_recovery.c`, `test_mp4_recovery_watchdog.c`, `test_mp4_writer_watchdog.c`, `test_mp4_segment_boundaries.c`, `test_mp4_segment_timestamps.c`; `tests/fixtures/eng/media/`, `tests/operational/media/`, native registration fragment `tests/unit/cmake/eng/media.cmake`.
- **description**: Fill only T3's missing assertions for disconnect/audio recovery, segment duration/order/timestamps and playable output, with synthetic FFmpeg sources and declared deadlines. Keep production fixes in separate remediation tasks. Preserve existing recent audio/watchdog coverage.
- **acceptance_criteria**:
  - Tests demonstrate actual media validity where claimed; fixture thresholds are explicit and unexpected gaps fail.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T27 --regex '^test_mp4_(audio_recovery|recovery_watchdog|writer_watchdog|segment_boundaries|segment_timestamps)$'`
  - Run each new real-media scenario using its documented synthetic source and ffprobe assertions; record output diagnostics.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T28: Close storage and retention failure gaps
- **depends_on**: [T3, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_storage_retention_sqlite.c`, `test_storage_pressure.c`, `test_storage_pressure_extended.c`, `test_storage_manager_retention.c`; `tests/fixtures/eng/storage/`, `tests/operational/storage/`, `tests/unit/cmake/eng/storage.cmake`.
- **description**: Exercise inherited/unlimited/finite retention, protected files, ENOSPC/read-only/unavailable storage, and recovery with real SQLite where applicable. Use fault injection or disposable constrained filesystems; never fill or remount the host's working filesystem.
- **acceptance_criteria**:
  - Rejected cleanup preserves protected media; failure/recovery updates correct metadata; scenario limits and failure injection are reproducible.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T28 --regex '^test_storage_(retention_sqlite|pressure|pressure_extended|manager_retention)$'`
  - Run isolated storage fault scenarios and inspect database integrity plus retained-file assertions.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T29: Test shutdown, restart, and upgrade continuity
- **depends_on**: [T3, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_shutdown_coordinator.c`, `tests/operational/lifecycle/`, `tests/fixtures/eng/upgrade/`, `tests/unit/cmake/eng/lifecycle.cmake`.
- **description**: Add missing shutdown-deadline, forced-kill recovery, committed/incomplete media reconciliation, previous-supported-release database/config upgrade, and browser preference compatibility evidence. Store provenance for synthetic old-version fixtures. Observe graceful timeout separately from teardown's fallback SIGKILL so a hang cannot appear to pass.
- **acceptance_criteria**:
  - Database integrity, preserved recordings, preferences, and expected lifecycle state are checked after restart/upgrade.
  - Unsupported downgrade paths are described, not assumed safe or exercised on real state.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T29 --regex '^test_shutdown_coordinator$'`
  - Execute the recorded lifecycle/upgrade commands against isolated old-version fixtures; verify deadline and persistence assertions independently of process exit.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T30: Define and run repeated lifecycle/resource scenarios
- **depends_on**: [T5, T27, T28, T29]
- **agent_type**: worker
- **ownership**: `scripts/quality/soak.py`, `tests/operational/soak/`, `quality/soak-profiles.json`, `quality/evidence/T30.md`.
- **description**: Compose the tested failure cases into bounded soak profiles with declared duration/load/recovery tolerances; sample RSS/CPU/FD/thread counts and recording gaps. Support unattended artifact collection and clear pass/fail; keep physical calibration under T36. Assess a separate race-detection build without combining TSan with ASan/UBSan or hiding incompatible dependencies.
- **acceptance_criteria**:
  - Unbounded growth, missed recovery deadlines, deadlocks, sanitizer findings, and unexpected gaps fail; unsupported observations are reported as unavailable.
- **validation**:
  - `python3 scripts/quality/soak.py --profile ci --output build/eng-artifacts/T30`
  - Exercise a controlled failing recovery/resource fixture to prove the summary cannot report green.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T31: Select and materialize one bounded C improvement
- **depends_on**: [T3, T7]
- **agent_type**: local
- **ownership**: this plan, `quality/c-boundary.json`; production code read-only.
- **description**: Choose one evidenced finding or heavily changed responsibility. Document ownership, allocation/free, thread affinity, callbacks, locks, cancellation and shutdown, or pure-policy/I/O separation as applicable. Apply the expansion protocol: exact precondition tests, one bounded refactor, and local verification tasks; add terminal IDs to T32. Add explicit dependencies from child refactor tasks to the operational/precondition tests they consume, and serialize overlapping test-file ownership. `/proc` is a candidate, not a mandate to change sound code. If no benefit is demonstrated, record the unresolved selection and keep acceptance open.
- **acceptance_criteria**:
  - Worker ownership is exact, behavioral tests precede implementation, and shared lifecycle/schema changes are excluded or locally integrated.
  - No broad backend rewrite or Rust dependency is introduced.
- **validation**:
  - `python3 -m json.tool quality/c-boundary.json`
  - Review documented invariants and the added task DAG against actual callers and T3 evidence.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T32: Integrate and verify the C boundary improvement
- **depends_on**: [T5, T27, T28, T29, T31]
- **agent_type**: local
- **ownership**: selected boundary integration paths from T31, `quality/evidence/T32.md`; no unrelated native modules.
- **description**: **Locked until completed T31.* dependencies are inserted.** Reconcile the selected refactor, verify evidence of simpler ownership/reduced coupling or removal of the defect, and rerun affected operational/sanitizer checks.
- **acceptance_criteria**:
  - Bounded C1 outcome is demonstrated with unchanged public/persistence behavior or a separately documented bug fix; resource limits hold.
- **validation**:
  - Run the exact focused native/operational commands recorded by T31 plus its relevant T27–T29 scenarios.
  - Reproduce the affected workload under ASan/UBSan and compare its T5 resource profile.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T33: Verify remediation closure and refresh findings evidence
- **depends_on**: [T7, T14]
- **agent_type**: local
- **ownership**: findings dispositions/inventory, `quality/evidence/T33.md`; production changes only through explicit fix tasks.
- **description**: **Locked until all T7.* terminal dependencies are inserted and complete.** Re-run scanners on the integrated revision. Resolve any new actionable findings through additional atomic tasks and dependencies. Check exceptions against current exposure/upstream state and expiry. Do not replace evidence with bulk suppression or warning-count reduction.
- **acceptance_criteria**:
  - PRD release/final findings policy is met with complete expected report coverage; inaccessible required reports or unresolved first-party issues block completion.
- **validation**:
  - `python3 scripts/quality/findings.py --check --reports build/eng-artifacts/T33 --inventory quality/findings.json`
  - Review original-to-current finding identities, all exception records, and actual required scanner job results.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T34: Close migration escapes and retire temporary bridges
- **depends_on**: [T26]
- **agent_type**: local
- **ownership**: web/root type/lint configs, inventoried import shims and entrypoint references, `quality/application-inventory.json`, `quality/evidence/T34.md`.
- **description**: Verify that every first-party browser implementation is checked, remove temporary duplicate extensions/shims, and tighten final include/exclude policy. Keep only documented checked build/vendor exceptions. Reconcile tests, HTML entrypoints, source-path assertions, i18n baseline keys, and UI-operation coverage. Any substantial missed migration becomes an atomic dependency, not a catch-all cleanup.
- **acceptance_criteria**:
  - No unchecked implementation is hidden behind an allowJs flag, excluded directory, broad declaration, or ts-nocheck.
  - Strict checks, hooks/accessibility/localization enforcement, generated consistency, and full test discovery hold after bridge removal.
- **validation**:
  - `npm --prefix web run typecheck`
  - `npm run typecheck:tests`
  - `npm --prefix web run lint`
  - `npm --prefix web test -- --runInBand`
  - `node scripts/quality/contracts.mjs --check`
  - Compare final inventory to `rg --files web/js` and inspect all recorded escapes/exclusions.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T35: Integrate complete PR, scheduled, and release qualification gates
- **depends_on**: [T30, T32, T33, T34, T38]
- **agent_type**: local
- **ownership**: workflow/config/manifest integration, native test registration integration, `quality/required-checks.json`, contributor instructions, `quality/evidence/T35.md`.
- **description**: Wire PR type/lint/assets/contract/regression lanes, scheduled media/soak lanes, supported-feature scans, and release upgrade/device evidence. Revalidate repository required-check settings and path-filter behavior. Full integration must include serial/shared resources and optional-feature production coverage, not only lean tests. Keep deployment/publishing outside test commands.
- **acceptance_criteria**:
  - All required checks have truthful outcomes and retained diagnostics; scheduled/release failures have owners and cannot be counted as passed by quarantine/retry.
  - Local contributor commands reproduce CI checks and the combined system passes on one revision.
- **validation**:
  - Run full type/lint/unit/API/UI/go2rtc suites through the isolated runners and the native profiles recorded by T3.
  - Run T6 findings, T12 generation, T4 asset checks, T27–T30 operational lanes and upgrade checks against the same revision.
  - Verify remote required statuses and intentional failure propagation; missing authority leaves this acceptance gate open.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T36: Qualify supported targets and constrained operation
- **depends_on**: [T5, T35]
- **agent_type**: local
- **ownership**: `quality/evidence/T36.md`, target/runtime evidence records; budget changes require a separate reviewed rationale.
- **description**: Run actual target builds and the agreed physical NVR/browser workload, repeated lifecycle/recording tests, both web profiles, and required production feature combinations. Compare against P0 numbers with identical fixtures, not an easier workload. Confirm prior-release upgrade behavior for supported packages.
- **acceptance_criteria**:
  - All required target/feature/resource profiles have passing evidence; unsupported or inaccessible hardware remains explicitly incomplete.
  - No performance claim is based solely on an emulator or cross-compiler; no silent budget reset.
- **validation**:
  - Execute the exact build/workload commands captured in `quality/target-profiles.json` and `quality/runtime-budgets.json`.
  - `node scripts/quality/assets.mjs --check --profile default`
  - `node scripts/quality/assets.mjs --check --profile modern`
  - `python3 scripts/quality/soak.py --profile constrained --output build/eng-artifacts/T36`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T37: Reconcile final PRD acceptance and handoff
- **depends_on**: [T36]
- **agent_type**: local
- **ownership**: PRD implementation/acceptance status, `docs/prd/README.md`, `quality/acceptance.md`, this plan's final status/log.
- **description**: Map Q1–Q3, T1–T3, A1–A3, R1, B1–B2, C1 and P0–P6 to completed evidence and exact revision. Include all expanded tasks. Keep deferred/blocked outcomes visible and leave the PRD incomplete if any required gate is missing. Document maintenance commands and open exception review dates. Rust remains a separate decision.
- **acceptance_criteria**:
  - Every PRD acceptance item has a passing check or its explicitly permitted current exception; no planning/scaffold task is substituted for implementation.
  - Final working-tree/ownership review preserves unrelated changes and identifies precisely what was implemented.
- **validation**:
  - `python3 scripts/check_docs_links.py`
  - `git diff --check`
  - Manually trace every PRD acceptance bullet through the evidence matrix and verify final CI revision/target records agree.
- **status**: Not Completed
- **log**:
- **files edited/created**:

## Parallel Execution Groups

These are dependency frontiers, not permission to exceed three worker slots or
run shared-checkout builds concurrently. Local tasks requiring shared config or
whole-tree evidence get an exclusive integration/validation lease. Expansion
tasks must update this table before their children are scheduled.

| Wave | Tasks | Can Start When |
| --- | --- | --- |
| 1 | T1, T2, T3, T4 | Immediately; batch into available slots; reserve frontend build lease for T4 |
| 2 | T5, T6, T7, T11; then T8 | Respective inventory/baseline dependencies complete; T7 waits for archived T4 baseline; T8 also waits for T11 |
| 3 | T9, T10, T12; eligible T7.* | T8/T11 prerequisites complete; fix tasks honor concrete ownership edges |
| 4 | T13, T14, T31 | T9, T6/T10, or T3/T7 respectively; T13 and T14 serialize if shared infra needs overlap |
| 5 | T15; T27, T28, T29; eligible T31.* | Harness/foundation prerequisites complete; operational writers have disjoint files |
| 6 | T16, T17, T18; T30, T32, T33 when ready | T15 for pilot; separate operational/fix gates follow their task dependencies |
| 7 | T19, T39; then T38 | T16 for query; T18 for real contract CI; T15/T39 for remote enforcement |
| 8 | T20, T21 | Async primitives, actual C contracts, and typed client complete |
| 9 | T22 | Form and card integrated |
| 10 | T23 | Pilot wiring complete |
| 11 | T24 | UI evidence, T5 resource profiles, and T38 actual pilot enforcement ready |
| 12 | T25 then T25.* in newly recorded waves | Pilot accepted; exact rollout DAG and ownership validated |
| 13 | T26 then T34 | All materialized rollout dependencies complete |
| 14 | T35 | Soak, C improvement, remediation, and strict migration closure complete |
| 15 | T36 | Final CI integrated and actual target resources available |
| 16 | T37 | All qualification evidence complete |

## Integration Strategy

- T15 accepts infrastructure, T22/T24 accept the pilot, T26 accepts rollout,
  T32 accepts C changes, and T35 accepts the combined system. Review both file
  diffs and generated artifacts at each stable revision.
- Under shared checkout execution, workers edit directly and report files;
  only the coordinator updates this plan and reconciles shared files. If using
  isolated worktrees later, integrate one dependency-complete task at a time.
  Do not require commits simply to satisfy this plan.
- Give every rename a source/import/test/i18n mapping. Avoid concurrent writes
  to `openapi.yaml`, generated types, package locks, CMake, integration setup,
  shared test snapshots, and the findings inventory.
- If a task discovers a missing prerequisite, ownership conflict, excessive
  scope, incompatible dependency, or real bug, record the blocker and evidence
  in its log. The coordinator adds a bounded task/edge or obtains missing input;
  independent tasks continue. Never bypass a required gate, broaden ownership,
  weaken assertions, or mark a missing hardware/admin result completed.
- Finding fixes overlapping upcoming migrations take priority; incorporate the
  fix before the migration takes ownership and recapture affected baseline
  behavior intentionally. Do not freeze a known defect into the API contract.

## Testing Strategy

- Fast PR checks: strict application/test types, lint and i18n/hook rules,
  meaningful unit tests, generation drift, findings policy, and asset budgets.
- Real-server checks: role/redaction/side effects, Streams list/edit, all expanded
  API contracts, preserved Live and critical UI flows. Contract tests validate
  behavior independently of generated schema; snapshots alone are insufficient.
- Native checks: the existing supported CMake profiles, ASan/UBSan, focused
  fault paths, real SQLite persistence/retention, media metadata/playability,
  and shutdown deadlines. Separate race instrumentation if feasible.
- Scheduled/release checks: real media recovery, repeated lifecycle/soak,
  prior-release upgrades, optional production feature builds, and physical
  constrained-target qualification. Establish duration/tolerances before results.
- Negative gate demonstrations are temporary fixtures or meaningful policy-tool
  tests. Do not commit deliberately broken application code or tests that merely
  duplicate a config line. Report flakes, retries, and quarantine honestly.

## Risks and Mitigations

- **Risk**: An adaptive plan completes before its discovered work is performed.
  **Mitigation**: Mandatory graph expansion and explicit terminal dependencies
  fence T26/T32/T33; T37 checks the complete PRD evidence matrix.
- **Risk**: TSX makes unchanged tests disappear or stops i18n enforcement.
  **Mitigation**: T2 inventories source-path consumers; T8/T10 handle discovery,
  AST semantics and baseline keys before conversions; T34 verifies closure.
- **Risk**: Shared test ports, scratch storage, or PIDs cause destructive races.
  **Mitigation**: T13 proves isolation or enforces serialization; unique build
  directories and validation leases apply even to otherwise disjoint tasks.
- **Risk**: Strict typing forces the whole large Streams page into the pilot.
  **Mitigation**: T22 extracts a typed list/edit feature with explicit adapters;
  T25 assigns remaining create/delete/ONVIF work as separate bounded tasks.
- **Risk**: OpenAPI types create false confidence about runtime input or secrecy.
  **Mitigation**: Unknown input plus bounded validators; independent redaction,
  permission and side-effect assertions against C; explicit dialect support.
- **Risk**: Babel 8/Jest/ESLint additions or generated validators inflate scope.
  **Mitigation**: Pin compatible tools in one task, preserve current majors and
  Preact, measure output, and avoid a test-runner/framework replacement.
- **Risk**: Scanner reports or supported targets are unavailable.
  **Mitigation**: Record missing access/hardware; keep affected acceptance open
  while independent development proceeds. Do not substitute a green lean build.

## Primary Documentation Checked

These sources inform toolchain choices; resolve exact compatible versions during
T8 and retain the locked choices. Preact v10 documentation matches the current
runtime even though a newer major exists.

- [Preact TypeScript](https://preactjs.com/guide/v10/typescript/): built-in types
  and JSX configuration.
- [Vite TypeScript](https://vite.dev/guide/features.html#typescript): transpilation
  is separate from type checking.
- [Babel TypeScript preset](https://babeljs.io/docs/babel-preset-typescript) and
  [Jest TypeScript](https://jestjs.io/docs/getting-started#using-typescript): retain
  Babel-based execution while checking types independently.
- [typescript-eslint](https://typescript-eslint.io/getting-started/): flat-config
  integration; verify selected package peer compatibility.
- [openapi-typescript](https://openapi-ts.dev/introduction): OpenAPI 3.1 support
  and runtime-free generated types.
- [Ajv JSON Schema](https://ajv.js.org/json-schema.html) and
  [standalone validators](https://ajv.js.org/standalone.html): verify the required
  dialect and generate validators at build time.

## Reviewer Notes

- Initial local review identified shared test ports/storage, extension-dependent
  tests/i18n baselines, the large mixed-purpose Streams page, unknown findings,
  and absent physical/admin evidence. The plan assigns explicit owners and gates
  rather than assuming these risks away.
- Independent reviewer `review_foundation_plan` reviewed the complete draft and
  PRD without editing files or running builds. Applied its dependency fixes:
  T8 produces test commands and generation dependencies before consumers;
  local T25.* checkpoints produce generated artifacts before rollout workers;
  T32 explicitly waits for resource/operational evidence; and T3/T4 archive
  immutable baseline artifacts for T5.
- Added T39 to activate real C contract CI before pilot acceptance and T38 to
  verify remote required checks separately from local foundation work. T35
  rechecks the final ruleset after later statuses are added. T10 resolves i18n
  extension identity before workers rename files. T9's optional expansion now
  updates every consumer, and its validation avoids starting load.
- Execution resumed on 2026-10-31 after the previous executor hit its usage limit.
  Task entries and their evidence are authoritative; incomplete acceptance gates
  remain open. User explicitly deferred physical-device measurements.
