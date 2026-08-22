# PRD — Privacy Masking & Exclusion Zones

**Status**: Draft
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 8d — privacy controls
**Scope**: Permanent privacy masks, camera-side ONVIF preference, explicit
software fallback, and detection exclusion zones distinct from existing privacy
pause and inclusion/detection zones.

---

## 1. Problem

lightNVR can pause a camera for privacy and can configure detection zones, but
these do not solve two different needs:

1. Permanently obscure a neighbor's window, keypad, workspace, or public area from
   live view, recordings, and snapshots.
2. Ignore a region for motion/analytics while leaving its pixels viewable.

Software masking can force decode and re-encode, defeating passthrough and raising
CPU usage. Many cameras can apply masks themselves, but capabilities and behavior
vary. Operators need an explicit, testable choice rather than a mask that appears
in one output and leaks through another.

## 2. Goals

- Model privacy masks and analytic exclusion zones as separate entities.
- Prefer device-side ONVIF privacy configuration when supported.
- Offer software burn-in only after clear compatibility/performance validation.
- Apply privacy masks consistently to every relevant output.
- Explain when masking changes passthrough, codec, resolution, latency, or CPU use.
- Audit creation, change, disablement, and deletion.

## 3. Non-goals

- Automatic face/person/license-plate redaction.
- Editing historical recordings made before a mask existed.
- Claiming a software overlay is a legally certified privacy system.
- Replacing whole-camera privacy pause.
- Treating detection inclusion zones and exclusion zones as interchangeable.

## 4. Concepts

| Control | Changes pixels? | Changes analytics? | Typical use |
| --- | --- | --- | --- |
| Privacy pause | Stops capture/processing | Yes | Temporarily disable whole camera |
| Privacy mask | Permanently obscures configured region | May, depending on device pipeline | Neighbor window |
| Detection inclusion zone | No | Analyze only selected region | Driveway |
| Detection exclusion zone | No | Ignore selected region | Moving tree/road |

## 5. Requirements

### 5.1 Mask geometry and identity

- Store stable mask UUID, camera UUID, name, enabled state, source mode, polygon,
  coordinate reference, creation/update actor/time, and applied capability state.
- Geometry uses normalized coordinates and is validated for bounds, minimum area,
  vertex count, and self-intersection.
- Editing UI uses a current snapshot with explicit age and aspect ratio.
- Transform geometry deliberately when main/sub/detection profiles differ in
  aspect ratio; refuse an ambiguous crop rather than guessing silently.

### 5.2 Device-side masks

- ONVIF 01 capability probe reports mask/config support at operation level.
- Read existing device masks when supported and distinguish external/unmanaged
  masks from lightNVR-managed masks.
- Preview proposed geometry and apply through an authenticated, audited operation.
- Read back configuration and verify persistence after camera reboot where the
  device permits.
- Report whether the device applies the mask to main/sub streams, snapshots,
  metadata, and edge recordings; unknown is not shown as guaranteed.
- Do not delete or overwrite unmanaged masks without explicit confirmation.

### 5.3 Software masking

- Software mode is unavailable until the selected hardware/software pipeline can
  sustain the configured stream load in a validation test.
- UI explains that passthrough is disabled and estimates CPU/resource impact.
- Mask is burned before any live output, recording write, snapshot, detector input
  when configured, event thumbnail, or downstream restream can observe pixels.
- Fail closed for configured privacy outputs: pipeline failure must not silently
  fall back to an unmasked stream.
- Low-resolution proxies and cached thumbnails are invalidated on mask changes.
- Existing historical recordings remain unchanged and the UI states this clearly.

### 5.4 Exclusion zones

- Add named normalized polygons independent of privacy masks and inclusion zones.
- A camera may have multiple exclusion zones with schedule and applicable detector
  source where supported.
- Generic motion and local object-event generation ignore excluded pixels/objects
  according to documented overlap semantics.
- Default overlap rule: suppress only when the tracked object's configured anchor
  point lies inside an exclusion zone; future alternatives require explicit UI.
- Exclusion changes emit an audit record but do not imply pixel privacy.

### 5.5 Verification and visibility

- Configuration screen provides tested previews for main, sub, detection, snapshot,
  and recording paths.
- Camera detail shows `device mask`, `software mask`, `unverified`, or `failed`
  state; a generic “privacy enabled” badge is insufficient.
- Fleet 03 emits apply/verify/failure events without mask images or sensitive
  geometry by default.
- Fleet 04 can identify mask drift or device reset across selected cameras.

## 6. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Separate exclusion-zone model and local detection integration |
| P1 | ONVIF device-mask capability/read/apply/read-back for tested cameras |
| P2 | Cross-output verification, fleet drift, reboot persistence tests |
| P3 | Guarded software masking on supported pipelines with fail-closed behavior |

## 7. Acceptance criteria

- A configured exclusion zone suppresses eligible local detections while its
  pixels remain visible.
- A device-side mask is read back and verified on every camera profile/output the
  UI claims it covers.
- Unmanaged device masks survive lightNVR edits unless explicitly selected.
- Software mode cannot be enabled without acknowledging passthrough/resource impact
  and passing configured validation.
- No live, recording, snapshot, thumbnail, or restream output marked protected can
  fall back to unmasked pixels after a simulated masking-pipeline failure.
- UI clearly states that recordings created before mask activation are unchanged.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Device claims masking but omits an output | Per-output verification and conservative “unknown” state |
| Software masking overloads the NVR | Preflight benchmark, capacity limit, explicit opt-in |
| Aspect-ratio transform leaks an edge | Normalized geometry, preview each profile, refuse ambiguous crop |
| User assumes exclusion means privacy | Separate terminology, icons, APIs, and explanatory copy |

## 9. Dependencies

- Fleet 01 camera UUIDs and Fleet 02 configuration permissions/audit.
- ONVIF 01 capability registry for device-side masks.
- Fleet 03 operational failures and Fleet 04 drift reporting.
- Software masking phase requires an explicit performance design and is not a
  prerequisite for device-side masking.
