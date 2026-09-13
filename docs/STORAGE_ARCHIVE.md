# Hot storage and external recording archives

LightNVR records to filesystem storage, then moves or copies completed recordings
to an S3 bucket or another mounted filesystem such as NFS/SAN. One recording ID
continues to own its footage, tags, detections, protection, and retention across
all copies. Migration never resets capture time.

Use an image built from this implementation; an older published image will not
have these endpoints. Configure **Settings → Storage** as a storage administrator.
No external archive is enabled automatically on upgrade.

## Connect storage

1. Keep the default filesystem/PVC target for capture. Size it for the hot window,
   peak ingest, retrieval/transcode headroom, and the longest archive outage you
   intend to tolerate. Archive storage does not eliminate local capture needs.
2. Create a dedicated, private, **never-versioned** general-purpose S3 bucket.
   Use a stable prefix owned by exactly one LightNVR instance/catalog. Enable the
   provider's public-access blocking and encryption at rest. Bucket versioning,
   suspended versioning, Object Lock/WORM, object expiry, and storage-class
   transitions are unsupported. Files must remain available for immediate GET.
3. Grant the instance access only to that bucket/prefix. The example
   [IAM policy](../deployment/archive/aws-iam-policy.json) separates bucket
   inspection from object access. Replace both placeholders. Additional KMS
   permissions are needed if your bucket uses a customer-managed encryption key.
   Bucket-level `s3:ListBucket` is also required: [S3 returns 404 for an absent
   object only with that permission](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadObject.html).
   Without it, confirmation after deletion returns 403 and cleanup stays pending.
4. Mount an ordinary credentials file, mode `0600`, owned by root or the NVR UID:

   ```json
   {"access_key_id":"REPLACE_ME","secret_access_key":"REPLACE_ME"}
   ```

   An optional `session_token` is supported. The default directory is
   `/etc/lightnvr/archive-credentials`; the filename, for example `instance`, is
   the **credential reference**. Credentials never enter the catalog or target
   responses. Files and the final credentials directory must not be symlinks.
   Rotate by atomically replacing the file with the same owner/mode. Each request
   reads it again. Updating the target's reference also supports rotation.
5. Add a target of type **S3-compatible bucket**. Supply the HTTPS endpoint origin
   (for example `https://s3.us-east-1.amazonaws.com`), region, bucket, credential
   reference, and root `s3://BUCKET/INSTANCE_PREFIX`. Use the regional endpoint;
   redirects are not followed. Configure a managed-byte budget if desired.
6. Run the connection test. It checks bucket configuration and performs a tiny
   signed upload, verified read, anonymous-read rejection, and confirmed delete.
   A failed probe leaves the target unavailable and preserves hot recordings.

Only an `AbortIncompleteMultipartUpload` lifecycle rule with a 1–7 day delay is
accepted. The supplied [seven-day rule](../deployment/archive/bucket-lifecycle.json)
limits orphaned parts if a process dies between initiating an upload and saving
its upload ID. This rule affects unfinished uploads, never completed recordings.
See [AWS's lifecycle documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/mpu-abort-incomplete-mpu-lifecycle-config.html).
Apply this rule before connecting the bucket. Providers without this feature
need an equivalent operator cleanup process for abandoned multipart uploads.

The adapter uses path-style S3 requests and libcurl Signature V4. Providers must
support versioning/lifecycle inspection, multipart uploads, HEAD, byte ranges,
and private GET/PUT/DELETE. A successful fixture test is not provider
certification: run the connection test and outage/restart pilot on your actual
provider before moving production footage. Bucket policy changes outside LightNVR
remain the operator's responsibility; do not modify managed object keys manually.

For NFS/SAN, mount the filesystem outside LightNVR and add a filesystem target
with its mount guard enabled. The same archive policy and verified migration
worker apply. S3 targets cannot be capture defaults, capture pool members selected
for placement, or capture fallback destinations.

## Define a storage policy

Use the existing camera selector and primary capture target/pool. Set the archive
destination and these independent controls:

| Field | Meaning |
| --- | --- |
| `archive_after_seconds` | Capture age before archival; `-1` disables age-based archival, `0` means once finalized. |
| `hot_residency_seconds` | Minimum hot age; `-1` means no extra residency minimum. Early archival creates a verified second copy, then promotes it when residency ends. |
| `archive_protected` | Archive protected recordings even after ordinary retention would have expired. |
| `archive_on_pressure` | Archive early and permit eviction before the hot-residency minimum when the source reaches its high watermark/reserve. |
| `maximum_retention_days` | Logical expiry cap across all tiers for inherited retention; `0` disables this policy cap. Recording overrides take precedence. |
| `minimum_retention_days` | Minimum automatic retention. Protection always prevents deletion. |
| `required_copy_count` | Required durable copies before hot-copy removal; playback cache does not count. |

The older `migration_after_days` remains supported and takes precedence over
`archive_after_seconds` when positive. Transfer windows and bandwidth limits are
configured on the destination target. Ages use recording `start_time` and require
finalization. Already-expired unprotected recordings should be deleted, not
archived for a new retention window.

Example: archive after 6 hours (`21600`), keep 2 days hot (`172800`), cap logical
retention at 30 days, and enable protected archival. Keep the stream/system
retention at the intended duration too: existing stream retention, tier rules,
quotas, and recording overrides still apply. A policy cap of zero does not turn
off an independently configured stream expiry. Recording overrides map `-1`/NULL
to inheritance, positive days to finite retention, and `0` to indefinite expiry.
Protection prevents manual and automatic deletion until explicitly released.
Unprotecting old footage can make it immediately eligible for retention cleanup.

The applied storage-policy UUID/revision is retained in the catalog. Editing a
policy affects new recordings. **Review existing recordings** previews the
recordings and bytes affected, how many are already past the new retention limit,
and unfinished transfers. Applying that revision is a separate audited action;
resolve unfinished transfers first. Placement selector changes do not silently
reassign historical recordings to a different policy.

Under hot pressure, disposable cache is reclaimed first. An archive policy
prevents pressure from deleting the logical recording simply to reclaim its hot
copy. Hot eviction requires verified durable storage. If archival is unavailable
and all safe candidates are exhausted, existing capture-pressure pause/fallback
behavior applies; protected footage is not sacrificed.

## Playback, export, and deletion

Playback and individual downloads authorize the camera/recording before accessing
storage. Archived originals stream through LightNVR using bounded byte-range
reads, including seeks. Clients receive no bucket credentials or public object
URLs. Two concurrent archive streams, each with at most a 1 MiB transfer buffer,
reserve HTTP worker capacity for other operations. Further streams get a retryable
503. Socket backpressure controls subsequent reads; disconnects stop the transfer.
Active responses renew their read lease while socket writes are pending, so a
slow client cannot lose its archive to concurrent cleanup. Renewal ends when the
response completes or its cancelled I/O finishes. The shared lease then has up to
120 seconds of expiry grace for other readers.

Thumbnails, compatibility conversion, and ZIP exports use a verified staging
cache beneath `storage_path/archive-cache`. The default budget is 2048 MiB,
configurable with `LIGHTNVR_ARCHIVE_CACHE_MB`; `0` disables staging. The queue is
bounded to 16 queued or fetching jobs; completed cache entries and failed attempts
do not consume queue slots. Downloads are deduplicated, and space checks preserve the
capture reserve. Idle cache can be evicted without losing recordings. If the
catalog is unavailable, emergency reclamation scans only old, completed files
in the app-owned archive cache, preserving recent files, partial fetches, and
symlinks. A source larger than the configured cache budget cannot be staged; original playback and
individual downloads still stream directly. ZIP export currently uses ZIP32 and
rejects incomplete or over-4-GiB exports rather than returning a misleading ZIP.

Playback, retrieval, and availability status share replica selection. Usable
filesystem copies and verified cached files remain available during provider
outages. Failed retrievals can retry another verified S3 replica; each fetch
persists the chosen target and object identity.

The recordings library labels hot, archived, both, transferring, preparing,
unavailable, and pending-deletion states. Existing permissions apply to listing,
playback, thumbnails, export, and delete.

Deletion creates a durable tombstone and inventories primary media, replicas,
incomplete transfer destinations, source-cleanup remnants, cache, and derivatives.
Pending media remains catalogued until physical deletion is confirmed. Provider
errors retry with bounded backoff. New reads/protection changes are rejected once
deletion is claimed; active read leases delay removal. A 202 delete response means
accepted and pending, not reclaimed storage. **Archive operations** shows pending
deletions and retries. Batch-delete progress distinguishes accepted requests from
pending physical cleanup. Filesystem orphan detection and rebuild exclude remote
recordings and tracked migration/copy/deletion objects.

Uploads use durable multipart checkpoints from 8 MiB upward. Every destination
is read back and checked with SHA-256 before commit. A multipart ETag is not used
as a content checksum. Completed HTTP 200 responses are parsed for embedded
provider errors, as required by [S3's completion contract](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html).
A retry can recover an ambiguous completion by verifying the object at the job's
immutable key. Failed/cancelled artifacts remain owned by cleanup journals.

## Administration API

All storage administration routes require `storage.configure`. Existing replay,
export, and recording-delete camera permissions still gate media access.

| Route | Purpose |
| --- | --- |
| `GET/POST /api/storage-targets` | List/create filesystem or S3 targets. |
| `PUT /api/storage-targets/:uuid` | Change writable settings with the expected `revision`; target kind, S3 endpoint, bucket, region, and namespace stay immutable. |
| `POST /api/storage-targets/:uuid/probe` | Test connection/permissions. |
| `GET/POST /api/storage-policies` | Policy inventory and creation with the fields above. |
| `POST /api/storage-policies/:uuid/apply` | `{"revision":2,"preview":true}` previews; `preview:false` explicitly applies it to associated historical recordings. |
| `GET/POST /api/storage-migrations` | Durable jobs; create with `recording_id`, `destination_target_uuid`, and `operation` (`move` or `copy`). |
| `POST /api/storage-migrations/:uuid/retry` | Retry a failed/cancelled transfer. |
| `POST /api/storage-migrations/:uuid/cancel` | Cancel an uncommitted transfer, preserving the source. |
| `GET /api/storage-archive` | Managed archive bytes, transfer backlog, cache/queue totals, recovery mode, and up to 100 oldest pending deletions. |
| `POST /api/storage-archive/retry` | Queue an immediate retry with `{"deletion_uuid":"..."}`. |
| `GET /api/recordings/play/:id?prepare=1&transcode=0` | Source readiness only; no compatibility encode. |
| `GET /api/recordings/download/:id?prepare=1` | Download readiness without downloading a media body. |

Budgets cover catalogued primary/replica bytes and pending transfers. They are
application admission limits, not cloud billing guarantees. Provider traffic,
verification reads, request charges, incomplete uploads, and unrelated objects
can incur additional costs. Monitor the bucket's actual usage independently.

## Deployment, recovery, and rollback

The [Kubernetes patch](../deployment/archive/kubernetes-secret-patch.yaml) adds
credential mounts to an existing deployment without replacing its config or PVC.
Create `lightnvr-archive` with a `credentials.json` key using your secret manager.
The init container copies the projected secret into an ordinary protected file;
this accommodates Kubernetes' projected-file behavior while retaining the NVR's
no-symlink credential rule. Restart the pod after rotating that Secret, or have
a trusted sidecar atomically replace the ordinary file. Match file ownership to
the NVR UID. The patch assumes root, matching the current container image.
See [Kubernetes Secret volumes](https://kubernetes.io/docs/concepts/configuration/secret/).

For Compose, add a read-only bind mount of your ordinary credentials directory to
`/etc/lightnvr/archive-credentials` and optionally set the cache environment
variable. Continue mounting data at `/var/lib/lightnvr/data`; do not hide the
web assets under `/var/lib/lightnvr`. Pin the image you built with this feature.

Back up the SQLite catalog, including WAL-consistent policy versions, copy
inventory, upload checkpoints, deletion journals, and recording metadata. Use
SQLite's online backup mechanism or stop the process before copying the database.
Keep these backups outside the hot PVC with the instance/bucket ownership data.
The bucket alone cannot reconstruct all metadata or the latest protection and
deletion decisions. The existing database backup does not upload itself to S3.

After restoring an older catalog, first start with recording disabled and
`LIGHTNVR_STORAGE_READ_ONLY=1`. This pauses migration and recording deletion while
allowing inspection and playback. Verify catalog/instance ownership, restore the
latest protection and deletion journal, confirm bucket connectivity, and inspect
sample recordings before removing recovery mode. Never run two catalog owners
against the same namespace. Do not run an old binary against this migrated schema.

To drain an archive, disable its target to stop new placement/migrations into it,
then use `move` jobs to an enabled filesystem target with sufficient space.
Reads and source deletion remain available from a disabled target. S3-to-filesystem
restores stream into verified temporary destination files and do not require the
retrieval cache. To move between buckets, stage through filesystem storage; direct
bucket-to-bucket moves use the bounded cache and therefore inherit its size limit.
Drain all primary/replica inventory and resolve transfer/deletion work before
removing a target. Historical jobs/policy references can also block removal.
Keep the archive-aware binary until the catalog is fully drained; schema rollback
is deliberately non-destructive and retains archive inventory.

This repository contains the NVR, not the separate cloud control plane. The cloud
service must provision/private-scope buckets, distribute/rotate credentials,
back up catalog state, meter provider costs, retain bucket ownership through
suspension, and coordinate deletion on account closure. Retention stops when the
NVR is stopped; provider expiry must not be substituted because it can erase
protected recordings. Account suspension/termination and billing integrations
must be implemented in that service using this contract.

## Build and validation

S3 support defaults on and needs libcurl 7.75+; `-DENABLE_S3_ARCHIVE=OFF` builds
filesystem-only variants. `LIGHTNVR_ARCHIVE_ALLOW_HTTP=1` is an explicit local-test
escape hatch; production deployments require HTTPS.

`ctest --test-dir build -R test_storage_archive_s3 --output-on-failure` runs an
isolated S3 protocol fixture with signature validation, corrupt reads, failed
DELETE, interrupted/resumed multipart transfer, policy-driven protected archival,
hot-copy promotion, source-preserving cleanup, archive expiry, restoration,
real HTTP range streaming, and disconnect handling. Storage/API regressions and
the frontend test/build suite cover the existing workflows. The fixture also
checks ambiguous multipart completion, retention revalidation, shared capture
reserves, and retrieval waits beyond the migration retry limit. No production bucket
is needed for these tests. A live provider pilot, multi-day outage soak, and cloud
billing/control-plane rollout are separate deployment qualification work.
