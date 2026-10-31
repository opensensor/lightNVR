# Plan: Security, Quality, and Typed Frontend Foundation

**Generated**: 2026-10-31
**Mode**: Swarm-ready dependency plan
**Source**: [Engineering foundation PRD](docs/prd/ENG_01_Safety_Contracts_TypeScript.md)
**Inspected baseline**: `main`, `1b8d4332` (PR #691's unified Live view)
**Execution status**: In progress; resumed under user authorization on 2026-10-31. User explicitly kept development-tool exception proposals pending; continue independent work without approving them. T1–T4 and T6 were committed before resumption. Physical-device measurements in T5/T36 are deferred at the user's request; this is not passing qualification evidence.

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
- **status**: Inventory/protocol completed (commit `e0544701`); physical calibration deferred by user
- **log**: Nine release target combinations and two immutable web profiles recorded; JSON/invariant checks pass, 12 native archive files and 221/165 web files hash-verified. Three host-only idle runs (180 samples) recorded 55,524–56,272 KiB RSS, 18 FDs, 21 threads and graceful exits; these are not constrained acceptance. Physical NVR/browser targets and 18 runtime limits remain null/unqualified, explicitly deferred by the user. Independent development may consume the protocol; physical-resource acceptance remains open.
- **files edited/created**: `quality/target-profiles.json`, `quality/runtime-budgets.json`, `quality/evidence/T5.md`

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
- **status**: Completed (commits `3eb4d251`, `f9b2f0cf`, `4a390c02`; coordinator integrated expanded records)
- **log**: All 743 fingerprints and 17 T11 candidates mapped to bounded tasks; 335 child records with exact ownership, assertions, dependencies and planned commands. Same-file writers serialized; T33 fenced by every child; pilot/dependency gates updated. Static DAG/coverage checks and eight negative controls pass. No findings approvals or remediation completion inferred from planning. T7.1 already fixed in c285a0c2.
- **files edited/created**: `quality/remediation-map.json`, `quality/evidence/T7.md`, `quality/evidence/T7-expansion.md`, `quality/evidence/T7-check.py`, this plan

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
- **status**: Completed (commit `17de8155`)
- **log**: RED missing TSX discovery -> GREEN 55 suites/571 tests; strict Preact fixture checks and executes, deliberate invalid prop fails. Root/web clean installs and peer compatibility verified; both Vite profiles build. Pinned TS5.9.3 and Babel8-compatible transforms without changing installed majors. Root test type command exists but intentionally awaits T9 config. All legacy application JS remains explicitly unchecked. T11 compatibility proof covers 91 fixtures, generated types and browser-bundled Ajv helpers.
- **files edited/created**: root/web package manifests and locks, web Babel/Jest/ESLint configs, web tsconfigs, `web/tests/types/`, `quality/toolchain.json`, `quality/evidence/T8.md`

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
- **status**: Completed (commit `b0bcf4c2`)
- **log**: Strict config plus four bounded repair children complete; 38 original diagnostics eliminated across 12 files, all 48 integration/load/config TS files included; 213 test identities unchanged. Three imported browser JS modules explicitly unchecked pending T25/T34.
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
- **status**: Completed (commit `4f82c1e2`)
- **log**: Rules and T10.1–T10.3 closure complete: 31 focused checks, 55 suites/571 tests, lint zero errors and unchanged 897 existing warnings. Canonical i18n allowance bytes unchanged; TS hooks/a11y/type escapes and duplicate identity enforced.
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
- **status**: Completed (commit `30aecdfc`)
- **log**: RED missing generator -> GREEN 12 isolated tests and all 91 fixture expectations; deterministic read-only drift checks reject missing/untracked/stale outputs and invalid refs. Enforces embedded JSON and UTF-8 byte limits; types/declarations pass strict checking. Per-schema standalone validators tree-shake (client-config 1.6KB gzip, viewer 7KB, update 5.3KB); full metadata stays in test output. No compiler runtime shipped or package changes.
- **files edited/created**: `scripts/quality/contracts.mjs`, `web/js/api/generated/`, `tests/contracts/generated/`, `tests/quality/contracts.test.mjs`, `quality/evidence/T12.md`

### T13: Isolate native, API, and UI validation runs
- **depends_on**: [T3, T9, T9.4]
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
- **status**: Runner implementation completed (commit `1de1fa7e`); discovered application/test defects remain explicit downstream tasks
- **log**: 17 ownership/failure tests and strict test types pass; real ASan/UBSan proc test passes. Full API run 43 pass/2 skip/2 stale expectation failures; separate go2rtc smoke exposed actual shutdown deadlock and correctly failed forced teardown. Final lean smoke 4/4 passes with graceful cleanup, not a substitute for go2rtc evidence. T31 fixes reproduced lifecycle defect; T13.1–T13.3 own bounded API corrections. Runtime lock, token/config/exe/starttime checks and pidfd descendant cleanup verified; no unrelated process/file cleanup. Added sorted native fragment seam and pinned FFmpeg ABI probe.
- **files edited/created**: quality native/integration runners, runner tests, owned-process fixtures, global setup/teardown, Playwright results path, native fragment seam, two existing go2rtc fixture path/port repairs, T13 evidence

### T14: Wire analysis reports and blocking policy into CI
- **depends_on**: [T6, T8, T10, T7.1, T10.3, T14.1]
- **agent_type**: local
- **ownership**: `.github/workflows/static-analysis.yml`, `codeql-analysis.yml`, `container-scan.yml`, `quality/scanners.json` scope extensions, `quality/evidence/T14.md`.
- **description**: Preserve raw-report artifacts while applying the T6 policy. Add frontend JS/TS security coverage and explicit production/development dependency reports. Record scanner exit/report status independently; ensure changed tool/database versions remain visible. Do not mistake CodeQL upload success for an enforced no-new-alert gate; connect actual finding policy or verified repository code-scanning rules.
- **acceptance_criteria**:
  - New findings and scanner errors produce required failing statuses; artifact upload still occurs.
  - Optional-feature exclusions and production image/architecture scope are visible; valid no-finding scans remain distinguishable from no scan.
- **validation**:
  - `python3 -m unittest discover -s tests/quality -p test_findings.py`
  - Run CI on an authorized test branch or equivalent isolated runner; attach evidence for clean, new-finding, and scanner-error cases.
- **status**: Implemented locally — hosted scanner execution and full coverage pending (commit `0901a9d2`)
- **log**: Same-run manifests and full aggregate wired;4 workflow,25 scanner subprocess and69 findings tests pass; actionlint passes. Actual hosted extraction/scanning unverified; full gate intentionally fails for5 missing scanner identities and unapproved proposals. No push or remote enforcement.
- **files edited/created**:

### T15: Integrate foundation checks and required-status policy
- **depends_on**: [T4, T7, T9, T10, T12, T13, T14, T7.8, T7.9, T7.10, T9.4, T10.3, T13.2, T15.1, T15.2, T15.3]
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
- **depends_on**: [T15, T7.334]
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
- **depends_on**: [T16, T7.334]
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
- **depends_on**: [T17, T18, T19, T7.167, T7.177, T7.180, T7.198]
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
- **depends_on**: [T17, T18, T19, T7.161, T7.180, T7.189, T7.193, T7.197]
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
- **depends_on**: [T20, T21, T7.161, T7.168, T7.180, T7.189, T7.198]
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
- **depends_on**: [T2, T24, T7.160, T7.161, T7.162, T7.163, T7.164, T7.165, T7.166, T7.167, T7.168, T7.169, T7.170, T7.171, T7.172, T7.173, T7.174, T7.175, T7.176, T7.177, T7.178, T7.179, T7.180, T7.181, T7.182, T7.183, T7.184, T7.185, T7.186, T7.187, T7.188, T7.189, T7.190, T7.191, T7.192, T7.193, T7.194, T7.195, T7.196, T7.197, T7.198, T7.199, T7.200, T7.201, T7.202, T7.273]
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
- **depends_on**: [T3, T13, T27.0, T27.1, T27.2]
- **agent_type**: worker
- **ownership**: `tests/unit/test_mp4_audio_recovery.c`, `test_mp4_recovery_watchdog.c`, `test_mp4_writer_watchdog.c`, `test_mp4_segment_boundaries.c`, `test_mp4_segment_timestamps.c`; `tests/fixtures/eng/media/`, `tests/operational/media/`, native registration fragment `tests/unit/cmake/eng/media.cmake`.
- **description**: Fill only T3's missing assertions for disconnect/audio recovery, segment duration/order/timestamps and playable output, with synthetic FFmpeg sources and declared deadlines. Keep production fixes in separate remediation tasks. Preserve existing recent audio/watchdog coverage.
- **acceptance_criteria**:
  - Tests demonstrate actual media validity where claimed; fixture thresholds are explicit and unexpected gaps fail.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T27 --regex '^test_mp4_(audio_recovery|recovery_watchdog|writer_watchdog|segment_boundaries|segment_timestamps)$'`
  - Run each new real-media scenario using its documented synthetic source and ffprobe assertions; record output diagnostics.
- **status**: Completed (commit `586d8d89`)
- **log**: T27.0 producer plus separate collision/retry identity (3b62c681) and paused-reader reset (586d8d89) close the actual failure cases. Latest native84/84 and both unchanged real-source scenarios pass; declared one-frame overlap preserved. Host functional validation only; no constrained-device claim.
- **files edited/created**:

### T28: Close storage and retention failure gaps
- **depends_on**: [T3, T13, T28.0, T28.1, T28.2]
- **agent_type**: worker
- **ownership**: `tests/unit/test_storage_retention_sqlite.c`, `test_storage_pressure.c`, `test_storage_pressure_extended.c`, `test_storage_manager_retention.c`; `tests/fixtures/eng/storage/`, `tests/operational/storage/`, `tests/unit/cmake/eng/storage.cmake`.
- **description**: Exercise inherited/unlimited/finite retention, protected files, ENOSPC/read-only/unavailable storage, and recovery with real SQLite where applicable. Use fault injection or disposable constrained filesystems; never fill or remount the host's working filesystem.
- **acceptance_criteria**:
  - Rejected cleanup preserves protected media; failure/recovery updates correct metadata; scenario limits and failure injection are reproducible.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T28 --regex '^test_storage_(retention_sqlite|pressure|pressure_extended|manager_retention)$'`
  - Run isolated storage fault scenarios and inspect database integrity plus retained-file assertions.
- **status**: Completed — declared isolated storage scenarios (commits `0eb7e5aa`, `6d254658`, `e56fc168`, `4deb2b4e`)
- **log**: Final six-target ASan/UBSan run passes73/73 cases: real SQLite FULL/READONLY, retention modes/protected files, unavailable-root/retry recovery, real segment ENOSPC/EIO/EROFS and decodable recovery, bounded writer pause/resume. Missing-root guard cannot detect an unmounted filesystem whose underlying directory remains; target mount guards unchanged. Broader real-media/health/physical qualification remains separately scoped.
- **files edited/created**:

### T29: Test shutdown, restart, and upgrade continuity
- **depends_on**: [T3, T13, T29.0, T29.1, T29.4, T29.2, T29.3]
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
- **depends_on**: [T5, T27, T28, T29, T30.1]
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
- **status**: Completed — bounded disposal improvement selected and verified by T31.1–T31.3
- **log**: Source review found global state shutdown destroys one of two initialized mutexes while normal removal destroys both; T13 then reproduced a registry-lock reentry deadlock in the same teardown path. Selected shared private final-disposal helper; exact lifecycle regression, implementation and verification tasks T31.1–T31.3 inserted. No T7 source ownership overlap; runner/test registration follows T13. JSON and graph review pass; no implementation outcome claimed.
- **files edited/created**: `quality/c-boundary.json`, `quality/evidence/T31.md`, this plan

### T32: Integrate and verify the C boundary improvement
- **depends_on**: [T5, T27, T28, T29, T31, T31.1, T31.2, T31.3]
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
- **depends_on**: [T7, T14, T7.1, T7.2, T7.3, T7.4, T7.5, T7.6, T7.7, T7.8, T7.9, T7.10, T7.11, T7.12, T7.13, T7.14, T7.15, T7.16, T7.17, T7.18, T7.19, T7.20, T7.21, T7.22, T7.23, T7.24, T7.25, T7.26, T7.27, T7.28, T7.29, T7.30, T7.31, T7.32, T7.33, T7.34, T7.35, T7.36, T7.37, T7.38, T7.39, T7.40, T7.41, T7.42, T7.43, T7.44, T7.45, T7.46, T7.47, T7.48, T7.49, T7.50, T7.51, T7.52, T7.53, T7.54, T7.55, T7.56, T7.57, T7.58, T7.59, T7.60, T7.61, T7.62, T7.63, T7.64, T7.65, T7.66, T7.67, T7.68, T7.69, T7.70, T7.71, T7.72, T7.73, T7.74, T7.75, T7.76, T7.77, T7.78, T7.79, T7.80, T7.81, T7.82, T7.83, T7.84, T7.85, T7.86, T7.87, T7.88, T7.89, T7.90, T7.91, T7.92, T7.93, T7.94, T7.95, T7.96, T7.97, T7.98, T7.99, T7.100, T7.101, T7.102, T7.103, T7.104, T7.105, T7.106, T7.107, T7.108, T7.109, T7.110, T7.111, T7.112, T7.113, T7.114, T7.115, T7.116, T7.117, T7.118, T7.119, T7.120, T7.121, T7.122, T7.123, T7.124, T7.125, T7.126, T7.127, T7.128, T7.129, T7.130, T7.131, T7.132, T7.133, T7.134, T7.135, T7.136, T7.137, T7.138, T7.139, T7.140, T7.141, T7.142, T7.143, T7.144, T7.145, T7.146, T7.147, T7.148, T7.149, T7.150, T7.151, T7.152, T7.153, T7.154, T7.155, T7.156, T7.157, T7.158, T7.159, T7.160, T7.161, T7.162, T7.163, T7.164, T7.165, T7.166, T7.167, T7.168, T7.169, T7.170, T7.171, T7.172, T7.173, T7.174, T7.175, T7.176, T7.177, T7.178, T7.179, T7.180, T7.181, T7.182, T7.183, T7.184, T7.185, T7.186, T7.187, T7.188, T7.189, T7.190, T7.191, T7.192, T7.193, T7.194, T7.195, T7.196, T7.197, T7.198, T7.199, T7.200, T7.201, T7.202, T7.203, T7.204, T7.205, T7.206, T7.207, T7.208, T7.209, T7.210, T7.211, T7.212, T7.213, T7.214, T7.215, T7.216, T7.217, T7.218, T7.219, T7.220, T7.221, T7.222, T7.223, T7.224, T7.225, T7.226, T7.227, T7.228, T7.229, T7.230, T7.231, T7.232, T7.233, T7.234, T7.235, T7.236, T7.237, T7.238, T7.239, T7.240, T7.241, T7.242, T7.243, T7.244, T7.245, T7.246, T7.247, T7.248, T7.249, T7.250, T7.251, T7.252, T7.253, T7.254, T7.255, T7.256, T7.257, T7.258, T7.259, T7.260, T7.261, T7.262, T7.263, T7.264, T7.265, T7.266, T7.267, T7.268, T7.269, T7.270, T7.271, T7.272, T7.273, T7.274, T7.275, T7.276, T7.277, T7.278, T7.279, T7.280, T7.281, T7.282, T7.283, T7.284, T7.285, T7.286, T7.287, T7.288, T7.289, T7.290, T7.291, T7.292, T7.293, T7.294, T7.295, T7.296, T7.297, T7.298, T7.299, T7.300, T7.301, T7.302, T7.303, T7.304, T7.305, T7.306, T7.307, T7.308, T7.309, T7.310, T7.311, T7.312, T7.313, T7.314, T7.315, T7.316, T7.317, T7.318, T7.319, T7.320, T7.321, T7.322, T7.323, T7.324, T7.325, T7.326, T7.327, T7.328, T7.329, T7.330, T7.331, T7.332, T7.333, T7.334, T7.335, T38, T13.3, T13.4, T28.1, T28.2, T29.1, T27.1, T29.3, T27.2, T7.2.1, T7.2.2, T7.2.3, T7.2.4]
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


## Expanded remediation tasks (T7)


T7 → T7.1 approval enforcement → T14; T8 → dependency remediations → T15; T10 → web remediation → overlapping pilot migrations and T25; T13 → native/pilot remediations; T12 + pilot remediations → contract reconciliation → T18/T19; every T7.* → T33. Within these frontiers, depends_on includes each previous writer of the same path.

| Frontier | Tasks | Resource constraints |
| --- | --- | --- |
| Policy/reviews | Tasks whose only producer is T6/T7/T11; T7.2.1–T7.2.4 afterT13 | Reviews do not approve exceptions; advisory regression children use serialized native leases |
| Dependency | kind=dependency after T8 | One package/install/build writer, then T15 |
| Web | kind=web-fix after T10 | Max three workers; unique focused Jest files, exclusive full lint/build |
| Native/pilot | kind=native-fix/pilot-fix after T13 | Shared server/native operations serialized by runner lease; exact CMake fragments |
| Reconciliation | kind=integration after T12 and all pilot fixes | Exclusive generation, followed by T18/T19 |
| Closure | all T7.* then T33 | Fresh complete scanners; unresolved reviews block |

### T7.1: Reject unapproved findings and coverage exceptions
- **depends_on**: [T7, T6]
- **agent_type**: local
- **ownership**: `scripts/quality/findings.py`, `tests/quality/test_findings.py`, `quality/findings-policy.json`, `quality/evidence/T7.1.md`.
- **priority**: high
- **description**: Require explicit reviewed approval evidence before an exception can affect --check or --release. The existing proposed-by-T1/T6 dates and maintainers/<area> owners are proposals, not approvals. Preserve diagnostic reporting and the read-only normal check. Do not silently bless old records during schema migration.
- **findings**: policy/integration requirement
- **acceptance_criteria**:
  - Tests first prove proposed, missing, expired and malformed approvals fail; an explicitly approved current exception succeeds only in its permitted mode.
  - An approval must name an accountable reviewer/owner, date and reason; placeholders do not qualify. --release still rejects coverage gaps.
  - Run against actual T1 reports: unapproved proposals must cause policy failure. Hash the inventory before/after --check.
- **validation**:
  - `python3 -m unittest discover -s tests/quality -p test_findings.py`
  - `python3 scripts/quality/findings.py --check --reports build/eng-artifacts/T1 --inventory quality/findings.json`
- **status**: Completed (commit `c285a0c2`)
- **log**: Coordinator implemented RED -> GREEN approval enforcement: 65 tests pass; real T1 fails 575 inventory and 4 gap approval proposals; no accepted gaps. Metadata validation does not authenticate a human approval; do not fabricate production approval records.
- **files edited/created**:

### T7.2: Verify disposition and regression evidence for ADVISORY-GHSA-cv53-mq7p-6jp2
- **depends_on**: [T7, T7.2.1, T7.2.2, T7.2.3, T7.2.4]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-ghsa-cv53-mq7p-6jp2.json`, `quality/evidence/T7.2.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-GHSA-cv53-mq7p-6jp2
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-ghsa-cv53-mq7p-6jp2.json`
- **status**: In progress — exact regression coverage pending
- **log**: Public advisory and ancestor fixes d558e69d/1bdf6fe8 reviewed; current source has later successors. Materialized four bounded tests-only child tasks before closure; version exclusion/generic tests are insufficient.
- **files edited/created**:

### T7.3: Verify disposition and regression evidence for ADVISORY-repo-advisory-draft-1
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-repo-advisory-draft-1.json`, `quality/evidence/T7.3.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-repo-advisory-draft-1
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-repo-advisory-draft-1.json`
- **status**: Completed (commit `73fb07ec`)
- **log**: Authorized read-only source refresh confirms unchanged private report state/body; ancestor fixes verified against actual routing/transport regressions (18Unity cases), plus current real viewer-policy controls for proxy aliases (44Unity cases). No private identifiers/body published, no remote state or baseline changed.
- **files edited/created**:

### T7.4: Verify disposition and regression evidence for ADVISORY-repo-advisory-closed-1
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-repo-advisory-closed-1.json`, `quality/evidence/T7.4.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-repo-advisory-closed-1
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-repo-advisory-closed-1.json`
- **status**: Completed (commit `73fb07ec`)
- **log**: Authorized read-only source refresh confirms unchanged private report state/body; ancestor fixes verified against actual routing/transport regressions (18Unity cases), plus current real viewer-policy controls for proxy aliases (44Unity cases). No private identifiers/body published, no remote state or baseline changed.
- **files edited/created**:

### T7.5: Verify disposition and regression evidence for ADVISORY-repo-advisory-draft-2
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-repo-advisory-draft-2.json`, `quality/evidence/T7.5.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-repo-advisory-draft-2
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-repo-advisory-draft-2.json`
- **status**: Completed (commit `73fb07ec`)
- **log**: Authorized read-only source refresh confirms unchanged private report state/body; ancestor fixes verified against actual routing/transport regressions (18Unity cases), plus current real viewer-policy controls for proxy aliases (44Unity cases). No private identifiers/body published, no remote state or baseline changed.
- **files edited/created**:

### T7.6: Verify disposition and regression evidence for ADVISORY-GHSA-rqj3-pch5-rq4f
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-ghsa-rqj3-pch5-rq4f.json`, `quality/evidence/T7.6.md`.
- **priority**: critical
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-GHSA-rqj3-pch5-rq4f
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-ghsa-rqj3-pch5-rq4f.json`
- **status**: Completed (commit `4ac385d4`)
- **log**: Pinned bundled Go security16/16 and native configuration6/6 pass; reviewed parent/submodule fixes, loopback defaults and non-root/no-direct-API deployment recipes. Current faststart2/2 also passes. No published release/advisory change, baseline mutation or exception approval.
- **files edited/created**:

### T7.7: Verify disposition and regression evidence for ADVISORY-GHSA-pvw7-7j8c-qx96
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-ghsa-pvw7-7j8c-qx96.json`, `quality/evidence/T7.7.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-GHSA-pvw7-7j8c-qx96
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-ghsa-pvw7-7j8c-qx96.json`
- **status**: Completed (commit `22012e65`)
- **log**: Public advisory state/source reviewed; existing fix507bce1c is an ancestor. Integration-profile ASan/UBSan passes3 targets/18tests, including decoded query identity, cache update and delayed worker shutdown. No baseline/approval or published advisory mutation.
- **files edited/created**:

### T7.8: Remediate glob dependency advisory GHSA-3g3m-2fjr-6pw4
- **depends_on**: [T7, T8]
- **agent_type**: local
- **ownership**: `web/package.json`, `web/package-lock.json`, `quality/evidence/T7.8.md`.
- **priority**: medium
- **description**: Resolve GHSA-3g3m-2fjr-6pw4 for glob using an actually available compatible patched transitive version or remove the vulnerable dependency path. Inspect npm explain and production/dev reachability. Preserve Node 24 and installed direct major versions. If no compatible fix exists, submit a specific owner/expiry/rationale proposal and keep this task open until explicitly reviewed; do not force an unrelated major upgrade.
- **findings**: NPM-WEB-GHSA-3g3m-2fjr-6pw4-glob
- **acceptance_criteria**:
  - npm audit no longer reports the advisory on each original dependency path, or an explicitly reviewed current exception documents the residual exposure.
  - Dependency graph, unit tests and both web builds remain compatible; no broad overrides that violate parent ranges.
- **validation**:
  - `npm --prefix web explain glob`
  - `npm --prefix web audit --json`
  - `npm --prefix web audit --omit=dev --json`
  - `npm --prefix web test -- --runInBand`
  - `npm --prefix web run build`
  - `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build`
- **status**: Blocked — review prepared, awaiting compatible fix or maintainer decision
- **log**: Concrete review packet committed in `713b3025`; no package mutation, approval or disposition change. Current development-tool advisory ranges and dependency parents checked; production web audit zero. See task evidence and `quality/reviews/exception-ownership.json`.
- **files edited/created**:

### T7.9: Remediate minimatch dependency advisory GHSA-g3hv-75wg-m249
- **depends_on**: [T7, T8, T7.8]
- **agent_type**: local
- **ownership**: `web/package.json`, `web/package-lock.json`, `package.json`, `package-lock.json`, `quality/evidence/T7.9.md`.
- **priority**: medium
- **description**: Resolve GHSA-g3hv-75wg-m249 for minimatch using an actually available compatible patched transitive version or remove the vulnerable dependency path. Inspect npm explain and production/dev reachability. Preserve Node 24 and installed direct major versions. If no compatible fix exists, submit a specific owner/expiry/rationale proposal and keep this task open until explicitly reviewed; do not force an unrelated major upgrade.
- **findings**: NPM-WEB-GHSA-g3hv-75wg-m249-minimatch
 T8 adds the root openapi-typescript/redocly dev path for the same minimatch advisory; root manifests are included and T33 must reconcile the new occurrence without automatic baseline changes.
- **acceptance_criteria**:
  - npm audit no longer reports the advisory on each original dependency path, or an explicitly reviewed current exception documents the residual exposure.
  - Dependency graph, unit tests and both web builds remain compatible; no broad overrides that violate parent ranges.
- **validation**:
  - `npm --prefix web explain minimatch`
  - `npm --prefix web audit --json`
  - `npm --prefix web audit --omit=dev --json`
  - `npm --prefix web test -- --runInBand`
  - `npm --prefix web run build`
  - `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build`
  - `npm explain minimatch`
  - `npm audit --json`
  - `npm audit --omit=dev --json`
- **status**: Blocked — review prepared, awaiting compatible fix or maintainer decision
- **log**: Concrete review packet committed in `713b3025`; no package mutation, approval or disposition change. Current development-tool advisory ranges and dependency parents checked; production web audit zero. See task evidence and `quality/reviews/exception-ownership.json`.
- **files edited/created**:

### T7.10: Remediate deepmerge dependency advisory GHSA-gcqx-w48m-w2h4
- **depends_on**: [T7, T8, T7.9]
- **agent_type**: local
- **ownership**: `web/package.json`, `web/package-lock.json`, `quality/evidence/T7.10.md`.
- **priority**: medium
- **description**: Resolve GHSA-gcqx-w48m-w2h4 for deepmerge using an actually available compatible patched transitive version or remove the vulnerable dependency path. Inspect npm explain and production/dev reachability. Preserve Node 24 and installed direct major versions. If no compatible fix exists, submit a specific owner/expiry/rationale proposal and keep this task open until explicitly reviewed; do not force an unrelated major upgrade.
- **findings**: DEPENDABOT-101, NPM-WEB-GHSA-gcqx-w48m-w2h4-deepmerge
- **acceptance_criteria**:
  - npm audit no longer reports the advisory on each original dependency path, or an explicitly reviewed current exception documents the residual exposure.
  - Dependency graph, unit tests and both web builds remain compatible; no broad overrides that violate parent ranges.
- **validation**:
  - `npm --prefix web explain deepmerge`
  - `npm --prefix web audit --json`
  - `npm --prefix web audit --omit=dev --json`
  - `npm --prefix web test -- --runInBand`
  - `npm --prefix web run build`
  - `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build`
- **status**: Blocked — review prepared, awaiting compatible fix or maintainer decision
- **log**: Concrete review packet committed in `713b3025`; no package mutation, approval or disposition change. Current development-tool advisory ranges and dependency parents checked; production web audit zero. See task evidence and `quality/reviews/exception-ownership.json`.
- **files edited/created**:

### T7.11: cpp/cleartext-storage-database in db_event_destinations.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_event_destinations.c`, `tests/unit/test_t7_11.c`, `tests/unit/cmake/eng/t7_11.cmake`, `quality/evidence/T7.11.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-316, CODEQL-307
- **acceptance_criteria**:
  - Trace credential persistence and read access; document existing at-rest security and migration constraints. Test redaction and restricted access. If encryption changes the storage contract, materialize a separate bounded migration task instead of inventing a key-management scheme.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.11 --regex '^test_t7_11$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.11`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.12: cpp/cleartext-storage-database in db_streams.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_streams.c`, `tests/unit/test_t7_12.c`, `tests/unit/cmake/eng/t7_12.cmake`, `quality/evidence/T7.12.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-185, CODEQL-183, CODEQL-181
- **acceptance_criteria**:
  - Trace credential persistence and read access; document existing at-rest security and migration constraints. Test redaction and restricted access. If encryption changes the storage contract, materialize a separate bounded migration task instead of inventing a key-management scheme.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.12 --regex '^test_t7_12$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.12`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.13: cpp/cleartext-storage-file in config.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/config.c`, `tests/unit/test_t7_13.c`, `tests/unit/cmake/eng/t7_13.cmake`, `quality/evidence/T7.13.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-267, CODEQL-64, CODEQL-63
- **acceptance_criteria**:
  - Verify file permissions, creation umask, redacted logs and credential read access; preserve compatible config loading. Residual necessary plaintext needs explicit reviewed exposure acceptance.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.13 --regex '^test_t7_13$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.13`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.14: cpp/cleartext-transmission in hls_unified_thread.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_unified_thread.c`, `tests/unit/test_t7_14.c`, `tests/unit/cmake/eng/t7_14.cmake`, `quality/evidence/T7.14.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-264
- **acceptance_criteria**:
  - Assert credential-bearing payloads are not disclosed to an untrusted/insecure destination; verify supported camera transport behavior. Any required legacy cleartext transport needs an explicit reviewed exposure exception.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.14 --regex '^test_t7_14$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.14`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.15: cpp/cleartext-transmission in stream_protocol.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/stream_protocol.c`, `tests/unit/test_t7_15.c`, `tests/unit/cmake/eng/t7_15.cmake`, `quality/evidence/T7.15.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-14
- **acceptance_criteria**:
  - Assert credential-bearing payloads are not disclosed to an untrusted/insecure destination; verify supported camera transport behavior. Any required legacy cleartext transport needs an explicit reviewed exposure exception.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.15 --regex '^test_t7_15$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.15`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.16: cpp/integer-multiplication-cast-to-long in mp4_writer_utils.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_16.c`, `tests/unit/cmake/eng/t7_16.cmake`, `quality/evidence/T7.16.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-329
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.16 --regex '^test_t7_16$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.16`; inspect the raw T1 report and trace the caller.
- **status**: Completed (commit `60bc1b68`)
- **log**: Widen AAC bitrate operand before multiplication;16tests (3expected unsigned-wrap RED cases) nowGREEN, plus related media controls36/36 ASan/UBSan. Huge-channel cases use explicit seams; actual FAAC support/invalid-input controls remain unchanged. Fresh CodeQL remains T33.
- **files edited/created**:

### T7.17: cpp/non-https-url in go2rtc_api.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_api.c`, `tests/unit/test_t7_17.c`, `tests/unit/cmake/eng/t7_17.cmake`, `quality/evidence/T7.17.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-78, CODEQL-77, CODEQL-76, CODEQL-75, CODEQL-71, CODEQL-70, CODEQL-69, CODEQL-20, CODEQL-19
- **acceptance_criteria**:
  - Prove endpoint trust and transport policy for each reported URL; retain HTTP-only camera compatibility only with an explicitly reviewed exposure exception. Do not rewrite device endpoints to HTTPS without negotiation/support.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.17 --regex '^test_t7_17$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.17`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.18: cpp/non-https-url in onvif_detection.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_detection.c`, `tests/unit/test_t7_18.c`, `tests/unit/cmake/eng/t7_18.cmake`, `quality/evidence/T7.18.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-337, CODEQL-336, CODEQL-335, CODEQL-334, CODEQL-272, CODEQL-271, CODEQL-270
- **acceptance_criteria**:
  - Prove endpoint trust and transport policy for each reported URL; retain HTTP-only camera compatibility only with an explicitly reviewed exposure exception. Do not rewrite device endpoints to HTTPS without negotiation/support.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.18 --regex '^test_t7_18$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.18`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.19: cpp/non-https-url in onvif_discovery.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_discovery.c`, `tests/unit/test_t7_19.c`, `tests/unit/cmake/eng/t7_19.cmake`, `quality/evidence/T7.19.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-59
- **acceptance_criteria**:
  - Prove endpoint trust and transport policy for each reported URL; retain HTTP-only camera compatibility only with an explicitly reviewed exposure exception. Do not rewrite device endpoints to HTTPS without negotiation/support.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.19 --regex '^test_t7_19$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.19`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.20: cpp/non-https-url in api_handlers_ptz.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_ptz.c`, `tests/unit/test_t7_20.c`, `tests/unit/cmake/eng/t7_20.cmake`, `quality/evidence/T7.20.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-73
- **acceptance_criteria**:
  - Prove endpoint trust and transport policy for each reported URL; retain HTTP-only camera compatibility only with an explicitly reviewed exposure exception. Do not rewrite device endpoints to HTTPS without negotiation/support.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.20 --regex '^test_t7_20$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.20`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.21: cpp/path-injection in storage_s3.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_21.c`, `tests/unit/cmake/eng/t7_21.cmake`, `quality/evidence/T7.21.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-325
- **acceptance_criteria**:
  - Reject dot segments, separators and encoded traversal from untrusted names before filesystem access; assert normal names still select the intended file and no outside-tree path is opened.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.21 --regex '^test_t7_21$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.21`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.22: cpp/path-injection in linux_cgroup.c, linux_network.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_cgroup.c`, `src/telemetry/collectors/linux_network.c`, `tests/unit/test_t7_22.c`, `tests/unit/cmake/eng/t7_22.cmake`, `quality/evidence/T7.22.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-320, CODEQL-319, CODEQL-318, CODEQL-317
- **acceptance_criteria**:
  - Reject dot segments, separators and encoded traversal from untrusted names before filesystem access; assert normal names still select the intended file and no outside-tree path is opened.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.22 --regex '^test_t7_22$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.22`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.23: cpp/sql-injection in sqlite_migrate.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/sqlite_migrate.c`, `tests/unit/test_t7_23.c`, `tests/unit/cmake/eng/t7_23.cmake`, `quality/evidence/T7.23.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-67, CODEQL-66
- **acceptance_criteria**:
  - Use quote, semicolon and comment-bearing identifiers in a disposable database; ensure they cannot execute a second statement or alter unrelated schema, while valid migration names still work.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.23 --regex '^test_t7_23$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.23`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.24: cpp/suspicious-pointer-scaling in frame.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_24-upstream.json`, `quality/evidence/T7.24.md`.
- **priority**: high
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CODEQL-330
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_24-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.24`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.25: cpp/toctou-race-condition in db_core.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_core.c`, `tests/unit/test_t7_25.c`, `tests/unit/cmake/eng/t7_25.cmake`, `quality/evidence/T7.25.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-321
- **acceptance_criteria**:
  - Exercise replacement/symlink races between validation and use; operations must hold and use the validated descriptor or reject changed identity, preserving normal success and cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.25 --regex '^test_t7_25$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.25`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.26: cpp/toctou-race-condition in storage_migration.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_26.c`, `tests/unit/cmake/eng/t7_26.cmake`, `quality/evidence/T7.26.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-310
- **acceptance_criteria**:
  - Exercise replacement/symlink races between validation and use; operations must hold and use the validated descriptor or reject changed identity, preserving normal success and cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.26 --regex '^test_t7_26$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.26`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.27: cpp/toctou-race-condition in api_handlers_operator_floor_plans.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_operator_floor_plans.c`, `tests/unit/test_t7_27.c`, `tests/unit/cmake/eng/t7_27.cmake`, `quality/evidence/T7.27.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-315, CODEQL-314, CODEQL-313
- **acceptance_criteria**:
  - Exercise replacement/symlink races between validation and use; operations must hold and use the validated descriptor or reject changed identity, preserving normal success and cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.27 --regex '^test_t7_27$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.27`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.28: cpp/uncontrolled-allocation-size in event_router.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/event_router.c`, `tests/unit/test_t7_28.c`, `tests/unit/cmake/eng/t7_28.cmake`, `quality/evidence/T7.28.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-306, CODEQL-305
- **acceptance_criteria**:
  - Exercise zero, maximum accepted, one-past maximum and overflow-sized user inputs; reject before allocation/multiplication, leave state unchanged, and preserve ordinary delivery.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.28 --regex '^test_t7_28$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.28`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.29: cpp/world-writable-file-creation in mqtt_client.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_client.c`, `tests/unit/test_t7_29.c`, `tests/unit/cmake/eng/t7_29.cmake`, `quality/evidence/T7.29.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-293
- **acceptance_criteria**:
  - Create the credential-bearing file under a permissive umask and assert effective permissions exclude group/other access; retain existing owner and normal reload semantics.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.29 --regex '^test_t7_29$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.29`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.30: arrayIndexOutOfBounds in stream_protocol.c
- **depends_on**: [T7, T13, T7.15]
- **agent_type**: worker
- **ownership**: `src/video/stream_protocol.c`, `tests/unit/test_t7_30.c`, `tests/unit/cmake/eng/t7_30.cmake`, `quality/evidence/T7.30.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-0dfdce44e3f2c522
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.30 --regex '^test_t7_30$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.30`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.31: clang-analyzer-core.CallAndMessage in system_health_policy.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health_policy.c`, `tests/unit/test_t7_31.c`, `tests/unit/cmake/eng/t7_31.cmake`, `quality/evidence/T7.31.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-9b21a14b97b3a15e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.31 --regex '^test_t7_31$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.31`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.32: clang-analyzer-core.DivideZero in db_storage_pools.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_pools.c`, `tests/unit/test_t7_32.c`, `tests/unit/cmake/eng/t7_32.cmake`, `quality/evidence/T7.32.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-b7cb62f9287bca53
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.32 --regex '^test_t7_32$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.32`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.33: clang-analyzer-core.NonNullParamChecker in camera_collection_filter.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/camera_collection_filter.c`, `tests/unit/test_t7_33.c`, `tests/unit/cmake/eng/t7_33.cmake`, `quality/evidence/T7.33.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a918c6008bfe9234
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.33 --regex '^test_t7_33$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.33`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.34: clang-analyzer-core.NonNullParamChecker in db_storage_policies.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_policies.c`, `tests/unit/test_t7_34.c`, `tests/unit/cmake/eng/t7_34.cmake`, `quality/evidence/T7.34.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a5ab5fe6add78d8e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.34 --regex '^test_t7_34$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.34`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.35: clang-analyzer-core.NonNullParamChecker in storage_s3.c
- **depends_on**: [T7, T13, T7.21]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_35.c`, `tests/unit/cmake/eng/t7_35.cmake`, `quality/evidence/T7.35.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-cb260984bd5750f8
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.35 --regex '^test_t7_35$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.35`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.36: clang-analyzer-core.NullDereference in event_envelope.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/event_envelope.c`, `tests/unit/test_t7_36.c`, `tests/unit/cmake/eng/t7_36.cmake`, `quality/evidence/T7.36.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f240093f03b567fb
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.36 --regex '^test_t7_36$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.36`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.37: clang-analyzer-core.NullDereference in db_live_saved_layouts.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_live_saved_layouts.c`, `tests/unit/test_t7_37.c`, `tests/unit/cmake/eng/t7_37.cmake`, `quality/evidence/T7.37.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f8203edcc8863cd8
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.37 --regex '^test_t7_37$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.37`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.38: clang-analyzer-core.NullDereference in db_storage_policies.c
- **depends_on**: [T7, T13, T7.34]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_policies.c`, `tests/unit/test_t7_38.c`, `tests/unit/cmake/eng/t7_38.cmake`, `quality/evidence/T7.38.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-179513c629f1ecac
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.38 --regex '^test_t7_38$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.38`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.39: clang-analyzer-core.NullDereference in smartctl_provider.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/smartctl_provider.c`, `tests/unit/test_t7_39.c`, `tests/unit/cmake/eng/t7_39.cmake`, `quality/evidence/T7.39.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-33d1c32daba1778e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.39 --regex '^test_t7_39$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.39`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.40: clang-analyzer-core.NullDereference in go2rtc_api.c
- **depends_on**: [T7, T13, T7.17]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_api.c`, `tests/unit/test_t7_40.c`, `tests/unit/cmake/eng/t7_40.cmake`, `quality/evidence/T7.40.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-14fc6cb721e4ff46
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.40 --regex '^test_t7_40$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.40`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.41: clang-analyzer-core.NullDereference in hls_writer.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/hls_writer.c`, `tests/unit/test_t7_41.c`, `tests/unit/cmake/eng/t7_41.cmake`, `quality/evidence/T7.41.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-848024a1059f9105, CLANGTIDY-c18dbfa67bfd519e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.41 --regex '^test_t7_41$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.41`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.42: clang-analyzer-core.NullDereference in api_handlers_authorization.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_authorization.c`, `tests/unit/test_t7_42.c`, `tests/unit/cmake/eng/t7_42.cmake`, `quality/evidence/T7.42.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-3853c804fd823d76, CLANGTIDY-e903e0b58c1ab7fb, CLANGTIDY-7037bcf50d8dd6e1, CLANGTIDY-468c829b1cc867f5, CLANGTIDY-5546a3c4710ffc07, CLANGTIDY-9c42b9e1d1f45117, CLANGTIDY-b45db718e3e94b0d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.42 --regex '^test_t7_42$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.42`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.43: clang-analyzer-core.NullDereference in api_handlers_fleet.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_fleet.c`, `tests/unit/test_t7_43.c`, `tests/unit/cmake/eng/t7_43.cmake`, `quality/evidence/T7.43.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-dc1d8989b6dc0f0a
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.43 --regex '^test_t7_43$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.43`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.44: clang-analyzer-core.NullDereference in api_handlers_health.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_health.c`, `tests/unit/test_t7_44.c`, `tests/unit/cmake/eng/t7_44.cmake`, `quality/evidence/T7.44.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-18224b07c8f96008
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.44 --regex '^test_t7_44$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.44`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.45: clang-analyzer-core.NullDereference in api_handlers_lpr.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_lpr.c`, `tests/unit/test_t7_45.c`, `tests/unit/cmake/eng/t7_45.cmake`, `quality/evidence/T7.45.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-36ab8111fbec813b, CLANGTIDY-21f4fa94b0f2d17d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.45 --regex '^test_t7_45$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.45`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.46: clang-analyzer-core.NullDereference in api_handlers_recording_control.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_recording_control.c`, `tests/unit/test_t7_46.c`, `tests/unit/cmake/eng/t7_46.cmake`, `quality/evidence/T7.46.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-e722e2369c13444f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.46 --regex '^test_t7_46$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.46`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.47: clang-analyzer-core.NullDereference in api_handlers_retention.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_retention.c`, `tests/unit/test_t7_47.c`, `tests/unit/cmake/eng/t7_47.cmake`, `quality/evidence/T7.47.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-67d853afc9b7eefe
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.47 --regex '^test_t7_47$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.47`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.48: clang-analyzer-core.NullDereference in api_handlers_storage_migrations.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_storage_migrations.c`, `tests/unit/test_t7_48.c`, `tests/unit/cmake/eng/t7_48.cmake`, `quality/evidence/T7.48.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-44c26e7cc4f14d25, CLANGTIDY-7f68b57abf95cdae
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.48 --regex '^test_t7_48$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.48`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.49: clang-analyzer-core.NullDereference in api_handlers_streams_get.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `tests/unit/test_t7_49.c`, `tests/unit/cmake/eng/t7_49.cmake`, `quality/evidence/T7.49.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-2b9b559954a51bc7
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.49 --regex '^test_t7_49$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.49`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.50: clang-analyzer-core.NullDereference in api_handlers_streams_modify.c
- **depends_on**: [T7, T13, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/unit/test_t7_50.c`, `tests/unit/cmake/eng/t7_50.cmake`, `quality/evidence/T7.50.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-3ac69a72123df761
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.50 --regex '^test_t7_50$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.50`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.51: clang-analyzer-core.NullDereference in api_handlers_system_health.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system_health.c`, `tests/unit/test_t7_51.c`, `tests/unit/cmake/eng/t7_51.cmake`, `quality/evidence/T7.51.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-2eb8f8220e64672b, CLANGTIDY-8935718b76906f1e, CLANGTIDY-db6d4840afaa43ce, CLANGTIDY-6253a319b7ed3b31
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.51 --regex '^test_t7_51$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.51`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.52: clang-analyzer-core.UndefinedBinaryOperatorResult in path_utils.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/path_utils.c`, `tests/unit/test_t7_52.c`, `tests/unit/cmake/eng/t7_52.cmake`, `quality/evidence/T7.52.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d86514e7015dbd70
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.52 --regex '^test_t7_52$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.52`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.53: clang-analyzer-security.ArrayBound in authorization.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/authorization.c`, `tests/unit/test_t7_53.c`, `tests/unit/cmake/eng/t7_53.cmake`, `quality/evidence/T7.53.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c4536c7eb269c97d, CLANGTIDY-dc57edf3e0b50ae4
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.53 --regex '^test_t7_53$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.53`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.54: clang-analyzer-security.ArrayBound in logger_json.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/logger_json.c`, `tests/unit/test_t7_54.c`, `tests/unit/cmake/eng/t7_54.cmake`, `quality/evidence/T7.54.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ff30996a0be1b3ef
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.54 --regex '^test_t7_54$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.54`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.55: clang-analyzer-security.ArrayBound in main.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/main.c`, `tests/unit/test_t7_55.c`, `tests/unit/cmake/eng/t7_55.cmake`, `quality/evidence/T7.55.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-470e897ffa76864e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.55 --regex '^test_t7_55$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.55`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.56: clang-analyzer-security.ArrayBound in sqlite_migrate.c
- **depends_on**: [T7, T13, T7.23]
- **agent_type**: worker
- **ownership**: `src/database/sqlite_migrate.c`, `tests/unit/test_t7_56.c`, `tests/unit/cmake/eng/t7_56.cmake`, `quality/evidence/T7.56.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-2772ec37ee463512
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.56 --regex '^test_t7_56$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.56`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.57: clang-analyzer-security.ArrayBound in linux_network.c, linux_process.c, linux_restart.c, linux_thermal.c
- **depends_on**: [T7, T13, T7.22]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `src/telemetry/collectors/linux_process.c`, `src/telemetry/collectors/linux_restart.c`, `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_57.c`, `tests/unit/cmake/eng/t7_57.cmake`, `quality/evidence/T7.57.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-9259d0adb44e3ab3, CLANGTIDY-55f597a830e65b4d, CLANGTIDY-eaf20c1f0de62870, CLANGTIDY-c05a923060c50809, CLANGTIDY-30cd36651cb697e6
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.57 --regex '^test_t7_57$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.57`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.58: clang-analyzer-security.ArrayBound in linux_hardware.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/linux_hardware.c`, `tests/unit/test_t7_58.c`, `tests/unit/cmake/eng/t7_58.cmake`, `quality/evidence/T7.58.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-29fe00c504db3f7e, CLANGTIDY-93f7df7451e39223
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.58 --regex '^test_t7_58$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.58`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.59: clang-analyzer-security.ArrayBound in system_health.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health.c`, `tests/unit/test_t7_59.c`, `tests/unit/cmake/eng/t7_59.cmake`, `quality/evidence/T7.59.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-192595ad5f7437f2
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.59 --regex '^test_t7_59$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.59`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.60: clang-analyzer-security.ArrayBound in go2rtc_process.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_process.c`, `tests/unit/test_t7_60.c`, `tests/unit/cmake/eng/t7_60.cmake`, `quality/evidence/T7.60.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6eb7be7d5d4be88a
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.60 --regex '^test_t7_60$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.60`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.61: clang-analyzer-security.ArrayBound in hls_cache_cleanup.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_cache_cleanup.c`, `tests/unit/test_t7_61.c`, `tests/unit/cmake/eng/t7_61.cmake`, `quality/evidence/T7.61.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-e88c078f85b44a5e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.61 --regex '^test_t7_61$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.61`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.62: clang-analyzer-security.ArrayBound in stream_manager.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/stream_manager.c`, `tests/unit/test_t7_62.c`, `tests/unit/cmake/eng/t7_62.cmake`, `quality/evidence/T7.62.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-87fe5c4f9b2c4103
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.62 --regex '^test_t7_62$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.62`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.63: clang-analyzer-security.ArrayBound in api_handlers_fleet.c
- **depends_on**: [T7, T13, T7.43]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_fleet.c`, `tests/unit/test_t7_63.c`, `tests/unit/cmake/eng/t7_63.cmake`, `quality/evidence/T7.63.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-b6500f12e4ee30b7, CLANGTIDY-a01b119b5b6a4463
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.63 --regex '^test_t7_63$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.63`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.64: clang-analyzer-security.ArrayBound in api_handlers_streams_get.c
- **depends_on**: [T7, T13, T7.49]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `tests/unit/test_t7_64.c`, `tests/unit/cmake/eng/t7_64.cmake`, `quality/evidence/T7.64.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d562d8dbfbb1dcc3, CLANGTIDY-24b2873ab4fb113e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.64 --regex '^test_t7_64$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.64`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.65: clang-analyzer-security.ArrayBound in api_handlers_system.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system.c`, `tests/unit/test_t7_65.c`, `tests/unit/cmake/eng/t7_65.cmake`, `quality/evidence/T7.65.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-4f6abbd42f9a1855, CLANGTIDY-c7df8f3cce68c5a4
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.65 --regex '^test_t7_65$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.65`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.66: clang-analyzer-security.insecureAPI.strcpy in recording_transcode.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/recording_transcode.c`, `tests/unit/test_t7_66.c`, `tests/unit/cmake/eng/t7_66.cmake`, `quality/evidence/T7.66.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d6eef8565b934dd8, CLANGTIDY-4976fee144691228
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.66 --regex '^test_t7_66$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.66`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.67: clang-analyzer-security.insecureAPI.strcpy in api_handlers_go2rtc_proxy.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_go2rtc_proxy.c`, `tests/unit/test_t7_67.c`, `tests/unit/cmake/eng/t7_67.cmake`, `quality/evidence/T7.67.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-4c710ebaa0f3ef18, CLANGTIDY-e7f2b500346b1db4, CLANGTIDY-59d7e244d1294daa, CLANGTIDY-de5c9750f9e973f2
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.67 --regex '^test_t7_67$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.67`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.68: clang-analyzer-unix.BlockInCriticalSection in health_helper_runner.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/health_helper_runner.c`, `tests/unit/test_t7_68.c`, `tests/unit/cmake/eng/t7_68.cmake`, `quality/evidence/T7.68.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-8a0289975bb00ddb
- **acceptance_criteria**:
  - Exercise the blocking operation and concurrent shutdown/contending thread with bounded deadlines; assert no lock is held across unbounded I/O and lock-protected invariants remain intact.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.68 --regex '^test_t7_68$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.68`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.69: clang-analyzer-unix.BlockInCriticalSection in hls_unified_thread.c
- **depends_on**: [T7, T13, T7.14]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_unified_thread.c`, `tests/unit/test_t7_69.c`, `tests/unit/cmake/eng/t7_69.cmake`, `quality/evidence/T7.69.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-1728287dd1cfa5a4
- **acceptance_criteria**:
  - Exercise the blocking operation and concurrent shutdown/contending thread with bounded deadlines; assert no lock is held across unbounded I/O and lock-protected invariants remain intact.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.69 --regex '^test_t7_69$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.69`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.70: clang-analyzer-unix.Errno in storage_migration.c
- **depends_on**: [T7, T13, T7.26]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_70.c`, `tests/unit/cmake/eng/t7_70.cmake`, `quality/evidence/T7.70.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f15ef4cfb6675401, CLANGTIDY-d8903761e409b32d
- **acceptance_criteria**:
  - Inject failing and successful calls with stale errno; branch only on actual call failure and preserve the original error before cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.70 --regex '^test_t7_70$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.70`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.71: clang-analyzer-unix.StdCLibraryFunctions in hls_cache_cleanup.c
- **depends_on**: [T7, T13, T7.61]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_cache_cleanup.c`, `tests/unit/test_t7_71.c`, `tests/unit/cmake/eng/t7_71.cmake`, `quality/evidence/T7.71.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f70e12d3b12447f9, CLANGTIDY-81dd4bf2eb1bd4a8
- **acceptance_criteria**:
  - Exercise every reported format/library call with boundary values; argument types/counts, buffer extents and returned errors must match the library contract with no UB.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.71 --regex '^test_t7_71$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.71`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.72: clang-analyzer-unix.Stream in linux_network.c
- **depends_on**: [T7, T13, T7.57]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `tests/unit/test_t7_72.c`, `tests/unit/cmake/eng/t7_72.cmake`, `quality/evidence/T7.72.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-b38fd5e59ea07d0e, CLANGTIDY-ebeb3bd843bc6421
- **acceptance_criteria**:
  - Inject each allocation/open and downstream failure at the reported site; assert allocations, FILE handles and descriptors return to baseline on every exit and successful ownership transfers remain valid.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.72 --regex '^test_t7_72$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.72`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.73: integerOverflow in config.c
- **depends_on**: [T7, T13, T7.13]
- **agent_type**: worker
- **ownership**: `src/core/config.c`, `tests/unit/test_t7_73.c`, `tests/unit/cmake/eng/t7_73.cmake`, `quality/evidence/T7.73.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-a8a2fee09e04d7a2
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.73 --regex '^test_t7_73$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.73`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.74: memleak in sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_74-upstream.json`, `quality/evidence/T7.74.md`.
- **priority**: medium
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-582ed0e8d5a10fc1
- **acceptance_criteria**:
  - Inject each allocation/open and downstream failure at the reported site; assert allocations, FILE handles and descriptors return to baseline on every exit and successful ownership transfers remain valid.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_74-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.74`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.75: memleak in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_75-upstream.json`, `quality/evidence/T7.75.md`.
- **priority**: medium
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-d14ab8cc9d321477, CPPCHECK-7bc58272fe78e01c, CPPCHECK-d1406d76ca075628, CPPCHECK-187d49bac01afe0b, CPPCHECK-01d35c4f8b07a420, CPPCHECK-7ed0f3ac85abfc0f, CPPCHECK-afd0bb6171ab9102, CPPCHECK-6c275f8832cf9762, CPPCHECK-340c5f9dec875049, CPPCHECK-94d7d3c523b01523, CPPCHECK-65f97e38c1e1c5df, CPPCHECK-377feb103d4c3b66
- **acceptance_criteria**:
  - Inject each allocation/open and downstream failure at the reported site; assert allocations, FILE handles and descriptors return to baseline on every exit and successful ownership transfers remain valid.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_75-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.75`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.76: preprocessorErrorDirective in nvme_provider.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/nvme_provider.c`, `tests/unit/test_t7_76.c`, `tests/unit/cmake/eng/t7_76.cmake`, `quality/evidence/T7.76.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-1c7b293c20ba8b3d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.76 --regex '^test_t7_76$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.76`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.77: preprocessorErrorDirective in mp4_writer_utils.c
- **depends_on**: [T7, T13, T7.16]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_77.c`, `tests/unit/cmake/eng/t7_77.cmake`, `quality/evidence/T7.77.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-1afeb1315d85ff61
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.77 --regex '^test_t7_77$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.77`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.78: arrayIndexOutOfBoundsCond in stream_protocol.c
- **depends_on**: [T7, T13, T7.30]
- **agent_type**: worker
- **ownership**: `src/video/stream_protocol.c`, `tests/unit/test_t7_78.c`, `tests/unit/cmake/eng/t7_78.cmake`, `quality/evidence/T7.78.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-1d3bfb4e5fc184fe
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.78 --regex '^test_t7_78$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.78`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.79: bugprone-assignment-in-if-condition in db_system_health_incidents.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_system_health_incidents.c`, `tests/unit/test_t7_79.c`, `tests/unit/cmake/eng/t7_79.cmake`, `quality/evidence/T7.79.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-808f26556ac98110, CLANGTIDY-7e1ce15fe947a21d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.79 --regex '^test_t7_79$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.79`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.80: bugprone-assignment-in-if-condition in api_detection.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/api_detection.c`, `tests/unit/test_t7_80.c`, `tests/unit/cmake/eng/t7_80.cmake`, `quality/evidence/T7.80.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c34509f97f5eb3bf, CLANGTIDY-6ac00b1420332d40, CLANGTIDY-1848d1ca57216f44, CLANGTIDY-be52eb80a1a9b4e2
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.80 --regex '^test_t7_80$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.80`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.81: bugprone-assignment-in-if-condition in api_handlers_metrics.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_metrics.c`, `tests/unit/test_t7_81.c`, `tests/unit/cmake/eng/t7_81.cmake`, `quality/evidence/T7.81.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-02365646d53a2c50, CLANGTIDY-a219d74f760de10e, CLANGTIDY-96f8eef93ec11127, CLANGTIDY-221d97bd8b8ca157, CLANGTIDY-b6b7c0de32a0c6b7, CLANGTIDY-4edbb01e2c9075ed, CLANGTIDY-ec9cd5d6ab7a3d6e, CLANGTIDY-2f30db61a3888f56
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.81 --regex '^test_t7_81$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.81`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.82: bugprone-assignment-in-if-condition in go2rtc_websocket.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_websocket.c`, `tests/unit/test_t7_82.c`, `tests/unit/cmake/eng/t7_82.cmake`, `quality/evidence/T7.82.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-82315a3f137f4aff
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.82 --regex '^test_t7_82$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.82`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.83: bugprone-branch-clone in linux_cgroup.c
- **depends_on**: [T7, T13, T7.22]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_cgroup.c`, `tests/unit/test_t7_83.c`, `tests/unit/cmake/eng/t7_83.cmake`, `quality/evidence/T7.83.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-eac1097bd3780c16
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.83 --regex '^test_t7_83$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.83`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.84: bugprone-branch-clone in api_handlers_authorization.c
- **depends_on**: [T7, T13, T7.42]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_authorization.c`, `tests/unit/test_t7_84.c`, `tests/unit/cmake/eng/t7_84.cmake`, `quality/evidence/T7.84.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a03bd79d6040e625
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.84 --regex '^test_t7_84$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.84`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.85: bugprone-branch-clone in api_handlers_timeline.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_timeline.c`, `tests/unit/test_t7_85.c`, `tests/unit/cmake/eng/t7_85.cmake`, `quality/evidence/T7.85.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f1f70f55c95dfbaf
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.85 --regex '^test_t7_85$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.85`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.86: bugprone-branch-clone in audit_log.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/audit_log.c`, `tests/unit/test_t7_86.c`, `tests/unit/cmake/eng/t7_86.cmake`, `quality/evidence/T7.86.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-5bdf00371aedee4a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.86 --regex '^test_t7_86$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.86`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.87: bugprone-branch-clone in thumbnail_thread.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/thumbnail_thread.c`, `tests/unit/test_t7_87.c`, `tests/unit/cmake/eng/t7_87.cmake`, `quality/evidence/T7.87.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ccff9a5b097b862e
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.87 --regex '^test_t7_87$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.87`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.88: bugprone-implicit-widening-of-multiplication-result in event_bus.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/event_bus.c`, `tests/unit/test_t7_88.c`, `tests/unit/cmake/eng/t7_88.cmake`, `quality/evidence/T7.88.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-73322317a31f8bcc, CLANGTIDY-dbe8bf00e4b146ee
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.88 --regex '^test_t7_88$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.88`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.89: bugprone-implicit-widening-of-multiplication-result in event_envelope.c
- **depends_on**: [T7, T13, T7.36]
- **agent_type**: worker
- **ownership**: `src/core/event_envelope.c`, `tests/unit/test_t7_89.c`, `tests/unit/cmake/eng/t7_89.cmake`, `quality/evidence/T7.89.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-2fcad01867c4b146, CLANGTIDY-51ab98673b990b2c, CLANGTIDY-a48d8d980b8dbfcc
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.89 --regex '^test_t7_89$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.89`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.90: bugprone-implicit-widening-of-multiplication-result in event_router.c
- **depends_on**: [T7, T13, T7.28]
- **agent_type**: worker
- **ownership**: `src/core/event_router.c`, `tests/unit/test_t7_90.c`, `tests/unit/cmake/eng/t7_90.cmake`, `quality/evidence/T7.90.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-767515747827ef14
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.90 --regex '^test_t7_90$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.90`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.91: bugprone-implicit-widening-of-multiplication-result in mqtt_delivery_worker.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_delivery_worker.c`, `tests/unit/test_t7_91.c`, `tests/unit/cmake/eng/t7_91.cmake`, `quality/evidence/T7.91.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ced78137a6871fda
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.91 --regex '^test_t7_91$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.91`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.92: bugprone-implicit-widening-of-multiplication-result in db_operator_floor_plans.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_operator_floor_plans.c`, `tests/unit/test_t7_92.c`, `tests/unit/cmake/eng/t7_92.cmake`, `quality/evidence/T7.92.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-4ce72e7a43d0bcce
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.92 --regex '^test_t7_92$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.92`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.93: bugprone-implicit-widening-of-multiplication-result in storage_migration.c
- **depends_on**: [T7, T13, T7.70]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_93.c`, `tests/unit/cmake/eng/t7_93.cmake`, `quality/evidence/T7.93.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-50703d3018f9bed5, CLANGTIDY-15fa53ad8b574808, CLANGTIDY-e5960a2d15356c27, CLANGTIDY-1fac122c6010eb36
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.93 --regex '^test_t7_93$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.93`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.94: bugprone-implicit-widening-of-multiplication-result in storage_s3.c
- **depends_on**: [T7, T13, T7.35]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_94.c`, `tests/unit/cmake/eng/t7_94.cmake`, `quality/evidence/T7.94.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-612c846365824225, CLANGTIDY-1e0ea76108754cd5, CLANGTIDY-ab4dc4b3768799ca, CLANGTIDY-14f55668eaa243b0, CLANGTIDY-fbf040917d8b6cf7, CLANGTIDY-52dada64b4a6e625, CLANGTIDY-d1d1cf8dcb7070f8, CLANGTIDY-bfb7d2825452ce0c, CLANGTIDY-842c6bb171a64e75, CLANGTIDY-4a2a60d787933d7d, CLANGTIDY-a145bd05b0189193, CLANGTIDY-f72760acc6c09e16, CLANGTIDY-0866b8e57f533141, CLANGTIDY-d08422316656b39c, CLANGTIDY-d7c0ab1bf29a42a3
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.94 --regex '^test_t7_94$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.94`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.95: bugprone-implicit-widening-of-multiplication-result in linux_network.c, linux_process.c, linux_thermal.c
- **depends_on**: [T7, T13, T7.72, T7.57]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `src/telemetry/collectors/linux_process.c`, `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_95.c`, `tests/unit/cmake/eng/t7_95.cmake`, `quality/evidence/T7.95.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-8d33cee7806f5d43, CLANGTIDY-02b18b5e0f4e3b93, CLANGTIDY-b84c577b6e369ac3, CLANGTIDY-8b66085f0250d8e1, CLANGTIDY-15838d8765a7623e, CLANGTIDY-df8a271dac7dc00c
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.95 --regex '^test_t7_95$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.95`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.96: bugprone-implicit-widening-of-multiplication-result in ffmpeg_rss_guard.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/tools/ffmpeg_rss_guard.c`, `tests/unit/test_t7_96.c`, `tests/unit/cmake/eng/t7_96.cmake`, `quality/evidence/T7.96.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-802a0570e39a96a6, CLANGTIDY-a072eb98578bd9cf
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.96 --regex '^test_t7_96$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.96`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.97: bugprone-implicit-widening-of-multiplication-result in lpr_crypto.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/utils/lpr_crypto.c`, `tests/unit/test_t7_97.c`, `tests/unit/cmake/eng/t7_97.cmake`, `quality/evidence/T7.97.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-576d7acbaee5cef6
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.97 --regex '^test_t7_97$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.97`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.98: bugprone-implicit-widening-of-multiplication-result in go2rtc_api.c, go2rtc_consumer.c, go2rtc_integration.c, go2rtc_snapshot.c, go2rtc_stream.c
- **depends_on**: [T7, T13, T7.40]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_api.c`, `src/video/go2rtc/go2rtc_consumer.c`, `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_snapshot.c`, `src/video/go2rtc/go2rtc_stream.c`, `tests/unit/test_t7_98.c`, `tests/unit/cmake/eng/t7_98.cmake`, `quality/evidence/T7.98.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-eadc49a35e808a77, CLANGTIDY-bdcbae59a275ce50, CLANGTIDY-5582e7df3dd25730, CLANGTIDY-23826be3117b0a5e, CLANGTIDY-cac166160ef5ab94, CLANGTIDY-6eced00c619f459a, CLANGTIDY-a35257b195053d18, CLANGTIDY-9fbcc527095ebbbb, CLANGTIDY-c263543788ddaa36, CLANGTIDY-d06458b2bdfca56d, CLANGTIDY-1592d34d4e3dc54c, CLANGTIDY-957e3e8e5e143726, CLANGTIDY-76ebb6a603073b54, CLANGTIDY-2c1116134a2ee5a3, CLANGTIDY-0ce2e7833fc885fd, CLANGTIDY-4e94aa9ff957dec6, CLANGTIDY-f6ddc3b14e23e020
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.98 --regex '^test_t7_98$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.98`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.99: bugprone-implicit-widening-of-multiplication-result in mp4_writer_utils.c
- **depends_on**: [T7, T13, T7.77]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_99.c`, `tests/unit/cmake/eng/t7_99.cmake`, `quality/evidence/T7.99.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a19387fbb98806ae
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.99 --regex '^test_t7_99$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.99`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.100: bugprone-implicit-widening-of-multiplication-result in onvif_detection.c
- **depends_on**: [T7, T13, T7.18]
- **agent_type**: worker
- **ownership**: `src/video/onvif_detection.c`, `tests/unit/test_t7_100.c`, `tests/unit/cmake/eng/t7_100.cmake`, `quality/evidence/T7.100.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-18362aef7fade326, CLANGTIDY-eade1f2d38582a47, CLANGTIDY-8b074e54e83681ff
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.100 --regex '^test_t7_100$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.100`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.101: bugprone-implicit-widening-of-multiplication-result in onvif_event.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_event.c`, `tests/unit/test_t7_101.c`, `tests/unit/cmake/eng/t7_101.cmake`, `quality/evidence/T7.101.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-eb4e649dccdb530f
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.101 --regex '^test_t7_101$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.101`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.102: bugprone-implicit-widening-of-multiplication-result in api_handlers_investigations.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_investigations.c`, `tests/unit/test_t7_102.c`, `tests/unit/cmake/eng/t7_102.cmake`, `quality/evidence/T7.102.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6829e02764d9d278, CLANGTIDY-113735d7300e4f01, CLANGTIDY-25b00beae5a3f4a8, CLANGTIDY-2387b22f15e219d3
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.102 --regex '^test_t7_102$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.102`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.103: bugprone-implicit-widening-of-multiplication-result in api_handlers_lpr.c
- **depends_on**: [T7, T13, T7.45]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_lpr.c`, `tests/unit/test_t7_103.c`, `tests/unit/cmake/eng/t7_103.cmake`, `quality/evidence/T7.103.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-0818b8d62b6628c5
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.103 --regex '^test_t7_103$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.103`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.104: bugprone-implicit-widening-of-multiplication-result in api_handlers_operator_floor_plans.c
- **depends_on**: [T7, T13, T7.27]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_operator_floor_plans.c`, `tests/unit/test_t7_104.c`, `tests/unit/cmake/eng/t7_104.cmake`, `quality/evidence/T7.104.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-53e88d8983baf290, CLANGTIDY-dbd6a5de3a7e191c
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.104 --regex '^test_t7_104$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.104`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.105: bugprone-implicit-widening-of-multiplication-result in api_handlers_storage.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_storage.c`, `tests/unit/test_t7_105.c`, `tests/unit/cmake/eng/t7_105.cmake`, `quality/evidence/T7.105.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-88f84938e1b8feb0
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.105 --regex '^test_t7_105$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.105`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.106: bugprone-implicit-widening-of-multiplication-result in api_handlers_system_go2rtc.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system_go2rtc.c`, `tests/unit/test_t7_106.c`, `tests/unit/cmake/eng/t7_106.cmake`, `quality/evidence/T7.106.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-101a7d6e0cfda715, CLANGTIDY-c586f58fa85c527e
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.106 --regex '^test_t7_106$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.106`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.107: bugprone-implicit-widening-of-multiplication-result in api_handlers_timeline.c
- **depends_on**: [T7, T13, T7.85]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_timeline.c`, `tests/unit/test_t7_107.c`, `tests/unit/cmake/eng/t7_107.cmake`, `quality/evidence/T7.107.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-18895086ef6f0ed9
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.107 --regex '^test_t7_107$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.107`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.108: bugprone-implicit-widening-of-multiplication-result in go2rtc_websocket.c
- **depends_on**: [T7, T13, T7.82]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_websocket.c`, `tests/unit/test_t7_108.c`, `tests/unit/cmake/eng/t7_108.cmake`, `quality/evidence/T7.108.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6e45cad03ed5a9f3
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.108 --regex '^test_t7_108$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.108`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.109: bugprone-implicit-widening-of-multiplication-result in recording_archive.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/recording_archive.c`, `tests/unit/test_t7_109.c`, `tests/unit/cmake/eng/t7_109.cmake`, `quality/evidence/T7.109.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-5bcac8af8164c1b0, CLANGTIDY-e2b358165654b79e
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.109 --regex '^test_t7_109$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.109`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.110: bugprone-inc-dec-in-conditions in storage_source.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_source.c`, `tests/unit/test_t7_110.c`, `tests/unit/cmake/eng/t7_110.cmake`, `quality/evidence/T7.110.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-9613b39c2e0a1e4c
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.110 --regex '^test_t7_110$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.110`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.111: bugprone-macro-parentheses in cJSON.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_111-upstream.json`, `quality/evidence/T7.111.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CLANGTIDY-5f6ae30cde3aaf3d, CLANGTIDY-bc48f96894493869, CLANGTIDY-01e4c541adfa23e3, CLANGTIDY-3abdad2ed8a71fe5, CLANGTIDY-09829500c354c2b5, CLANGTIDY-661c1d9d010f4873, CLANGTIDY-f69ef126d8e55b2b, CLANGTIDY-6d1385324d08479d
- **acceptance_criteria**:
  - Compile/evaluate precedence-sensitive macro arguments and side effects; results must match the intended fully parenthesized expression without double evaluation.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_111-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.111`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.112: bugprone-macro-parentheses in api_detection.c
- **depends_on**: [T7, T13, T7.80]
- **agent_type**: worker
- **ownership**: `src/video/api_detection.c`, `tests/unit/test_t7_112.c`, `tests/unit/cmake/eng/t7_112.cmake`, `quality/evidence/T7.112.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ae25a6959341dd10
- **acceptance_criteria**:
  - Compile/evaluate precedence-sensitive macro arguments and side effects; results must match the intended fully parenthesized expression without double evaluation.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.112 --regex '^test_t7_112$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.112`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.113: bugprone-misplaced-widening-cast in db_fleet_query.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_fleet_query.c`, `tests/unit/test_t7_113.c`, `tests/unit/cmake/eng/t7_113.cmake`, `quality/evidence/T7.113.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-7bff304a77769e6c
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.113 --regex '^test_t7_113$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.113`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.114: bugprone-multi-level-implicit-pointer-conversion in authorization.c
- **depends_on**: [T7, T13, T7.53]
- **agent_type**: worker
- **ownership**: `src/core/authorization.c`, `tests/unit/test_t7_114.c`, `tests/unit/cmake/eng/t7_114.cmake`, `quality/evidence/T7.114.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-4505cfde8bb2e42a, CLANGTIDY-ff0e443bb0b29339
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.114 --regex '^test_t7_114$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.114`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.115: bugprone-multi-level-implicit-pointer-conversion in camera_collection_filter.c
- **depends_on**: [T7, T13, T7.33]
- **agent_type**: worker
- **ownership**: `src/core/camera_collection_filter.c`, `tests/unit/test_t7_115.c`, `tests/unit/cmake/eng/t7_115.cmake`, `quality/evidence/T7.115.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-3a7569f24a5603a4, CLANGTIDY-54a9eba3b05f0872, CLANGTIDY-a221992baafec96f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.115 --regex '^test_t7_115$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.115`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.116: bugprone-multi-level-implicit-pointer-conversion in camera_selector.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/camera_selector.c`, `tests/unit/test_t7_116.c`, `tests/unit/cmake/eng/t7_116.cmake`, `quality/evidence/T7.116.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-8e1debc0e42f328a, CLANGTIDY-1fb6fbd990b73e55, CLANGTIDY-52e7ee5b7b20725e, CLANGTIDY-a816d3388754e9a3, CLANGTIDY-bdf2049affa5a3fd, CLANGTIDY-946781b07a8a19f6
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.116 --regex '^test_t7_116$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.116`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.117: bugprone-multi-level-implicit-pointer-conversion in db_storage_lifecycle.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_lifecycle.c`, `tests/unit/test_t7_117.c`, `tests/unit/cmake/eng/t7_117.cmake`, `quality/evidence/T7.117.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-39f0ad6017069f02, CLANGTIDY-c666354bdbc1df4b, CLANGTIDY-c8b906651c5b2c7f, CLANGTIDY-8a2d068dc3e8b0c0, CLANGTIDY-ad7afdbd70ca0a46
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.117 --regex '^test_t7_117$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.117`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.118: bugprone-multi-level-implicit-pointer-conversion in hls_cache_cleanup.c
- **depends_on**: [T7, T13, T7.71]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_cache_cleanup.c`, `tests/unit/test_t7_118.c`, `tests/unit/cmake/eng/t7_118.cmake`, `quality/evidence/T7.118.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-48d30c8fb35cc9cd, CLANGTIDY-1634947d8289644f, CLANGTIDY-cb9a5afe872113f7, CLANGTIDY-098d117140abdf3a, CLANGTIDY-a89af13dd80ae187
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.118 --regex '^test_t7_118$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.118`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.119: bugprone-multi-level-implicit-pointer-conversion in mp4_segment_recorder.c
- **depends_on**: [T7, T13, T28.2]
- **agent_type**: worker
- **ownership**: `src/video/mp4_segment_recorder.c`, `tests/unit/test_t7_119.c`, `tests/unit/cmake/eng/t7_119.cmake`, `quality/evidence/T7.119.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-30c3f7edea8fbd8d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.119 --regex '^test_t7_119$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.119`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.120: bugprone-multi-level-implicit-pointer-conversion in mp4_writer_core.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_core.c`, `tests/unit/test_t7_120.c`, `tests/unit/cmake/eng/t7_120.cmake`, `quality/evidence/T7.120.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d421e791a6b63cc9
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.120 --regex '^test_t7_120$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.120`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.121: bugprone-multi-level-implicit-pointer-conversion in mp4_writer_utils.c
- **depends_on**: [T7, T13, T7.99]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_121.c`, `tests/unit/cmake/eng/t7_121.cmake`, `quality/evidence/T7.121.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c536433a0089e4b9, CLANGTIDY-4e34d6b524364f03
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.121 --regex '^test_t7_121$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.121`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.122: bugprone-multi-level-implicit-pointer-conversion in unified_detection_thread.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/unified_detection_thread.c`, `tests/unit/test_t7_122.c`, `tests/unit/cmake/eng/t7_122.cmake`, `quality/evidence/T7.122.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-5dc4623d12ccf5b2
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.122 --regex '^test_t7_122$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.122`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.123: bugprone-multi-level-implicit-pointer-conversion in api_handlers_camera_collections.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_camera_collections.c`, `tests/unit/test_t7_123.c`, `tests/unit/cmake/eng/t7_123.cmake`, `quality/evidence/T7.123.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-1f9c86e17ca383f9, CLANGTIDY-316e32d23b15b780, CLANGTIDY-7464eb7864628fd1, CLANGTIDY-5e2b3b364d163173, CLANGTIDY-81fbfb962103b645, CLANGTIDY-c41e983019497e88, CLANGTIDY-308b0349ef001b56, CLANGTIDY-7a3f68ca1e19c9db, CLANGTIDY-475d0f82a4e9b764, CLANGTIDY-2276a7bff678cf41, CLANGTIDY-63c736995704d2d6, CLANGTIDY-f5ac788b77f1417a, CLANGTIDY-ae6a96ea4d41087d, CLANGTIDY-f617ccfb4cfc3cc2, CLANGTIDY-169001b0559c7bcb, CLANGTIDY-6bce10f282924906
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.123 --regex '^test_t7_123$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.123`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.124: bugprone-multi-level-implicit-pointer-conversion in api_handlers_fleet.c
- **depends_on**: [T7, T13, T7.63]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_fleet.c`, `tests/unit/test_t7_124.c`, `tests/unit/cmake/eng/t7_124.cmake`, `quality/evidence/T7.124.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-0b0bcefaf7a0b24f, CLANGTIDY-d8c6998c9398adf1, CLANGTIDY-79ebe40a23b7323a, CLANGTIDY-6da59a8629d16699, CLANGTIDY-773e04e041de5410, CLANGTIDY-f12d6e4a615966e9, CLANGTIDY-ede4da279d79ec69, CLANGTIDY-36c8164a04c2420c, CLANGTIDY-4053884d5188ab2f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.124 --regex '^test_t7_124$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.124`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.125: bugprone-multi-level-implicit-pointer-conversion in api_handlers_retention.c
- **depends_on**: [T7, T13, T7.47]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_retention.c`, `tests/unit/test_t7_125.c`, `tests/unit/cmake/eng/t7_125.cmake`, `quality/evidence/T7.125.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-0cc9a9ea64babfdc, CLANGTIDY-234cb88ea5651731
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.125 --regex '^test_t7_125$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.125`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.126: bugprone-multi-level-implicit-pointer-conversion in api_handlers_storage_policies.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_storage_policies.c`, `tests/unit/test_t7_126.c`, `tests/unit/cmake/eng/t7_126.cmake`, `quality/evidence/T7.126.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-312e6eafc5ebdca6, CLANGTIDY-1985156ff8e5c87d, CLANGTIDY-ca7e963b9f3e3214, CLANGTIDY-aede9d1024b66426, CLANGTIDY-07c63776b48ad48f, CLANGTIDY-ea16598ca26b9108
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.126 --regex '^test_t7_126$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.126`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.127: bugprone-narrowing-conversions in api_timing.h
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `include/web/api_timing.h`, `tests/unit/test_t7_127.c`, `tests/unit/cmake/eng/t7_127.cmake`, `quality/evidence/T7.127.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ca2f4806dea5f177, CLANGTIDY-054f4f2a2583f814
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.127 --regex '^test_t7_127$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.127`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.128: bugprone-narrowing-conversions in path_utils.c
- **depends_on**: [T7, T13, T7.52]
- **agent_type**: worker
- **ownership**: `src/core/path_utils.c`, `tests/unit/test_t7_128.c`, `tests/unit/cmake/eng/t7_128.cmake`, `quality/evidence/T7.128.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-dda225859c807d72
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.128 --regex '^test_t7_128$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.128`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.129: bugprone-narrowing-conversions in db_storage_lifecycle.c
- **depends_on**: [T7, T13, T7.117]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_lifecycle.c`, `tests/unit/test_t7_129.c`, `tests/unit/cmake/eng/t7_129.cmake`, `quality/evidence/T7.129.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-76e54a4e3ac5efb3, CLANGTIDY-84fbbd387f95947a
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.129 --regex '^test_t7_129$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.129`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.130: bugprone-narrowing-conversions in mp4_writer_utils.c
- **depends_on**: [T7, T13, T7.121]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_130.c`, `tests/unit/cmake/eng/t7_130.cmake`, `quality/evidence/T7.130.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c74d7d04edeb4ee2, CLANGTIDY-cb47dbd2bd0937a1, CLANGTIDY-6f7bee0a60d5152f
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.130 --regex '^test_t7_130$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.130`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.131: bugprone-narrowing-conversions in stream_frame_rate.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/stream_frame_rate.c`, `tests/unit/test_t7_131.c`, `tests/unit/cmake/eng/t7_131.cmake`, `quality/evidence/T7.131.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-e1b115ce18cb0926, CLANGTIDY-9bb912a1e3d2bf88
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.131 --regex '^test_t7_131$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.131`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.132: bugprone-narrowing-conversions in api_handlers_storage_compliance.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_storage_compliance.c`, `tests/unit/test_t7_132.c`, `tests/unit/cmake/eng/t7_132.cmake`, `quality/evidence/T7.132.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6a756d858b2a4590, CLANGTIDY-0f6724ca27b42ed9, CLANGTIDY-ec3e72154d735330, CLANGTIDY-e85c88c08460d2ef, CLANGTIDY-9c90ce2f531301e0, CLANGTIDY-3e6ff569aac7699e, CLANGTIDY-ca4b8a2f2a2f60bf, CLANGTIDY-c990523bb862f2b6, CLANGTIDY-7a5a584e25342a04, CLANGTIDY-8d9dd908cb4d8452
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.132 --regex '^test_t7_132$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.132`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.133: bugprone-narrowing-conversions in httpd_utils.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/httpd_utils.c`, `tests/unit/test_t7_133.c`, `tests/unit/cmake/eng/t7_133.cmake`, `quality/evidence/T7.133.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-cb7511d8b0a3b651
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.133 --regex '^test_t7_133$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.133`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.134: bugprone-not-null-terminated-result in mqtt_destination_client.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_destination_client.c`, `tests/unit/test_t7_134.c`, `tests/unit/cmake/eng/t7_134.cmake`, `quality/evidence/T7.134.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d7edea27d80824ed
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.134 --regex '^test_t7_134$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.134`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.135: bugprone-not-null-terminated-result in linux_hardware.c
- **depends_on**: [T7, T13, T7.58]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/linux_hardware.c`, `tests/unit/test_t7_135.c`, `tests/unit/cmake/eng/t7_135.cmake`, `quality/evidence/T7.135.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-65ab86fae5fd2213
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.135 --regex '^test_t7_135$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.135`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.136: bugprone-suspicious-string-compare in db_storage_targets.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_targets.c`, `tests/unit/test_t7_136.c`, `tests/unit/cmake/eng/t7_136.cmake`, `quality/evidence/T7.136.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-709dbb22240f409f, CLANGTIDY-d68060f1555ca324, CLANGTIDY-b922ced0e34aacea, CLANGTIDY-295d32efbfee9145
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.136 --regex '^test_t7_136$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.136`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.137: bugprone-suspicious-string-compare in storage_deletion.c
- **depends_on**: [T7, T13, T28.1, T29.3]
- **agent_type**: worker
- **ownership**: `src/storage/storage_deletion.c`, `tests/unit/test_t7_137.c`, `tests/unit/cmake/eng/t7_137.cmake`, `quality/evidence/T7.137.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-56955e0bf792f087
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.137 --regex '^test_t7_137$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.137`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.138: bugprone-suspicious-string-compare in storage_migration.c
- **depends_on**: [T7, T13, T7.93]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_138.c`, `tests/unit/cmake/eng/t7_138.cmake`, `quality/evidence/T7.138.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d6379e87cfa699fa
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.138 --regex '^test_t7_138$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.138`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.139: bugprone-suspicious-string-compare in storage_s3.c
- **depends_on**: [T7, T13, T7.94]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_139.c`, `tests/unit/cmake/eng/t7_139.cmake`, `quality/evidence/T7.139.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-789d9f9254afa1b1, CLANGTIDY-2f98dec54b741594, CLANGTIDY-852f97f8aacc5432, CLANGTIDY-ba57287cdb3eb096, CLANGTIDY-bfbbf95e2bedfe38, CLANGTIDY-65cc9415acf1be4d, CLANGTIDY-5561b78de6be772b, CLANGTIDY-6b719694dd2640c7
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.139 --regex '^test_t7_139$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.139`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.140: bugprone-suspicious-string-compare in storage_source.c
- **depends_on**: [T7, T13, T7.110]
- **agent_type**: worker
- **ownership**: `src/storage/storage_source.c`, `tests/unit/test_t7_140.c`, `tests/unit/cmake/eng/t7_140.cmake`, `quality/evidence/T7.140.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6a07b625f331b20e
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.140 --regex '^test_t7_140$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.140`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.141: bugprone-suspicious-string-compare in recording_archive.c
- **depends_on**: [T7, T13, T7.109]
- **agent_type**: worker
- **ownership**: `src/web/recording_archive.c`, `tests/unit/test_t7_141.c`, `tests/unit/cmake/eng/t7_141.cmake`, `quality/evidence/T7.141.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-adf6edf7670e9010
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.141 --regex '^test_t7_141$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.141`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.142: bugprone-unsafe-functions,cert-msc24-c,cert-msc33-c in storage_s3.c
- **depends_on**: [T7, T13, T7.139]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_142.c`, `tests/unit/cmake/eng/t7_142.cmake`, `quality/evidence/T7.142.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-3a081de1dd30ebb2
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.142 --regex '^test_t7_142$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.142`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.143: cert-err34-c in linux_device_map.c, linux_hardware.c
- **depends_on**: [T7, T13, T7.135]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/linux_device_map.c`, `src/telemetry/providers/linux_hardware.c`, `tests/unit/test_t7_143.c`, `tests/unit/cmake/eng/t7_143.cmake`, `quality/evidence/T7.143.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c52ffccc8c0fb590, CLANGTIDY-811ab08155776051
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.143 --regex '^test_t7_143$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.143`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.144: cert-err34-c in ffmpeg_rss_guard.c
- **depends_on**: [T7, T13, T7.96]
- **agent_type**: worker
- **ownership**: `src/tools/ffmpeg_rss_guard.c`, `tests/unit/test_t7_144.c`, `tests/unit/cmake/eng/t7_144.cmake`, `quality/evidence/T7.144.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-11d1a497d98b8490
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.144 --regex '^test_t7_144$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.144`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.145: cert-err34-c in go2rtc_integration.c, go2rtc_process.c
- **depends_on**: [T7, T13, T7.98, T7.60]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_process.c`, `tests/unit/test_t7_145.c`, `tests/unit/cmake/eng/t7_145.cmake`, `quality/evidence/T7.145.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a7a71d348e2c49f5, CLANGTIDY-41876b44c3442c4b, CLANGTIDY-3a3cc8c9282e0fb5, CLANGTIDY-43d0d5170d6a8434
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.145 --regex '^test_t7_145$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.145`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.146: cert-err34-c in onvif_detection.c
- **depends_on**: [T7, T13, T7.100]
- **agent_type**: worker
- **ownership**: `src/video/onvif_detection.c`, `tests/unit/test_t7_146.c`, `tests/unit/cmake/eng/t7_146.cmake`, `quality/evidence/T7.146.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a6e1105b549e0cf0
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.146 --regex '^test_t7_146$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.146`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.147: cert-err34-c in api_handlers_go2rtc_proxy.c
- **depends_on**: [T7, T13, T7.67]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_go2rtc_proxy.c`, `tests/unit/test_t7_147.c`, `tests/unit/cmake/eng/t7_147.cmake`, `quality/evidence/T7.147.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ab5e571ba64af645
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.147 --regex '^test_t7_147$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.147`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.148: cert-err34-c in api_handlers_investigations.c
- **depends_on**: [T7, T13, T7.102]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_investigations.c`, `tests/unit/test_t7_148.c`, `tests/unit/cmake/eng/t7_148.cmake`, `quality/evidence/T7.148.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-e5ec24184e5b758c
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.148 --regex '^test_t7_148$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.148`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.149: cert-err34-c in api_handlers_system_logs.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system_logs.c`, `tests/unit/test_t7_149.c`, `tests/unit/cmake/eng/t7_149.cmake`, `quality/evidence/T7.149.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-0fda904c59353043
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.149 --regex '^test_t7_149$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.149`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.150: cert-err34-c in go2rtc_websocket.c
- **depends_on**: [T7, T13, T7.108]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_websocket.c`, `tests/unit/test_t7_150.c`, `tests/unit/cmake/eng/t7_150.cmake`, `quality/evidence/T7.150.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-5c42b38e8a9c25ff
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.150 --regex '^test_t7_150$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.150`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.151: cert-int09-c in authorization.h
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `include/core/authorization.h`, `tests/unit/test_t7_151.c`, `tests/unit/cmake/eng/t7_151.cmake`, `quality/evidence/T7.151.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-1539a5c009afd0df
- **acceptance_criteria**:
  - Verify enum constants remain representable and unique on the supported ABI without changing public numeric values.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.151 --regex '^test_t7_151$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.151`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.152: cert-int09-c in faac.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_152-upstream.json`, `quality/evidence/T7.152.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CLANGTIDY-b6947bb24c18c8ac, CLANGTIDY-ddf685d0be472b86, CLANGTIDY-296d5f7499179bc0, CLANGTIDY-4c6957160cb5ae0c
- **acceptance_criteria**:
  - Verify enum constants remain representable and unique on the supported ABI without changing public numeric values.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_152-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.152`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.153: clang-analyzer-deadcode.DeadStores in db_system_health_incidents.c
- **depends_on**: [T7, T13, T7.79]
- **agent_type**: worker
- **ownership**: `src/database/db_system_health_incidents.c`, `tests/unit/test_t7_153.c`, `tests/unit/cmake/eng/t7_153.cmake`, `quality/evidence/T7.153.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-fe8af642c27ce051
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.153 --regex '^test_t7_153$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.153`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.154: clang-analyzer-deadcode.DeadStores in go2rtc_integration.c
- **depends_on**: [T7, T13, T7.145]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_integration.c`, `tests/unit/test_t7_154.c`, `tests/unit/cmake/eng/t7_154.cmake`, `quality/evidence/T7.154.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-88fbdf8930f67549
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.154 --regex '^test_t7_154$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.154`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.155: clang-analyzer-deadcode.DeadStores in mp4_segment_recorder.c
- **depends_on**: [T7, T13, T7.119, T28.2]
- **agent_type**: worker
- **ownership**: `src/video/mp4_segment_recorder.c`, `tests/unit/test_t7_155.c`, `tests/unit/cmake/eng/t7_155.cmake`, `quality/evidence/T7.155.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-27ac318897664d8a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.155 --regex '^test_t7_155$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.155`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.156: clang-analyzer-deadcode.DeadStores in onvif_ptz.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_ptz.c`, `tests/unit/test_t7_156.c`, `tests/unit/cmake/eng/t7_156.cmake`, `quality/evidence/T7.156.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-7d7d3a3c4f1824bd
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.156 --regex '^test_t7_156$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.156`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.157: clang-analyzer-deadcode.DeadStores in recording_path.c
- **depends_on**: [T7, T13, T27.1]
- **agent_type**: worker
- **ownership**: `src/video/recording_path.c`, `tests/unit/test_t7_157.c`, `tests/unit/cmake/eng/t7_157.cmake`, `quality/evidence/T7.157.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-44c350a8b7e2fd4a, CLANGTIDY-a7fa9aad275f9425
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.157 --regex '^test_t7_157$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.157`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.158: clang-analyzer-deadcode.DeadStores in api_handlers_health.c
- **depends_on**: [T7, T13, T7.44]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_health.c`, `tests/unit/test_t7_158.c`, `tests/unit/cmake/eng/t7_158.cmake`, `quality/evidence/T7.158.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-cf5f09b5a02a9bca
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.158 --regex '^test_t7_158$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.158`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.159: invalidPointerCast in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_159-upstream.json`, `quality/evidence/T7.159.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-7975c2d572bc32c0, CPPCHECK-291bebc3b95eca4e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_159-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.159`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.160: jsx-a11y/anchor-is-valid in Header.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/Header.jsx`, `web/tests/remediation/t7_160.spec.jsx`, `quality/evidence/T7.160.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-366138c5d194685c
- **acceptance_criteria**:
  - Use valid navigable hrefs for links and buttons for actions; verify keyboard behavior and no unexpected page navigation.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_160`
  - `mkdir -p build/eng-artifacts/T7.160`
  - `web/node_modules/.bin/eslint web/js/components/preact/Header.jsx --format json --output-file build/eng-artifacts/T7.160/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.160/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/anchor-is-valid'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.160`; inspect the raw T1 report and trace the caller.
- **status**: Completed (commit `233859ab`)
- **log**: Header profile action is a native disabled-capable button; links retain hrefs. Eight explicit tests including actual Chromium keyboard controls pass (seven default Node cases). Exactly one mapped lint warning removed; types, both builds and43/66 budgets pass.
- **files edited/created**:

### T7.161: jsx-a11y/click-events-have-key-events in GridPicker.jsx, HLSVideoCell.jsx, HealthView.jsx, MSEVideoCell.jsx, StreamCard.jsx, StreamsView.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/GridPicker.jsx`, `web/js/components/preact/HLSVideoCell.jsx`, `web/js/components/preact/HealthView.jsx`, `web/js/components/preact/MSEVideoCell.jsx`, `web/js/components/preact/StreamCard.jsx`, `web/js/components/preact/StreamsView.jsx`, `web/tests/remediation/t7_161.spec.jsx`, `quality/evidence/T7.161.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-ed278d9594c2abdb, ESLINT-BUCKET-67fcff31d1a4cf75, ESLINT-BUCKET-a642f51dfd2a124a, ESLINT-BUCKET-404de092ea97cc43, ESLINT-BUCKET-6243e36ec0fdb5cd, ESLINT-BUCKET-5c63d53fbef3ba6d
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_161`
  - `mkdir -p build/eng-artifacts/T7.161`
  - `web/node_modules/.bin/eslint web/js/components/preact/GridPicker.jsx web/js/components/preact/HLSVideoCell.jsx web/js/components/preact/HealthView.jsx web/js/components/preact/MSEVideoCell.jsx web/js/components/preact/StreamCard.jsx web/js/components/preact/StreamsView.jsx --format json --output-file build/eng-artifacts/T7.161/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.161/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.161`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.162: jsx-a11y/click-events-have-key-events in ToastContainer.jsx, UI.jsx, WebRTCVideoCell.jsx, ZoneEditor.jsx, LanguageSelector.jsx, ModalDialog.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/ToastContainer.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/js/components/preact/common/LanguageSelector.jsx`, `web/js/components/preact/common/ModalDialog.jsx`, `web/tests/remediation/t7_162.spec.jsx`, `quality/evidence/T7.162.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-ab2c863c82952346, ESLINT-BUCKET-f53a71c368c33ab9, ESLINT-BUCKET-45887456841ead8b, ESLINT-BUCKET-cb235968867c79e3, ESLINT-BUCKET-02418efc105b6ab9, ESLINT-BUCKET-4e8530c09d5fd082
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_162`
  - `mkdir -p build/eng-artifacts/T7.162`
  - `web/node_modules/.bin/eslint web/js/components/preact/ToastContainer.jsx web/js/components/preact/UI.jsx web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/ZoneEditor.jsx web/js/components/preact/common/LanguageSelector.jsx web/js/components/preact/common/ModalDialog.jsx --format json --output-file build/eng-artifacts/T7.162/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.162/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.162`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.163: jsx-a11y/click-events-have-key-events in LiveOperatorNavigator.jsx, RecordingsGrid.jsx, Go2rtcTab.jsx, ClearLogsModal.jsx, RestartModal.jsx, TimelinePlayer.jsx
- **depends_on**: [T7, T10, T10.2, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/js/components/preact/system/ClearLogsModal.jsx`, `web/js/components/preact/system/RestartModal.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/tests/remediation/t7_163.spec.jsx`, `quality/evidence/T7.163.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-f16c5e93d86bc6f9, ESLINT-BUCKET-41381cd9bd64279e, ESLINT-BUCKET-422485a76dc61a9b, ESLINT-BUCKET-44ea0e63c82a7151, ESLINT-BUCKET-155480cbfd270011, ESLINT-BUCKET-79b6e7adf4959fbc
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_163`
  - `mkdir -p build/eng-artifacts/T7.163`
  - `web/node_modules/.bin/eslint web/js/components/preact/live/LiveOperatorNavigator.jsx web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/settings/Go2rtcTab.jsx web/js/components/preact/system/ClearLogsModal.jsx web/js/components/preact/system/RestartModal.jsx web/js/components/preact/timeline/TimelinePlayer.jsx --format json --output-file build/eng-artifacts/T7.163/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.163/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.163`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.164: jsx-a11y/click-events-have-key-events in AccessPolicyModal.jsx, AddUserModal.jsx, ApiKeyModal.jsx, AuditHistoryModal.jsx, DeleteUserModal.jsx, EditUserModal.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/AccessPolicyModal.jsx`, `web/js/components/preact/users/AddUserModal.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/DeleteUserModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/tests/remediation/t7_164.spec.jsx`, `quality/evidence/T7.164.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-0a786d21e1ee7bd2, ESLINT-BUCKET-c2dc58af6543d1f5, ESLINT-BUCKET-3b71a4c47dfd1075, ESLINT-BUCKET-ea11ae604e62541e, ESLINT-BUCKET-8d573ae471211e73, ESLINT-BUCKET-22666b8c08cd887e
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_164`
  - `mkdir -p build/eng-artifacts/T7.164`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/AccessPolicyModal.jsx web/js/components/preact/users/AddUserModal.jsx web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/DeleteUserModal.jsx web/js/components/preact/users/EditUserModal.jsx --format json --output-file build/eng-artifacts/T7.164/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.164/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.164`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.165: jsx-a11y/click-events-have-key-events in RoleManagerModal.jsx, TotpSetupModal.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/RoleManagerModal.jsx`, `web/js/components/preact/users/TotpSetupModal.jsx`, `web/tests/remediation/t7_165.spec.jsx`, `quality/evidence/T7.165.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-9190884804c0c65d, ESLINT-BUCKET-e8ce699e42be07b3
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_165`
  - `mkdir -p build/eng-artifacts/T7.165`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/RoleManagerModal.jsx web/js/components/preact/users/TotpSetupModal.jsx --format json --output-file build/eng-artifacts/T7.165/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.165/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.165`; inspect the raw T1 report and trace the caller.
- **status**: Completed (commit `fe93b78c`)
- **log**: Four assigned dialog boundary warnings resolved; native keyboard controls and existing dialog behavior preserved. Eight default Node cases and nine explicit Chromium-enabled cases pass. Four directly related interaction warnings also disappear; types, both builds and43/66budgets pass. Lint now887warnings/0errors; broader TOTP modal semantics remain outside scope.
- **files edited/created**:

### T7.166: jsx-a11y/control-has-associated-label in AuthGate.jsx, BatchDownloadModal.jsx, DetectionOverlay.jsx, FleetView.jsx, FullscreenManager.jsx, HLSVideoCell.jsx
- **depends_on**: [T7, T10, T7.161]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/AuthGate.jsx`, `web/js/components/preact/BatchDownloadModal.jsx`, `web/js/components/preact/DetectionOverlay.jsx`, `web/js/components/preact/FleetView.jsx`, `web/js/components/preact/FullscreenManager.jsx`, `web/js/components/preact/HLSVideoCell.jsx`, `web/tests/remediation/t7_166.spec.jsx`, `quality/evidence/T7.166.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-0f4bbeb4ab715fee, ESLINT-BUCKET-ef00a494fd69875c, ESLINT-BUCKET-e93ca9cb727a0aee, ESLINT-BUCKET-20e1a03b32e40002, ESLINT-BUCKET-e7e0cc4f8fd4c513, ESLINT-BUCKET-aa839344c6e5b30d
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_166`
  - `mkdir -p build/eng-artifacts/T7.166`
  - `web/node_modules/.bin/eslint web/js/components/preact/AuthGate.jsx web/js/components/preact/BatchDownloadModal.jsx web/js/components/preact/DetectionOverlay.jsx web/js/components/preact/FleetView.jsx web/js/components/preact/FullscreenManager.jsx web/js/components/preact/HLSVideoCell.jsx --format json --output-file build/eng-artifacts/T7.166/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.166/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.166`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.167: jsx-a11y/control-has-associated-label in LiveView.jsx, LoginView.jsx, MSEVideoCell.jsx, SetupWizard.jsx, SnapshotManager.jsx, StreamConfigModal.jsx
- **depends_on**: [T7, T10, T7.161, T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/LiveView.jsx`, `web/js/components/preact/LoginView.jsx`, `web/js/components/preact/MSEVideoCell.jsx`, `web/js/components/preact/SetupWizard.jsx`, `web/js/components/preact/SnapshotManager.jsx`, `web/js/components/preact/StreamConfigModal.jsx`, `web/tests/remediation/t7_167.spec.jsx`, `quality/evidence/T7.167.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-ae74d74f4f8f96b4, ESLINT-BUCKET-33c31857a85153c5, ESLINT-BUCKET-2b401ef7608cff74, ESLINT-BUCKET-6b623e0db3f455c7, ESLINT-BUCKET-63e9c68b97a2b704, ESLINT-BUCKET-3d882f111ea61422
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_167`
  - `mkdir -p build/eng-artifacts/T7.167`
  - `web/node_modules/.bin/eslint web/js/components/preact/LiveView.jsx web/js/components/preact/LoginView.jsx web/js/components/preact/MSEVideoCell.jsx web/js/components/preact/SetupWizard.jsx web/js/components/preact/SnapshotManager.jsx web/js/components/preact/StreamConfigModal.jsx --format json --output-file build/eng-artifacts/T7.167/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.167/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.167`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.168: jsx-a11y/control-has-associated-label in StreamsView.jsx, ThemeCustomizer.jsx, UI.jsx, WebRTCVideoCell.jsx, ZoneEditor.jsx, ModalDialog.jsx
- **depends_on**: [T7, T10, T7.161, T7.162]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamsView.jsx`, `web/js/components/preact/ThemeCustomizer.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/js/components/preact/common/ModalDialog.jsx`, `web/tests/remediation/t7_168.spec.jsx`, `quality/evidence/T7.168.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-ba71da8483425ebf, ESLINT-BUCKET-6ffa955639ab6418, ESLINT-BUCKET-6faae6bddf434aab, ESLINT-BUCKET-9bd9a8994dceced4, ESLINT-BUCKET-e845a2e465702318, ESLINT-BUCKET-1f35f119bab13f92
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_168`
  - `mkdir -p build/eng-artifacts/T7.168`
  - `web/node_modules/.bin/eslint web/js/components/preact/StreamsView.jsx web/js/components/preact/ThemeCustomizer.jsx web/js/components/preact/UI.jsx web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/ZoneEditor.jsx web/js/components/preact/common/ModalDialog.jsx --format json --output-file build/eng-artifacts/T7.168/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.168/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.168`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.169: jsx-a11y/control-has-associated-label in EventDestinationEditor.jsx, EventRouteEditor.jsx, BulkOrganizeModal.jsx, CollectionManager.jsx, CollectionSelectorBuilder.jsx, FleetFilters.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/events/EventDestinationEditor.jsx`, `web/js/components/preact/events/EventRouteEditor.jsx`, `web/js/components/preact/fleet/BulkOrganizeModal.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/CollectionSelectorBuilder.jsx`, `web/js/components/preact/fleet/FleetFilters.jsx`, `web/tests/remediation/t7_169.spec.jsx`, `quality/evidence/T7.169.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-5088d976b993e707, ESLINT-BUCKET-fea403cece7fffa3, ESLINT-BUCKET-20d0442e85244a5d, ESLINT-BUCKET-03b8b95f3bd44235, ESLINT-BUCKET-993ef9a219cc29a2, ESLINT-BUCKET-eaa9041228587f50
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_169`
  - `mkdir -p build/eng-artifacts/T7.169`
  - `web/node_modules/.bin/eslint web/js/components/preact/events/EventDestinationEditor.jsx web/js/components/preact/events/EventRouteEditor.jsx web/js/components/preact/fleet/BulkOrganizeModal.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/CollectionSelectorBuilder.jsx web/js/components/preact/fleet/FleetFilters.jsx --format json --output-file build/eng-artifacts/T7.169/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.169/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.169`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.170: jsx-a11y/control-has-associated-label in OrganizationManager.jsx, SavedViewsBar.jsx, StaticCollectionMembers.jsx, InvestigationActions.jsx, InvestigationBookmarks.jsx, InvestigationView.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/fleet/OrganizationManager.jsx`, `web/js/components/preact/fleet/SavedViewsBar.jsx`, `web/js/components/preact/fleet/StaticCollectionMembers.jsx`, `web/js/components/preact/investigation/InvestigationActions.jsx`, `web/js/components/preact/investigation/InvestigationBookmarks.jsx`, `web/js/components/preact/investigation/InvestigationView.jsx`, `web/tests/remediation/t7_170.spec.jsx`, `quality/evidence/T7.170.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-19d5fc636e3f15e6, ESLINT-BUCKET-2cdd7ef73529551e, ESLINT-BUCKET-5a25b9ace393f230, ESLINT-BUCKET-12c7bb31966e53aa, ESLINT-BUCKET-274240d1fc0300f7, ESLINT-BUCKET-2a20bf23c61a102e
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_170`
  - `mkdir -p build/eng-artifacts/T7.170`
  - `web/node_modules/.bin/eslint web/js/components/preact/fleet/OrganizationManager.jsx web/js/components/preact/fleet/SavedViewsBar.jsx web/js/components/preact/fleet/StaticCollectionMembers.jsx web/js/components/preact/investigation/InvestigationActions.jsx web/js/components/preact/investigation/InvestigationBookmarks.jsx web/js/components/preact/investigation/InvestigationView.jsx --format json --output-file build/eng-artifacts/T7.170/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.170/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.170`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.171: jsx-a11y/control-has-associated-label in LiveBuildingPlan.jsx, LiveOperatorNavigator.jsx, PlanSketchControls.jsx, FiltersSidebar.jsx, RecordingsGrid.jsx, RecordingsTable.jsx
- **depends_on**: [T7, T10, T7.163, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/js/components/preact/live/PlanSketchControls.jsx`, `web/js/components/preact/recordings/FiltersSidebar.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/recordings/RecordingsTable.jsx`, `web/tests/remediation/t7_171.spec.jsx`, `quality/evidence/T7.171.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-bb1a968a245405ec, ESLINT-BUCKET-3a6fb0a94ef6ffdf, ESLINT-BUCKET-e0e4b387af698b40, ESLINT-BUCKET-389226ea0147ea5d, ESLINT-BUCKET-aab67d5bbb7b5489, ESLINT-BUCKET-f492731b13ec277c
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_171`
  - `mkdir -p build/eng-artifacts/T7.171`
  - `web/node_modules/.bin/eslint web/js/components/preact/live/LiveBuildingPlan.jsx web/js/components/preact/live/LiveOperatorNavigator.jsx web/js/components/preact/live/PlanSketchControls.jsx web/js/components/preact/recordings/FiltersSidebar.jsx web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/recordings/RecordingsTable.jsx --format json --output-file build/eng-artifacts/T7.171/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.171/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.171`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.172: jsx-a11y/control-has-associated-label in TagsOverlay.jsx, AdvancedTab.jsx, AuthTab.jsx, DetectionTab.jsx, GeneralTab.jsx, Go2rtcTab.jsx
- **depends_on**: [T7, T10, T7.163, T10.2, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/recordings/TagsOverlay.jsx`, `web/js/components/preact/settings/AdvancedTab.jsx`, `web/js/components/preact/settings/AuthTab.jsx`, `web/js/components/preact/settings/DetectionTab.jsx`, `web/js/components/preact/settings/GeneralTab.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/tests/remediation/t7_172.spec.jsx`, `quality/evidence/T7.172.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-d03afa69ead3203e, ESLINT-BUCKET-8ce22b0413a485e1, ESLINT-BUCKET-40ef7b2f328bb9ef, ESLINT-BUCKET-93d2b8db97a42555, ESLINT-BUCKET-ea86635ccd79513e, ESLINT-BUCKET-5ec0649750441f81
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_172`
  - `mkdir -p build/eng-artifacts/T7.172`
  - `web/node_modules/.bin/eslint web/js/components/preact/recordings/TagsOverlay.jsx web/js/components/preact/settings/AdvancedTab.jsx web/js/components/preact/settings/AuthTab.jsx web/js/components/preact/settings/DetectionTab.jsx web/js/components/preact/settings/GeneralTab.jsx web/js/components/preact/settings/Go2rtcTab.jsx --format json --output-file build/eng-artifacts/T7.172/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.172/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.172`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.173: jsx-a11y/control-has-associated-label in HealthTab.jsx, MqttTab.jsx, StorageTab.jsx, StreamsDefaultsTab.jsx, StreamStorage.jsx, CalendarPicker.jsx
- **depends_on**: [T7, T10, T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/settings/HealthTab.jsx`, `web/js/components/preact/settings/MqttTab.jsx`, `web/js/components/preact/settings/StorageTab.jsx`, `web/js/components/preact/settings/StreamsDefaultsTab.jsx`, `web/js/components/preact/system/StreamStorage.jsx`, `web/js/components/preact/timeline/CalendarPicker.jsx`, `web/tests/remediation/t7_173.spec.jsx`, `quality/evidence/T7.173.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-fe8c1e237393fb23, ESLINT-BUCKET-59e343114dc251b1, ESLINT-BUCKET-5a1cbbfddf8f3fcc, ESLINT-BUCKET-350da16b1c3b2274, ESLINT-BUCKET-44b516dc2eeb97bf, ESLINT-BUCKET-5da42ce096a9fa82
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_173`
  - `mkdir -p build/eng-artifacts/T7.173`
  - `web/node_modules/.bin/eslint web/js/components/preact/settings/HealthTab.jsx web/js/components/preact/settings/MqttTab.jsx web/js/components/preact/settings/StorageTab.jsx web/js/components/preact/settings/StreamsDefaultsTab.jsx web/js/components/preact/system/StreamStorage.jsx web/js/components/preact/timeline/CalendarPicker.jsx --format json --output-file build/eng-artifacts/T7.173/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.173/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.173`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.174: jsx-a11y/control-has-associated-label in TimelineControls.jsx, TimelinePlayer.jsx, AddUserModal.jsx, ApiKeyModal.jsx, AuditHistoryModal.jsx, EditUserModal.jsx
- **depends_on**: [T7, T10, T7.163, T7.164]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/timeline/TimelineControls.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/js/components/preact/users/AddUserModal.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/tests/remediation/t7_174.spec.jsx`, `quality/evidence/T7.174.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-7b00d558899dd292, ESLINT-BUCKET-4e38c49052b49016, ESLINT-BUCKET-6af902622576c683, ESLINT-BUCKET-098293b0fb5320bb, ESLINT-BUCKET-f25f197e02c782fc, ESLINT-BUCKET-ccf3e6e7959d2bd7
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_174`
  - `mkdir -p build/eng-artifacts/T7.174`
  - `web/node_modules/.bin/eslint web/js/components/preact/timeline/TimelineControls.jsx web/js/components/preact/timeline/TimelinePlayer.jsx web/js/components/preact/users/AddUserModal.jsx web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/EditUserModal.jsx --format json --output-file build/eng-artifacts/T7.174/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.174/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.174`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.175: jsx-a11y/control-has-associated-label in RoleManagerModal.jsx, TotpSetupModal.jsx, UsersTable.jsx
- **depends_on**: [T7, T10, T7.165]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/RoleManagerModal.jsx`, `web/js/components/preact/users/TotpSetupModal.jsx`, `web/js/components/preact/users/UsersTable.jsx`, `web/tests/remediation/t7_175.spec.jsx`, `web/tests/remediation/t7_165.spec.jsx` (useId hook mock compatibility only; preserve assertions), `quality/evidence/T7.175.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-400e44e71536be4e, ESLINT-BUCKET-d3aede71a08c3a5f, ESLINT-BUCKET-fcf1de019f6b4533
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_175`
  - `mkdir -p build/eng-artifacts/T7.175`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/RoleManagerModal.jsx web/js/components/preact/users/TotpSetupModal.jsx web/js/components/preact/users/UsersTable.jsx --format json --output-file build/eng-artifacts/T7.175/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.175/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.175`; inspect the raw T1 report and trace the caller.
- **status**: Completed (commit `359ade6d`)
- **log**: All11 mapped control-name warnings resolved using existing localized visible text/button titles;14/14 combined browser-enabled tests preserve modal boundaries and callbacks. Full lint887→873 with0errors/no additions (3 related RoleManager label-for warnings also clear); types and both43/66asset budgets pass, default restored.
- **files edited/created**:

### T7.176: jsx-a11y/interactive-supports-focus in MobileTileGestures.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/MobileTileGestures.jsx`, `web/tests/remediation/t7_176.spec.jsx`, `quality/evidence/T7.176.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-3469a0a703cdbc12
- **acceptance_criteria**:
  - Assert each interactive control is keyboard focusable and acts once for its expected activation keys.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_176`
  - `mkdir -p build/eng-artifacts/T7.176`
  - `web/node_modules/.bin/eslint web/js/components/preact/MobileTileGestures.jsx --format json --output-file build/eng-artifacts/T7.176/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.176/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/interactive-supports-focus'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.176`; inspect the raw T1 report and trace the caller.
- **status**: Completed (commit `29b55fd6`)
- **log**: Composite menu receives tabIndex=-1 while keeping first-native-child focus/activation. Seven explicit tests including Chromium focus/Escape/Tab/Enter/Space pass (six default Node cases). Exactly one mapped warning removed; lint now895warnings/0errors, types/builds/budgets pass. No full APG conformance claim.
- **files edited/created**:

### T7.177: jsx-a11y/label-has-associated-control in BatchDownloadModal.jsx, StreamConfigModal.jsx, EventDestinationEditor.jsx, EventRouteEditor.jsx, CollectionManager.jsx, StaticCollectionMembers.jsx
- **depends_on**: [T7, T10, T7.166, T7.167, T7.169, T7.170, T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/BatchDownloadModal.jsx`, `web/js/components/preact/StreamConfigModal.jsx`, `web/js/components/preact/events/EventDestinationEditor.jsx`, `web/js/components/preact/events/EventRouteEditor.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/StaticCollectionMembers.jsx`, `web/tests/remediation/t7_177.spec.jsx`, `quality/evidence/T7.177.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-199d3d91208953b4, ESLINT-BUCKET-81a756fe7228ad5b, ESLINT-BUCKET-ddc6803a9b16e391, ESLINT-BUCKET-7cde28afaa9ffb65, ESLINT-BUCKET-7dcecd444f491721, ESLINT-BUCKET-9308fe6583bcdce7
- **acceptance_criteria**:
  - Assert label/input association is programmatic, unique in repeated forms, localized and preserved during conditional rendering.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_177`
  - `mkdir -p build/eng-artifacts/T7.177`
  - `web/node_modules/.bin/eslint web/js/components/preact/BatchDownloadModal.jsx web/js/components/preact/StreamConfigModal.jsx web/js/components/preact/events/EventDestinationEditor.jsx web/js/components/preact/events/EventRouteEditor.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/StaticCollectionMembers.jsx --format json --output-file build/eng-artifacts/T7.177/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.177/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-associated-control'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.177`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.178: jsx-a11y/label-has-associated-control in HealthTab.jsx, StorageTab.jsx, ApiKeyModal.jsx
- **depends_on**: [T7, T10, T7.173, T7.174, T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/settings/HealthTab.jsx`, `web/js/components/preact/settings/StorageTab.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/tests/remediation/t7_178.spec.jsx`, `quality/evidence/T7.178.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-5e7cfcb56208eaa9, ESLINT-BUCKET-e84d458d8fcd3eef, ESLINT-BUCKET-cf4b93673820960b
- **acceptance_criteria**:
  - Assert label/input association is programmatic, unique in repeated forms, localized and preserved during conditional rendering.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_178`
  - `mkdir -p build/eng-artifacts/T7.178`
  - `web/node_modules/.bin/eslint web/js/components/preact/settings/HealthTab.jsx web/js/components/preact/settings/StorageTab.jsx web/js/components/preact/users/ApiKeyModal.jsx --format json --output-file build/eng-artifacts/T7.178/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.178/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-associated-control'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.178`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.179: jsx-a11y/label-has-for in AuthGate.jsx, BatchDownloadModal.jsx, FleetView.jsx, LiveView.jsx, LoginView.jsx, SetupWizard.jsx
- **depends_on**: [T7, T10, T7.166, T7.177, T7.167]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/AuthGate.jsx`, `web/js/components/preact/BatchDownloadModal.jsx`, `web/js/components/preact/FleetView.jsx`, `web/js/components/preact/LiveView.jsx`, `web/js/components/preact/LoginView.jsx`, `web/js/components/preact/SetupWizard.jsx`, `web/tests/remediation/t7_179.spec.jsx`, `quality/evidence/T7.179.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-00c9eb79387b90d1, ESLINT-BUCKET-c603f4c09210c0bd, ESLINT-BUCKET-07e48ad6ea9ce45e, ESLINT-BUCKET-dfeaa6c4a9abf5fa, ESLINT-BUCKET-98f2a6813a7ad225, ESLINT-BUCKET-e73c0530b2243573
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_179`
  - `mkdir -p build/eng-artifacts/T7.179`
  - `web/node_modules/.bin/eslint web/js/components/preact/AuthGate.jsx web/js/components/preact/BatchDownloadModal.jsx web/js/components/preact/FleetView.jsx web/js/components/preact/LiveView.jsx web/js/components/preact/LoginView.jsx web/js/components/preact/SetupWizard.jsx --format json --output-file build/eng-artifacts/T7.179/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.179/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.179`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.180: jsx-a11y/label-has-for in StreamCard.jsx, StreamConfigModal.jsx, StreamsView.jsx, UI.jsx, ZoneEditor.jsx, ModalDialog.jsx
- **depends_on**: [T7, T10, T7.161, T7.177, T7.168, T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamCard.jsx`, `web/js/components/preact/StreamConfigModal.jsx`, `web/js/components/preact/StreamsView.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/js/components/preact/common/ModalDialog.jsx`, `web/tests/remediation/t7_180.spec.jsx`, `quality/evidence/T7.180.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-c4c40aa44f4e62e4, ESLINT-BUCKET-d900a116e1f06cf4, ESLINT-BUCKET-de63541e57266afb, ESLINT-BUCKET-e810ef87137eb5d8, ESLINT-BUCKET-38ac9714530327f7, ESLINT-BUCKET-b1a74f710eb47a22
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_180`
  - `mkdir -p build/eng-artifacts/T7.180`
  - `web/node_modules/.bin/eslint web/js/components/preact/StreamCard.jsx web/js/components/preact/StreamConfigModal.jsx web/js/components/preact/StreamsView.jsx web/js/components/preact/UI.jsx web/js/components/preact/ZoneEditor.jsx web/js/components/preact/common/ModalDialog.jsx --format json --output-file build/eng-artifacts/T7.180/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.180/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.180`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.181: jsx-a11y/label-has-for in EventDestinationEditor.jsx, EventRouteEditor.jsx, BulkOrganizeModal.jsx, CollectionManager.jsx, CollectionSelectorBuilder.jsx, FleetTable.jsx
- **depends_on**: [T7, T10, T7.177, T7.169]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/events/EventDestinationEditor.jsx`, `web/js/components/preact/events/EventRouteEditor.jsx`, `web/js/components/preact/fleet/BulkOrganizeModal.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/CollectionSelectorBuilder.jsx`, `web/js/components/preact/fleet/FleetTable.jsx`, `web/tests/remediation/t7_181.spec.jsx`, `quality/evidence/T7.181.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-2d6e882afea8fed1, ESLINT-BUCKET-f3bd5129ab03f352, ESLINT-BUCKET-7ba2e66a4eb3914f, ESLINT-BUCKET-05ad31a80cc2b3a2, ESLINT-BUCKET-30a2e484727d4a5a, ESLINT-BUCKET-b32370126e568d01
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_181`
  - `mkdir -p build/eng-artifacts/T7.181`
  - `web/node_modules/.bin/eslint web/js/components/preact/events/EventDestinationEditor.jsx web/js/components/preact/events/EventRouteEditor.jsx web/js/components/preact/fleet/BulkOrganizeModal.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/CollectionSelectorBuilder.jsx web/js/components/preact/fleet/FleetTable.jsx --format json --output-file build/eng-artifacts/T7.181/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.181/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.181`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.182: jsx-a11y/label-has-for in OrganizationManager.jsx, SavedViewsBar.jsx, StaticCollectionMembers.jsx, InvestigationActions.jsx, InvestigationBookmarks.jsx, InvestigationView.jsx
- **depends_on**: [T7, T10, T7.170, T7.177]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/fleet/OrganizationManager.jsx`, `web/js/components/preact/fleet/SavedViewsBar.jsx`, `web/js/components/preact/fleet/StaticCollectionMembers.jsx`, `web/js/components/preact/investigation/InvestigationActions.jsx`, `web/js/components/preact/investigation/InvestigationBookmarks.jsx`, `web/js/components/preact/investigation/InvestigationView.jsx`, `web/tests/remediation/t7_182.spec.jsx`, `quality/evidence/T7.182.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-f77074e66ed65721, ESLINT-BUCKET-e26c8068ece9c0e0, ESLINT-BUCKET-4246d4277a0b8f04, ESLINT-BUCKET-642b5326a1034302, ESLINT-BUCKET-c0802f3ddbb20b2e, ESLINT-BUCKET-137cbeae12c87193
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_182`
  - `mkdir -p build/eng-artifacts/T7.182`
  - `web/node_modules/.bin/eslint web/js/components/preact/fleet/OrganizationManager.jsx web/js/components/preact/fleet/SavedViewsBar.jsx web/js/components/preact/fleet/StaticCollectionMembers.jsx web/js/components/preact/investigation/InvestigationActions.jsx web/js/components/preact/investigation/InvestigationBookmarks.jsx web/js/components/preact/investigation/InvestigationView.jsx --format json --output-file build/eng-artifacts/T7.182/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.182/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.182`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.183: jsx-a11y/label-has-for in LiveBuildingPlan.jsx, LiveOperatorNavigator.jsx, FiltersSidebar.jsx, RecordingsGrid.jsx, RecordingsTable.jsx, TagsOverlay.jsx
- **depends_on**: [T7, T10, T7.171, T7.172, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/js/components/preact/recordings/FiltersSidebar.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/recordings/RecordingsTable.jsx`, `web/js/components/preact/recordings/TagsOverlay.jsx`, `web/tests/remediation/t7_183.spec.jsx`, `quality/evidence/T7.183.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-a167dfb879b1b9cf, ESLINT-BUCKET-b0e1dbc8e8cc536e, ESLINT-BUCKET-50994f387baeaa69, ESLINT-BUCKET-dcde47440ba091e3, ESLINT-BUCKET-114c375fcc3b4802, ESLINT-BUCKET-8b431e9fa35d81ba
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_183`
  - `mkdir -p build/eng-artifacts/T7.183`
  - `web/node_modules/.bin/eslint web/js/components/preact/live/LiveBuildingPlan.jsx web/js/components/preact/live/LiveOperatorNavigator.jsx web/js/components/preact/recordings/FiltersSidebar.jsx web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/recordings/RecordingsTable.jsx web/js/components/preact/recordings/TagsOverlay.jsx --format json --output-file build/eng-artifacts/T7.183/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.183/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.183`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.184: jsx-a11y/label-has-for in AdvancedTab.jsx, AuthTab.jsx, DetectionTab.jsx, GeneralTab.jsx, Go2rtcTab.jsx, HealthTab.jsx
- **depends_on**: [T7, T10, T7.172, T7.178, T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/settings/AdvancedTab.jsx`, `web/js/components/preact/settings/AuthTab.jsx`, `web/js/components/preact/settings/DetectionTab.jsx`, `web/js/components/preact/settings/GeneralTab.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/js/components/preact/settings/HealthTab.jsx`, `web/tests/remediation/t7_184.spec.jsx`, `quality/evidence/T7.184.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-a4e1217d107c230c, ESLINT-BUCKET-4a0d0e1ea3635a24, ESLINT-BUCKET-d6ed69efe339ce37, ESLINT-BUCKET-61d1473d84fbeea6, ESLINT-BUCKET-dd680edebf6809ff, ESLINT-BUCKET-e267f8f47c3806d3
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_184`
  - `mkdir -p build/eng-artifacts/T7.184`
  - `web/node_modules/.bin/eslint web/js/components/preact/settings/AdvancedTab.jsx web/js/components/preact/settings/AuthTab.jsx web/js/components/preact/settings/DetectionTab.jsx web/js/components/preact/settings/GeneralTab.jsx web/js/components/preact/settings/Go2rtcTab.jsx web/js/components/preact/settings/HealthTab.jsx --format json --output-file build/eng-artifacts/T7.184/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.184/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.184`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.185: jsx-a11y/label-has-for in MqttTab.jsx, StorageTab.jsx, StreamsDefaultsTab.jsx, WorkspacesTab.jsx, TimelinePage.jsx, TimelinePlayer.jsx
- **depends_on**: [T7, T10, T7.173, T7.178, T7.174, T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/settings/MqttTab.jsx`, `web/js/components/preact/settings/StorageTab.jsx`, `web/js/components/preact/settings/StreamsDefaultsTab.jsx`, `web/js/components/preact/settings/WorkspacesTab.jsx`, `web/js/components/preact/timeline/TimelinePage.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/tests/remediation/t7_185.spec.jsx`, `quality/evidence/T7.185.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-d5ecbc0292c55216, ESLINT-BUCKET-c067bb3093f57d49, ESLINT-BUCKET-4e8b56001c3a0722, ESLINT-BUCKET-223399111dfd4016, ESLINT-BUCKET-7d2f50f4d066f6e4, ESLINT-BUCKET-00d366c5691b4bb5
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_185`
  - `mkdir -p build/eng-artifacts/T7.185`
  - `web/node_modules/.bin/eslint web/js/components/preact/settings/MqttTab.jsx web/js/components/preact/settings/StorageTab.jsx web/js/components/preact/settings/StreamsDefaultsTab.jsx web/js/components/preact/settings/WorkspacesTab.jsx web/js/components/preact/timeline/TimelinePage.jsx web/js/components/preact/timeline/TimelinePlayer.jsx --format json --output-file build/eng-artifacts/T7.185/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.185/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.185`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.186: jsx-a11y/label-has-for in AccessPolicyModal.jsx, AddUserModal.jsx, ApiKeyModal.jsx, AuditDecisionModes.jsx, AuditHistoryModal.jsx, EditUserModal.jsx
- **depends_on**: [T7, T10, T7.164, T7.174, T7.178]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/AccessPolicyModal.jsx`, `web/js/components/preact/users/AddUserModal.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditDecisionModes.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/tests/remediation/t7_186.spec.jsx`, `quality/evidence/T7.186.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-7f43090b7b01626a, ESLINT-BUCKET-17ad0e14291e8ec3, ESLINT-BUCKET-18b7f3add93999e4, ESLINT-BUCKET-d3157833e1568912, ESLINT-BUCKET-184f5809a1f3d621, ESLINT-BUCKET-682efc8151a90daf
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_186`
  - `mkdir -p build/eng-artifacts/T7.186`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/AccessPolicyModal.jsx web/js/components/preact/users/AddUserModal.jsx web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditDecisionModes.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/EditUserModal.jsx --format json --output-file build/eng-artifacts/T7.186/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.186/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.186`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.187: jsx-a11y/label-has-for in RoleManagerModal.jsx, TotpSetupModal.jsx
- **depends_on**: [T7, T10, T7.175]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/RoleManagerModal.jsx`, `web/js/components/preact/users/TotpSetupModal.jsx`, `web/tests/remediation/t7_187.spec.jsx`, `web/tests/remediation/t7_165.spec.jsx` (fixed-heading selector compatibility only; preserve saving-guard assertion), `quality/evidence/T7.187.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-2e14d60ae388c180, ESLINT-BUCKET-9a54a74c58d29d4c
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input. Repeated RoleManager dialogs must also resolve their own uniquely identified heading; reuse the existing per-instance ID.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_187`
  - `mkdir -p build/eng-artifacts/T7.187`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/RoleManagerModal.jsx web/js/components/preact/users/TotpSetupModal.jsx --format json --output-file build/eng-artifacts/T7.187/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.187/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.187`; inspect the raw T1 report and trace the caller.
- **status**: Completed (implementation `89474721`, full-suite evidence `a02a3cac`)
- **log**: Tests-first missing TOTP nesting and duplicate repeated dialog-heading IDs reproduced;17/17 combined browser-enabled cases pass. Exact lint873→872 with0errors/no new warnings; types and both43/66asset budgets pass. Full web suite60/60,598passed,5opt-in browser tests unselected; browser supplement executed separately. Default dist restored.
- **files edited/created**:

### T7.188: jsx-a11y/media-has-caption in HLSVideoCell.jsx, MSEVideoCell.jsx, UI.jsx, WebRTCVideoCell.jsx, InvestigationView.jsx, TimelinePlayer.jsx
- **depends_on**: [T7, T10, T7.166, T7.167, T7.180, T7.168, T7.182, T7.185]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/HLSVideoCell.jsx`, `web/js/components/preact/MSEVideoCell.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/investigation/InvestigationView.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/tests/remediation/t7_188.spec.jsx`, `quality/evidence/T7.188.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-165a348ebdcecd71, ESLINT-BUCKET-1760eab07e0cce49, ESLINT-BUCKET-df0c5f209b6ef6ab, ESLINT-BUCKET-c464a6c1e23bf196, ESLINT-BUCKET-de50a4c99ae7d421, ESLINT-BUCKET-7189b497b5a855df
- **acceptance_criteria**:
  - Identify prerecorded speech versus live/silent surveillance streams; provide applicable caption support, or an explicitly reviewed narrow semantic exception with evidence. Never add a fake empty caption track.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_188`
  - `mkdir -p build/eng-artifacts/T7.188`
  - `web/node_modules/.bin/eslint web/js/components/preact/HLSVideoCell.jsx web/js/components/preact/MSEVideoCell.jsx web/js/components/preact/UI.jsx web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/investigation/InvestigationView.jsx web/js/components/preact/timeline/TimelinePlayer.jsx --format json --output-file build/eng-artifacts/T7.188/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.188/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/media-has-caption'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.188`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.189: jsx-a11y/mouse-events-have-key-events in DayNightControl.jsx, HLSVideoCell.jsx, SnapshotManager.jsx, StreamCard.jsx, StreamsView.jsx, Toast.jsx
- **depends_on**: [T7, T10, T7.188, T7.167, T7.180]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/DayNightControl.jsx`, `web/js/components/preact/HLSVideoCell.jsx`, `web/js/components/preact/SnapshotManager.jsx`, `web/js/components/preact/StreamCard.jsx`, `web/js/components/preact/StreamsView.jsx`, `web/js/components/preact/Toast.jsx`, `web/tests/remediation/t7_189.spec.jsx`, `quality/evidence/T7.189.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-bc4e9a8b10d83163, ESLINT-BUCKET-c31a4d9816cac7ce, ESLINT-BUCKET-e715a00f3dbe2f4f, ESLINT-BUCKET-9a70a1ccf37b9405, ESLINT-BUCKET-e876245abbef0a79, ESLINT-BUCKET-c811624672d6bfff
- **acceptance_criteria**:
  - Assert focus/blur and mouse enter/leave expose and dismiss the same content without trapping focus.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_189`
  - `mkdir -p build/eng-artifacts/T7.189`
  - `web/node_modules/.bin/eslint web/js/components/preact/DayNightControl.jsx web/js/components/preact/HLSVideoCell.jsx web/js/components/preact/SnapshotManager.jsx web/js/components/preact/StreamCard.jsx web/js/components/preact/StreamsView.jsx web/js/components/preact/Toast.jsx --format json --output-file build/eng-artifacts/T7.189/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.189/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/mouse-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.189`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.190: jsx-a11y/mouse-events-have-key-events in WebRTCVideoCell.jsx, RecordingsTable.jsx, TimelinePage.jsx
- **depends_on**: [T7, T10, T7.188, T7.183, T7.185, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/recordings/RecordingsTable.jsx`, `web/js/components/preact/timeline/TimelinePage.jsx`, `web/tests/remediation/t7_190.spec.jsx`, `quality/evidence/T7.190.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-a4f59c579d19ce1e, ESLINT-BUCKET-7a540f91903a1e57, ESLINT-BUCKET-dcdb657c2038e332
- **acceptance_criteria**:
  - Assert focus/blur and mouse enter/leave expose and dismiss the same content without trapping focus.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_190`
  - `mkdir -p build/eng-artifacts/T7.190`
  - `web/node_modules/.bin/eslint web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/recordings/RecordingsTable.jsx web/js/components/preact/timeline/TimelinePage.jsx --format json --output-file build/eng-artifacts/T7.190/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.190/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/mouse-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.190`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.191: jsx-a11y/no-autofocus in AuthGate.jsx, LoginView.jsx, EventDestinationEditor.jsx, EventRouteEditor.jsx, CollectionManager.jsx, OrganizationManager.jsx
- **depends_on**: [T7, T10, T7.179, T7.181, T7.182]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/AuthGate.jsx`, `web/js/components/preact/LoginView.jsx`, `web/js/components/preact/events/EventDestinationEditor.jsx`, `web/js/components/preact/events/EventRouteEditor.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/OrganizationManager.jsx`, `web/tests/remediation/t7_191.spec.jsx`, `quality/evidence/T7.191.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-20016996a3c7e096, ESLINT-BUCKET-92718cbd3fc79c29, ESLINT-BUCKET-71520b6429344586, ESLINT-BUCKET-82f90000b805b238, ESLINT-BUCKET-2acf09b0886c70c8, ESLINT-BUCKET-e6884093b3a77737
- **acceptance_criteria**:
  - Assert initial render does not steal focus; explicit dialog-open focus management must return focus to the invoker on close.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_191`
  - `mkdir -p build/eng-artifacts/T7.191`
  - `web/node_modules/.bin/eslint web/js/components/preact/AuthGate.jsx web/js/components/preact/LoginView.jsx web/js/components/preact/events/EventDestinationEditor.jsx web/js/components/preact/events/EventRouteEditor.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/OrganizationManager.jsx --format json --output-file build/eng-artifacts/T7.191/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.191/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-autofocus'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.191`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.192: jsx-a11y/no-autofocus in InvestigationBookmarks.jsx, TotpSetupModal.jsx
- **depends_on**: [T7, T10, T7.182, T7.187]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/investigation/InvestigationBookmarks.jsx`, `web/js/components/preact/users/TotpSetupModal.jsx`, `web/tests/remediation/t7_192.spec.jsx`, `quality/evidence/T7.192.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-24d767472f9eac0b, ESLINT-BUCKET-6040ad491a730d6d
- **acceptance_criteria**:
  - Assert initial render does not steal focus; explicit dialog-open focus management must return focus to the invoker on close.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_192`
  - `mkdir -p build/eng-artifacts/T7.192`
  - `web/node_modules/.bin/eslint web/js/components/preact/investigation/InvestigationBookmarks.jsx web/js/components/preact/users/TotpSetupModal.jsx --format json --output-file build/eng-artifacts/T7.192/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.192/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-autofocus'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.192`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.193: jsx-a11y/no-noninteractive-element-interactions in FisheyeEptzCanvas.jsx, StreamCard.jsx, ZoneEditor.jsx, LanguageSelector.jsx, InvestigationView.jsx, LiveBuildingPlan.jsx
- **depends_on**: [T7, T10, T7.189, T7.180, T7.162, T7.188, T7.183, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/FisheyeEptzCanvas.jsx`, `web/js/components/preact/StreamCard.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/js/components/preact/common/LanguageSelector.jsx`, `web/js/components/preact/investigation/InvestigationView.jsx`, `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/tests/remediation/t7_193.spec.jsx`, `quality/evidence/T7.193.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-d8c2127f8046b560, ESLINT-BUCKET-e4342be069acaf24, ESLINT-BUCKET-141e103765b7c505, ESLINT-BUCKET-370c727cb2690828, ESLINT-BUCKET-bf7012d93c48094c, ESLINT-BUCKET-11c6ba5b7bf02e40
- **acceptance_criteria**:
  - Remove unintended interactive behavior from structural elements or expose correct interactive semantics; preserve nested-control propagation and keyboard behavior.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_193`
  - `mkdir -p build/eng-artifacts/T7.193`
  - `web/node_modules/.bin/eslint web/js/components/preact/FisheyeEptzCanvas.jsx web/js/components/preact/StreamCard.jsx web/js/components/preact/ZoneEditor.jsx web/js/components/preact/common/LanguageSelector.jsx web/js/components/preact/investigation/InvestigationView.jsx web/js/components/preact/live/LiveBuildingPlan.jsx --format json --output-file build/eng-artifacts/T7.193/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.193/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-noninteractive-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.193`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.194: jsx-a11y/no-noninteractive-element-interactions in LiveOperatorNavigator.jsx, RecordingsGrid.jsx, TimelineCursor.jsx, TimelineSegments.jsx, AccessPolicyModal.jsx, AddUserModal.jsx
- **depends_on**: [T7, T10, T7.183, T7.186, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/timeline/TimelineCursor.jsx`, `web/js/components/preact/timeline/TimelineSegments.jsx`, `web/js/components/preact/users/AccessPolicyModal.jsx`, `web/js/components/preact/users/AddUserModal.jsx`, `web/tests/remediation/t7_194.spec.jsx`, `quality/evidence/T7.194.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-87cfe9ce5d499faf, ESLINT-BUCKET-4d679d89b4da6865, ESLINT-BUCKET-d670cad0625d4f2c, ESLINT-BUCKET-03caeab009bdc4fd, ESLINT-BUCKET-939df4abd7adefdd, ESLINT-BUCKET-4cf2eab3dda13b90
- **acceptance_criteria**:
  - Remove unintended interactive behavior from structural elements or expose correct interactive semantics; preserve nested-control propagation and keyboard behavior.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_194`
  - `mkdir -p build/eng-artifacts/T7.194`
  - `web/node_modules/.bin/eslint web/js/components/preact/live/LiveOperatorNavigator.jsx web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/timeline/TimelineCursor.jsx web/js/components/preact/timeline/TimelineSegments.jsx web/js/components/preact/users/AccessPolicyModal.jsx web/js/components/preact/users/AddUserModal.jsx --format json --output-file build/eng-artifacts/T7.194/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.194/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-noninteractive-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.194`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.195: jsx-a11y/no-noninteractive-element-interactions in ApiKeyModal.jsx, AuditHistoryModal.jsx, EditUserModal.jsx, RoleManagerModal.jsx
- **depends_on**: [T7, T10, T7.186, T7.187]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/js/components/preact/users/RoleManagerModal.jsx`, `web/tests/remediation/t7_195.spec.jsx`, `quality/evidence/T7.195.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-68103860cfe78d89, ESLINT-BUCKET-be996c224380b85e, ESLINT-BUCKET-65f403c439392041, ESLINT-BUCKET-752682010e0c301c
- **acceptance_criteria**:
  - Remove unintended interactive behavior from structural elements or expose correct interactive semantics; preserve nested-control propagation and keyboard behavior.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_195`
  - `mkdir -p build/eng-artifacts/T7.195`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/EditUserModal.jsx web/js/components/preact/users/RoleManagerModal.jsx --format json --output-file build/eng-artifacts/T7.195/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.195/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-noninteractive-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.195`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**: Related occurrence already resolved by T7.165 (fe93b78c): RoleManagerModal no-noninteractive-element-interactions1→0. Recheck after dependencies and other owned files before closing this task; evidence quality/evidence/T7.165.md.
- **files edited/created**:

### T7.196: jsx-a11y/no-noninteractive-tabindex in FisheyeEptzCanvas.jsx, LanguageSelector.jsx
- **depends_on**: [T7, T10, T7.193]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/FisheyeEptzCanvas.jsx`, `web/js/components/preact/common/LanguageSelector.jsx`, `web/tests/remediation/t7_196.spec.jsx`, `quality/evidence/T7.196.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-f32c4fd5a51337d3, ESLINT-BUCKET-d5958c65428cdc31
- **acceptance_criteria**:
  - Assert the tab order includes only intended interactive controls; keyboard navigation of composite widgets must follow their documented role.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_196`
  - `mkdir -p build/eng-artifacts/T7.196`
  - `web/node_modules/.bin/eslint web/js/components/preact/FisheyeEptzCanvas.jsx web/js/components/preact/common/LanguageSelector.jsx --format json --output-file build/eng-artifacts/T7.196/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.196/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-noninteractive-tabindex'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.196`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.197: jsx-a11y/no-static-element-interactions in GridPicker.jsx, HLSVideoCell.jsx, HealthView.jsx, LiveView.jsx, MSEVideoCell.jsx, StreamCard.jsx
- **depends_on**: [T7, T10, T7.161, T7.189, T7.179, T7.188, T7.193]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/GridPicker.jsx`, `web/js/components/preact/HLSVideoCell.jsx`, `web/js/components/preact/HealthView.jsx`, `web/js/components/preact/LiveView.jsx`, `web/js/components/preact/MSEVideoCell.jsx`, `web/js/components/preact/StreamCard.jsx`, `web/tests/remediation/t7_197.spec.jsx`, `quality/evidence/T7.197.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-45ebdb33421d89d5, ESLINT-BUCKET-d6af678aacfbb026, ESLINT-BUCKET-e48ab90f6d76f69a, ESLINT-BUCKET-6cf65d94c0e817c8, ESLINT-BUCKET-d0493f3174391c77, ESLINT-BUCKET-134e1087e8cda477
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_197`
  - `mkdir -p build/eng-artifacts/T7.197`
  - `web/node_modules/.bin/eslint web/js/components/preact/GridPicker.jsx web/js/components/preact/HLSVideoCell.jsx web/js/components/preact/HealthView.jsx web/js/components/preact/LiveView.jsx web/js/components/preact/MSEVideoCell.jsx web/js/components/preact/StreamCard.jsx --format json --output-file build/eng-artifacts/T7.197/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.197/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.197`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.198: jsx-a11y/no-static-element-interactions in StreamConfigModal.jsx, StreamsView.jsx, ToastContainer.jsx, UI.jsx, WebRTCVideoCell.jsx, ZoneEditor.jsx
- **depends_on**: [T7, T10, T7.180, T7.189, T7.162, T7.188, T7.190, T7.193, T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamConfigModal.jsx`, `web/js/components/preact/StreamsView.jsx`, `web/js/components/preact/ToastContainer.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/tests/remediation/t7_198.spec.jsx`, `quality/evidence/T7.198.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-8a059d804561f236, ESLINT-BUCKET-21e6c2fa9c26c964, ESLINT-BUCKET-b18881a630321191, ESLINT-BUCKET-d9d81163d1e1193d, ESLINT-BUCKET-3322f39793a9258c, ESLINT-BUCKET-6bdf7c909180cb5e
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_198`
  - `mkdir -p build/eng-artifacts/T7.198`
  - `web/node_modules/.bin/eslint web/js/components/preact/StreamConfigModal.jsx web/js/components/preact/StreamsView.jsx web/js/components/preact/ToastContainer.jsx web/js/components/preact/UI.jsx web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/ZoneEditor.jsx --format json --output-file build/eng-artifacts/T7.198/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.198/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.198`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.199: jsx-a11y/no-static-element-interactions in ModalDialog.jsx, BulkOrganizeModal.jsx, CollectionManager.jsx, OrganizationManager.jsx, LiveBuildingPlan.jsx, LiveOperatorNavigator.jsx
- **depends_on**: [T7, T10, T7.180, T7.181, T7.191, T7.193, T7.194, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/common/ModalDialog.jsx`, `web/js/components/preact/fleet/BulkOrganizeModal.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/OrganizationManager.jsx`, `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/tests/remediation/t7_199.spec.jsx`, `quality/evidence/T7.199.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-243fa74ece4acc83, ESLINT-BUCKET-aa87081e964848f0, ESLINT-BUCKET-b9126919c0a748d2, ESLINT-BUCKET-ac70bbab0047fa80, ESLINT-BUCKET-23073b0c71d7d2e3, ESLINT-BUCKET-be705595f12d1850
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_199`
  - `mkdir -p build/eng-artifacts/T7.199`
  - `web/node_modules/.bin/eslint web/js/components/preact/common/ModalDialog.jsx web/js/components/preact/fleet/BulkOrganizeModal.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/OrganizationManager.jsx web/js/components/preact/live/LiveBuildingPlan.jsx web/js/components/preact/live/LiveOperatorNavigator.jsx --format json --output-file build/eng-artifacts/T7.199/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.199/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.199`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.200: jsx-a11y/no-static-element-interactions in RecordingsGrid.jsx, Go2rtcTab.jsx, ClearLogsModal.jsx, RestartModal.jsx, TimelinePlayer.jsx, AccessPolicyModal.jsx
- **depends_on**: [T7, T10, T7.194, T7.184, T7.163, T7.188, T10.2, T10.3]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/js/components/preact/system/ClearLogsModal.jsx`, `web/js/components/preact/system/RestartModal.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/js/components/preact/users/AccessPolicyModal.jsx`, `web/tests/remediation/t7_200.spec.jsx`, `quality/evidence/T7.200.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-55a74c5d3180cf15, ESLINT-BUCKET-fd743aa60d9abb1f, ESLINT-BUCKET-fbc7ca1e0083c1d1, ESLINT-BUCKET-259ca1824a1d1447, ESLINT-BUCKET-c06de8e6c5473e61, ESLINT-BUCKET-6bab06cb3c8102a9
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_200`
  - `mkdir -p build/eng-artifacts/T7.200`
  - `web/node_modules/.bin/eslint web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/settings/Go2rtcTab.jsx web/js/components/preact/system/ClearLogsModal.jsx web/js/components/preact/system/RestartModal.jsx web/js/components/preact/timeline/TimelinePlayer.jsx web/js/components/preact/users/AccessPolicyModal.jsx --format json --output-file build/eng-artifacts/T7.200/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.200/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.200`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.201: jsx-a11y/no-static-element-interactions in AddUserModal.jsx, ApiKeyModal.jsx, AuditHistoryModal.jsx, DeleteUserModal.jsx, EditUserModal.jsx, RoleManagerModal.jsx
- **depends_on**: [T7, T10, T7.194, T7.195, T7.164]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/AddUserModal.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/DeleteUserModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/js/components/preact/users/RoleManagerModal.jsx`, `web/tests/remediation/t7_201.spec.jsx`, `quality/evidence/T7.201.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-3ace786e8c2cd1a8, ESLINT-BUCKET-08d8785eb7476330, ESLINT-BUCKET-51093f104f4e1cbb, ESLINT-BUCKET-7c82f257c4fb1103, ESLINT-BUCKET-c4dd2be3483455e5, ESLINT-BUCKET-5c8c07852ed0b107
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_201`
  - `mkdir -p build/eng-artifacts/T7.201`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/AddUserModal.jsx web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/DeleteUserModal.jsx web/js/components/preact/users/EditUserModal.jsx web/js/components/preact/users/RoleManagerModal.jsx --format json --output-file build/eng-artifacts/T7.201/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.201/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.201`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**: Related occurrence already resolved by T7.165 (fe93b78c): RoleManagerModal no-static-element-interactions1→0. Recheck after dependencies and other owned files before closing this task; evidence quality/evidence/T7.165.md.
- **files edited/created**:

### T7.202: jsx-a11y/no-static-element-interactions in TotpSetupModal.jsx
- **depends_on**: [T7, T10, T7.192]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/TotpSetupModal.jsx`, `web/tests/remediation/t7_202.spec.jsx`, `quality/evidence/T7.202.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-e046fbe269eed0dc
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_202`
  - `mkdir -p build/eng-artifacts/T7.202`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/TotpSetupModal.jsx --format json --output-file build/eng-artifacts/T7.202/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.202/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.202`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**: Related occurrence already resolved by T7.165 (fe93b78c): TotpSetupModal no-static-element-interactions2→0. Recheck after dependencies and other owned files before closing this task; evidence quality/evidence/T7.165.md.
- **files edited/created**:

### T7.203: nullPointerArithmeticRedundantCheck in sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_203-upstream.json`, `quality/evidence/T7.203.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-38d664fd8e77f607, CPPCHECK-3d1693ad77a15fa2, CPPCHECK-3508a7db3ef7fc03, CPPCHECK-ca79e52beb5f22bb, CPPCHECK-5941fdb9ccb989f3
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_203-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.203`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.204: nullPointerRedundantCheck in sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_204-upstream.json`, `quality/evidence/T7.204.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-10a0fa2f24fefc90, CPPCHECK-ffdd97ac6ebb6b4b, CPPCHECK-7dc87bf617f52146, CPPCHECK-347fec696ca3fcab, CPPCHECK-1980a8233237856a
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_204-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.204`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.205: wrongPrintfScanfArgNum in linux_thermal.c
- **depends_on**: [T7, T13, T7.95]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_205.c`, `tests/unit/cmake/eng/t7_205.cmake`, `quality/evidence/T7.205.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-e9cd8f29e136cbb8
- **acceptance_criteria**:
  - Exercise every reported format/library call with boundary values; argument types/counts, buffer extents and returned errors must match the library contract with no UB.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.205 --regex '^test_t7_205$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.205`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.206: zerodivcond in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_206-upstream.json`, `quality/evidence/T7.206.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-ac6f81ae03a685f3
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_206-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.206`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.207: arrayIndexThenCheck in sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_207-upstream.json`, `quality/evidence/T7.207.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-8bb35cef096bd68d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_207-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.207`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.208: constParameter in linux_network.c
- **depends_on**: [T7, T13, T7.95]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `tests/unit/test_t7_208.c`, `tests/unit/cmake/eng/t7_208.cmake`, `quality/evidence/T7.208.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-7e66ded89b37cb4f
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.208 --regex '^test_t7_208$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.208`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.209: constParameter in api_handlers_investigations.c
- **depends_on**: [T7, T13, T7.148]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_investigations.c`, `tests/unit/test_t7_209.c`, `tests/unit/cmake/eng/t7_209.cmake`, `quality/evidence/T7.209.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-c6d1df4e6770f40e
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.209 --regex '^test_t7_209$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.209`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.210: constParameterCallback in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_210-upstream.json`, `quality/evidence/T7.210.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-fb37582a72f59582, CPPCHECK-BUCKET-c7b3ef530f996e26
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_210-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.210`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.211: constParameterCallback in storage_migration.c
- **depends_on**: [T7, T13, T7.138]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_211.c`, `tests/unit/cmake/eng/t7_211.cmake`, `quality/evidence/T7.211.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-90ec02278e01bcac
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.211 --regex '^test_t7_211$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.211`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.212: constParameterCallback in yaml_redact.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/utils/yaml_redact.c`, `tests/unit/test_t7_212.c`, `tests/unit/cmake/eng/t7_212.cmake`, `quality/evidence/T7.212.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-6291a48798ed350c
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.212 --regex '^test_t7_212$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.212`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.213: constParameterCallback in api_detection.c
- **depends_on**: [T7, T13, T7.112]
- **agent_type**: worker
- **ownership**: `src/video/api_detection.c`, `tests/unit/test_t7_213.c`, `tests/unit/cmake/eng/t7_213.cmake`, `quality/evidence/T7.213.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-681aedac30d87a1e
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.213 --regex '^test_t7_213$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.213`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.214: constParameterCallback in go2rtc_api.c, go2rtc_integration.c, go2rtc_snapshot.c
- **depends_on**: [T7, T13, T7.98, T7.154]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_api.c`, `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_snapshot.c`, `tests/unit/test_t7_214.c`, `tests/unit/cmake/eng/t7_214.cmake`, `quality/evidence/T7.214.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-ee781b371da39bf5, CPPCHECK-BUCKET-dc056676c7084f53, CPPCHECK-BUCKET-0130dae8fb623afa
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.214 --regex '^test_t7_214$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.214`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.215: constParameterCallback in onvif_detection.c
- **depends_on**: [T7, T13, T7.146]
- **agent_type**: worker
- **ownership**: `src/video/onvif_detection.c`, `tests/unit/test_t7_215.c`, `tests/unit/cmake/eng/t7_215.cmake`, `quality/evidence/T7.215.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-365ebbda46eacf74
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.215 --regex '^test_t7_215$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.215`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.216: constParameterCallback in onvif_device_management.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_device_management.c`, `tests/unit/test_t7_216.c`, `tests/unit/cmake/eng/t7_216.cmake`, `quality/evidence/T7.216.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-8737ababcc06f484
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.216 --regex '^test_t7_216$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.216`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.217: constParameterCallback in onvif_imaging.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_imaging.c`, `tests/unit/test_t7_217.c`, `tests/unit/cmake/eng/t7_217.cmake`, `quality/evidence/T7.217.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-baf28ba17bc25dd0
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.217 --regex '^test_t7_217$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.217`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.218: constParameterCallback in onvif_ptz.c
- **depends_on**: [T7, T13, T7.156]
- **agent_type**: worker
- **ownership**: `src/video/onvif_ptz.c`, `tests/unit/test_t7_218.c`, `tests/unit/cmake/eng/t7_218.cmake`, `quality/evidence/T7.218.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-462bf13ae3867115
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.218 --regex '^test_t7_218$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.218`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.219: constParameterCallback in api_handlers_go2rtc_proxy.c
- **depends_on**: [T7, T13, T7.147]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_go2rtc_proxy.c`, `tests/unit/test_t7_219.c`, `tests/unit/cmake/eng/t7_219.cmake`, `quality/evidence/T7.219.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b6f82c4c9b287eba
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.219 --regex '^test_t7_219$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.219`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.220: constParameterCallback in api_handlers_health.c
- **depends_on**: [T7, T13, T7.158]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_health.c`, `tests/unit/test_t7_220.c`, `tests/unit/cmake/eng/t7_220.cmake`, `quality/evidence/T7.220.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-c7a8e9a51802c60d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.220 --regex '^test_t7_220$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.220`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.221: constParameterCallback in go2rtc_proxy_thread.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_proxy_thread.c`, `tests/unit/test_t7_221.c`, `tests/unit/cmake/eng/t7_221.cmake`, `quality/evidence/T7.221.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d3985740b2ac6585
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.221 --regex '^test_t7_221$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.221`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.222: constParameterPointer in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_222-upstream.json`, `quality/evidence/T7.222.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-82a085f0cc04468f, CPPCHECK-BUCKET-27ab7ab1e74c2e72
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_222-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.222`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.223: constParameterPointer in authorization.c
- **depends_on**: [T7, T13, T7.114]
- **agent_type**: worker
- **ownership**: `src/core/authorization.c`, `tests/unit/test_t7_223.c`, `tests/unit/cmake/eng/t7_223.cmake`, `quality/evidence/T7.223.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d0de2e4ac47c3191
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.223 --regex '^test_t7_223$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.223`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.224: constParameterPointer in mqtt_destination_client.c
- **depends_on**: [T7, T13, T7.134]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_destination_client.c`, `tests/unit/test_t7_224.c`, `tests/unit/cmake/eng/t7_224.cmake`, `quality/evidence/T7.224.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-fbb9e9402242eaba
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.224 --regex '^test_t7_224$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.224`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.225: constParameterPointer in db_camera_collections.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_camera_collections.c`, `tests/unit/test_t7_225.c`, `tests/unit/cmake/eng/t7_225.cmake`, `quality/evidence/T7.225.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-fd8d6a4b8cc77400
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.225 --regex '^test_t7_225$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.225`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.226: constParameterPointer in ezxml.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_226-upstream.json`, `quality/evidence/T7.226.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-43fcd9340a9e63ed
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_226-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.226`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.227: constParameterPointer in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_227-upstream.json`, `quality/evidence/T7.227.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-e7f8f80f0cd95f0d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_227-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.227`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.228: constParameterPointer in linux_cgroup.c
- **depends_on**: [T7, T13, T7.83]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_cgroup.c`, `tests/unit/test_t7_228.c`, `tests/unit/cmake/eng/t7_228.cmake`, `quality/evidence/T7.228.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-70e0496568faa30f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.228 --regex '^test_t7_228$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.228`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.229: constParameterPointer in nvme_provider.c
- **depends_on**: [T7, T13, T7.76]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/nvme_provider.c`, `tests/unit/test_t7_229.c`, `tests/unit/cmake/eng/t7_229.cmake`, `quality/evidence/T7.229.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-5bdf073f41a6c65d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.229 --regex '^test_t7_229$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.229`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.230: constParameterPointer in system_health_evaluator.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health_evaluator.c`, `tests/unit/test_t7_230.c`, `tests/unit/cmake/eng/t7_230.cmake`, `quality/evidence/T7.230.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-527aa4750d3ac84a
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.230 --regex '^test_t7_230$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.230`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.231: constParameterPointer in go2rtc_base_path.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/utils/go2rtc_base_path.c`, `tests/unit/test_t7_231.c`, `tests/unit/cmake/eng/t7_231.cmake`, `quality/evidence/T7.231.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d9684d3aa203568c
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.231 --regex '^test_t7_231$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.231`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.232: constParameterPointer in strings.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/utils/strings.c`, `tests/unit/test_t7_232.c`, `tests/unit/cmake/eng/t7_232.cmake`, `quality/evidence/T7.232.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-e228634622467328
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.232 --regex '^test_t7_232$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.232`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.233: constParameterPointer in faac_encoder.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/faac_encoder.c`, `tests/unit/test_t7_233.c`, `tests/unit/cmake/eng/t7_233.cmake`, `quality/evidence/T7.233.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-bdb708e597b5cade
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.233 --regex '^test_t7_233$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.233`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.234: constParameterPointer in hls_unified_thread.c
- **depends_on**: [T7, T13, T7.69]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_unified_thread.c`, `tests/unit/test_t7_234.c`, `tests/unit/cmake/eng/t7_234.cmake`, `quality/evidence/T7.234.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-cbab871259b9963d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.234 --regex '^test_t7_234$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.234`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.235: constParameterPointer in unified_detection_thread.c
- **depends_on**: [T7, T13, T7.122]
- **agent_type**: worker
- **ownership**: `src/video/unified_detection_thread.c`, `tests/unit/test_t7_235.c`, `tests/unit/cmake/eng/t7_235.cmake`, `quality/evidence/T7.235.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-8c644cc847574bfe
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.235 --regex '^test_t7_235$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.235`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.236: constParameterPointer in go2rtc_websocket.c
- **depends_on**: [T7, T13, T7.150]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_websocket.c`, `tests/unit/test_t7_236.c`, `tests/unit/cmake/eng/t7_236.cmake`, `quality/evidence/T7.236.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-4c0052bc9de6e496
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.236 --regex '^test_t7_236$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.236`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.237: constParameterPointer in recording_archive.c
- **depends_on**: [T7, T13, T7.141]
- **agent_type**: worker
- **ownership**: `src/web/recording_archive.c`, `tests/unit/test_t7_237.c`, `tests/unit/cmake/eng/t7_237.cmake`, `quality/evidence/T7.237.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-45cfee19afe8668f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.237 --regex '^test_t7_237$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.237`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.238: constVariable in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_238-upstream.json`, `quality/evidence/T7.238.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-933d7ae37be3c395, CPPCHECK-BUCKET-b1c0c559c6e17a23
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_238-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.238`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.239: constVariable in mqtt_delivery_worker.c
- **depends_on**: [T7, T13, T7.91]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_delivery_worker.c`, `tests/unit/test_t7_239.c`, `tests/unit/cmake/eng/t7_239.cmake`, `quality/evidence/T7.239.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-8596888f0dd1cc7e
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.239 --regex '^test_t7_239$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.239`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.240: constVariable in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_240-upstream.json`, `quality/evidence/T7.240.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-298feb4a490d6252
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_240-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.240`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.241: constVariable in sod_ingenic_mxu.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_241-upstream.json`, `quality/evidence/T7.241.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-a8ce2c3168c021de
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_241-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.241`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.242: constVariablePointer in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_242-upstream.json`, `quality/evidence/T7.242.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-9575437f939001d5, CPPCHECK-BUCKET-df230b0a5672f192
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_242-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.242`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.243: constVariablePointer in path_utils.c
- **depends_on**: [T7, T13, T7.128]
- **agent_type**: worker
- **ownership**: `src/core/path_utils.c`, `tests/unit/test_t7_243.c`, `tests/unit/cmake/eng/t7_243.cmake`, `quality/evidence/T7.243.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-8e110c5360beb299
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.243 --regex '^test_t7_243$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.243`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.244: constVariablePointer in db_auth.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_auth.c`, `tests/unit/test_t7_244.c`, `tests/unit/cmake/eng/t7_244.cmake`, `quality/evidence/T7.244.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-6c5cf9bd3f338f84
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.244 --regex '^test_t7_244$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.244`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.245: constVariablePointer in db_authorization.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_authorization.c`, `tests/unit/test_t7_245.c`, `tests/unit/cmake/eng/t7_245.cmake`, `quality/evidence/T7.245.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-08f60b7ec76ce9d3
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.245 --regex '^test_t7_245$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.245`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.246: constVariablePointer in db_camera_tags.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_camera_tags.c`, `tests/unit/test_t7_246.c`, `tests/unit/cmake/eng/t7_246.cmake`, `quality/evidence/T7.246.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-7e7f6cc12d937123
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.246 --regex '^test_t7_246$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.246`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.247: constVariablePointer in db_core.c
- **depends_on**: [T7, T13, T7.25]
- **agent_type**: worker
- **ownership**: `src/database/db_core.c`, `tests/unit/test_t7_247.c`, `tests/unit/cmake/eng/t7_247.cmake`, `quality/evidence/T7.247.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-f21c65912e355ace
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.247 --regex '^test_t7_247$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.247`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.248: constVariablePointer in db_fleet_query.c
- **depends_on**: [T7, T13, T7.113]
- **agent_type**: worker
- **ownership**: `src/database/db_fleet_query.c`, `tests/unit/test_t7_248.c`, `tests/unit/cmake/eng/t7_248.cmake`, `quality/evidence/T7.248.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9f651ab0c8ea6422
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.248 --regex '^test_t7_248$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.248`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.249: constVariablePointer in db_storage_lifecycle.c
- **depends_on**: [T7, T13, T7.129]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_lifecycle.c`, `tests/unit/test_t7_249.c`, `tests/unit/cmake/eng/t7_249.cmake`, `quality/evidence/T7.249.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-353deb8bba479029
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.249 --regex '^test_t7_249$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.249`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.250: constVariablePointer in ezxml.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_250-upstream.json`, `quality/evidence/T7.250.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-c6ee03168003661f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_250-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.250`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.251: constVariablePointer in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_251-upstream.json`, `quality/evidence/T7.251.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-5d8aaf042fefce78
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_251-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.251`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.252: constVariablePointer in storage_deletion.c
- **depends_on**: [T7, T13, T7.137, T28.1, T29.3]
- **agent_type**: worker
- **ownership**: `src/storage/storage_deletion.c`, `tests/unit/test_t7_252.c`, `tests/unit/cmake/eng/t7_252.cmake`, `quality/evidence/T7.252.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b4e128a1af325d4f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.252 --regex '^test_t7_252$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.252`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.253: constVariablePointer in storage_source.c
- **depends_on**: [T7, T13, T7.140]
- **agent_type**: worker
- **ownership**: `src/storage/storage_source.c`, `tests/unit/test_t7_253.c`, `tests/unit/cmake/eng/t7_253.cmake`, `quality/evidence/T7.253.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-77479b4ba7ef9849
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.253 --regex '^test_t7_253$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.253`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.254: constVariablePointer in linux_network.c, linux_process.c, linux_thermal.c
- **depends_on**: [T7, T13, T7.208, T7.95, T7.205]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `src/telemetry/collectors/linux_process.c`, `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_254.c`, `tests/unit/cmake/eng/t7_254.cmake`, `quality/evidence/T7.254.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-c9404ed2a73b0977, CPPCHECK-BUCKET-e924700b1413accd, CPPCHECK-BUCKET-2ca7413944026780
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.254 --regex '^test_t7_254$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.254`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.255: constVariablePointer in linux_device_map.c, linux_hardware.c, nvme_provider.c
- **depends_on**: [T7, T13, T7.143, T7.229]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/linux_device_map.c`, `src/telemetry/providers/linux_hardware.c`, `src/telemetry/providers/nvme_provider.c`, `tests/unit/test_t7_255.c`, `tests/unit/cmake/eng/t7_255.cmake`, `quality/evidence/T7.255.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-a1edb744b382f8f5, CPPCHECK-BUCKET-4001e3b91223c7ad, CPPCHECK-BUCKET-ae15015525d96c08
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.255 --regex '^test_t7_255$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.255`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.256: constVariablePointer in system_health.c
- **depends_on**: [T7, T13, T7.59]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health.c`, `tests/unit/test_t7_256.c`, `tests/unit/cmake/eng/t7_256.cmake`, `quality/evidence/T7.256.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-f3bb2108694b562f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.256 --regex '^test_t7_256$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.256`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.257: constVariablePointer in system_health_evaluator.c
- **depends_on**: [T7, T13, T7.230]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health_evaluator.c`, `tests/unit/test_t7_257.c`, `tests/unit/cmake/eng/t7_257.cmake`, `quality/evidence/T7.257.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-57093ce27e4aabc5
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.257 --regex '^test_t7_257$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.257`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.258: constVariablePointer in go2rtc_base_path.c
- **depends_on**: [T7, T13, T7.231]
- **agent_type**: worker
- **ownership**: `src/utils/go2rtc_base_path.c`, `tests/unit/test_t7_258.c`, `tests/unit/cmake/eng/t7_258.cmake`, `quality/evidence/T7.258.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-4dc0544d99c6a120
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.258 --regex '^test_t7_258$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.258`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.259: constVariablePointer in detection_model.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/detection_model.c`, `tests/unit/test_t7_259.c`, `tests/unit/cmake/eng/t7_259.cmake`, `quality/evidence/T7.259.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-aa0082260172b3b9
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.259 --regex '^test_t7_259$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.259`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.260: constVariablePointer in go2rtc_integration.c, go2rtc_process.c
- **depends_on**: [T7, T13, T7.214, T7.145]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_process.c`, `tests/unit/test_t7_260.c`, `tests/unit/cmake/eng/t7_260.cmake`, `quality/evidence/T7.260.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-5f736ad7b9095aea, CPPCHECK-BUCKET-eabea03354c7b4b7
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.260 --regex '^test_t7_260$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.260`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.261: constVariablePointer in mp4_recording_core.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_recording_core.c`, `tests/unit/test_t7_261.c`, `tests/unit/cmake/eng/t7_261.cmake`, `quality/evidence/T7.261.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-e9a38e72fa2e4a7c
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.261 --regex '^test_t7_261$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.261`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.262: constVariablePointer in mp4_recording_writer.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_recording_writer.c`, `tests/unit/test_t7_262.c`, `tests/unit/cmake/eng/t7_262.cmake`, `quality/evidence/T7.262.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-3ec7089cfaa16e94
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.262 --regex '^test_t7_262$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.262`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.263: constVariablePointer in stream_state_adapter.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/stream_state_adapter.c`, `tests/unit/test_t7_263.c`, `tests/unit/cmake/eng/t7_263.cmake`, `quality/evidence/T7.263.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-4ffd5c5d72b2c6a0
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.263 --regex '^test_t7_263$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.263`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.264: constVariablePointer in api_handlers_go2rtc_proxy.c
- **depends_on**: [T7, T13, T7.219]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_go2rtc_proxy.c`, `tests/unit/test_t7_264.c`, `tests/unit/cmake/eng/t7_264.cmake`, `quality/evidence/T7.264.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-f4469b09cc19dae9
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.264 --regex '^test_t7_264$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.264`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.265: constVariablePointer in api_handlers_recordings_list.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_recordings_list.c`, `tests/unit/test_t7_265.c`, `tests/unit/cmake/eng/t7_265.cmake`, `quality/evidence/T7.265.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-611000c5c122ad1e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.265 --regex '^test_t7_265$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.265`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.266: constVariablePointer in api_handlers_recordings_thumbnail.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_recordings_thumbnail.c`, `tests/unit/test_t7_266.c`, `tests/unit/cmake/eng/t7_266.cmake`, `quality/evidence/T7.266.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d41611e4fb1c1abd
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.266 --regex '^test_t7_266$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.266`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.267: constVariablePointer in api_handlers_system_logs.c
- **depends_on**: [T7, T13, T7.149]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system_logs.c`, `tests/unit/test_t7_267.c`, `tests/unit/cmake/eng/t7_267.cmake`, `quality/evidence/T7.267.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b14c25475c91b307
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.267 --regex '^test_t7_267$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.267`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.268: constVariablePointer in api_handlers_timeline.c
- **depends_on**: [T7, T13, T7.107]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_timeline.c`, `tests/unit/test_t7_268.c`, `tests/unit/cmake/eng/t7_268.cmake`, `quality/evidence/T7.268.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-03f92e362a140589
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.268 --regex '^test_t7_268$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.268`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.269: constVariablePointer in api_handlers_users_backend_agnostic.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_users_backend_agnostic.c`, `tests/unit/test_t7_269.c`, `tests/unit/cmake/eng/t7_269.cmake`, `quality/evidence/T7.269.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d45c6fa42b05db96
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.269 --regex '^test_t7_269$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.269`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.270: constVariablePointer in httpd_utils.c
- **depends_on**: [T7, T13, T7.133]
- **agent_type**: worker
- **ownership**: `src/web/httpd_utils.c`, `tests/unit/test_t7_270.c`, `tests/unit/cmake/eng/t7_270.cmake`, `quality/evidence/T7.270.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9551a431943d8273
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.270 --regex '^test_t7_270$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.270`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.271: duplicateAssignExpression in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_271-upstream.json`, `quality/evidence/T7.271.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-86335be98f6f2e90
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_271-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.271`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.272: duplicateCondition in motion_detection.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/motion_detection.c`, `tests/unit/test_t7_272.c`, `tests/unit/cmake/eng/t7_272.cmake`, `quality/evidence/T7.272.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-f814692d9029a8a1
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.272 --regex '^test_t7_272$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.272`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.273: js/unused-local-variable in LanguageSelector.jsx
- **depends_on**: [T7, T10, T7.196]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/common/LanguageSelector.jsx`, `web/tests/remediation/t7_273.spec.jsx`, `quality/evidence/T7.273.md`.
- **priority**: info
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: CODEQUALITY-941
- **acceptance_criteria**:
  - Remove only the proven unused binding; assert language selection/persistence and localized rendering still work.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_273`
  - `mkdir -p build/eng-artifacts/T7.273`
  - `web/node_modules/.bin/eslint web/js/components/preact/common/LanguageSelector.jsx --format json --output-file build/eng-artifacts/T7.273/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.273/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'js/unused-local-variable'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.273`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.274: knownConditionTrueFalse in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_274-upstream.json`, `quality/evidence/T7.274.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-1f586f8741e770e9
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_274-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.274`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.275: knownConditionTrueFalse in event_router.c
- **depends_on**: [T7, T13, T7.90]
- **agent_type**: worker
- **ownership**: `src/core/event_router.c`, `tests/unit/test_t7_275.c`, `tests/unit/cmake/eng/t7_275.cmake`, `quality/evidence/T7.275.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-95058fe138843b1f
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.275 --regex '^test_t7_275$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.275`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.276: knownConditionTrueFalse in mqtt_event_adapter.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_event_adapter.c`, `tests/unit/test_t7_276.c`, `tests/unit/cmake/eng/t7_276.cmake`, `quality/evidence/T7.276.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9c71189558451cd4
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.276 --regex '^test_t7_276$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.276`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.277: knownConditionTrueFalse in db_storage_targets.c
- **depends_on**: [T7, T13, T7.136]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_targets.c`, `tests/unit/test_t7_277.c`, `tests/unit/cmake/eng/t7_277.cmake`, `quality/evidence/T7.277.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-2f8dc61aeb7cdda6
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.277 --regex '^test_t7_277$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.277`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.278: knownConditionTrueFalse in ezxml.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_278-upstream.json`, `quality/evidence/T7.278.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-c7e9176ca688d280
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_278-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.278`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.279: knownConditionTrueFalse in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_279-upstream.json`, `quality/evidence/T7.279.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-064299ecf7d09b55
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_279-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.279`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.280: knownConditionTrueFalse in linux_process.c
- **depends_on**: [T7, T13, T7.254]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_process.c`, `tests/unit/test_t7_280.c`, `tests/unit/cmake/eng/t7_280.cmake`, `quality/evidence/T7.280.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-28b7ea4b18c686f4
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.280 --regex '^test_t7_280$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.280`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.281: knownConditionTrueFalse in health_helper_runner.c
- **depends_on**: [T7, T13, T7.68]
- **agent_type**: worker
- **ownership**: `src/telemetry/health_helper_runner.c`, `tests/unit/test_t7_281.c`, `tests/unit/cmake/eng/t7_281.cmake`, `quality/evidence/T7.281.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-1ac7ae81e97d3df2
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.281 --regex '^test_t7_281$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.281`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.282: knownConditionTrueFalse in api_handlers_imaging.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_imaging.c`, `tests/unit/test_t7_282.c`, `tests/unit/cmake/eng/t7_282.cmake`, `quality/evidence/T7.282.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-0251d17e5b89df2a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.282 --regex '^test_t7_282$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.282`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.283: redundantCondition in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_283-upstream.json`, `quality/evidence/T7.283.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-eee46ed30daef891
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_283-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.283`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.284: redundantInitialization in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_284-upstream.json`, `quality/evidence/T7.284.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-dcdbd8e6357d7e12
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_284-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.284`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.285: redundantInitialization in db_authorization.c
- **depends_on**: [T7, T13, T7.245]
- **agent_type**: worker
- **ownership**: `src/database/db_authorization.c`, `tests/unit/test_t7_285.c`, `tests/unit/cmake/eng/t7_285.cmake`, `quality/evidence/T7.285.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-bbeef7abfa51d073
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.285 --regex '^test_t7_285$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.285`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.286: redundantInitialization in recording_path.c
- **depends_on**: [T7, T13, T7.157, T27.1]
- **agent_type**: worker
- **ownership**: `src/video/recording_path.c`, `tests/unit/test_t7_286.c`, `tests/unit/cmake/eng/t7_286.cmake`, `quality/evidence/T7.286.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-0be349a4e3fc0518
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.286 --regex '^test_t7_286$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.286`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.287: redundantPointerOp in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_287-upstream.json`, `quality/evidence/T7.287.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-8f929a70e127f155
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_287-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.287`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.288: shadowFunction in db_storage_targets.c
- **depends_on**: [T7, T13, T7.277]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_targets.c`, `tests/unit/test_t7_288.c`, `tests/unit/cmake/eng/t7_288.cmake`, `quality/evidence/T7.288.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-147fba26f7ae351b
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.288 --regex '^test_t7_288$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.288`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.289: shadowFunction in stream_manager.c
- **depends_on**: [T7, T13, T7.62]
- **agent_type**: worker
- **ownership**: `src/video/stream_manager.c`, `tests/unit/test_t7_289.c`, `tests/unit/cmake/eng/t7_289.cmake`, `quality/evidence/T7.289.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-05b22ae697eaa4ca
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.289 --regex '^test_t7_289$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.289`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.290: shadowVariable in config.h
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `include/core/config.h`, `tests/unit/test_t7_290.c`, `tests/unit/cmake/eng/t7_290.cmake`, `quality/evidence/T7.290.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9d2c058ee958f85a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.290 --regex '^test_t7_290$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.290`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.291: unpreciseMathCall in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_291-upstream.json`, `quality/evidence/T7.291.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-426773bc9bc5a5d5
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_291-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.291`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.292: unreadVariable in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_292-upstream.json`, `quality/evidence/T7.292.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-eda9b7001501e847
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_292-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.292`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.293: unreadVariable in mqtt_client.c
- **depends_on**: [T7, T13, T7.29]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_client.c`, `tests/unit/test_t7_293.c`, `tests/unit/cmake/eng/t7_293.cmake`, `quality/evidence/T7.293.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-779ca0994d939da1
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.293 --regex '^test_t7_293$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.293`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.294: unreadVariable in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_294-upstream.json`, `quality/evidence/T7.294.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-93112ea9a6a3c10d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_294-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.294`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.295: unreadVariable in onvif_ptz.c
- **depends_on**: [T7, T13, T7.218]
- **agent_type**: worker
- **ownership**: `src/video/onvif_ptz.c`, `tests/unit/test_t7_295.c`, `tests/unit/cmake/eng/t7_295.cmake`, `quality/evidence/T7.295.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b1b17a4b9a2646a0
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.295 --regex '^test_t7_295$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.295`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.296: unsignedLessThanZero in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_296-upstream.json`, `quality/evidence/T7.296.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-a4ae78869ec196b6
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_296-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.296`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.297: unusedStructMember in mqtt_destination_client.c
- **depends_on**: [T7, T13, T7.224]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_destination_client.c`, `tests/unit/test_t7_297.c`, `tests/unit/cmake/eng/t7_297.cmake`, `quality/evidence/T7.297.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-efd7988ca87c95ed
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.297 --regex '^test_t7_297$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.297`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.298: unusedStructMember in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_298-upstream.json`, `quality/evidence/T7.298.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-49117790281d294c
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_298-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.298`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.299: unusedStructMember in sod_detection.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/sod_detection.c`, `tests/unit/test_t7_299.c`, `tests/unit/cmake/eng/t7_299.cmake`, `quality/evidence/T7.299.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-a12a87ba590d2536
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.299 --regex '^test_t7_299$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.299`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.300: uselessAssignmentArg in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_300-upstream.json`, `quality/evidence/T7.300.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-56c122b53a2e50cb
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_300-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.300`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.301: variableScope in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_301-upstream.json`, `quality/evidence/T7.301.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-ae4dbf08b9cfd1fe, CPPCHECK-BUCKET-b9ff6009cfa26f69
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_301-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.301`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.302: variableScope in db_detection_engines.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_detection_engines.c`, `tests/unit/test_t7_302.c`, `tests/unit/cmake/eng/t7_302.cmake`, `quality/evidence/T7.302.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b614f3b94230fe9a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.302 --regex '^test_t7_302$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.302`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.303: variableScope in db_lpr_reads.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_lpr_reads.c`, `tests/unit/test_t7_303.c`, `tests/unit/cmake/eng/t7_303.cmake`, `quality/evidence/T7.303.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-664e70c550c3ef36
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.303 --regex '^test_t7_303$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.303`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.304: variableScope in db_recordings.c
- **depends_on**: [T7, T13, T29.2]
- **agent_type**: worker
- **ownership**: `src/database/db_recordings.c`, `tests/unit/test_t7_304.c`, `tests/unit/cmake/eng/t7_304.cmake`, `quality/evidence/T7.304.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-2afdb0b1f1146eeb
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.304 --regex '^test_t7_304$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.304`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.305: variableScope in ezxml.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_305-upstream.json`, `quality/evidence/T7.305.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-96d5b13210206feb
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_305-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.305`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.306: variableScope in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_306-upstream.json`, `quality/evidence/T7.306.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-50bc6e286ac69abd
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_306-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.306`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.307: variableScope in storage_deletion.c
- **depends_on**: [T7, T13, T7.252, T28.1, T29.3]
- **agent_type**: worker
- **ownership**: `src/storage/storage_deletion.c`, `tests/unit/test_t7_307.c`, `tests/unit/cmake/eng/t7_307.cmake`, `quality/evidence/T7.307.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-89a47a59067748c1
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.307 --regex '^test_t7_307$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.307`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.308: variableScope in storage_migration.c
- **depends_on**: [T7, T13, T7.211]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_308.c`, `tests/unit/cmake/eng/t7_308.cmake`, `quality/evidence/T7.308.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-da5006735d5c5165
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.308 --regex '^test_t7_308$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.308`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.309: variableScope in storage_placement.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_placement.c`, `tests/unit/test_t7_309.c`, `tests/unit/cmake/eng/t7_309.cmake`, `quality/evidence/T7.309.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-51dd1a644803f48d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.309 --regex '^test_t7_309$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.309`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.310: variableScope in storage_s3.c
- **depends_on**: [T7, T13, T7.142]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_310.c`, `tests/unit/cmake/eng/t7_310.cmake`, `quality/evidence/T7.310.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-a3e8d56a93d61112
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.310 --regex '^test_t7_310$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.310`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.311: variableScope in linux_cgroup.c, linux_thermal.c
- **depends_on**: [T7, T13, T7.228, T7.254]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_cgroup.c`, `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_311.c`, `tests/unit/cmake/eng/t7_311.cmake`, `quality/evidence/T7.311.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-771b3fde1937be8a, CPPCHECK-BUCKET-e4dae70b5d1bd770
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.311 --regex '^test_t7_311$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.311`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.312: variableScope in stream_metrics.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/stream_metrics.c`, `tests/unit/test_t7_312.c`, `tests/unit/cmake/eng/t7_312.cmake`, `quality/evidence/T7.312.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9cc73ce911540ad4
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.312 --regex '^test_t7_312$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.312`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.313: variableScope in api_detection.c
- **depends_on**: [T7, T13, T7.213]
- **agent_type**: worker
- **ownership**: `src/video/api_detection.c`, `tests/unit/test_t7_313.c`, `tests/unit/cmake/eng/t7_313.cmake`, `quality/evidence/T7.313.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-905555e786df0459
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.313 --regex '^test_t7_313$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.313`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.314: variableScope in hls_directory.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_directory.c`, `tests/unit/test_t7_314.c`, `tests/unit/cmake/eng/t7_314.cmake`, `quality/evidence/T7.314.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-5d0aacd6263110c5
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.314 --regex '^test_t7_314$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.314`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.315: variableScope in recording_transcode.c
- **depends_on**: [T7, T13, T7.66]
- **agent_type**: worker
- **ownership**: `src/video/recording_transcode.c`, `tests/unit/test_t7_315.c`, `tests/unit/cmake/eng/t7_315.cmake`, `quality/evidence/T7.315.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-ec17ee32f6d27cdb
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.315 --regex '^test_t7_315$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.315`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.316: variableScope in unified_detection_thread.c
- **depends_on**: [T7, T13, T7.235]
- **agent_type**: worker
- **ownership**: `src/video/unified_detection_thread.c`, `tests/unit/test_t7_316.c`, `tests/unit/cmake/eng/t7_316.cmake`, `quality/evidence/T7.316.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d4393cc822a0c2d3
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.316 --regex '^test_t7_316$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.316`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.317: Authenticate before exposing stream existence
- **depends_on**: [T7, T11, T13, T7.64, T7.50, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_317.api.spec.ts`, `quality/evidence/T7.317.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-01
- **acceptance_criteria**:
  - Unauthenticated known and unknown names both return the authentication failure; authenticated unauthorized users retain intentional policy response without leaking privileged data.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.317 --project api --grep @t7_317`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.318: Correct documented Streams wire examples
- **depends_on**: [T7, T11, T7.319, T7.320, T7.321, T7.322, T7.323, T7.324, T7.325, T7.326, T7.327, T7.328, T7.329, T7.330, T7.331, T7.332, T7.333]
- **agent_type**: worker
- **ownership**: `docs/API.md`, `quality/evidence/T7.318.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Update documentation only after the wire behavior is reconciled.
- **findings**: F-T11-02
- **acceptance_criteria**:
  - Examples use the real array/page envelopes, integer percentage threshold and emitted names/statuses; every example maps to the revised contract.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `python3 scripts/check_docs_links.py`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.319: Remove query credentials from viewer URLs
- **depends_on**: [T7, T11, T13, T7.317]
- **agent_type**: worker
- **ownership**: `src/core/url_utils.c`, `src/web/api_handlers_streams_get.c`, `tests/integration/specs/t7_319.api.spec.ts`, `quality/evidence/T7.319.md`.
- **priority**: high
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-03
- **acceptance_criteria**:
  - Viewer responses contain neither userinfo nor sensitive query credentials, including encoded/mixed-case keys; configurers retain required edit data and benign query options remain usable.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.319 --project api --grep @t7_319`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.320: Reject non-object stream updates
- **depends_on**: [T7, T11, T13, T7.317, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_320.api.spec.ts`, `quality/evidence/T7.320.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-04
- **acceptance_criteria**:
  - null, arrays and primitives return 400 with no mutation, ONVIF request or reload; an empty object has explicit unchanged semantics.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.320 --project api --grep @t7_320`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.321: Reject wrong-typed stream fields
- **depends_on**: [T7, T11, T13, T7.320, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_321.api.spec.ts`, `quality/evidence/T7.321.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-05
- **acceptance_criteria**:
  - Wrong types return 400 with state unchanged; supported nullable clears retain their documented semantics and validation precedes all side effects.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.321 --project api --grep @t7_321`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.322: Validate bounded stream numeric fields
- **depends_on**: [T7, T11, T13, T7.321, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_322.api.spec.ts`, `quality/evidence/T7.322.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-06
- **acceptance_criteria**:
  - Protocol enum, priorities, interval, buffer, storage, PTZ and port limits reject out-of-range and fractional values before mutation; boundaries agree with storage and runtime callers.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.322 --project api --grep @t7_322`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.323: Validate complete boolean schedules
- **depends_on**: [T7, T11, T13, T7.322, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_323.api.spec.ts`, `quality/evidence/T7.323.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-07
- **acceptance_criteria**:
  - Only exactly 168 booleans are accepted; short/long/non-array or wrong element types fail atomically and valid week schedules round-trip unchanged.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.323 --project api --grep @t7_323`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.324: Address encoded slash names without aliasing
- **depends_on**: [T7, T11, T13, T7.319]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `tests/integration/specs/t7_324.api.spec.ts`, `quality/evidence/T7.324.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-08
- **acceptance_criteria**:
  - An encoded slash may either address the complete supported legacy name or be explicitly rejected; it must never select/authorize a different truncated camera.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.324 --project api --grep @t7_324`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.325: Enforce decoded stream name limits consistently
- **depends_on**: [T7, T11, T13, T7.324]
- **agent_type**: worker
- **ownership**: `src/web/request_response.c`, `src/web/api_handlers_streams_get.c`, `tests/integration/specs/t7_325.api.spec.ts`, `quality/evidence/T7.325.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-09
- **acceptance_criteria**:
  - UTF-8 names at the stored byte limit are consistently addressable by PUT/full/zones; encoded suffix length does not truncate/alias a name. Malformed escapes/NUL/dot segments remain rejected. Current encoded suffix includes /full or /zones.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.325 --project api --grep @t7_325`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.326: Handle missing fleet inventory rows consistently
- **depends_on**: [T7, T11, T13, T7.325]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `tests/integration/specs/t7_326.api.spec.ts`, `quality/evidence/T7.326.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-10
- **acceptance_criteria**:
  - A missing authorization inventory row cannot expose a camera; document fail-closed behavior and isolate an inconsistent row only if policy permits, while preserving authorized list results and surfacing the inconsistency.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.326 --project api --grep @t7_326`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.327: Preserve omitted ONVIF update state
- **depends_on**: [T7, T11, T13, T7.323, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_327.api.spec.ts`, `quality/evidence/T7.327.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-11
- **acceptance_criteria**:
  - A tags-only or empty update preserves isOnvif and credentials without an ONVIF network call; explicit ONVIF changes retain their intended behavior.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.327 --project api --grep @t7_327`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.328: Make privacy mutations atomic and report failure
- **depends_on**: [T7, T11, T13, T7.327, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_328.api.spec.ts`, `quality/evidence/T7.328.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-12
- **acceptance_criteria**:
  - Privacy-only success follows a successful database write; database failure returns failure. Mixed privacy/edit requests are rejected before enable/reload/network side effects or handled through a separately specified atomic contract.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.328 --project api --grep @t7_328`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.329: Expose demo mode for authenticated users
- **depends_on**: [T7, T11, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_auth_backend_agnostic.c`, `tests/integration/specs/t7_329.api.spec.ts`, `quality/evidence/T7.329.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-13
- **acceptance_criteria**:
  - Authenticated demo responses carry authoritative demo state while roles remain accurate, server-side redaction remains enforced and normal authenticated responses remain compatible.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.329 --project api --grep @t7_329`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.330: Reject overlong stream strings without truncation
- **depends_on**: [T7, T11, T13, T7.328, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_330.api.spec.ts`, `quality/evidence/T7.330.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-14
- **acceptance_criteria**:
  - Each field checks UTF-8 byte length against the destination buffer before mutation; maximum valid values round-trip and one-byte overflow is rejected atomically, including detection_model temporary-buffer limits.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.330 --project api --grep @t7_330`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.331: Bound ONVIF edit validation latency
- **depends_on**: [T7, T11, T13, T7.330, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_331.api.spec.ts`, `quality/evidence/T7.331.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-15
- **acceptance_criteria**:
  - An unresponsive synthetic ONVIF endpoint cannot monopolize a worker indefinitely; preserve accepted/persisted semantics and avoid blind retries of possibly successful mutations. Record the deadline before the test, below the client timeout or explicitly reconcile client behavior.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.331 --project api --grep @t7_331`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.332: Preserve detection threshold round trips
- **depends_on**: [T7, T11, T13, T7.326, T7.331, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_332.api.spec.ts`, `quality/evidence/T7.332.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-16
- **acceptance_criteria**:
  - All integer percentages 0..100 survive PUT→GET round trips; specifically 53 and 59 must not become 52 and 58. Preserve representation for existing stored floats with a documented rounding policy.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.332 --project api --grep @t7_332`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.333: Specify password omission and explicit clear semantics
- **depends_on**: [T7, T11, T13, T7.332, T13.3, T13.4]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_333.api.spec.ts`, `quality/evidence/T7.333.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-17
- **acceptance_criteria**:
  - Omitted onvif_password leaves the secret unchanged; explicit empty clears only as an intentional authorized action. Client form construction never echoes a redacted empty value as a destructive update.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.333 --project api --grep @t7_333`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.334: Reconcile Streams contracts after behavioral fixes
- **depends_on**: [T7, T12, T7.317, T7.318, T7.319, T7.320, T7.321, T7.322, T7.323, T7.324, T7.325, T7.326, T7.327, T7.328, T7.329, T7.330, T7.331, T7.332, T7.333]
- **agent_type**: local
- **ownership**: `docs/api/contracts/streams.yaml`, `docs/api/contracts/common.yaml`, `docs/api/openapi.yaml`, `tests/contracts/fixtures/streams/auth-verify.auth-disabled.valid.json`, `tests/contracts/fixtures/streams/auth-verify.demo-as-admin.invalid.json`, `tests/contracts/fixtures/streams/auth-verify.demo.valid.json`, `tests/contracts/fixtures/streams/auth-verify.user.valid.json`, `tests/contracts/fixtures/streams/client-config.camelcase-only.invalid.json`, `tests/contracts/fixtures/streams/client-config.valid.json`, `tests/contracts/fixtures/streams/detection-models.403-viewer.valid.json`, `tests/contracts/fixtures/streams/detection-models.no-directory.valid.json`, `tests/contracts/fixtures/streams/detection-models.unsupported-without-reason.invalid.json`, `tests/contracts/fixtures/streams/detection-models.valid.json`, `tests/contracts/fixtures/streams/error.400-retention.valid.json`, `tests/contracts/fixtures/streams/error.400-surface.valid.json`, `tests/contracts/fixtures/streams/error.401.valid.json`, `tests/contracts/fixtures/streams/error.403-put-viewer.valid.json`, `tests/contracts/fixtures/streams/error.404-camera.valid.json`, `tests/contracts/fixtures/streams/error.404-stream.valid.json`, `tests/contracts/fixtures/streams/error.413-publish-url.valid.json`, `tests/contracts/fixtures/streams/error.empty-message.invalid.json`, `tests/contracts/fixtures/streams/error.message-key.invalid.json`, `tests/contracts/fixtures/streams/full.any.generic-schema.valid.json`, `tests/contracts/fixtures/streams/full.configurer.legacy-name-plain-credentials.valid.json`, `tests/contracts/fixtures/streams/full.configurer.missing-onvif-password.invalid.json`, `tests/contracts/fixtures/streams/full.configurer.onvif-url-userinfo.invalid.json`, `tests/contracts/fixtures/streams/full.configurer.onvif.valid.json`, `tests/contracts/fixtures/streams/full.error-message-without-error.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.claims-configure.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.error-status.valid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-admin-url.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-detection-url.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-go2rtc-override.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-onvif-password.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-onvif-username.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-publish-url.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-sub-stream-url.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.redacted.valid.json`, `tests/contracts/fixtures/streams/full.live-only.sensitive-keys-absent.valid.json`, `tests/contracts/fixtures/streams/full.live-only.url-query-secret.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.url-userinfo.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.url-username-only.invalid.json`, `tests/contracts/fixtures/streams/full.null-tags.invalid.json`, `tests/contracts/fixtures/streams/full.protocol-out-of-enum.invalid.json`, `tests/contracts/fixtures/streams/full.retention-below-inherit.invalid.json`, `tests/contracts/fixtures/streams/full.retention-over-max.invalid.json`, `tests/contracts/fixtures/streams/full.schedule-167-entries.invalid.json`, `tests/contracts/fixtures/streams/full.status-docs-value.invalid.json`, `tests/contracts/fixtures/streams/full.threshold-as-fraction.invalid.json`, `tests/contracts/fixtures/streams/full.unwrapped-object.invalid.json`, `tests/contracts/fixtures/streams/index.json`, `tests/contracts/fixtures/streams/path-encoding.cases.json`, `tests/contracts/fixtures/streams/put-response.200-privacy.valid.json`, `tests/contracts/fixtures/streams/put-response.202-onvif-failed.valid.json`, `tests/contracts/fixtures/streams/put-response.202-onvif-status-without-message.invalid.json`, `tests/contracts/fixtures/streams/put-response.202.valid.json`, `tests/contracts/fixtures/streams/put.clear-with-null.valid.json`, `tests/contracts/fixtures/streams/put.detection-retention-minus-2.invalid.json`, `tests/contracts/fixtures/streams/put.detection-url-file-scheme.invalid.json`, `tests/contracts/fixtures/streams/put.detection-url-too-long.invalid.json`, `tests/contracts/fixtures/streams/put.edit-form.valid.json`, `tests/contracts/fixtures/streams/put.empty-object.valid.json`, `tests/contracts/fixtures/streams/put.eptz-not-string.invalid.json`, `tests/contracts/fixtures/streams/put.eptz-wrong-projection.invalid.json`, `tests/contracts/fixtures/streams/put.is-deleted.invalid.json`, `tests/contracts/fixtures/streams/put.name-in-body.invalid.json`, `tests/contracts/fixtures/streams/put.playback-transport-bogus.invalid.json`, `tests/contracts/fixtures/streams/put.privacy-mixed-with-edit.invalid.json`, `tests/contracts/fixtures/streams/put.protocol-out-of-enum.invalid.json`, `tests/contracts/fixtures/streams/put.publish-and-detection-url.valid.json`, `tests/contracts/fixtures/streams/put.publish-url-http.invalid.json`, `tests/contracts/fixtures/streams/put.publish-url-quote.invalid.json`, `tests/contracts/fixtures/streams/put.record-as-string.invalid.json`, `tests/contracts/fixtures/streams/put.retention-366.invalid.json`, `tests/contracts/fixtures/streams/put.retention-finite-boundary.valid.json`, `tests/contracts/fixtures/streams/put.retention-tristate.valid.json`, `tests/contracts/fixtures/streams/put.schedule-weekday.valid.json`, `tests/contracts/fixtures/streams/put.schedule-wrong-length.invalid.json`, `tests/contracts/fixtures/streams/put.set-privacy-mode.valid.json`, `tests/contracts/fixtures/streams/put.tags-too-long.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.admin-url-without-configure.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.configurer-with-admin-url.valid.json`, `tests/contracts/fixtures/streams/summary.admin.contains-url.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.empty.valid.json`, `tests/contracts/fixtures/streams/summary.admin.missing-can-configure.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.page-beyond-end.valid.json`, `tests/contracts/fixtures/streams/summary.admin.page-size-over-max.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.status-from-detail-enum.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.viewer.valid.json`, `tests/contracts/fixtures/streams/summary.live.contains-admin-url.invalid.json`, `tests/contracts/fixtures/streams/summary.live.contains-onvif-password.invalid.json`, `tests/contracts/fixtures/streams/summary.live.viewer.valid.json`, `tests/contracts/fixtures/streams/summary.lpr.leaks-location.invalid.json`, `tests/contracts/fixtures/streams/summary.lpr.valid.json`, `tests/contracts/fixtures/streams/zones.empty.valid.json`, `tests/contracts/fixtures/streams/zones.valid.json`, `web/js/api/generated/`, `tests/contracts/generated/`, `quality/evidence/T7.334.md`.
- **priority**: high
- **description**: Under the exclusive generation lease, reconcile the exact schemas and affected fixture files after the preceding fixes. The exact fixture list is recorded in ownership; generated directories are exclusively owned outputs of T12. Regenerate only through T12 tooling; no concurrent generation. Preserve separate desired-client and current-wire cases where intentionally different.
- **findings**: policy/integration requirement
- **acceptance_criteria**:
  - All valid and invalid fixtures retain explicit reasons and agree with the corrected real backend.
  - Generate deterministic artifacts and run real pilot regressions; T18/T19 cannot use stale contracts.
- **validation**:
  - `node scripts/quality/contracts.mjs --generate`
  - `node scripts/quality/contracts.mjs --check`
  - `node --test tests/quality/contracts.test.mjs`
  - `npm --prefix web run typecheck`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.335: Resolve proposed ownership and exception review records
- **depends_on**: [T7, T7.1]
- **agent_type**: local
- **ownership**: `quality/reviews/exception-ownership.json`, `quality/evidence/T7.335.md`.
- **priority**: high
- **description**: Collect accountable owners and explicit approvals/denials for T1/T6 proposals and any new residual exceptions. Existing maintainers/<area> strings and proposed dates are not acceptance. Prepare a concrete review record; leave pending entries unresolved until actual authorized reviewer input arrives.
- **findings**: policy/integration requirement
- **acceptance_criteria**:
  - Every unresolved proposal names its exact finding/gap, requested owner, exposure, rationale, proposed expiry and approval state.
  - No approval, repository protection change, CODEOWNERS assignment or publication is inferred from implementation authorization.
- **validation**:
  - `python3 -m json.tool quality/reviews/exception-ownership.json`
- **status**: Blocked — review prepared, awaiting compatible fix or maintainer decision
- **log**: Concrete review packet committed in `713b3025`; no package mutation, approval or disposition change. Current development-tool advisory ranges and dependency parents checked; production web audit zero. See task evidence and `quality/reviews/exception-ownership.json`.
- **files edited/created**:

## Expanded test typing tasks (T9)

### T9.1: Repair load-test and page-helper type boundaries
- **depends_on**: [T9]
- **agent_type**: worker
- **ownership**: `tests/load/config.ts`, `tests/load/reporter.ts`, `tests/load/runner.ts`, `tests/integration/pages/StreamsPage.ts`, `tests/integration/pages/TimelinePage.ts`, `quality/evidence/T9.1.md`.
- **description**: Repair three array-index return types, two unknown caught errors, and date extraction. Use checked existence/narrowing, preserve behavior for valid inputs, and reject impossible empty selections explicitly. No load execution or test removal.
- **acceptance_criteria**: All six initial diagnostics in these five files disappear without broad declarations, ts-ignore, or new `any`; valid CLI/scenario/report/page behavior stays intact.
- **validation**: `npm run typecheck:tests` (initial 38 diagnostics; this cluster must reach zero while unrelated clusters remain RED); Node assertions for parseArgs valid/missing values and zero-request runLoadTest without network; final whole-config GREEN owned by T9.4.
- **status**: Completed (commit `d36e4e65`)
- **log**: Six cluster diagnostics repaired with checked values and narrowed errors; no load traffic.


### T9.2: Type media DOM and mocked routes
- **depends_on**: [T9.1]
- **agent_type**: worker
- **ownership**: `tests/integration/specs/investigation.ui.spec.ts`, `tests/integration/specs/recordings.ui.spec.ts`, `tests/integration/specs/streams.ui.spec.ts`, `tests/integration/specs/timeline.ui.spec.ts`, `quality/evidence/T9.2.md`.
- **description**: Narrow selected video elements to the HTMLVideoElement contract; retain controls/readyState/play/pause assertions. Type mocked route setup parameters as Playwright Page so handlers infer Route. Use fixed ISO date slicing for non-optional date strings. Handle controlsList as a narrow browser capability (DOMTokenList) because it is not part of TypeScript's portable HTMLMediaElement interface.
- **acceptance_criteria**: All 23 original diagnostics across these four files disappear; all runtime test definitions/assertions remain present; no ambient browser-wide `any` added.
- **validation**: `npm run typecheck:tests` with zero cluster errors; compare Playwright --list test identities to baseline (ignore moved line numbers); final full GREEN in T9.4.
- **status**: Completed (commit `90f26a8d`)
- **log**: 23 media DOM/route diagnostics repaired across four files without changing test identities.


### T9.3: Preserve timeline/snapshot fixture bounds
- **depends_on**: [T9.2]
- **agent_type**: worker
- **ownership**: `tests/integration/specs/timeline-boundary.ui.spec.ts`, `tests/integration/specs/stream-snapshot-requests.ui.spec.ts`, `quality/evidence/T9.3.md`.
- **description**: Validate parsed local date/time components and nonempty segment fixtures; narrow fixed fixture entries and captured URLs through explicit checks or known tuple definitions. Preserve existing route responses and snapshot/timeline assertions, rejecting malformed fixture setup instead of changing tests to pass.
- **acceptance_criteria**: All eight original diagnostics in these two files disappear without optional-default values hiding missing test data; test identities remain unchanged.
- **validation**: `npm run typecheck:tests` with zero cluster errors; Playwright --list identities unchanged; final full GREEN in T9.4.
- **status**: Completed (commit `688eb332`)
- **log**: Eight timeline/snapshot diagnostics repaired with explicit fixture bounds and tuple types.


### T9.4: Integrate the legacy JS test boundary and verify coverage
- **depends_on**: [T9.3]
- **agent_type**: local
- **ownership**: `tests/tsconfig.json`, `tests/integration/recordings-url-utils.ui.spec.ts`, optional narrowly scoped test interop helper under `tests/integration/fixtures/`, `quality/evidence/T9.4.md`, `quality/evidence/T9.md`.
- **description**: Resolve the existing recordings JS import without wildcard-any modules or hiding test files. Keep the exact legacy JS boundary explicit and runtime-checked where values cross into the test. Any incremental allowJs/checkJs setting must inventory the exact imported application files as still unchecked and assign retirement to T34; it is not migration completion. Preserve the four existing URL utility test definitions and assertions. Reconcile compiler coverage and discovery across all clusters.
- **acceptance_criteria**: Root typecheck is GREEN for all existing integration/load TS and configuration; no unchecked test file/excluded diagnostic; 213 Playwright tests in 28 files remain discovered; no load traffic sent. Legacy application JS remains explicitly accounted for in evidence.
- **validation**: `npm run typecheck:tests`; `npx playwright test --list --reporter=list`; inspect `tsc --listFilesOnly -p tests/tsconfig.json` against `rg --files tests/integration tests/load`; run the four URL utility tests with a temporary isolated Playwright config disabling global setup/teardown (they need neither browser launch nor server); all results captured under `build/eng-artifacts/T9/`.
- **status**: Completed (commit `b0bcf4c2`)

- **log**: Root strict typecheck GREEN; all 48 TS files included, 213 Playwright identities preserved, four existing URL utility tests pass without server/browser/load. Three legacy JS dependencies explicitly inventoried as unchecked.

Coordinator verified this serial task chain and inserted T9.4 into T13/T15 before child edits.

## Expanded C boundary tasks (T31)


### T31.1: Reproduce stream-state mutex disposal imbalance
- **depends_on**: [T31, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_stream_state_disposal.c`, `tests/unit/cmake/eng/stream_state_disposal.cmake`, `quality/evidence/T31.1.md`.
- **description**: Wrap pthread_mutex_destroy in a focused native test and track only the two mutex addresses of created states. Test normal removal, extra-ref refusal, global shutdown, repeated shutdown and repeated lifecycle cycles. Also reproduce registry lookup from a wrapped stop callback in a child process with a bounded deadline; verify teardown rejects create/remove/recursive shutdown during the unlocked callback. Add registration through T13's fragment seam. Do not modify production code in this task.
- **acceptance_criteria**: The original shutdown path fails because state_mutex is not destroyed and an active-state stop callback deadlocks when looking up the registry; normal removal and refusal assertions establish unchanged preconditions. No live stream or remote server is used.
- **validation**: `python3 scripts/quality/run-native.py --task T31.1 --regex '^test_stream_state_disposal$'` must fail for the expected missing destruction, with exact RED output retained.
- **status**: Completed (commits `172d5fb6`, `a3074ea3`)
- **log**: Five focused regressions reproduced missing mutex destruction and bounded callback-reentry deadlock before the fix; normal removal/refusal controls retained. Ignored CMake fragment explicitly tracked.
- **files edited/created**: `tests/unit/test_stream_state_disposal.c`, `tests/unit/cmake/eng/stream_state_disposal.cmake`, `quality/evidence/T31.1.md`

### T31.2: Unify final disposal of stream-state resources
- **depends_on**: [T31.1]
- **agent_type**: worker
- **ownership**: `src/video/stream_state.c`, `quality/evidence/T31.2.md`.
- **description**: Introduce a private final-disposal helper destroying both initialized per-state mutexes before freeing. Global shutdown must release the registry mutex around existing stop callbacks, with a lock-protected teardown guard rejecting registry create/remove/recursive shutdown while ownership is handed off. Reacquire the registry lock before final disposal. Document quiescence/lock preconditions; keep public API, stop ordering, reference checks and persisted state unchanged.
- **acceptance_criteria**: T31.1 regressions pass; every final-disposal caller uses the helper and no duplicate destroy is added. No general concurrency redesign or new allocation is introduced; stop callbacks can safely reenter registry lookup during teardown.
- **validation**: `python3 scripts/quality/run-native.py --task T31.2 --regex '^test_stream_(state_disposal|state|manager)$'`.
- **status**: Completed (commit `9a7e6188`)
- **log**: Shared final disposal destroys both mutexes; guarded shutdown releases the registry lock around stop callbacks. Five new regressions and 34 related Unity tests pass under ASan/UBSan.
- **files edited/created**: `src/video/stream_state.c`, `quality/evidence/T31.2.md`

### T31.3: Verify resource-disposal regression under sanitizers
- **depends_on**: [T31.2]
- **agent_type**: local
- **ownership**: `quality/evidence/T31.3.md`.
- **description**: Rerun the focused disposal/state/manager tests in the isolated native runner and its ASan/UBSan profile; record exact toolchain, revision and flags. Repeat the T13 go2rtc API smoke on the rebuilt integration profile and require graceful server/child cleanup within the existing deadline. Assert repeated-cycle success and no diagnostics. Physical device budgets remain deferred and are not replaced by sanitizer results.
- **acceptance_criteria**: Functional and ASan/UBSan runs pass with nonzero test discovery, no invalid/double destroy observations, and no sanitizer errors; the reproduced go2rtc smoke exits gracefully without fallback kill. No runtime performance claim is made.
- **validation**: `python3 scripts/quality/run-native.py --task T31.3 --regex '^test_stream_(state_disposal|state|manager)$'`; run the same selection with the sanitizer option produced/documented by T13.
- **status**: Completed (commit `00f9394e`)
- **log**: Committed production revision passes 34 tests in both Release and ASan/UBSan. Exact four-case real go2rtc reproduction passes and exits gracefully in 3647 ms without forced parent/child cleanup; physical calibration remains deferred.
- **files edited/created**: `quality/evidence/T31.3.md`

## Expanded lint compatibility tasks (T10)

### T10.1: Supply attribute translation keys and regression fixtures
- **depends_on**: [T10]
- **agent_type**: worker
- **ownership**: `web/public/locales/en.json`, `tests/quality/localized-attributes.test.mjs`, `quality/evidence/T10.1.md`.
- **description**: Add explicit English keys for newly exposed placeholder examples, plan labels/shortcuts, recording thumbnail/count and tag-removal labels. Reuse existing settings.tabsLabel, recordings.download/protect/unprotect keys. Test real i18n interpolation and English fallback for a partial locale; do not translate protocol examples or add fabricated translations to other catalogs.
- **acceptance_criteria**:
  - Required source keys exist and interpolate variable values, singular/plural counts, shortcut labels and multiline examples without raw key names or dropped parameters.
  - Existing English output is preserved except the intentional reuse of the more descriptive existing Protect/Unprotect recording labels.
  - No i18n literal allowance is increased; non-English partial-catalog fallback follows existing behavior.
- **validation**:
  - `node --test tests/quality/localized-attributes.test.mjs`
  - `npm --prefix web test -- --runInBand --testPathPatterns='i18n|locale-coverage'`
- **status**: Completed (commit `a69d3bad`, `6591c330`)
- **log**: Three real interpolation/fallback tests pass after RED; 14 English keys with existing partial-locale fallback, preserving translated Undo/Redo action names.
- **files edited/created**:

### T10.2: Localize settings and stream editor attributes
- **depends_on**: [T10.1]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/SettingsView.jsx`, `web/js/components/preact/StreamConfigModal.jsx`, `web/js/components/preact/settings/DetectionTab.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/js/components/preact/settings/StorageTab.jsx`, `quality/evidence/T10.2.md`.
- **description**: Replace the nine newly exposed literals in five settings files with T10.1 keys. Remove redundant English fallback for the existing settings.tabsLabel key. Preserve dynamic bucket interpolation and full multiline URL/config examples, handlers, field values and disabled-state logic.
- **acceptance_criteria**:
  - All reported attribute literals in these five files pass the strengthened i18n rule with unchanged baseline counts.
  - Example URL schemes, credentials-as-examples, protocol options, whitespace/newlines and storage bucket interpolation remain accurate.
  - No strings are moved to helper constants merely to evade AST inspection.
- **validation**:
  - `node --test tests/quality/localized-attributes.test.mjs`
  - `(cd web && node_modules/.bin/eslint js/components/preact/SettingsView.jsx js/components/preact/StreamConfigModal.jsx js/components/preact/settings/DetectionTab.jsx js/components/preact/settings/Go2rtcTab.jsx js/components/preact/settings/StorageTab.jsx)`
  - `npm --prefix web test -- --runInBand --testPathPatterns='i18n|locale-coverage|go2rtcSettings'`
- **status**: Completed (commit `f2b93b68`)
- **log**: Five settings/editor files localized; scoped lint zero errors, 41 relevant tests pass.
- **files edited/created**:

### T10.3: Localize live and recording attributes and close lint integration
- **depends_on**: [T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/js/components/preact/live/PlanSketchControls.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/recordings/RecordingsTable.jsx`, `web/js/components/preact/recordings/TagsOverlay.jsx`, `quality/evidence/T10.3.md`.
- **description**: Replace remaining newly detected attribute literals with T10.1 keys and existing download/protection keys. Obtain i18n in TagChip at component top level, preserve callback and focus behavior, and preserve count/singular/plural and filename/size interpolation. Run final whole-tree lint and compatibility tests after serial source fixes.
- **acceptance_criteria**:
  - No new i18n errors remain; the 897 existing accessibility warnings stay explicit rather than being suppressed or rebaselined.
  - Labels retain the stream/plan/tag/count/size values and shortcut information; existing action handlers remain unchanged.
  - T10 compatibility tests and relevant localization/live/recording regressions pass; baseline JSON bytes remain unchanged.
- **validation**:
  - `node --test tests/quality/lint-compat.test.mjs tests/quality/localized-attributes.test.mjs`
  - `npm --prefix web run lint`
  - `npm --prefix web test -- --runInBand --testPathPatterns='i18n|locale-coverage|planSketch|buildingPlan|recording|tag'`
- **status**: Completed (commit `4f82c1e2`)
- **log**: Five live/recording files localized; 31 focused tests plus all 571 frontend tests pass, lint zero errors/897 unchanged warnings, baseline byte-identical.
- **files edited/created**:

## Expanded scanner integration tasks (T14)

### T14.1: Capture scanner provenance and enforce explicit lane scope
- **depends_on**: [T6, T8, T7.1]
- **agent_type**: local
- **ownership**: `scripts/quality/scanner_run.py`, `scripts/quality/findings_lane.py`, manifest validation only in `scripts/quality/findings.py`, `tests/quality/test_scanner_ci.py`, manifest negative cases in `tests/quality/test_findings.py`, `quality/evidence/T14.1.md`.
- **description**: Capture real subprocess exit/status, tool version, nonempty actual scanned-file evidence, report hashes and revision in a run manifest. Add explicitly scoped per-lane evaluation announcing all omitted scanners; it cannot substitute for the full expected scanner coverage gate. Harden required manifest validation without changing approval policy, baseline-writing authorization, or scanner normalization. T14 switches the required-manifest policy when integrated.
- **acceptance_criteria**:
  - Missing, malformed, stale, wrong-revision or tampered required provenance fails closed; crashed commands cannot become clean reports.
  - Clean/new-finding/scanner-error fixtures exercise the actual subprocess and checker chain. Normal checks leave baseline untouched.
  - Candidate source inventory is labeled as such and cannot masquerade as actual CodeQL scan evidence; absent evidence remains missing.
  - Lane results are explicitly partial with omitted expected scanner identities; the global check still requires all expected scanners.
- **validation**:
  - `python3 -m unittest discover -s tests/quality -p test_scanner_ci.py`
  - `python3 -m unittest discover -s tests/quality -p test_findings.py`
  - Exercise clean/new/error lane commands against isolated scanner fixtures and verify missing-global-scanner failure.
- **status**: Completed (commit `7a304140`)
- **log**: 22 subprocess/provenance/partial-lane tests and 69 findings tests pass. Required manifests fail closed for missing, stale, tampered and wrong-revision evidence; lanes explicitly list omitted scanners and never claim global coverage.
- **files edited/created**: `scripts/quality/scanner_run.py`, `scripts/quality/findings_lane.py`, `scripts/quality/findings.py`, `tests/quality/test_scanner_ci.py`, `tests/quality/test_findings.py`, `quality/evidence/T14.1.md`

## Expanded API baseline corrections (T13)

### T13.1: Verify policy-user self-update denials without side effects
- **depends_on**: [T13]
- **agent_type**: worker
- **ownership**: `tests/integration/specs/auth.api.spec.ts`, `quality/evidence/T13.1.md`.
- **description**: Correct the existing policy-user role-change expectation to the actual exact 400 rejection (policy metadata validation precedes self-permission checks). Independently assert exact 403 for prohibited self activation/password/CIDR changes. Snapshot relevant user/grant state before/after, preserve working original credentials, and verify no admin access is gained. No production authorization change.
- **acceptance_criteria**: Tests discriminate invalid role metadata from forbidden mutations, assert unchanged persisted user/grants, and preserve existing coverage; no broad success/error status list is used.
- **validation**: `npm run typecheck:tests`; `node scripts/quality/run-integration.mjs --task T13.1 --project api --grep 'Users API'`; original RED is retained in T13 full API evidence.
- **status**: Completed (commit `8f11a3a0`)
- **log**: Nine Users API tests pass with exact denial statuses and persisted/security invariants; original credentials remain valid and rejected changes confer no admin access. Root types pass.
- **files edited/created**:

### T13.2: Verify accepted stream disable and permanent deletion complete
- **depends_on**: [T13.1]
- **agent_type**: worker
- **ownership**: `tests/integration/specs/comprehensive.api.spec.ts`, `quality/evidence/T13.2.md`.
- **description**: Use self-contained enabled fixtures, assert exact 202 accepted responses and permanent:false for default soft deletion, then boundedly poll the DB-backed list for disabled state. Exercise permanent=true separately, require accepted response and poll disappearance/detail404. Preserve mutation assertions and cleanup; never treat a pre-disabled/missing fixture as successful async completion.
- **acceptance_criteria**: Both soft-disable and permanent-delete behavior are verified through completion with bounded polling; the old broad 200/204/404 list is replaced by the observed contract, not expanded. Full API suite passes or additional actual failures become explicit bounded tasks.
- **validation**: `npm run typecheck:tests`; `node scripts/quality/run-integration.mjs --task T13.2 --project api --grep @api`.
- **status**: Completed (commit `486a3a84`)
- **log**: Full API suite passes 48/48 with no skips. Enabled soft-delete fixture reaches disabled DB-backed state; permanent delete reaches list absence and detail 404. Real go2rtc teardown is graceful without forced cleanup.
- **files edited/created**:

### T13.3: Reject stream deletion when its worker cannot start
- **depends_on**: [T13.2]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/unit/test_delete_stream_launch_failure.c`, `tests/unit/cmake/eng/delete_stream_launch_failure.cmake`, `quality/evidence/T13.3.md`.
- **description**: Reproduce the source-inspected delete handler bug with a bounded pthread_create failure injection: it currently prepares a 202 accepted response then only logs worker creation failure. Return an explicit server error instead of false acceptance when launch fails, preserving successful asynchronous deletion and authorization ordering. Response fields are not transmitted until the handler returns; do not assume the misleading old comment is accurate.
- **acceptance_criteria**: RED test shows 202 on failed launch; GREEN rejects it with 5xx and unchanged persisted/operational stream, no leaked task or callback execution. Successful launch remains 202 and T13.2 completion tests pass. No deletion/auth redesign.
- **validation**: `python3 scripts/quality/run-native.py --task T13.3 --regex '^test_delete_stream_launch_failure$'`; rerun focused T13.2 deletion API tests against the rebuilt server.
- **status**: Completed (commits `cb8f0063`, `853c0d9c`, `a737c4f0`)
- **log**: Injected worker failure reproduced false202 before production fix. Final sanitizer3/3 and rebuilt deletion API2/2 pass; graceful teardown4450ms without forced cleanup.
- **files edited/created**:

### T13.4: Reject stream updates when their worker cannot start
- **depends_on**: [T13.3]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/comprehensive.api.spec.ts` existing PUT case only, `tests/unit/test_put_stream_launch_failure.c`, `tests/unit/cmake/eng/put_stream_launch_failure.cmake`, `quality/evidence/T13.4.md`.
- **description**: Reproduce the analogous PUT worker-creation failure discovered during T13.3: the handler prepares a 202 response and only logs pthread_create failure. Return a server error after disposing the task. Keep successful asynchronous updates and authorization behavior unchanged; source inspection must distinguish any existing synchronous side effects before claiming state invariance.
- **acceptance_criteria**: Tests first demonstrate false acceptance under injected launch failure; after fix return 5xx, free owned task resources once and do not execute the worker. Use a simple non-ONVIF update to assert persisted and runtime stream fields unchanged on rejection. Strengthen the existing log-only PUT API case to require initial enabled state, exact 202, bounded persisted completion and preserved unrelated fields. Authorization controls retain coverage. Report any prelaunch side effects as separate bounded findings rather than extending this task silently.
- **validation**: `python3 scripts/quality/run-native.py --task T13.4 --regex '^test_(put|delete)_stream_launch_failure$'`; run focused real API update/deletion checks against a rebuilt server.
- **status**: Completed (commits `43c65a16`, `2da05c1e`, `dc292a0c`)
- **log**: Injected PUT launch failure reproduced false202, now returns500 with owned task disposal. DELETE+PUT sanitizer5/5 and rebuilt Streams API14/14 pass, graceful4246ms. Existing prelaunch side effects remain separately mapped F-T11-11/-12/-15.
- **files edited/created**:

## Independent foundation command integration (T15)

### T15.1: Route existing local and CI integration commands through owned runners
- **depends_on**: [T13, T9.4, T10.3, T12]
- **agent_type**: local
- **ownership**: root `package.json` integration scripts only, `scripts/quality/run-integration.mjs` CLI/config/artifact compatibility, `tests/quality/test_runners.py`, `.github/workflows/integration-test.yml` existing Playwright invocations/artifacts/cleanup, `docs/TESTING.md` or an exact existing contributor test guide, `quality/evidence/T15.1.md`.
- **description**: Keep existing npm test/api/ui/go2rtc and workflow health/UI shard commands functional after T13 requires owned run context. Support explicit all-project selection, safe file/shard/grep filters, and discovery without starting a server. Preserve the known health/MQTT lane's feature intent via a bounded named configuration; never adopt arbitrary external production configuration. Replace old raw Playwright setup calls with the wrapper and the prebuilt binary. Retain per-run reports/logs and remove broad pkill/tmp deletion now handled by ownership-aware cleanup. This is command integration, not risk acceptance or full T15 qualification.
- **acceptance_criteria**:
  - Existing health/UI shard CI commands create isolated owner/config/PID context; root npm commands cannot bypass runner checks.
  - Safe CLI forwarding preserves requested test selection; zero-selected tests and discovery errors fail. Unknown dangerous configuration overrides are rejected.
  - Health/MQTT coverage is not silently disabled, UI shards stay distinct, and meaningful artifacts remain available after failures.
  - No exception approval, baseline update, dependency downgrade, remote push, or repository setting change occurs.
- **validation**:
  - `python3 -m unittest discover -s tests/quality -p test_runners.py`
  - `npm run typecheck:tests`
  - Run safe discovery for npm API/UI and both workflow shard selections; compare identities to pre-change discovery without starting load or servers.
  - Parse workflow YAML and inspect exact existing job changes; real remote CI remains pending until explicitly run.
- **status**: Completed (commit `3c15c0d7`)
- **log**: 22 runner tests and root TS pass; all 213 discovery identities preserved, including health files and UI shards. Existing npm/CI routes use owned runs with retained logs; no native runtime claim or exception approval.
- **files edited/created**:

### T15.2: Register independent foundation statuses and proposed enforcement settings
- **depends_on**: [T4, T9.4, T10.3, T12, T14.1, T15.1]
- **agent_type**: local
- **ownership**: `.github/workflows/engineering-foundation.yml`, `quality/required-checks.json`, `tests/quality/foundation-workflow.test.mjs`, `quality/evidence/T15.2.md`.
- **description**: Register separate type/lint/unit/schema/asset and quality-tooling CI statuses using existing commands. Run both asset profiles in isolated jobs and check without updating budgets or generated artifacts. Prepare concrete required-check identities and path behavior for T38; leave the future real-backend contract lane explicitly pending T39. No remote rules mutation and no dependency exception approval.
- **acceptance_criteria**: PR jobs have no path filters or conditional success bypass; failures propagate and diagnostics upload on failure. Only existing nonempty suites run. Proposed settings do not claim actual enforcement, full scanner coverage or migration completion; T15 acceptance remains gated by its existing dependencies.
- **validation**: Tests first reject missing workflow/identity mapping, omitted build profile, mutating check commands and skipped required jobs; execute registered local type/lint/unit/schema/tooling checks and both build/asset commands. No remote publication required.
- **status**: Completed (commit `caf23285`)
- **log**: Seven foundation jobs and concrete un-applied status mapping registered. Tests-first controls pass;63 Node+116 Python tooling tests,571 web unit tests, types/schema/lint and both asset profiles pass. T15 acceptance and remote enforcement remain open.
- **files edited/created**:

### T15.3: Register verified native regressions in the existing PR lane
- **depends_on**: [T3, T13.4, T15.1, T28, T31.3]
- **agent_type**: local
- **ownership**: `.github/workflows/integration-test.yml` Unity build/CTest selection only, `tests/quality/native-ci.test.mjs`, `quality/evidence/T15.3.md`.
- **description**: Include existing writer-watchdog, segment-timestamp and faststart tests plus newly verified stream-state disposal, DELETE/PUT launch failure, storage mux/backpressure and deletion-ledger regressions in both the existing Unity build list and its CTest selection. The broad existing stream_state regex already selects the new disposal test; explicitly build it rather than leaving a not-built failure. Preserve the previous selection and require nonempty discovery.
- **acceptance_criteria**: All fourteen added targets are built and selected, all previous entries retained, no empty success or failure masking. Workflow validation passes; local producer evidence covers actual test execution. Hosted execution remains unverified until authorized publication.
- **validation**: Tests first inspect actual YAML build/CTest commands for matching inclusion; actionlint and recorded focused native producer runs. No source/fixture change or remote action.
- **status**: Completed locally (commit `12dffcda`)
- **log**: Nine native regression targets added to build and selection; all73 prior build targets and full priorregex preserved. Ten initial tests-first workflow checks plus actionlint pass. Follow-up adds verified collision and stale-media validation targets; two new checks failed first, then12/12 pass. Verified pressure-reconnect registration adds one more RED-to-GREEN check; 13/13 pass plus actionlint. Verified AAC arithmetic and migration policy targets bring final checks to15/15, after2newRED omissions. Hosted execution unverified.
- **files edited/created**:

## Expanded media recovery tasks (T27)

### T27.0: Produce bounded native and actual-source media recovery checks
- **depends_on**: [T3, T13, T28.2]
- **agent_type**: worker
- **ownership**: T27 assigned native tests, `tests/fixtures/eng/media/`, `tests/operational/media/`, `tests/unit/cmake/eng/media.cmake`, `quality/evidence/T27.md`.
- **description**: Materialize isolated recovery/segment evidence with declared one-keyframe overlap tolerance, real source disconnect/restore and H264/AAC storage-pressure recovery. Preserve genuine RED outcomes as prerequisites for bounded fixes, no expected-failure suppression.
- **acceptance_criteria**: Native/oracle controls execute; real scenarios record frame/media/DB/metadata/recovery and graceful cleanup results, with source/config provenance and bounded deadlines. Failing product observations have exact evidence and successors.
- **validation**: T27 native selection and `tests/operational/media/recovery.py` documented scenario commands.
- **status**: Completed — tests-only producer, real collision failure retained (commit `4c513e4f`)
- **log**: 29 native and7 oracle tests pass; real video-only recovery passes. H264/AAC pressure/source restart reproduces two completed rows referencing one overwritten path (72731-byte first row vs94013-byte file). No production fix claimed; T27.1 owns closure.
- **files edited/created**: Exact8 paths in commit4c513e4f; evidence retains all artifacts.

### T27.1: Prevent MP4 path reuse from overwriting completed recordings
- **depends_on**: [T27.0, T28.2]
- **agent_type**: worker
- **ownership**: `src/video/recording_path.c`, contract comments only as needed in `include/video/recording_path.h`, bounded rotation/retry integration in `src/video/mp4_writer_thread.c`, `tests/operational/media/test_mp4_path_collision.c`, its `tests/unit/cmake/eng/media.cmake` registration, narrow reservation wrapper updates in `tests/unit/test_mp4_forced_reconnect.c` and `tests/operational/storage/test_storage_writer_backpressure.c`, `quality/evidence/T27.1.md`.
- **description**: Reproduce same-second path reuse with owned existing media, then ensure new segment creation cannot truncate a previous recording on rapid rotation/retry/restart. Inspect filename consumers before choosing a bounded uniqueness/reservation strategy; preserve timestamp meaning, supported layouts, mp4 extension, placement.object_key and DB path agreement. Keep planned-path helpers pure; atomically reserve immediately before the actual segment call, retain ownership through the call and remove only the same-inode still-empty reservation on failure. Handle buffer/allocation/reservation failure explicitly and clean up any owned reservation on failure; do not claim atomic exclusion from a racy existence check alone.
- **acceptance_criteria**: Tests first reproduce collision; GREEN preserves previous bytes/metadata and gives subsequent recordings distinct identities, including rapid retry/restart and existing-file controls. No existing recording is truncated, placement paths remain consistent, and the exact real H264/AAC pressure/source-restart scenario passes distinct-file/path/size/placement assertions. The separately reproduced buffered timestamp failure remains gated by T27.2. No storage-schema redesign or weakening of graceful teardown/oracles.
- **validation**: Focused collision sanitizer target plus existing path/layout/storage-policy, forced-reconnect/audio/boundary and recording-sync/playback controls selected after source review; rerun exact real failing T27 scenario and verify both source modes before original T27 closes.
- **status**: Completed (commit `3b62c681`)
- **log**: Focused collision4RED→12GREEN; zero/negative-duration2RED→GREEN. Thirteen sanitizer targets/111Unity pass. Real pressure case retains separate original72731-byte andnew94013-byte files with matching object keys/DB sizes; timing defect preserved forT27.2.
- **files edited/created**:

### T27.2: Reset stale media state when storage pressure pauses recording
- **depends_on**: [T27.1]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_thread.c` emergency-pause branch, `tests/operational/media/test_mp4_pressure_reconnect.c`, its `tests/unit/cmake/eng/media.cmake` registration, `quality/evidence/T27.2.md`.
- **description**: Reproduce retained pre-pause input/pending-keyframe/audio state being flushed after source restart with current wall-clock metadata. Reset/dispose the old reader, pending boundary packet and audio state on entering emergency storage pause, before sleeping, so recovery opens a fresh source. Preserve pressure policy, shutdown deadlines, one-frame normal boundary tolerance and ordinary reconnect semantics.
- **acceptance_criteria**: Tests first demonstrate stale state survives pause; GREEN disposes/resets it once without resource errors. Actual statvfs-based H264/AAC pressure/source-restart scenario passes unchanged duration/DB timestamp/size/decoded-media oracle, as does video-only recovery. No old packet is carried across the paused-session boundary and normal graceful cleanup remains required.
- **validation**: Focused pressure-reconnect sanitizer target plus existing watchdog/forced-reconnect/audio/boundary/pressure controls; rebuild actual server and rerun exact two T27 scenarios without relaxing their limits.
- **status**: Completed (commit `586d8d89`)
- **log**: Four stale-state controls failed first, then84/84 native cases pass. Both unchanged real scenarios pass:14.700s video-only and12.485s H264/AAC+pressure,12files decode and matchDBsizes/timing/continuity; all owned processes exit normally.
- **files edited/created**:

## Expanded storage fault tasks (T28)

### T28.0: Implement isolated storage fault regressions
- **depends_on**: [T3, T13]
- **agent_type**: worker
- **ownership**: T28 test/fixture/registration paths, `quality/evidence/T28.md`.
- **description**: Produce T28 tests-only coverage with real SQLite, owned files, scoped faults and playable recovered media. Record each genuine RED production defect without marking it expected or weakening assertions. T28.1/T28.2 own fixes; T28 final integration requires their GREEN results.
- **acceptance_criteria**: Each new assertion runs under the isolated native runner; existing controls pass; reproduced product failures have exact evidence and bounded successors. No production changes or host storage disruption.
- **validation**: T28 focused sanitizer commands and fault-injection diagnostics in task evidence.
- **status**: Completed — tests-only producer, genuine failures retained (commit `0eb7e5aa`)
- **log**: 70 Unity cases execute under ASan/UBSan:68 pass,2 genuine RED product defects (unavailable legacy root metadata loss, swallowed EROFS segment failure). Scoped fixtures avoid host disruption. T28.1/T28.2 own fixes; originalT28 acceptance pending.
- **files edited/created**:

### T28.1: Preserve legacy recording metadata while its root is unavailable
- **depends_on**: [T28.0]
- **agent_type**: worker
- **ownership**: `src/storage/storage_deletion.c`, narrowly related missing-file controls in `tests/unit/test_storage_manager_retention.c`, `tests/unit/test_storage_retention_sqlite.c` owned-root setup for real deletion fixture only, `quality/evidence/T28.1.md`.
- **description**: Fix the T28 unavailable-root regression: legacy deletion interprets ENOENT beneath an unavailable recordings root as authoritative file absence and finalizes metadata deletion. Keep deletion pending/retryable while root availability cannot be established; after restoration reclaim eligible expired files normally. Preserve registered-target mount guards, protected media and legitimate missing-file cleanup. Document the exact legacy-root identity rule, avoiding a new persistence or storage redesign.
- **acceptance_criteria**: Original unavailable-root regression reaches GREEN with metadata/file retained until restoration; ordinary existing-file and legitimately absent-file deletions still complete. Protected/open files and fault retry controls remain intact. No pretend successful deletion while root is unavailable.
- **validation**: `python3 scripts/quality/run-native.py --task T28.1 --regex '^test_storage_(manager_retention|retention_sqlite|deletion.*)$'`; exact owned-root rename/restore regression under ASan/UBSan.
- **status**: Completed (commit `6d254658`)
- **log**: Tests-first legacy root guard preserves metadata on unavailable/unknown roots.39/39 sanitizer cases pass, including healthy-root absent-file cleanup and owned-root fixture correction. No storage-schema change; configured-directory check is not persisted mount-identity proof.
- **files edited/created**:

### T28.2: Stop MP4 segments on terminal read-only write failures
- **depends_on**: [T28.0]
- **agent_type**: worker
- **ownership**: `src/video/mp4_segment_recorder.c`, narrowly related controls in `tests/operational/storage/test_storage_recording_faults.c`, `quality/evidence/T28.2.md`.
- **description**: Fix the T28 EROFS regression: an actual segment packet mux failure repeats and a later successful trailer overwrites it with success. Treat read-only storage write failure as terminal alongside disk-full/I/O failure across the existing packet paths, preserve its negative result through cleanup and keep timestamp recovery behavior distinct. Use the existing recording I/O diagnostics and close owned resources without new dependencies.
- **acceptance_criteria**: Injected EROFS stops on the first affected packet with negative result, existing ENOSPC/EIO controls pass, no sanitizer issue, and removing the fault produces decodable media. Existing timestamp/audio watchdog behavior remains unchanged.
- **validation**: Run T28 real segment fault target and existing segment/timestamp/audio recovery selections under the isolated sanitizer runner; record exact selections and decoding assertions.
- **status**: Completed (commit `e56fc168`)
- **log**: Terminal EROFS handling added to existing final-video/video/audio paths; first failed packet retains negative status and cleanup. Five real mux fault controls pass, including final keyframe, plus existing timestamp/audio/watchdog controls. T28 full integration73/73 passes. T27 initially over-strict zero-overlap oracle is being corrected to its declared tolerance; no boundary behavior changed.
- **files edited/created**:

## Expanded shutdown observation tasks (T29)

### T29.0: Reproduce false successful coordinator timeout
- **depends_on**: [T3, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_shutdown_coordinator.c`, `quality/evidence/T29.0.md`.
- **description**: Strengthen the existing timeout test, which currently discards its result, to require false on deadline and preserve actual live component state. Exercise actual successful completion, condition signaling without completion, and reuse of genuinely stopped component slots. Source remains unchanged until observed RED.
- **acceptance_criteria**: Bounded RED distinguishes a real stop from forcing bookkeeping to STOPPED. Existing zero-component/already-stopped and slot-reuse controls remain; test does not rely on an unbounded sleep or fabricate state completion.
- **validation**: `python3 scripts/quality/run-native.py --task T29.0 --regex '^test_shutdown_coordinator$'`; retain exact failing observations.
- **status**: Completed (commits `f2b12822`, `7b5acd88`)
- **log**: 17 cases executed; three expected RED failures demonstrate false timeout success and spurious-wake completion. Production untouched until RED; actual completion and64slot-reuse controls retained.
- **files edited/created**:

### T29.1: Report coordinator timeout without inventing stopped state
- **depends_on**: [T29.0]
- **agent_type**: worker
- **ownership**: `src/core/shutdown_coordinator.c`, corresponding contract comments in `include/core/shutdown_coordinator.h`, `quality/evidence/T29.1.md`.
- **description**: Honor existing wait contract: return false if the deadline expires before all live components stop, and preserve their true states. Wait in a predicate loop using the same absolute deadline so spurious/early condition signals cannot masquerade as successful shutdown. Preserve real completion notifications, already-stopped behavior and legitimate slot reuse; no caller/shutdown ordering redesign.
- **acceptance_criteria**: T29.0 reaches GREEN under ASan/UBSan; timeout stays bounded, live state is retained, real completion returns true and stopped slots can be reused. Main's existing warning branch becomes reachable. No new unconditional wait, forced kill, or persisted/API state change.
- **validation**: Focused coordinator sanitizer tests followed by T29 actual process lifecycle checks; forced cleanup must remain distinguishable from graceful completion.
- **status**: Completed (commits `4bb5cae3`, `c80872eb`)
- **log**: 17/17 coordinator sanitizer tests pass after observedRED. Wait uses original absolute deadline/predicate loop, returns false without inventing stopped state. Actual lifecycle normal stops3847/4240ms graceful; separate stale-media metadata defects remain under T29.2/T29.3.
- **files edited/created**:

### T29.4: Produce previous-release and actual-process lifecycle evidence
- **depends_on**: [T3, T13, T29.1]
- **agent_type**: worker
- **ownership**: `tests/operational/lifecycle/` producer harness, `tests/fixtures/eng/upgrade/`, `tests/unit/cmake/eng/lifecycle.cmake`, `quality/evidence/T29.md`.
- **description**: Record historical synthetic fixture provenance and run owned actual recording, graceful restart, forced termination/recovery, DB/media integrity and browser preference checks. Preserve genuine incomplete-media reconciliation failures for T29.2/T29.3 instead of weakening or skipping them.
- **acceptance_criteria**: Exact previous tagged-source SQL/config/preference inputs and hashes; real nonempty assertions, bounded deadlines and verified-process cleanup. Normal shutdown success excludes forced kills; deliberately injected SIGKILL is separately reported.
- **validation**: Recorded T29 fixture checks and actual lifecycle runner commands.
- **status**: Completed — tests-only producer with two genuine failures (commit `0a811fe0`)
- **log**: Historical89migration sources verified;13 runtime assertions execute,11pass/twoRED. Normal stops3847/4240ms with noforcedcleanup. Corrupt40byte MP4 promoted complete and stale zero/missing metadata retained; bounded successors required.
- **files edited/created**: Producer paths recorded in commit0a811fe0 and quality/evidence/T29.md.

### T29.2: Validate stale media before completion and guarantee scan progress
- **depends_on**: [T29.4]
- **agent_type**: worker
- **ownership**: `src/storage/storage_manager.c`, `src/database/db_recordings.c`, `include/database/db_recordings.h`, `tests/operational/lifecycle/test_incomplete_media_validation.c`, `tests/unit/cmake/eng/incomplete_media_validation.cmake`, `quality/evidence/T29.2.md`.
- **description**: Add focused tests for valid MP4, truncated nonempty MP4 and retained invalid entries spanning a500-row batch. Require usable local media before promoting stale incomplete rows; retain unreadable/corrupt evidence incomplete. Advance past retained rows through bounded query/progress semantics while preserving one-hour active-recording grace. Reuse existing probing where suitable; no damaged-file repair or deletion in this task.
- **acceptance_criteria**: RED reproduces false completion; GREEN leaves corrupt files unpromoted, finalizes valid stale media, preserves recent rows, reaches valid candidates behind at least500 retained invalid entries, and terminates within deadline without deleting files. Actual T29 truncated-media assertion passes after rebuild.
- **validation**: `python3 scripts/quality/run-native.py --task T29.2 --regex '^test_incomplete_media_validation$' --jobs 4`; existing recording DB tests and actual T29 scenario.
- **status**: Completed (production `8ad48404`, evidence `3a4bc326`)
- **log**: Tests-first decoded-video and bounded keyset scan controls pass44/44 Unity cases across3 ASan/UBSan targets. Real historical-upgrade/restart scenario now12/13, truncated-media assertion GREEN; only separate T29.3 phantom rows remain. Interruptible I/O/sample limits do not claim a decoder CPU deadline or full-file integrity.
- **files edited/created**:

### T29.3: Prune confirmed stale phantom recordings without weakening active deletion guards
- **depends_on**: [T29.2, T28.1, T29.5]
- **agent_type**: worker
- **ownership**: `src/storage/storage_manager.c`, `src/storage/storage_deletion.c`, `include/storage/storage_deletion.h`, `tests/operational/lifecycle/test_stale_recording_prune.c`, `tests/unit/cmake/eng/stale_recording_prune.cmake`, `quality/evidence/T29.3.md`.
- **description**: Add tests-first coverage and narrow stale-recovery transaction for aged missing/zero-byte recordings on confirmed available storage. Recheck stale/incomplete identity, protection and migration safeguards within the transaction; prove no live writer owns the candidate. Preserve ordinary API rejection of incomplete recordings and retain rows when availability/ownership cannot be established. Age or failed stat alone is insufficient authorization to prune. Persist explicit expected missing/empty identity guards through T29.5 so delayed/restarted deletion refuses replacement or newly nonempty files. Conservatively require persisted disabled stream and no live continuous/MP4 writer; retain enabled or unknown stream candidates. Support legacy paths and explicitly attributed filesystem targets only: snapshot full target UUID/object key, require actual mount_guard_active/access checks and canonical existing root, resolve key to the exact stored path, compare target/key inside the claim transaction and again during queued processing. No sentinel guarantee is invented; nonfilesystem/unknown targets remain retained.
- **acceptance_criteria**: RED reproduces phantom rows; GREEN removes only confirmed stale missing/empty metadata with consistent deletion bookkeeping. Recent/live, protected, migrating, unavailable-root and changed-since-selection candidates remain intact; general active-writer deletion tests stayGREEN. Attributed empty/missing positive controls and wrong-key, unavailable/mismatched mount/root, changed-target/key and re-enabled/live-after-queue negative controls pass. Corrected full T29 lifecycle passes13/13.
- **validation**: `python3 scripts/quality/run-native.py --task T29.3 --regex '^test_(stale_recording_prune|storage_deletion_ledger)$' --jobs 4`; existing T28 retention controls and rebuilt T29 process scenario.
- **status**: Not Completed
- **log**: F-T29-02 is an interaction with the correct general active-writer guard, not permission to relax it.
- **files edited/created**:

### T29.5: Persist stale-deletion identity guards across restart
- **depends_on**: [T29.4, T28.1]
- **agent_type**: local
- **ownership**: `db/migrations/0090_add_stale_deletion_guards.sql`, generated `include/database/db_embedded_migrations.h`, `tests/operational/lifecycle/test_stale_guard_migration.py`, current-schema expectation only in `tests/operational/lifecycle/run.mjs`, `quality/evidence/T29.5.md`.
- **description**: Add explicit nullable stale_guard_kind/dev/ino fields to the durable deletion-object ledger so T29.3 can distinguish expected absence from a specific empty inode after a restart. Ordinary deletion objects remain NULL/unmodified. Keep the historical89-source fixture immutable; require those prior versions/data plus the explicit new migration in the actual upgrade check. Use the repository embedded-migration generator; no hand-edited generated arrays.
- **acceptance_criteria**: Tests first cover missing guard schema, then actual historical-to-current SQL upgrade preserves existing rows and gives NULL guard defaults. Explicit missing/empty guard values survive reopening and invalid guard shapes fail or are rejected by the consumer. Migration/header check passes. Never silently remove pending safety guards on downgrade; unsupported rollback must fail explicitly or reject pending guarded operations, with honest documented limits. No production database is touched.
- **validation**: Focused SQLite migration tests using disposable databases, `python3 tests/unit/test_embedded_migration_generator.py`, generator --check, and rebuilt actual T29 upgrade/restart assertions after T29.3.
- **status**: Completed (schema `99a29f66`, evidence `32e7adb5`)
- **log**: Four real SQLite tests pass after missing-column RED; historical rows/defaultNULL preserved, guard values persist across reopen, invalid shapes rejected and downgrade explicitly unsupported. Generator4tests/--check pass. Real lifecycle schema continuity now verifies90versions, preserving historical89fixture; separate stale attribution failure remains T29.3.
- **files edited/created**:

## Advisory regression coverage tasks (T7.2)

### T7.2.1: Verify external migration trust boundaries
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_t7_2_migrations.c`, `tests/unit/cmake/eng/t7_2_migrations.cmake`, `quality/evidence/T7.2.1.md`; production read-only.
- **description**: Exercise actual disposable SQLite migrations through public migrate_init/up/down: valid SQL succeeds, external symlink/prefix escape and world-writable files never execute, rejected SQL cannot attach/write an outside fixture, and rollback applies the same file policy. Read src/database/sqlite_migrate.c and its public header; no production edits.
- **acceptance_criteria**: Allowed and rejected controls execute the real parser/SQLite path with precise schema/file effects. Preserve failure evidence and request a separately scoped fix if any boundary fails.
- **validation**: `python3 scripts/quality/run-native.py --task T7.2.1 --profile integration --regex '^test_t7_2_migrations$' --jobs 4`. Existing-fix verification can initially pass; do not fabricate a production RED or modify requirements to hide a failure.
- **status**: Completed — tests-only existing-fix verification
- **log**: Seven actual migration/SQLite/filesystem boundary cases pass integration ASan/UBSan, using mutable T7.7 cache (native-c9393ff3c3ba). Valid apply/rollback, symlink/prefix traversal, world-write mode, post-discovery replacement and disallowed ATTACH verified; no production edit or fabricated RED. Concurrent arbitrary filesystem mutation is not proven atomic.
- **files edited/created**:

### T7.2.2: Verify advisory buffer truncation boundaries
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_t7_2_buffers.c`, `tests/unit/cmake/eng/t7_2_buffers.cmake`, `quality/evidence/T7.2.2.md`; production read-only.
- **description**: Exercise current query builder, polygon serialization through public DB calls, and multi-source go2rtc URL construction with synthetic upstream capture. Cover valid boundary sizes and oversized inputs; no external camera/network. Read src/database/db_query_builder.c, src/database/db_zones.c, src/video/go2rtc/go2rtc_api.c and existing tests. No production edits.
- **acceptance_criteria**: ASan/UBSan controls verify valid outputs and rejection or safe documented truncation without buffer overflow; no HTTP request may silently target a changed source identity.
- **validation**: `python3 scripts/quality/run-native.py --task T7.2.2 --profile integration --regex '^test_t7_2_buffers$' --jobs 4`. Existing-fix verification can initially pass; do not fabricate a production RED or modify requirements to hide a failure.
- **status**: Not Completed
- **log**: Original advisory fixes predate the current plan; generic DB/media tests do not establish these exact boundary cases.
- **files edited/created**:

### T7.2.3: Verify descriptor and filesystem safety at advisory sites
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_t7_2_files.c`, `tests/unit/cmake/eng/t7_2_files.cmake`, `quality/evidence/T7.2.3.md`; production read-only.
- **description**: Trace current successors of advisory daemon/backup/storage/FFmpeg/HLS filesystem sites. Produce bounded disposable-file tests for descriptor chmod and deletion/symlink behavior, reusing existing exact coverage where available. Public API/link wrappers only where necessary; no daemonization, real mounts or production edits. Split distinct discovered gaps before fixes.
- **acceptance_criteria**: Record an individual disposition for every original filesystem site, with real-file controls and explicit race limitations. Static absence of an old path does not prove its replacement safe.
- **validation**: `python3 scripts/quality/run-native.py --task T7.2.3 --profile integration --regex '^test_t7_2_files$' --jobs 4`. Existing-fix verification can initially pass; do not fabricate a production RED or modify requirements to hide a failure.
- **status**: Not Completed
- **log**: Original advisory fixes predate the current plan; generic DB/media tests do not establish these exact boundary cases.
- **files edited/created**:

### T7.2.4: Verify settings and system operations avoid shell injection
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `tests/unit/test_t7_2_settings.c`, `tests/unit/cmake/eng/t7_2_settings.cmake`, `quality/evidence/T7.2.4.md`; production read-only.
- **description**: Read current settings path validation and system directory/process operations replacing the advisory shell sites. Exercise public settings handlers with temporary accepted/rejected paths and assert persistence/side effects. Verify system inspection invokes no shell using wrappers or direct execution evidence. No production edits.
- **acceptance_criteria**: Valid settings remain compatible; unsafe path inputs cannot cause shell execution or unwanted persistence. Source-only null-check cleanup is classified separately from a runtime defect.
- **validation**: `python3 scripts/quality/run-native.py --task T7.2.4 --profile integration --regex '^test_t7_2_settings$' --jobs 4`. Existing-fix verification can initially pass; do not fabricate a production RED or modify requirements to hide a failure.
- **status**: Not Completed
- **log**: Original advisory fixes predate the current plan; generic DB/media tests do not establish these exact boundary cases.
- **files edited/created**:


## Independent soak tooling (T30)

### T30.1: Implement bounded host soak composition and honest resource qualification
- **depends_on**: [T3, T4, T13, T27, T28, T29]
- **agent_type**: worker
- **ownership**: `scripts/quality/soak.py`, `tests/operational/soak/`, `quality/soak-profiles.json`, `quality/evidence/T30.1.md`; existing operational runners and runtime budgets read-only.
- **description**: Implement independent T30 tooling using the completed T5 protocol/inventory evidence e0544701, while its physical calibration remains explicitly deferred. Compose bounded repeated existing media/lifecycle scenarios, archive exact process/tool/source provenance, sample owned NVR and managed-child RSS/CPU/FD/thread data separately from source generators and browser, and evaluate supplied resource/recovery limits. Use existing T13 ownership and serialized runtime contracts; do not nest conflicting locks or adopt existing processes. Request an exact shared-runner extension before modifying files outside this ownership. Assess a separate race-detection build design/compatibility without combining TSan with ASan/UBSan.
- **acceptance_criteria**: Controlled resource growth, missed recovery deadline, child failure, sanitizer output, unexpected media gaps and incomplete/unsupported observations cannot produce a green overall summary. Preserve per-scenario functional results separately from resource qualification. Null physical limits from quality/runtime-budgets.json remain unqualified and make aggregate qualification nonpassing; never substitute made-up limits or silently skip them. Existing T27/T29 tests and tolerances are unchanged. Exercise a bounded actual host composition and record its honest functional/qualification results.
- **validation**: Tests first for profile validation, ownership/sampling identity, aggregation and failure propagation using disposable fixtures; `python3 scripts/quality/soak.py --profile ci --output build/eng-artifacts/T30` after an exclusive native/server/media lease. Tests may supply explicit synthetic thresholds to verify failures, but those thresholds are not physical qualification. Original T30 remains open for T5 physical/resource acceptance.
- **status**: Not Completed
- **log**: User deferral blocks physical acceptance, not implementation of the executor/reporting contract. No measured runtime limit is invented.
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
| 3 | T9 then T9.1–T9.4, T10, T12; eligible T7.* | T8/T11 prerequisites complete; fix tasks honor concrete ownership edges |
| Host soak tooling | T30.1 | T27–T29 complete; T5 protocol available, physical qualification stays deferred/nonpassing |
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
