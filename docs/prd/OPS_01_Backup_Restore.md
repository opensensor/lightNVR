# PRD — Backup & Restore

**Status**: Draft
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 8a — operational resilience
**Scope**: Complete system-configuration backup, scheduled retention, integrity
verification, restore preflight, and operator-safe recovery.

---

## 1. Problem

lightNVR can create scheduled SQLite backups and exposes pieces of configuration
export, but an operator does not yet have one supported workflow to answer:
“Can I rebuild this NVR after a failed disk, verify the backup before I need it,
and understand exactly what restore will replace?”

A database copy alone is insufficient if external configuration, zones, users,
templates, storage policies, organization, or future event routes are omitted.
Conversely, blindly packaging secrets and media creates avoidable security and
capacity risks.

## 2. Goals

- Define a versioned, complete system-state backup manifest.
- Create manual and scheduled backups without interrupting recording.
- Verify integrity and compatibility before restore.
- Provide dry-run, explicit replacement scope, and rollback safety.
- Handle credentials deliberately and securely.
- Make backup health observable through Fleet 03 events.

## 3. Non-goals

- Backing up all recorded video by default.
- Replacing Storage 01 replication or retention.
- General operating-system imaging.
- Cloud-specific backup providers in core.
- Zero-downtime restore; controlled service interruption is acceptable.

## 4. Backup artifact

A backup is a versioned archive containing:

- Manifest: format version, lightNVR version, installation UUID, timestamps,
  included components, checksums, size, encryption metadata, and compatibility.
- Consistent SQLite database snapshot.
- Non-database configuration required for system behavior.
- Optional public assets such as floor plans and user-supplied certificates.
- Optional encrypted secret payload.
- Human-readable summary that contains no secret values.

Recorded media and generated caches are excluded. Recording metadata may be
included with the database, but restore preflight must report whether referenced
media targets exist.

## 5. Requirements

### 5.1 Backup creation

- Use SQLite's online backup facility or equivalent consistent snapshot.
- Stage the archive outside the active database and publish atomically.
- Calculate a cryptographic checksum for every archive member and the manifest.
- Support manual download, scheduled local creation, and post-success hook/event.
- Configure destination, schedule, count/age retention, and minimum free space.
- Never delete the last verified backup during retention cleanup.
- Failure does not affect recording and remains visible until acknowledged or a
  later successful backup.

### 5.2 Secret handling

- Default backup excludes reusable secrets and states what must be re-entered.
- Optional secret-inclusive backup requires authenticated reauthorization and
  encryption with an operator-supplied passphrase or explicit key facility.
- Never store the backup passphrase in the backup or routine application logs.
- Camera, MQTT, API-token, and future IdP secrets follow one documented policy.
- Restored API token hashes do not reveal plaintext tokens.

### 5.3 Backup catalog and verification

- UI lists created time, source version, size, components, encryption state,
  verification result, destination, and retention eligibility.
- Verification checks archive readability, member checksums, manifest schema, and
  database integrity without mutating the running installation.
- Allow an operator to upload and verify an external backup before restore.
- Periodically reverify retained backups on configurable schedule.

### 5.4 Restore preflight

- Parse and verify the entire artifact before stopping services.
- Report source/target versions, migration path, included/excluded components,
  missing storage targets, unresolved paths, identity collision, and secrets that
  need re-entry.
- Offer restore scopes only where semantically safe: full system is required in
  v1; selective restore may be added per component later.
- Require explicit confirmation naming the installation and replacement scope.
- Restore is authorized and audited by Fleet 02.

### 5.5 Restore execution and rollback

- Create a local pre-restore safety backup automatically.
- Quiesce state-changing services, restore into staging, run migrations and
  integrity checks, then atomically activate.
- If validation or startup fails, return to the pre-restore state and retain error
  diagnostics.
- Do not delete media files merely because restored metadata differs.
- Reconcile restored recording metadata and Storage 01 targets as a separate,
  previewable post-restore operation.

### 5.6 Events and diagnostics

- Emit backup started/succeeded/failed, verification failed, and restore outcome
  through Fleet 03 without embedding secret details.
- Health UI shows last successful backup age, last verification, and next schedule.
- Diagnostic bundle includes metadata and errors, not backup payloads.

## 6. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Manifest format, complete component inventory, manual creation and verify |
| P1 | Catalog, scheduling, retention, health events |
| P2 | Full restore preflight, safety backup, activate/rollback |
| P3 | Encrypted secret payload and optional portable migration helpers |

## 7. Acceptance criteria

- A backup from a populated fixture verifies every member and includes all declared
  system configuration categories.
- A clean compatible installation can restore the fixture and reproduce cameras,
  users, zones, organization, policies, and routes included by the manifest.
- A corrupt member fails verification before any running state changes.
- A simulated post-restore startup failure returns to the pre-restore database and
  configuration.
- Secret-excluding archives contain no camera/MQTT passwords or API token values.
- Scheduled retention never removes the only successfully verified artifact.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| “Complete” backup silently omits a new subsystem | Component registry and manifest contract tests |
| Restore bricks the NVR | Full preflight, staging, safety backup, atomic activation/rollback |
| Portable archive leaks credentials | Exclude by default; explicit encrypted secret payload |
| Metadata/media mismatch causes deletion | Read-only reconciliation report; never delete media during restore |

## 9. Dependencies

- Each preceding PRD must register its persisted state with the backup component
  inventory as it lands.
- Fleet 02 supplies restore authorization/audit.
- Fleet 03 supplies operational events.
- Storage 01 supplies target reconciliation.
