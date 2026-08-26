# PRD — Multi-Target Storage Lifecycle

**Status**: Implementation complete through P3 — P4 remains intentionally
deferred until a concrete external-storage integration is selected; field,
scale, and event-upgrade acceptance remain
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 5 — storage scale and resilience
**Scope**: Storage targets and pools, selector-driven placement and lifecycle
policies, asynchronous migration, capacity forecasting, and failure behavior.

---

## 1. Problem

lightNVR already supports per-stream retention, detection-aware retention,
protection, manual tiers, per-recording overrides, and pressure cleanup. Those
policies operate primarily within one global storage path. Larger deployments need
to decide where recordings begin, when they migrate, how long different event
classes remain, and what happens when a target is unavailable or full.

Simply assigning each camera to one disk cannot express hot-to-warm archival,
spillover, evidence protection, or different retention for high-value events.

## 2. Goals

- Separate physical storage configuration from recording lifecycle policy.
- Support multiple local or mounted filesystem targets without blocking capture.
- Select policies using Fleet 01 locations, tags, cameras, and event metadata.
- Automate placement, migration, retention, replication, and pressure priority.
- Make failure and spillover behavior explicit and observable.
- Forecast whether configured capacity can meet desired retention.
- Preserve all existing recordings and retention semantics during migration.

## 3. Non-goals

- Bundling vendor-specific S3/cloud SDKs into the C core in v1.
- Distributed erasure coding or implementing a filesystem.
- Treating backup as recording retention; system backup is OPS 01.
- Duplicating continuous video into separate motion files.
- Profile G edge-camera retrieval; ONVIF 02 consumes this policy framework.

## 4. Product model

### 4.1 Storage target

A target represents one writable namespace and reports:

- Stable UUID, name, type, root path, enabled state.
- Capacity, free bytes, reserved headroom, high/low watermarks.
- Performance class (`hot`, `warm`, `cold`) and optional cost metadata.
- Health, latency, last successful probe, last write, and last error.
- Capabilities such as atomic rename, hard link, and read-only archive.

v1 target types are local filesystem and administrator-mounted filesystem/NFS.
Future cloud/object targets use an adapter contract rather than changing policy
semantics.

### 4.2 Storage pool

A pool is an ordered or weighted set of compatible targets with an allocation
strategy such as most-free, round-robin, or explicit priority. Policies reference
pools when spillover is acceptable and targets when exact placement is required.

### 4.3 Storage policy

A versioned policy contains:

- Fleet 01 camera selector.
- Recording predicates: trigger, object label, zone, severity, and schedule.
- Initial target/pool and fallback behavior.
- Minimum/desired/maximum retention.
- Migration steps after age thresholds.
- Required copy count where supported.
- Pressure-deletion priority and eligibility.
- Protection/hold interaction.

Precedence is explicit: evidence hold, per-recording override, event-specific
policy, camera-specific policy, selector policy, then system default.

## 5. Representative policies

| Use case | Policy |
| --- | --- |
| Ordinary continuous | Hot local pool; desired 14 days; pressure eligible after minimum 7 days |
| Entrance person event | Hot for 48 hours, then warm NAS; retain 90 days |
| Critical area | Keep two copies on distinct targets; alert rather than silently violate minimum retention |
| Protected incident | Hold indefinitely until authorized release; never pressure-delete |
| Low-priority utility view | Local spillover allowed; pressure-delete before all other classes |

These are examples, not hard-coded product defaults.

## 6. Requirements

### 6.1 Recording location schema

- New recordings store `target_uuid` plus a relative object key; APIs do not rely
  on an absolute path as durable identity.
- Existing absolute paths migrate to an automatically created default target.
- Migration is metadata-only until a lifecycle job explicitly moves a file.
- Playback, download, deletion, protection, and thumbnail lookup resolve through
  the target abstraction.
- Record policy version and placement reason for explainability.

### 6.2 Target health and allocation

- Validate and probe target paths before enablement.
- Monitor capacity, availability, writeability, and optional write latency.
- Each target owns its reserve and pressure watermarks; global pressure remains as
  a compatibility default.
- Allocation never chooses an unhealthy target unless policy explicitly allows a
  last-resort attempt.
- On failure, follow the policy's named behavior: alternate pool, local emergency
  target, pause new recording for affected cameras, or fail and emit an event.
- Emit Fleet 03 events for target and policy state changes.

### 6.3 Policy assignment and evaluation

- Policy editor uses Fleet 01 selector preview and recording predicates.
- Detect conflicting assignments before save and display effective precedence.
- Assign policy at recording creation, then retain the applied policy version.
- Object/zone-aware rules may upgrade a logical continuous segment without making
  a duplicate recording file.
- Policy simulation takes camera, recording metadata, and time and explains the
  chosen placement/lifecycle.

### 6.4 Lifecycle mover

- Migration, copy, and verification run asynchronously outside capture threads.
- Jobs persist progress and resume safely after restart.
- Copy to a temporary destination, verify size and configurable checksum, commit
  metadata atomically, then remove the old copy only when policy permits.
- Bound concurrency and bandwidth per target and schedule archival windows.
- Playback continues from the old or new verified copy during migration.
- Failed jobs retry with backoff and surface actionable error state.

### 6.5 Pressure and retention

- Retain existing age, detection, tier, protection, and per-recording override
  behavior through compatibility policies.
- Pressure cleanup evaluates only the affected target/pool and respects minimum
  retention, hold, copy-count, and pressure priority.
- If policy cannot be met, retain the recording when safe and emit a persistent
  policy-violation condition rather than silently claiming compliance.
- Provide achieved-versus-desired retention by policy and camera.

### 6.6 Capacity planning

- Estimate daily byte rate from observed history, not only configured bitrate.
- Forecast days to high watermark and expected achieved retention by target/policy.
- Identify policies whose minimum retention cannot fit current capacity.
- Forecast is advisory and states its sample window and confidence limitations.

### 6.7 Administration UI

- Target list shows health, utilization, reserve, watermarks, throughput, and jobs.
- Policy list shows selector, priority, recording predicates, lifecycle, and
  compliance state.
- Safe test verifies a target without writing camera data.
- Deleting a nonempty target requires relocation or an explicit, destructive
  procedure; disabling it does not orphan metadata.

## 7. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Implemented: target schema/resolver, default-target migration, target health, and administration UI/API |
| P1a | Implemented: selector-driven per-segment placement, named/default/pause/fail fallback, applied-policy audit metadata, policy administration UI/API, and mount-loss guards |
| P1b | Implemented: per-target pressure evaluation and cleanup; policy conflict/effective-precedence simulation |
| P2 | Implemented: durable move and retained-copy jobs, restart recovery, bounded single-worker execution, temporary copy plus SHA-256 verification, atomic commit, retry/backoff, cancellation/manual retry, per-target bandwidth limits and archival windows, API, audit, and job-status UI |
| P3 | Implemented: most-free/round-robin/priority pools, policy-driven distinct-target replication and age migration, retention bounds and pressure priority, persistent violations, 30-day observed-rate forecasts, and administration/compliance UI |
| P4 | Deferred by design: external storage adapter interface only when a concrete integration is chosen |

## 8. Acceptance criteria

- Existing installations upgrade with all recordings playable and one generated
  default target.
- Two cameras selected by different policies record to different healthy targets.
- A target outage follows configured fallback without blocking unrelated cameras.
- An interrupted migration resumes without duplicate metadata or loss of the only
  verified copy.
- Pressure cleanup cannot delete a protected/held recording or violate required
  copy count.
- Continuous video associated with an event gains richer lifecycle treatment
  without creating a second full recording.
- A 30-day observed-rate fixture produces target and policy retention forecasts.

Automated coverage exercises upgraded/embedded migrations, target and policy
placement/fallback, interrupted moves, retained copies, cancellation/retry,
pool allocation, lifecycle scheduling, pressure isolation, and a 30-day
forecast fixture. Hardware/filesystem diversity, sustained-scale throughput,
and the event-driven lifecycle-upgrade criterion remain release acceptance work.

## 9. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Move failure loses the only copy | Copy/verify/atomic metadata commit before source deletion |
| Complex precedence becomes inscrutable | Effective-policy explanation and simulator |
| Slow NAS stalls recorders | Async mover, target health, local emergency policy |
| Files moved outside lightNVR desynchronize DB | Reconciliation tooling and documented managed-root contract |

## 10. Dependencies

- Fleet 01 selectors are required for policy scopes.
- Fleet 03 is required for policy and target events before P2 is considered done.
- Fleet 04 provides bulk assignment and compliance queues.
- OPS 02 adds evidence holds that override lifecycle policy.

## 11. Current implementation boundary

P1a routes newly created continuous, rotated, retried, and detection MP4 files.
Policy assignment is cached per camera for up to 60 seconds and invalidated
immediately by policy CRUD, while target health is checked at every placement;
mount-guarded targets also re-read the live mount table before a segment path is
created. Existing files are never moved by a policy edit.

P1b evaluates each enabled target independently. A high watermark or reserve
breach deletes only complete, unprotected, pressure-eligible rows assigned to
that target, in existing retention-tier order, until the low-watermark/reserve
goal is reached or the bounded heartbeat batch is exhausted. The default
target's compatibility capacity/emergency queries are now target-filtered as
well. Target health responses expose the current pressure state and cleanup
goal.

The legacy capacity/emergency loops measure free space at `storage_path`, so
they evict only from the filesystem holding it. They additionally consider rows
that carry no target attribution -- footage written before targets existed, or
whose path the bootstrap backfill could not classify -- because those are
otherwise invisible to every pressure path; such a row is skipped when its file
turns out to live on another volume. When `record_mp4_directly` points
`mp4_storage_path` at a separate filesystem, the default target's root is that
other volume, and per-target cleanup owns its pressure instead: evicting from it
could not free space at `storage_path`, and doing so anyway would drain the
archive without ever clearing the condition that triggered the cleanup.

Policy drafts can be simulated against the current camera inventory before save.
The preview reports camera overlap with each enabled policy and shows the
effective winner using the same priority/name/UUID ordering as recording
placement.

P2 supports administrator-created and policy-created durable `move` and `copy`
jobs. Jobs snapshot both storage identities plus the destination bandwidth and
local-time archival window. The worker recovers interrupted work after restart,
copies to a job-specific temporary file, independently verifies SHA-256,
publishes atomically, and removes the source only for a committed move.
Operators can cancel active work and manually retry failed or cancelled jobs.
Verified retained copies are first-class metadata and target capacity counters;
deleting the logical recording also performs best-effort cleanup of its
retained-copy files.

P3 pools allocate only enabled, healthy members using most-free, round-robin,
or priority order. Policies can use a pool for initial spillover, require up to
eight distinct verified copies, move completed recordings after an age
threshold, declare minimum/desired/maximum retention, and set pressure-delete
priority. The idle migration worker reconciles policy intent into bounded jobs.
Minimum retention constrains normal and pressure cleanup; maximum retention
joins existing tiered eligibility while protection and per-recording overrides
continue to take precedence. Replicated policies are conservatively excluded
from pressure eviction.

`GET /api/storage-compliance` and the Storage compliance panel calculate target
and policy rates from the last 30 days of observed recording/copy history. They
show the sample window and confidence, days to each target's high watermark,
expected and achieved retention, copy deficits, and persistent minimum-capacity,
target-unavailable, copy-count, and failed-migration conditions. External
adapters remain out of scope until a concrete provider or integration supplies
real capability and credential requirements.
