# PRD — Evidence Cases & Integrity

**Status**: Draft
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 8b — incident preservation
**Scope**: Case-centered recording holds, provenance, auditable access, and
verifiable evidence export.

---

## 1. Problem

Individual recordings can be protected, but a real investigation is usually
defined by a time interval, cameras or location, incident narrative, and multiple
exports. Protecting files one by one is error-prone, and a downloaded video alone
does not explain its source, time basis, or whether its bytes changed after export.

## 2. Goals

- Create an incident/case that groups cameras, time intervals, recordings, notes,
  and exports.
- Apply a hold to all existing and newly finalized recordings matching the case.
- Make holds override ordinary retention and pressure cleanup.
- Record provenance and a complete audit history for case-sensitive operations.
- Export evidence with a machine- and human-readable manifest and checksums.
- Keep the core lightweight and avoid pretending to replace full evidence-
  management or legal case-management systems.

## 3. Non-goals

- Legal conclusions about admissibility or jurisdiction-specific compliance.
- Video redaction/editor tooling in v1.
- Facial identification, transcription, or investigative analytics.
- Digital signatures tied to a public certificate authority in v1.
- Cloud evidence sharing portals.

## 4. Case model

A case has stable UUID, title, status, description, created/closed timestamps,
creator, assignees, labels, one or more camera selectors, one or more UTC time
intervals, notes, attachments metadata, associated recording sources, holds, and
export history.

Statuses are `open`, `closed-held`, and `closed-released`. Closing a case does not
implicitly release its hold.

## 5. Requirements

### 5.1 Case creation and matching

- Create from the Recordings/Timeline selection, the
  [UXD 05 investigation workspace](UXD_05_Investigation_Workspace_Search.md), or
  from a blank case form.
- Selector uses Fleet 01 camera UUIDs, collections, location subtree, or tags.
- Time intervals are stored in UTC and displayed with the operator's selected zone.
- Preview lists matching recordings, gaps, alternate sources, and estimated bytes.
- A case retains the selector and a materialized membership/provenance record so
  later tag/location changes do not erase historical intent.

### 5.2 Holds

- On activation, mark all matching existing recordings as held by case UUID.
- Newly finalized recordings whose captured interval overlaps an active case rule
  are held transactionally before they become retention candidates.
- A recording can be held by multiple cases and is releasable only when no active
  hold remains.
- Storage 01 retention, migration, and pressure cleanup must recognize holds as
  stronger than protection or per-recording retention.
- Extending/reducing intervals shows an impact preview; reducing never releases
  recordings held by another case.
- Release requires a distinct Fleet 02 permission, reason, confirmation, and audit.

### 5.3 Integrity and provenance

- Store source camera UUID, recording UUID, capture interval, source type, original
  relative object identity, codec/container facts, byte length, and checksum.
- Calculate checksums asynchronously with visible pending/failed state.
- Imported ONVIF edge recordings include original device/recording identifiers and
  import job metadata.
- A mismatch creates a persistent integrity alert; it never silently updates the
  expected checksum.

### 5.4 Evidence export

- Export a selected case subset without modifying source recordings.
- Package media plus JSON manifest, human-readable summary, checksums, lightNVR
  version, installation UUID, export timestamp, requester, source/provenance, and
  known timeline gaps.
- Optional clip materialization references original segments and records exact
  requested and delivered intervals.
- Re-running an export produces a new export record and manifest, not an overwrite.
- Provide an offline verification command or small portable verifier specification.
- Export authorization and download are audited.

### 5.5 Access and audit

- Fleet 02 distinguishes case view, create/edit, hold, release, export, and delete.
- Case visibility is scoped independently but cannot grant access to cameras the
  principal otherwise cannot view.
- Audit case creation, selector/time changes, notes, membership changes, checksum
  results, holds/releases, exports, and access denial.
- Case deletion is either prohibited after export or implemented as tombstoning;
  no supported API erases its audit history.

### 5.6 UI

- Case list shows status, owner, interval, camera count, held bytes, integrity
  state, and last activity.
- Timeline overlays case intervals and held recordings.
- Case detail explains gaps and alternate edge/local sources.
- Storage dashboard attributes non-deletable capacity to cases.

## 6. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Case schema, manual recording association, multi-case holds |
| P1 | Selector/time rules and finalization-time automatic hold |
| P2 | Checksums, provenance, integrity monitoring |
| P3 | Evidence export manifest and offline verification |
| P4 | Edge-source provenance integration and optional redaction follow-up |

## 7. Acceptance criteria

- A case spanning ten cameras and two intervals holds all overlapping existing and
  newly finalized recordings.
- Retention and pressure cleanup cannot remove a held recording.
- Releasing one of two holds leaves the recording held by the other case.
- A byte change after checksum creates a visible integrity failure.
- An offline verifier validates every exported file and reports a deliberately
  corrupted file.
- Export manifests identify known gaps and distinguish local from ONVIF edge source.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Holds consume all storage | Capacity impact preview, alerts, explicit administrator policy; never silent deletion |
| “Integrity” is mistaken for legal certification | Precise product language and documented checksum guarantees only |
| Selector changes alter historical membership | Materialized membership plus stored original selector/version |
| Export exposes unauthorized cameras | Intersect case scope with current Fleet 02 access at operation time |

## 9. Dependencies

- Fleet 01 identities/selectors and Fleet 02 permissions/audit.
- UXD 05 supplies synchronized review, search results, bookmarks, and the exact
  camera/time/source handoff into a case.
- Storage 01 hold enforcement and location abstraction.
- Fleet 03 operational/integrity events.
- ONVIF 02 provenance for edge imports.
