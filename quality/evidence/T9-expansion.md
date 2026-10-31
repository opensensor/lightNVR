# T9 expansion proposal

Compiler RED at `17de8155`: `npm run typecheck:tests` exits 2 with 38
diagnostics in 12 files (`build/eng-artifacts/T9/red.log`). Playwright discovery
before repairs: 213 tests in 28 files, exit 0. No server or load started.

T9 provides `tests/tsconfig.json` with strict, noUncheckedIndexedAccess,
isolatedModules, noEmit, ES2022/DOM and explicit Node types. It includes all
integration/load TS and the Playwright configuration. No error-bearing file is
excluded. Compiler diagnostics serve as the RED verification contract; existing
runtime assertions are preserved. These are type repairs, not an application
migration. Existing legacy `any` uses are not claimed eliminated globally.

Insert the following task records into the plan; add **T9.4** to both T13 and
T15 dependencies. Graph: `T9 -> T9.1 -> T9.2 -> T9.3 -> T9.4`. This serial chain
reserves one root-typecheck validation lease and gives an integrated final gate.
No schema/package/native/shared setup ownership overlaps are introduced.

### T9.1: Repair load-test and page-helper type boundaries
- **depends_on**: [T9]
- **agent_type**: worker
- **ownership**: `tests/load/config.ts`, `tests/load/reporter.ts`, `tests/load/runner.ts`, `tests/integration/pages/StreamsPage.ts`, `tests/integration/pages/TimelinePage.ts`, `quality/evidence/T9.1.md`.
- **description**: Repair three array-index return types, two unknown caught errors, and date extraction. Use checked existence/narrowing, preserve behavior for valid inputs, and reject impossible empty selections explicitly. No load execution or test removal.
- **acceptance_criteria**: All six initial diagnostics in these five files disappear without broad declarations, ts-ignore, or new `any`; valid CLI/scenario/report/page behavior stays intact.
- **validation**: `npm run typecheck:tests` (initial 38 diagnostics; this cluster must reach zero while unrelated clusters remain RED); Node assertions for parseArgs valid/missing values and zero-request runLoadTest without network; final whole-config GREEN owned by T9.4.
- **status**: Not Completed

### T9.2: Type media DOM and mocked routes
- **depends_on**: [T9.1]
- **agent_type**: worker
- **ownership**: `tests/integration/specs/investigation.ui.spec.ts`, `tests/integration/specs/recordings.ui.spec.ts`, `tests/integration/specs/streams.ui.spec.ts`, `tests/integration/specs/timeline.ui.spec.ts`, `quality/evidence/T9.2.md`.
- **description**: Narrow selected video elements to the HTMLVideoElement contract; retain controls/readyState/play/pause assertions. Type mocked route setup parameters as Playwright Page so handlers infer Route. Use fixed ISO date slicing for non-optional date strings. Handle controlsList as a narrow browser capability (DOMTokenList) because it is not part of TypeScript's portable HTMLMediaElement interface.
- **acceptance_criteria**: All 23 original diagnostics across these four files disappear; all runtime test definitions/assertions remain present; no ambient browser-wide `any` added.
- **validation**: `npm run typecheck:tests` with zero cluster errors; compare Playwright --list test identities to baseline (ignore moved line numbers); final full GREEN in T9.4.
- **status**: Not Completed

### T9.3: Preserve timeline/snapshot fixture bounds
- **depends_on**: [T9.2]
- **agent_type**: worker
- **ownership**: `tests/integration/specs/timeline-boundary.ui.spec.ts`, `tests/integration/specs/stream-snapshot-requests.ui.spec.ts`, `quality/evidence/T9.3.md`.
- **description**: Validate parsed local date/time components and nonempty segment fixtures; narrow fixed fixture entries and captured URLs through explicit checks or known tuple definitions. Preserve existing route responses and snapshot/timeline assertions, rejecting malformed fixture setup instead of changing tests to pass.
- **acceptance_criteria**: All eight original diagnostics in these two files disappear without optional-default values hiding missing test data; test identities remain unchanged.
- **validation**: `npm run typecheck:tests` with zero cluster errors; Playwright --list identities unchanged; final full GREEN in T9.4.
- **status**: Not Completed

### T9.4: Integrate the legacy JS test boundary and verify coverage
- **depends_on**: [T9.3]
- **agent_type**: local
- **ownership**: `tests/tsconfig.json`, `tests/integration/recordings-url-utils.ui.spec.ts`, optional narrowly scoped test interop helper under `tests/integration/fixtures/`, `quality/evidence/T9.4.md`, `quality/evidence/T9.md`.
- **description**: Resolve the existing recordings JS import without wildcard-any modules or hiding test files. Keep the exact legacy JS boundary explicit and runtime-checked where values cross into the test. Any incremental allowJs/checkJs setting must inventory the exact imported application files as still unchecked and assign retirement to T34; it is not migration completion. Preserve the four existing URL utility test definitions and assertions. Reconcile compiler coverage and discovery across all clusters.
- **acceptance_criteria**: Root typecheck is GREEN for all existing integration/load TS and configuration; no unchecked test file/excluded diagnostic; 213 Playwright tests in 28 files remain discovered; no load traffic sent. Legacy application JS remains explicitly accounted for in evidence.
- **validation**: `npm run typecheck:tests`; `npx playwright test --list --reporter=list`; inspect `tsc --listFilesOnly -p tests/tsconfig.json` against `rg --files tests/integration tests/load`; run the four URL utility tests with a temporary isolated Playwright config disabling global setup/teardown (they need neither browser launch nor server); all results captured under `build/eng-artifacts/T9/`.
- **status**: Not Completed

Expansion is pending coordinator graph/ownership approval before child edits.
