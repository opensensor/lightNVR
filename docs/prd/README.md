# LightNVR PRDs

This folder collects Product Requirement Documents that scope larger pieces of work.  They are deliberately written as Markdown (rather than the historical `.docx` PRDs at the repo root) so they can be reviewed in PRs and linked from issues.

Each PRD is self-contained: problem, goals, requirements, phasing, acceptance, related issues.

## Competitive references

| Reference | Purpose |
| --- | --- |
| [exacqVision 26.1 parity assessment](COMPETITIVE_01_ExacqVision_Parity.md) | Separates meaningful VMS workflow gaps from appliance, legacy, and vendor-specific checklist parity |

## Active PRDs

### UX / Design

| PRD | Topic | Driving issues |
| --- | --- | --- |
| [UXD 01 — Mobile-First UX Foundation](UXD_01_MobileFirstFoundation.md) | Cross-cutting: async feedback, settings restructure, mobile chrome, theme polish, streams page layout | [#399](https://github.com/opensensor/lightNVR/issues/399) |
| [UXD 02 — Live View Ergonomics](UXD_02_LiveViewErgonomics.md) | Per-stream playback transport, unified grid placement, mobile gestures, badge clarity | [#326](https://github.com/opensensor/lightNVR/issues/326), [#397](https://github.com/opensensor/lightNVR/issues/397), [#399](https://github.com/opensensor/lightNVR/issues/399) |
| [UXD 03 — Recordings & Timeline UX](UXD_03_RecordingsAndTimeline.md) | Scrub continuity, ruler set-diff sync, refresh affordances, mobile timeline gestures | [#331](https://github.com/opensensor/lightNVR/issues/331), [#399](https://github.com/opensensor/lightNVR/issues/399) |
| [UXD 05 — Investigation Workspace & Search](UXD_05_Investigation_Workspace_Search.md) | Synchronized multi-camera review, scalable metadata/region search, thumbnail drill-down, and case/export handoff | [exacqVision parity assessment](COMPETITIVE_01_ExacqVision_Parity.md) |

### Features

| PRD | Topic |
| --- | --- |
| [Recording Retention Policies](PRD_Recording_Retention_Policies.md) | Per-stream retention and deletion policies (implemented — see [the summary](../internal/SUMMARY_Recording_Retention.md) and [the API quick reference](../QUICKREF_Retention_API.md)) |

### Fleet control plane

These PRDs turn the 2026 competitive audit into independently pursuable work. The
order is intentional: later PRDs reuse the camera identities, selectors, events,
and policy primitives established earlier.

| Order | PRD | Outcome | Status |
| --- | --- | --- | --- |
| 1 | [Fleet 01 — Camera Identity & Organization](FLEET_01_Camera_Identity_Organization.md) | Stable camera identity, location hierarchy, tags, and smart collections | Implemented in 0.38.0 |
| 2 | [Fleet 02 — Scoped Authorization & Audit](FLEET_02_Scoped_Authorization_Audit.md) | Action-level permissions over reusable fleet scopes | P0–P2 substantially implemented; enforcement hardening continues; SSO deferred |
| 3 | [Fleet 03 — Event Bus & MQTT Routes](FLEET_03_Event_Bus_MQTT_Routes.md) | Durable provider-neutral events for cloud and automation consumers | P0–P2 implemented; broader P3 producers remain |
| 4 | [Fleet 04 — Fleet Explorer & Bulk Operations](FLEET_04_Fleet_Explorer_Bulk_Operations.md) | Operate hundreds of cameras through search, queues, templates, and jobs | Initial query/table and bulk organization implemented; durable jobs/templates remain |
| 5 | [Storage 01 — Multi-Target Storage Lifecycle](STORAGE_01_Multi_Target_Lifecycle.md) | Policy-driven placement, migration, retention, and capacity management | Planned |
| 6 | [ONVIF 01 — Capability Onboarding & Events](ONVIF_01_Capability_Onboarding_Events.md) | Reliable discovery, profile pairing, capability inventory, and normalized events | Planned |
| 7 | [ONVIF 02 — Metadata & Edge Recovery](ONVIF_02_Metadata_Edge_Recovery.md) | Profile M analytics and Profile G recording backfill | Planned |
| 8a | [Operations 01 — Backup & Restore](OPS_01_Backup_Restore.md) | Complete, verified, operator-safe system recovery | Planned |
| 8b | [Operations 02 — Evidence Cases & Integrity](OPS_02_Evidence_Cases_Integrity.md) | Case holds, chain of custody, and verifiable exports | Planned |
| 8c | [UXD 04 — Maps & Operator Views](UXD_04_Maps_Operator_Views.md) | Spatial navigation, shared layouts, and camera sequences | Planned |
| 8d | [Privacy 01 — Masking & Exclusion Zones](PRIVACY_01_Masking_Exclusion_Zones.md) | Camera-side privacy masks with explicit software fallback | Planned |

The order expresses dependencies, not release promises. SSO is not a standalone
near-term project: it is the final, customer-triggered phase of Fleet 02. The
trigger is a committed organizational deployment with a real IdP and named
administrative counterpart, such as an SJC-scale adoption.

The three UXD PRDs share a primitive — an `<AsyncButton>` / `useAsyncAction` hook — defined in PRD 01 and consumed by 02 and 03.  Land 01 P0 first.
