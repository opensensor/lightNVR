# Storage API contract and lifecycle evidence (F5/F6)

This P4c delta starts at P3 commit `b96db492`, on
`codex/hardening-storage-contracts`. It accepts all 28 residual storage registrations.
The isolated branch now covers 116 operations on 81 paths; API coverage remains
explicitly partial. This is local storage qualification, not remote-provider certification.

## Accepted inventory

| Method | Native route | Operation | Success schema/status |
|---|---|---|---|
| GET | `/api/storage-targets` | `listStorageTargets` | 200 `StorageTargetsResponse` |
| POST | `/api/storage-targets` | `createStorageTarget` | 201 `StorageTarget` |
| GET | `/api/storage-targets/#` | `getStorageTarget` | 200 `StorageTarget` |
| PUT | `/api/storage-targets/#` | `updateStorageTarget` | 200 `StorageTarget` |
| DELETE | `/api/storage-targets/#` | `deleteStorageTarget` | 200 `FleetMutationAck` |
| GET | `/api/storage-pools` | `listStoragePools` | 200 `StoragePoolsResponse` |
| POST | `/api/storage-pools` | `createStoragePool` | 201 `StoragePool` |
| GET | `/api/storage-pools/#` | `getStoragePool` | 200 `StoragePool` |
| PUT | `/api/storage-pools/#` | `updateStoragePool` | 200 `StoragePool` |
| DELETE | `/api/storage-pools/#` | `deleteStoragePool` | 200 `FleetMutationAck` |
| GET | `/api/storage-policies` | `listStoragePolicies` | 200 `StoragePoliciesResponse` |
| POST | `/api/storage-policies` | `createStoragePolicy` | 201 `StoragePolicy` |
| GET | `/api/storage-policies/#` | `getStoragePolicy` | 200 `StoragePolicy` |
| PUT | `/api/storage-policies/#` | `updateStoragePolicy` | 200 `StoragePolicy` |
| DELETE | `/api/storage-policies/#` | `deleteStoragePolicy` | 200 `FleetMutationAck` |
| POST | `/api/storage-targets/#/probe` | `probeStorageTarget` | 200 `StorageTarget` |
| POST | `/api/storage-policies/preview` | `previewStoragePolicy` | 200 `StoragePolicyPreview` |
| POST | `/api/storage-policies/#/apply` | `applyStoragePolicy` | 200 `StoragePolicyApplyResponse` |
| GET | `/api/storage-migrations` | `listStorageMigrations` | 200 `StorageMigrationsResponse` |
| POST | `/api/storage-migrations` | `createStorageMigration` | 202 `StorageMigration` |
| GET | `/api/storage-migrations/#` | `getStorageMigration` | 200 `StorageMigration` |
| POST | `/api/storage-migrations/#/cancel` | `cancelStorageMigration` | 200 `StorageMigration` |
| POST | `/api/storage-migrations/#/retry` | `retryStorageMigration` | 200 `StorageMigration` |
| GET | `/api/storage-archive` | `getStorageArchive` | 200 `StorageArchiveResponse` |
| POST | `/api/storage-archive/retry` | `retryStorageArchive` | 202 `StorageArchiveRetryResponse` |
| GET | `/api/storage-compliance` | `getStorageCompliance` | 200 `StorageComplianceResponse` |
| GET | `/api/storage/health` | `getStorageHealth` | 200 `StorageHealthResponse` |
| POST | `/api/storage/cleanup` | `triggerStorageCleanup` | 200 `StorageCleanupResponse` |

Target/pool/policy deletion requires a positive revision **query parameter**;
updates carry revisions in JSON. Requests preserve zero retention, bandwidth and
budget values. Policy preview distinguishes a draft from the current policy and
supports null effective winners. Apply preview is independent from actually
updating captured recording policy revisions. Target probes return complete target
objects, including health. S3 responses expose credential references, never credential
material; disabled S3 configuration is exercised without contacting a provider.

## Reproduced behavior gap and fix

RED: native `POST /api/storage/cleanup` returned 200 and scheduled cleanup when
its JSON body was an array, boolean or contained a non-boolean `aggressive` value.
The previous implementation also truncated the body to 255 bytes and silently
ignored parse errors. The initial API regression failed expecting 400, receiving 200.

GREEN: parse the complete body, require a JSON object and optional boolean flag,
reject trailing non-whitespace bytes, and return before scheduling for invalid input.
An absent body still means ordinary cleanup. The native regression wraps
`trigger_storage_cleanup` and proves **zero scheduling calls** for malformed bodies,
including trailing garbage, then exactly one call per valid request with the right flag.
The test is now included in the CI sanitizer label.

A separate initial discovery regression failed because none of the 28 operations
was advertised. It passes after canonical definitions and native regeneration.

## Permissions and effects

Every storage operation uses the existing global `storage.configure` action.
Migration access remains global, including cancel/retry of jobs created by another
user; this slice does not invent creator-only job access. Scoped tokens with the
global action can discover and use storage APIs. Viewer/camera-only tokens cannot
read storage configuration, schedule cleanup, create targets or create migration
jobs. Tests independently assert the whole operation set for those principals and
verify denied writes leave target catalogs, job catalogs and owned directories unchanged.

Storage JSON response objects are closed, with explicit flags, nullable timestamps,
UUIDs, revisions, lifecycle states, counters, references, forecasts and nested objects.
Generated guards validate actual wire data before it enters the StorageTab and
StorageHealth consumers. `storageJSON` rejects unsupported routes/queries, validates
outbound bodies and expected status/content type, preserves cancellation, sanitizes
errors and never automatically replays mutations. `useStorageQuery` preserves query
options while routing actual cache population through the same boundary. Mutation
contract failures carry `mayHaveApplied`.

## Independent local lifecycle evidence

Six owned native-server cases exercise all 28 operations; accepted JSON responses
are checked against both the canonical schema and the actual principal projection.
The expected permission sets are authored independently of OpenAPI metadata.

- Target CRUD/probe, pool membership and strategy, policy CRUD/preview, stale
  revisions, and target/pool reference integrity. Disabled preview returns null
  effective winners; zero settings survive round trips.
- Policy apply preview reports a real captured recording without changing its
  revision; apply changes that revision durably. Stale apply fails. Recording
  fixtures are seeded directly because capture metadata has no administrative
  write API; reads independently inspect the owned SQLite catalog.
- A real synthetic MP4 is copied and moved to owned filesystem targets. Tests
  wait for terminal completion, compare destination bytes and SHA-256, verify copy
  preserves source bytes/protection and creates a replica row, and verify move
  updates primary catalog location and removes the old source file.
- A future transfer window deterministically keeps a job queued. A different
  global storage manager cancels and retries it; states are observed independently,
  and cancellation leaves the destination empty. No job-owner permission is implied.
- Cleanup acknowledgement is followed by polling controller completion, then
  checking that protected recording bytes remain intact.
- A deliberately deferred, owned deletion-ledger fixture exercises archive list
  and retry. The unauthorized retry leaves bytes intact. The authorized 202 only
  acknowledges scheduling; tests separately wait for actual file removal and the
  pending ledger to disappear. This fault/clock state is seeded directly because
  the public API cannot create it; all paths remain inside `lightnvr-contract-*`.

A real Chromium session opens Storage settings with a malformed target catalog.
The UI displays a contract error, does not render injected partial rows, and recovers
through Retry after the native endpoint is restored. Thirteen adapter unit cases
cover all catalog families, invalid outbound revisions/bodies before transport,
mutation ambiguity, no automatic replay, sanitization, cancellation and zero counts.

## Validation

- `npm run contracts:generate`, `contracts:check`, `contracts:lint`: pass. Lint
  retains four existing recordings-path ambiguity warnings.
- `npm run contracts:test-tools`: 10 pass.
- Root and contract TypeScript checks, web application/test TypeScript checks: pass.
- `npm --prefix web run build`: pass.
- `npm --prefix web test -- --runInBand`: 73 suites / 877 tests pass.
- `LIGHTNVR_CONTRACT_BIN=build/fleet-writes/bin/lightnvr npx playwright test -c tests/contracts/playwright.config.ts storage --project=api --project=ui`:
  7 pass. Expanded durable-apply/cleanup-completion API cases reran 6/6; final
  cleanup parser regression also reran after the last parser change.
- Optimized CTest storage target/policy, migration/publication, archive/deletion,
  retention and pressure selection: 12 test binaries pass.
- Debug ASan/UBSan selected registered target/policy/migration/deletion suites:
  4 binaries pass, leak detection/halt-on-error enabled. `test_storage_archive`
  builds but is not registered in this Debug CTest subset; its optimized test passes.
- The final cleanup target-handler suite was rebuilt and rerun optimized and
  ASan/UBSan after adding trailing-input validation; 5 cases pass in each.
- `git diff --check`: pass.

No operator storage, production database or remote archive was accessed. Native
builds disable go2rtc, MQTT, LiteRT, SOD and Sentry. Local synthetic footage,
filesystem destinations and the deletion ledger prove local lifecycle behavior.
Remote S3/NFS/provider credentials, durability and interoperability remain separate
qualification work; the disabled S3 configuration test does not claim a successful
remote transfer. Active-transfer race/fault handling retains existing native suite
coverage and is not claimed as a new browser or provider qualification.

## Integration notes

This commit is a delta after P3. Merge canonical schemas/paths/coverage additively
and regenerate all four artifacts. Add `StorageOperation` and its mutation
uncertainty cases to the shared contract error union alongside other slices.
CMake changes add only the scheduling wrapper and sanitizer label to the existing
storage-target handler test. Root owns event routing and later integration.
