# Storage 02 implementation record

Date: 2026-09-13. Implements the NVR portion of
[Storage 02](../prd/STORAGE_02_Hot_Cold_Archive.md). Configuration and deployment
instructions are in the [operator guide](../STORAGE_ARCHIVE.md).

## Delivered behavior

- Migration 0084 adds S3 target settings, archive timing/protection/pressure
  controls, policy revision snapshots, retrieval work, read leases, multipart
  checkpoints, and durable physical-deletion inventory. Existing filesystem
  targets and recordings migrate in place. Downgrade preserves archive state.
- The optional S3 adapter uses libcurl Signature V4 and mounted credential
  references. Connection tests enforce a private, never-versioned namespace with
  compatible lifecycle rules. Capture placement continues to require filesystem
  targets, including externally mounted NFS/SAN.
- The existing migration worker copies and verifies SHA-256 before committing
  a new location. Multipart work resumes after restart and reconciles ambiguous
  completion. Failed/cancelled artifacts stay journalled. Source cleanup rechecks
  the committed destination and active read leases before removing the old copy.
- Policy scheduling supports age, protected footage, pressure, hot residency,
  and required durable copies. Copy promotion reuses an existing archive object.
  Applied policy UUID/revision survives relocation; editing a policy affects new
  recordings until an explicit preview/apply action updates historical records.
- Logical deletion inventories primary media, replicas, unfinished destinations,
  source remnants, retrieval cache, and derivatives before physical work. Remote
  errors leave a visible pending deletion and retry. Policy expiry revalidates
  the current applied revision and override inside the deletion transaction.
  Filesystem maintenance and rebuild exclude tracked external inventory.
- Authorized original playback/downloads stream S3 ranges through the HTTP
  server, with two concurrent streams and one MiB buffers. Thumbnails, conversion,
  and ZIP exports use a deduplicated bounded cache that respects capture reserves.
  Direct S3-to-filesystem restores do not require that cache. Waiting for staged
  sources does not exhaust a migration's failure budget.
- Settings includes bucket/policy configuration, policy application preview,
  archive usage and deletion retries. Recording cards/rows show location and
  preparation/deletion status. Batch-delete progress identifies physical work
  still pending. Existing camera permissions gate media operations.
- IAM, multipart lifecycle, and Kubernetes credential-mount examples accompany
  the operator guide. No bucket, cloud account, or deployment was changed.

## Verification

The final Release build and existing regressions passed:

| Check | Result |
| --- | --- |
| CTest suite | 132/132 passed |
| Frontend Jest suite | 245/245 passed across 36 suites |
| Frontend production build | Passed |
| S3 archive protocol integration | 34 cases passed |
| CI selection in Debug with AddressSanitizer, UBSan, and coverage | 43/43 passed |
| MQTT-enabled health API, MQTT, and browser integration | 16/16 passed |
| S3 disabled: `lightnvr` and `rebuild_recordings` builds | Passed |
| Embedded SQL migration consistency | Passed |
| Deployment JSON/YAML syntax and diff whitespace | Passed |

The integration test uses real SQLite, a signature-validating local HTTP S3
fixture, real libuv TCP range responses, and client disconnects. Cases cover
verified archival/retrieval, corrupt destination reads, protected retention,
retrying remote deletion, rejected versioning/expiry, permitted multipart-abort
lifecycle, checkpoint resume after database reopen, hot-copy promotion under
pressure, missing destination during cleanup, archive expiry/indefinite overrides,
direct restore with cache disabled, policy expiry revalidation, long retrieval
waits, shared-filesystem reserve protection, and ambiguous multipart completion.
Review regressions also cover expiry during UploadPart and multipart completion
(HTTP 404 and embedded HTTP 200 errors), unpublished corrupt-object repair, preservation of referenced
copies, S3 replica failover, local/cache availability during provider outages,
active queue limits, catalog-independent cache reclamation, protected and legacy
retention overrides, admin authorization/retry, ZIP exports, and pending deletion
reporting. Missing filesystem source mounts defer copying; mounted paths are
rechecked before catalog commit. A two-client stalled-write regression verifies
lease renewal through pending deletion, disconnect, timer shutdown, and complete
playback delivery. The mutation hook waits for the final batch job result before
refreshing.

The health CI startup failure was a shared-connection transaction race: background
storage work could roll back default administrator creation. Bootstrap now holds
the database writer mutex; lifecycle reconciliation and deletion finalization do
not roll back if their BEGIN failed. Concurrent bootstrap/storage regression tests
and the MQTT-enabled browser integration gate pass.

Using the same CI selection in an isolated Debug coverage build, executable added
C lines increased from 1,443/1,825 (79.07%) before these changes to 1,885/2,025
(93.09%). Uncovered added lines fell from 382 to 140. This local gcov comparison
covers C changes; Codecov uses its own aggregation and patch denominator.

Useful commands:

```sh
cmake --build build -j4
ctest --test-dir build --output-on-failure -j4
ctest --test-dir build -R test_storage_archive_s3 --output-on-failure
python3 scripts/generate_embedded_migrations.py --check db/migrations include/database/db_embedded_migrations.h
npm --prefix web test -- --runInBand
npm --prefix web run build
```

The local machine had FFmpeg 7 headers/pkg-config files in `/usr/local` but only
FFmpeg 8 runtime libraries in the system library directory. Verification selected
the matching system pkg-config files and used an include directory of symlinks to
the system FFmpeg headers. This resolved an existing encoder boundary test's ABI
mismatch without changing repository code or system-installed files. LiteRT/SOD
were disabled in these builds, consistent with the existing local configuration.

## Supported limits and rollout work

The archive adapter requires immediately readable S3 objects; WORM, versioned
buckets, Glacier restoration, public media, and provider expiry are unsupported.
Managed-byte budgets do not measure provider request/egress bills. ZIP32 and the
configured retrieval-cache limit still constrain consumers needing local files.
Transfers retain the existing bounded retry policy and expose exhausted jobs for
operator retry; physical deletion retries persist until confirmed.

Policy application previews summarize affected records/bytes and already-expired
records; they do not provide a per-recording cost simulation. Existing stream,
tier, quota, and override retention settings remain active alongside the storage
policy cap. Historical job and policy references can prevent target removal even
after media has drained; disabling the target preserves reads and source cleanup.

This checkout contains the NVR, not the proprietary cloud control plane. Bucket
provisioning, account suspension/closure, secret distribution, external catalog
backups, and billing/metering integration remain work in that service. A live
provider pilot and multi-day outage/capacity soak have not been run; local fixture
coverage does not establish provider certification or fulfill those PRD rollout
gates. No production deployment was performed.
