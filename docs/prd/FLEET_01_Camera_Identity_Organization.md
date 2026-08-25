# PRD — Camera Identity & Organization

**Status**: P0–P3 implemented — UUIDs, locations, normalized tags, selector/query
APIs, collections, and organization/workflow UI are present; large-fixture
performance and camera-replacement acceptance remain to reconcile
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 1 — foundation
**Scope**: Stable camera identity, physical location hierarchy, normalized tags,
saved smart collections, and the selector language consumed by later fleet PRDs.

---

## 1. Problem

lightNVR can name and tag streams, which works well for small installations. At
hundreds of cameras, names and IP addresses are mutable, tags become inconsistent,
and flat lists cannot answer ordinary operational questions such as “show every
offline exterior camera in Building C.”

Authorization, storage, event routing, bulk configuration, and health monitoring
all need to select the same sets of cameras. Implementing a different grouping
model for each feature would create incompatible policy systems.

## 2. Goals

- Give every camera an immutable UUID independent of name, address, and hardware
  replacement.
- Represent one primary physical location path per camera.
- Preserve many-to-many tags as flexible facets rather than replacing them with
  flat groups.
- Add saved static and dynamic collections for reusable fleet queries.
- Define one versioned selector format shared by authorization, event routes,
  storage policies, fleet jobs, and operator views.
- Migrate existing installations without changing recording or playback behavior.

## 3. Non-goals

- Maps, floor plans, and GIS presentation; covered by UXD 04.
- Bulk editing or configuration templates; covered by Fleet 04.
- Permission enforcement; covered by Fleet 02.
- Multi-NVR federation or multi-tenant cloud management.
- Replacing the existing substream and detection-stream fields with separate
  camera entities in v1.

## 4. Product model

| Concept | Cardinality | Purpose | Example |
| --- | --- | --- | --- |
| Camera UUID | One per configured camera | Durable identity | `0192...` |
| Display name | One mutable value | Human recognition | `North Lobby` |
| Location | One leaf; inherits ancestors | Physical organization | Campus / Bldg A / Floor 2 |
| Tag | Many per camera | Cross-cutting facets | `outdoor`, `ptz`, `critical` |
| Static collection | Explicit camera membership | Curated group | `Guard Tour A` |
| Smart collection | Saved selector | Dynamic group | `offline AND tag:entrance` |

The configured stream remains the camera record in v1. Its main, sub, and
detection URLs are profiles of the same logical camera.

## 5. Requirements

### 5.1 Stable identity

- Add a non-null UUID to every camera/stream record.
- Generate UUIDs for existing rows during migration and never derive them from
  display name, URL, or IP address.
- Use UUIDs in new APIs, event subjects, policy bindings, and audit records.
- Retain name-based API compatibility during a documented transition period.
- A camera replacement workflow may change serial, MAC, URL, and credentials
  while preserving UUID, history, policy membership, and operator layouts.

### 5.2 Location hierarchy

- Store an adjacency-list hierarchy with stable node UUID, parent UUID, name,
  type, sort order, and optional metadata.
- Suggested types are organization, site, building, floor, and area, but custom
  labels are allowed.
- Prevent cycles and destructive deletion of nonempty nodes.
- Moving a subtree updates effective selectors without rewriting every camera.
- Existing cameras migrate under a root `Unassigned` node.

### 5.3 Normalized tags

- Normalize tag identity separately from its display label.
- Tags are case-insensitive for uniqueness and preserve display casing.
- Support optional color and description metadata.
- Rename and merge tags without editing each camera record individually.
- Existing tag strings migrate losslessly.

### 5.4 Collections and selectors

- Static collections store explicit camera UUID membership.
- Smart collections store a versioned selector AST, not raw SQL.
- v1 selector predicates: camera UUID, location subtree, tag any/all/none,
  enabled state, recording mode, vendor/model, ONVIF capability, and health state.
- Boolean `AND`, `OR`, and `NOT` composition is supported with bounded nesting.
- Selector evaluation returns a total count plus a paginated camera result.
- A dry-run/preview endpoint explains why a camera matched.
- Saved collections can be private or shared; access enforcement lands in Fleet
  02, while v1 defaults shared mutation to administrators.

### 5.5 APIs and migration

- CRUD APIs for locations, tags, and collections use camera UUIDs.
- Fleet query API supports server-side pagination, sorting, facets, and counts.
- Migration is idempotent and safe to resume after interruption.
- Backup/export formats include stable identities and organization metadata.
- Existing endpoints remain functional until their UUID replacements are adopted
  by the web UI.

## 6. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Implemented: UUID schema, migration, UUID-capable camera APIs |
| P1 | Implemented: location hierarchy and tag normalization |
| P2 | Implemented: selector evaluator and fleet query API |
| P3 | Implemented: static/smart collections and organization management UI/workflow integration |

## 7. Acceptance criteria

- A migrated installation retains all streams, tags, recordings, and playback.
- Renaming or replacing a camera does not change its UUID or lose history.
- An operator can represent at least five hierarchy levels and move a complete
  subtree without editing its cameras.
- A query such as `location:Building-C AND tag:outdoor AND health:offline`
  returns correct results and facet counts from a 1,000-camera fixture.
- Authorization, event, storage, and bulk-operation code can consume the same
  selector JSON without feature-specific translations.
- Malformed or excessively complex selectors are rejected with actionable errors.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| UUID migration breaks name-based callers | Dual lookup during transition; log deprecated name use |
| Free-form tags remain inconsistent | Central tag dictionary with rename and merge |
| Selector language becomes an unsafe query engine | Typed AST, bounded depth, parameterized SQL only |
| “Camera” and “stream” terminology diverge | Preserve stream internals in v1; document the product-level camera abstraction |

## 9. Dependencies and successors

This PRD has no new-feature dependency. It is required by Fleet 02–04, Storage
01, both ONVIF PRDs, and the later operations PRDs.

## 10. Open questions

- Whether location node types should remain suggestions or be administrator-
  defined vocabulary.
- Whether camera replacement needs history in v1 or only an audited overwrite.
- Whether private smart collections belong in the server database or user-local
  preferences for the first implementation.
