# PRD — Fleet Explorer & Bulk Operations

**Status**: Draft
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 4 — fleet operator experience
**Scope**: A scalable fleet explorer, health queues, configuration templates,
desired-state drift reporting, and durable bulk jobs for hundreds of cameras.

---

## 1. Problem

Card grids and one-camera-at-a-time forms become operationally expensive around
hundreds of cameras. Operators need to find exceptions, understand fleet health,
and apply a controlled change to all matching cameras—not click through 900
nearly identical pages or copy settings from a “golden camera.”

## 2. Goals

- Keep fleet browsing responsive with at least 1,000 configured cameras.
- Make unhealthy or noncompliant cameras easier to find than healthy ones.
- Support server-side facets, saved views, and organization-tree navigation.
- Replace ad-hoc copy-settings flows with versioned templates and overrides.
- Execute bulk changes as previewable, durable jobs with per-camera results.
- Enforce Fleet 02 permissions on query results and operations.

## 3. Non-goals

- Video wall customization or spatial maps; covered by UXD 04.
- Multi-NVR federation.
- Automatic firmware deployment.
- A generic orchestration language or arbitrary shell-command jobs.
- SSO.

## 4. Primary experience

The default Fleet page combines:

- Location tree with total, online, degraded, and offline counts.
- Virtualized/server-paginated table.
- Search and facets.
- Saved personal/shared views.
- Smart operational queues.
- Selection summary and safe bulk action bar.

Required columns include camera, location, tags, address, vendor/model, stream and
recording status, last frame, current storage policy, template compliance, and
last error. Operators can choose and persist visible columns.

## 5. Requirements

### 5.1 Server-side fleet query

- Page, sort, filter, facet, and aggregate on the server after authorization.
- Search by display name, UUID, IP, MAC, serial, manufacturer, model, tag, and
  location path.
- Return aggregate counts for tree nodes and active facets.
- Share the Fleet 01 selector grammar; saved views store selector plus columns and
  sort rather than result IDs.
- Use stable cursors or stable sorting so live health changes do not duplicate rows.

### 5.2 Operational queues

Ship defined smart collections for:

- Offline cameras.
- Authentication failures.
- Recording gaps.
- Stale frames or low effective FPS.
- Storage policy unmet.
- Configuration drift.
- Newly discovered/unclaimed devices after ONVIF 01.

Each queue shows count, oldest unresolved condition, severity, and relevant bulk
remediation. Health state definitions are versioned and explainable.

### 5.3 Configuration templates

- A template contains a versioned subset of camera configuration, not secrets by
  default.
- Templates may cover recording, retention-policy assignment, detection defaults,
  transport, audio, schedules, and selected ONVIF profile choices.
- Bind a template to a Fleet 01 selector with precedence rules.
- Per-camera overrides are explicit, visible, and removable.
- Drift compares desired effective configuration to actual configuration and
  distinguishes intentional overrides from errors.
- Updating a template creates a new version and a previewable reconciliation job;
  it does not mutate all cameras synchronously inside the save request.

### 5.4 Bulk jobs

- A job stores creator, action, selector snapshot, matched count, policy/template
  version, creation time, state, progress, and per-camera result.
- Preview shows affected cameras and changes before confirmation.
- Confirmation states whether selection is a fixed UUID snapshot or all cameras
  matching at execution; v1 defaults to a fixed snapshot for safety.
- Workers use bounded concurrency and support cancel-before-start, retry-failed,
  and export-results.
- Partial failure never reports overall success without a visible breakdown.
- Destructive operations require stronger confirmation and Fleet 02 action.

### 5.5 Discovery staging

After ONVIF 01, the Fleet page includes an inbox for discovered devices with
endpoint, IP/MAC, serial, vendor/model, capability summary, duplicate suspicion,
and last-seen time. Operators can batch claim devices, apply a credential profile,
pair media profiles, assign location/tags/template, test, and commit.

Credentials remain outside templates and job result logs.

### 5.6 Performance and accessibility

- Virtualize rendered rows; never create a DOM card for every camera.
- Update only changed health rows through polling deltas or event subscription.
- Preserve keyboard navigation, accessible names, focus, and 44px touch targets.
- Mobile prioritizes operational queues and search; dense bulk administration may
  use a responsive table/detail pattern rather than horizontal overflow.

## 6. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Fleet query API, virtualized table, location tree, facets |
| P1 | Health queues and saved views |
| P2 | Durable bulk-job framework and safe initial actions |
| P3 | Configuration templates, assignments, overrides, drift |
| P4 | ONVIF discovery staging and batch claim integration |

## 7. Acceptance criteria

- A 1,000-camera fixture loads its first authorized page and aggregate counts
  without returning all camera rows to the browser.
- Filtering by location, tags, health, model, and template compliance composes
  correctly and can be saved.
- An action over 900 matched cameras requires one preview and one confirmation,
  runs with bounded concurrency, and provides 900 individual outcomes.
- Retrying a partially failed job retries only eligible failures.
- A template change shows exact before/after fields and never overwrites explicit
  camera overrides silently.
- Users cannot infer counts or identities outside their Fleet 02 scopes.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Bulk jobs cause a fleet-wide outage | Preview, fixed snapshots, bounded concurrency, canary option, stop threshold |
| Templates hide surprising inheritance | Effective-value explanation and explicit override markers |
| Live health churn destabilizes paging | Stable sort/cursors and delta updates |
| UI becomes an enterprise dashboard maze | Default to exception queues and progressive disclosure |

## 9. Dependencies

- Fleet 01 is required for identities, organization, collections, and selectors.
- Fleet 02 is required before privileged bulk actions ship.
- Fleet 03 supplies eventual near-real-time health events.
- ONVIF 01 supplies discovery staging and capability data.
