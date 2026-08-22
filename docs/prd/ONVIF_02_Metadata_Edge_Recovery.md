# PRD — ONVIF Metadata & Edge Recovery

**Status**: Draft
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 7 — advanced interoperability
**Scope**: Profile M analytics metadata ingestion and Profile G edge-recording
search, playback, import, and gap backfill.

---

## 1. Problem

Many modern cameras can provide structured analytics metadata and retain video on
an SD card during an NVR or network outage. Without these capabilities lightNVR
must duplicate analytics work locally and accepts permanent recording gaps even
when the camera has the missing footage.

These are advanced features with heavy vendor variation. They should build on a
proven capability registry and event contract rather than being folded into basic
discovery.

## 2. Goals

- Ingest supported Profile M metadata with camera-native timestamps and identity.
- Normalize useful objects, classifications, counters, and analytic events.
- Search and play Profile G recordings without first copying every file.
- Detect eligible local gaps and recover matching edge recordings safely.
- Integrate imported recordings with normal timeline, storage policy, provenance,
  and audit behavior.
- Degrade explicitly when a camera implements only part of a profile.

## 3. Non-goals

- Replacing lightNVR's local detection pipeline.
- Claiming universal Profile M/G support from one tested vendor.
- Making camera SD storage the sole copy of required recordings.
- Automatically importing unlimited edge history.
- Facial recognition or identity matching.

## 4. Profile M metadata requirements

### 4.1 Metadata source and time alignment

- Discover metadata configurations and streams through ONVIF 01 capabilities.
- Pair metadata with the correct camera/media profile.
- Normalize timestamps using measured device clock offset and retain original
  device timestamp for diagnostics.
- Reconnect with backoff and detect discontinuities or metadata lag.

### 4.2 Normalized observations

When supplied by the device, normalize:

- Object track identifier and class.
- Bounding box and optional geolocation.
- Confidence, color, vehicle/license attributes, and count values.
- Analytic rule and zone/line identifiers.
- Appearance, update, and disappearance timestamps.

Unknown vendor extensions remain bounded raw metadata and never become trusted SQL
or arbitrary UI markup.

### 4.3 Event and recording integration

- Produce Fleet 03 events with explicit origin `onvif_metadata`.
- Deduplicate an equivalent PullPoint and metadata-stream event when possible while
  preserving provenance.
- Allow storage policies to match normalized class, rule, zone, or severity.
- Timeline and recording filters distinguish device metadata from local inference.
- A device observation never silently claims to be locally verified inference.

## 5. Profile G edge-recovery requirements

### 5.1 Edge inventory and search

- Discover Recording, Search, and Replay services and their supported operations.
- Inventory available recording tracks and time bounds without importing media.
- Search by camera and bounded time interval with strict result/page limits.
- Display camera-reported gaps and clock uncertainty.

### 5.2 Gap detection

- Define a local recording gap as an expected interval lacking complete playable
  local segments, with a configurable grace period.
- Exclude intentional privacy pause, disabled recording, and scheduled-off periods.
- Compare eligible gaps to camera edge availability and create proposed backfill
  jobs; automatic execution is opt-in by policy.
- Never infer completeness only from a successful search response.

### 5.3 Playback and import

- Permit authorized direct replay from an edge source when supported, clearly
  labeled as camera-hosted.
- Backfill runs as a persistent, bounded job outside capture threads.
- Import to a Storage 01 target, verify playable duration/size and optional
  checksum, then write recording metadata with source/provenance.
- Preserve the original edge recording identifier and requested/retrieved interval.
- Resolve overlap deterministically; do not create visually duplicated timeline
  coverage without explaining source alternatives.
- Interrupted jobs resume or restart safely and never replace the only valid local
  segment with a corrupt import.

### 5.4 Policy and controls

- Per-camera or selector policy controls edge search, automatic backfill, maximum
  age, daily byte budget, bandwidth, schedule, destination target, and minimum gap.
- Emit Fleet 03 proposed/started/completed/partial/failed events.
- Fleet 02 actions distinguish viewing edge video from initiating/importing it.
- Bulk backfill uses Fleet 04 jobs and stop thresholds.

## 6. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Profile M metadata discovery, parser, timestamp normalization, fixtures |
| P1 | Normalized observations, event/timeline/storage-policy integration |
| P2 | Profile G inventory, bounded search, direct replay |
| P3 | Gap detector and manual backfill jobs |
| P4 | Selector policies and guarded automatic backfill |

## 7. Acceptance criteria

- A supported metadata fixture produces stable object tracks and versioned Fleet
  03 events with original and normalized timestamps.
- The UI identifies whether an observation came from local inference or a camera.
- A controlled network outage creates a local gap; when camera footage exists, an
  operator can preview and import it into the correct timeline interval.
- Intentional recording-off and privacy intervals do not trigger automatic
  backfill proposals.
- Backfill interruption cannot produce a completed DB record pointing to a partial
  file.
- Daily transfer limits and target policy are respected across restart.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Device clock drift misaligns metadata/video | Preserve both clocks, periodic offset measurement, visible uncertainty |
| Vendors implement partial profiles | Operation-level capabilities and tested fixtures |
| Edge import saturates camera/network | Schedule, bandwidth/byte budgets, bounded concurrency |
| Overlap creates confusing evidence | Provenance, deterministic preference, alternate-source UI |

## 9. Dependencies

- ONVIF 01 capability inventory and diagnostics.
- Fleet 03 event contracts.
- Storage 01 targets and lifecycle jobs before import ships.
- Fleet 04 bulk-job framework for fleet backfill.
- Fleet 02 scoped replay/import permissions and audit.
