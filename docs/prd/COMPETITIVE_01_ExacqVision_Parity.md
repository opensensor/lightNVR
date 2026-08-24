# Competitive Reference — exacqVision Parity

**Status**: Working assessment
**Reviewed**: 2026-08-23
**Reference baseline**: exacqVision Client 26.1 and Enterprise Manager 26.0
**Purpose**: Identify operator outcomes that lightNVR should match, gaps already
covered by active PRDs, and mature-VMS features that should remain demand-driven
or explicit non-goals.

---

## 1. Meaning of parity

Parity does not mean copying every exacqVision appliance, licensing, desktop, or
legacy-media feature. It means that a lightNVR operator can complete the important
live monitoring, investigation, evidence, fleet, storage, and recovery workflows
without discovering a product-class omission.

The comparison uses current public product documentation, not marketing-era
memory. exacqVision 26.1 documents a broad client surface including camera
onboarding and configuration, PTZ and fisheye controls, physical-security inputs
and outputs, event linking, archives, maps and views, investigations, exports,
and enterprise administration. Its current product matrix also identifies
failover, cloud identity, enterprise camera management, and a recorder/camera
health dashboard as Enterprise capabilities.

lightNVR should preserve its advantages while closing high-value gaps:

- Browser-first operation with no required thick client.
- Lightweight C core and low dependency overhead.
- Open Prometheus telemetry and provider-neutral event contracts.
- Open formats and APIs instead of proprietary evidence players.
- Useful OSS behavior without requiring the lightnvr.com control plane.

## 2. Executive conclusion

lightNVR has already built more of the VMS foundation than the old PRD status
labels imply. Stream QoE metrics and health UI, ONVIF discovery and mechanical
PTZ, two-way audio plumbing, recording schedules and pre-event buffering,
per-camera retention, TOTP, scoped roles and API tokens, audit history, camera
identity/organization/collections, and durable MQTT event routing exist today.

The material exacqVision lead is concentrated in four workflows:

1. **Investigation** — synchronized multi-camera review, iterative thumbnail and
   region-of-interest search, bookmarks/cases, and resilient evidence export.
2. **Storage and recovery** — searchable archive targets, camera edge-storage
   recovery, and recorder failover.
3. **Rules and integrations** — generic authenticated webhooks, event-to-action
   linking, physical I/O, and searchable access-control/POS/serial metadata.
4. **Operator stations** — fisheye dewarping, shared layouts/maps/sequences, and
   managed video walls.

The first two deserve roadmap weight. The latter two should be split between
portable primitives and customer-triggered integrations rather than copied as a
large compatibility checklist.

## 3. Capability map

Status meanings:

- **Comparable** — the primary operator outcome is present, even if the UX differs.
- **Partial** — useful support exists but a material workflow or safety property is
  missing.
- **Planned** — an active PRD covers the gap.
- **Gap** — no active PRD fully owns it.
- **Demand-driven** — implement only with a concrete deployment or integration.

| Capability | exacqVision reference behavior | lightNVR today | Ownership / decision |
| --- | --- | --- | --- |
| Browser live view and flexible grids | Browser, desktop, and mobile clients; custom layouts | **Comparable** browser-first live view with HLS/WebRTC and responsive grids; no native clients | Keep web-first; native apps are not parity-critical |
| Mechanical PTZ and presets | Mechanical PTZ, presets, priority, and auxiliary commands | **Comparable/partial** ONVIF move, stop, home, presets, absolute and relative control; no operator PTZ priority or auxiliary command model | Add priority only when simultaneous staffed operation requires it |
| Two-way audio | Receive and send audio from the Live window | **Partial** backchannel/talk plumbing exists; the authorization inventory still flags direct go2rtc signaling as an enforcement gap | Close the tokenized/proxied talk authorization path before claiming parity |
| Digital PTZ and fisheye | Client dewarping, panorama/dual views, and digital presets | **Partial** ordinary digital zoom/pan exists; lens-aware dewarping and presets do not | Candidate `MEDIA 01`; P2 unless fisheye demand is demonstrated |
| Scheduled, event, pre/post, and time-lapse recording | Per-device schedules, event recording, pre/post buffers, and time-lapse mode | **Partial** schedules, continuous/detection recording, and pre-detection buffering exist; explicit low-rate time-lapse mode does not | Add time-lapse to a recording-policy follow-up only if storage economics justify it |
| Stream and recorder health | Enterprise dashboard, alerts, and camera inspections | **Partial with a lightNVR advantage** open metrics, recording-gap counters, player telemetry, and a built-in health page exist; multi-site history/alerting belongs in cloud | Continue the Stream Health/QoE split between OSS telemetry and cloud aggregation |
| Semantic image-health inspection | Daily checks for blocked/moved cameras, glare, low light, and blur | **Gap** transport health cannot tell whether a connected stream is useful | Add reference-frame and sampled image-quality checks to the QoE roadmap; P1 |
| Timeline, list, and thumbnail review | Timeline/list/thumbnail search across selected sources | **Planned** recordings list/grid, thumbnails, filters, and single-stream timeline exist; synchronized multi-camera investigation is specified in [UXD 05](UXD_05_Investigation_Workspace_Search.md) | UXD 05; P0 |
| Smart motion-area search | Post-event region selection finds motion frames in recorded video | **Planned** live detection zones and detection filters are not retrospective region search | UXD 05 starts with stored motion/object metadata before decode-every-frame search |
| Analytic metadata search | Search classifications, keywords, faces, and camera analytics metadata | **Partial/planned** object labels/zones/tags exist; normalized camera-native metadata is planned | ONVIF 02 plus UXD 05 search facets |
| Bookmarks and cases | Permanent clips/bookmarks grouped into Enterprise cases | **Planned** tags and protected recordings are primitives, not an investigation case | OPS 02 owns cases, holds, provenance, and integrity |
| Evidence export | Multi-camera, scheduled/resumable export; open formats and encrypted portable player | **Partial/planned** MP4 and batch downloads exist; no case manifest, checksum verifier, resumable scheduled export, or encrypted package | OPS 02; prefer open media + a checksummed manifest over an executable player |
| Retention policy | Per-camera minimum/maximum retention | **Comparable** per-stream policies, event-aware retention, protection, and pressure cleanup | Implemented; continue hardening |
| Extended/searchable archive | SMB, NFS, S3, and cloud archives remain searchable and retrievable | **Planned** mounted storage can be used as a path, but core still has one managed target and no searchable cold tier | Storage 01; raise from generic scale work to a competitive P0/P1 outcome |
| Camera edge-storage recovery | Camera edge storage can preserve or recover recordings | **Planned** | ONVIF 02 Profile G gap detection/backfill |
| Recorder failover | Enterprise Manager controls automatic or manual recorder failover/failback | **Gap** process recovery is not recorder/data-plane failover | New `OPS 03 — Recorder Continuity & Failover`; P1 after backup/restore and storage identity are stable |
| Camera identity, organization, and collections | Enterprise camera management and grouping | **Comparable for one lightNVR instance** stable UUIDs, locations, tags, selectors, and static/smart collections are implemented | Fleet 01 complete; cross-NVR federation remains cloud/control-plane work |
| Roles, MFA, scoped access, and audit | User roles, camera permissions, OAuth/LDAP, access schedules, second reviewer, and audit | **Partial** TOTP, scoped roles/grants/tokens, and durable audit exist; OIDC, access schedules, and dual approval do not | Fleet 02; keep OIDC customer-triggered, consider second-reviewer semantics inside OPS 02 |
| Fleet health and bulk configuration | Multi-server health, saved configurations, bulk application, inspections, and scheduled updates | **Partial/planned** fleet query/table and basic bulk organization exist; templates, drift, durable jobs, and product-level upgrade orchestration do not | Finish Fleet 04; cloud owns multi-NVR rollout coordination |
| Event routing and webhooks | Event linking maps sources to recording/alarm actions; authenticated webhooks publish events | **Partial** durable provider-neutral events, selectors, suppression, and multiple MQTT destinations exist; generic HTTPS webhook and local action targets do not | Add a signed HTTPS destination to Fleet 03; keep human email/SMS external |
| Physical security and transaction data | Access control, POS, serial data, trigger inputs, and alarm outputs are searchable/actionable | **Gap / demand-driven** | Define an integration/metadata adapter only with a named access/POS partner; avoid vendor-by-vendor code in core |
| Maps, shared views, and sequences | Maps, groups, views, tours, and layouts | **Planned** | UXD 04 |
| Managed video walls / VideoPush | Operators push cameras/views to receiving display clients | **Gap / demand-driven** | A later operator-station PRD; do not block ordinary shared layouts on it |
| Bandwidth profiles and optimized remote streams | Per-client throttling and generated remote-optimized streams | **Partial** HLS/WebRTC and go2rtc transcoding are available, but there is no operator bandwidth policy or automatic profile selection | Candidate follow-up after QoE metrics can drive adaptation |
| Native face/LPR and body-worn workflows | Face registry/matching, analytic appliances, LPR ecosystem, and body-worn support | **Demand-driven** generic detection is not identity or plate recognition | Integrate through normalized metadata/providers only when a deployment supplies legal and operational requirements |

## 4. Recommended roadmap changes

### P0 — close daily operator workflow gaps

1. **Deliver [UXD 05 — Investigation Workspace & Search](UXD_05_Investigation_Workspace_Search.md).**
   It joins the existing recordings timeline and OPS 02 rather than duplicating
   them. Required outcomes are synchronized multi-camera playback, a shared UTC
   cursor, event and analytic facets, bookmark-to-case flow, gap visibility, and
   export handoff.
2. **Complete Fleet 03 P3 and add a generic signed HTTPS destination.** HTTPS is a
   transport, not a native Slack/email integration. Require certificate
   verification, bounded timeouts, retry/dead-letter behavior, event IDs, and an
   HMAC signature. Add offline, stale-frame, recording-gap, storage-pressure, and
   security producers.
3. **Finish the useful half of Fleet 04.** Prioritize exception queues, saved
   views, templates/drift, and durable previewable jobs. A fleet table without
   remediation workflows does not reach Enterprise Manager parity.
4. **Promote searchable multi-target archive in Storage 01.** Local/NFS/SMB comes
   first; an object-storage adapter follows only after query/playback semantics
   work across hot and archive tiers.

### P1 — resilience and video usefulness

5. **Extend Stream Health/QoE to semantic inspection.** Store an operator-approved
   reference frame and detect moved/blocked, blur, glare/overexposure, darkness,
   frozen-image, and severe FPS/bitrate drift. Keep inference optional and expose
   explainable scores/metrics.
6. **Write OPS 03 — Recorder Continuity & Failover.** Define failure domains,
   recorder identity, camera ownership leases, fencing, storage visibility,
   database state, failover/failback, RPO/RTO, and split-brain tests before choosing
   active/passive implementation details.
7. **Deliver ONVIF 01/02 onboarding, metadata, and edge recovery.** These close
   interoperability and outage gaps without adopting vendor-specific camera SDKs.
8. **Deliver OPS 02 evidence cases/integrity** with optional dual approval for
   release/export policies.

### P2 or customer-triggered

- Fisheye dewarping and digital presets.
- Maps, operator views, sequences, and receiving-display/video-wall control.
- OIDC/LDAP group mapping.
- Physical I/O, access control, POS, serial, intercom, LPR, face, and body-worn
  integrations.
- Native mobile/desktop applications and hardware surveillance keyboards.

## 5. Explicit non-goals for parity

Do not spend roadmap capacity matching these merely because exacqVision supports
them:

- Analog-camera licensing and appliance PoE-port management.
- CD/DVD burning or Windows executable evidence players.
- A proprietary joystick/keyboard protocol.
- A direct integration for every access-control, POS, or analytics vendor.
- A mandatory desktop client where standards-based browser playback suffices.
- Built-in provider-specific email, SMS, or chat delivery in the recording core.

These can be supported by hardware, adapters, the cloud service, or external
automation without coupling them to recording reliability.

## 6. Source baseline

Official sources reviewed:

- [exacqVision Professional feature matrix](https://www.exacq.com/products/professional/)
- [exacqVision Client User Manual 26.1](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/System-configuration/Configure-system-window)
- [Digital PTZ/Fisheye behavior](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/System-configuration/Camera-settings-window/Digital-PTZ/Fisheye-tab)
- [Event linking](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/System-configuration/Event-linking-window)
- [Authenticated webhook options](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/System-configuration/Notifications-window/Webhooks-tab)
- [Archive targets and archive search](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/System-configuration/Schedule-window/Archiving-window)
- [Smart motion-area search](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/Search-window/Performing-a-smart-search)
- [Bookmarks and cases](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/Search-window/Bookmarks)
- [Multi-camera and encrypted export](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/Search-window/Exporting-video-files)
- [Enterprise Manager 26.0 feature index](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Enterprise-Manager-User-Manual/26.0)
- [Automated semantic camera inspections](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Enterprise-Manager-User-Manual/26.0/Cameras/Automated-camera-inspections)
- [Recorder failover](https://docs.johnsoncontrols.com/exacq/r/Exacq/en-US/exacqVision-Client-User-Manual/26.1/Enterprise-management/Enterprise-System-window/Manual-failover-tab)

This is a product comparison, not a protocol conformance claim. Re-check the
source baseline when a recommendation becomes an implementation PRD.
