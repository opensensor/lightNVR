# PRD — Maps & Operator Views

**Status**: Draft
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 8c — spatial and role-oriented operation
**Scope**: Floor-plan maps, nested navigation, shared/personal camera layouts, and
operator sequences built on fleet identities and permissions.

---

## 1. Problem

Names, filters, and tables are efficient for administration but not always for
responding to an incident. Operators reason spatially: which entrance is next to
the alarm, which camera sees the corridor, and what nearby view should be opened.
Different roles also need stable layouts and sequences without reorganizing the
underlying camera inventory.

## 2. Goals

- Place cameras and operational entities on uploaded floor/site plans.
- Navigate maps hierarchically from site to building to floor.
- Show live health and event state without rendering hundreds of live videos.
- Let authorized users create personal and shared layouts and camera sequences.
- Keep organization, tags, collections, maps, and views as distinct concepts.
- Respect Fleet 02 visibility at every level and aggregate.

## 3. Non-goals

- Full GIS, 3D building modeling, CAD editing, or automatic camera calibration.
- Replacing the Fleet 04 administrative table.
- A dispatch or access-control system.
- Persistently decoding every camera visible on a map.
- Multi-NVR federation in v1.

## 4. Product model

- **Map**: image/SVG asset, coordinate space, optional parent map and location node.
- **Map entity**: camera UUID, child-map link, location marker, or supported event
  input with position, icon, label, and optional orientation/FOV wedge.
- **Operator view**: ordered tile layout of cameras or collections with playback
  preferences and labels.
- **Sequence**: ordered cameras/views plus dwell time and transition behavior.

Maps describe space. Views describe presentation. Collections describe membership.
Locations describe canonical physical hierarchy.

## 5. Requirements

### 5.1 Map administration

- Upload bounded PNG/JPEG/SVG floor-plan assets with safe content handling.
- Associate a map with a Fleet 01 location and optionally a parent map marker.
- Place cameras through drag/drop; store normalized coordinates independent of
  display resolution.
- Configure orientation and approximate FOV wedge manually.
- Warn when a camera is placed on multiple maps but permit it for overview maps.
- Asset replacement preserves entity coordinates when aspect ratio is unchanged
  and requires preview/remap otherwise.

### 5.2 Operator map

- Display camera health, recording state, active event severity, and selected
  detection status through lightweight markers.
- Clicking a permitted camera opens preview/live/detail actions; unauthorized
  cameras and counts are not leaked.
- Navigate through child-map markers and breadcrumbs.
- Filter visible entities by tag, state, event type, and saved collection.
- Cluster or aggregate markers at overview scale rather than rendering unreadable
  overlap.
- Update state via Fleet 03 deltas or bounded polling.

### 5.3 Views and sequences

- Create personal and shared layouts using stable camera UUIDs and saved
  collections.
- Store grid size, tile order, labels, and playback transport preference without
  changing camera configuration.
- A dynamic collection view fills deterministic slots and explains membership
  changes.
- Sequences support ordered items, dwell duration, pause, previous/next, and
  optional skip-offline behavior.
- Shared view/sequence mutation requires Fleet 02 permission; viewing is the
  intersection of view membership and current camera access.

### 5.4 Incident navigation

- Event links can open the relevant map centered on the source camera.
- Map marker exposes adjacent cameras selected by spatial placement, not inferred
  solely from similarly named tags.
- Timeline/recordings link preserves camera and event time context.
- Map state never changes evidence or alert acknowledgement implicitly.

### 5.5 Performance and accessibility

- Initial map load does not initiate live streams until requested.
- Marker updates are batched; asset caching is versioned.
- Provide list and keyboard alternatives to every map action.
- Use shapes/icons in addition to color for state.
- Touch selection and controls meet 44px target guidance.

## 6. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Shared/personal operator views using stable camera UUIDs |
| P1 | Static floor-plan upload, camera placement, health markers |
| P2 | Nested maps, event filters, contextual navigation |
| P3 | Sequences, dynamic collection views, incident deep links |

## 7. Acceptance criteria

- A site/building/floor map hierarchy containing 900 cameras remains navigable
  without loading 900 video players.
- Moving/renaming a camera preserves its map and view placement by UUID.
- An operator cannot see a marker, aggregate count, preview, or event for a camera
  outside their Fleet 02 scope.
- A shared sequence skips an offline camera when configured and preserves order
  across restart.
- All map functions have keyboard/list equivalents.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Map becomes a heavy video wall | Markers first; streams only on demand |
| Floor plans expose sensitive layout | Fleet 02 map permissions and safe backup/export handling |
| Dynamic views surprise operators | Stable ordering and membership-change explanation |
| SVG carries active content | Sanitize or rasterize uploads; disallow scripts/external resources |

## 9. Dependencies

- Fleet 01 identities, hierarchy, tags, and collections.
- Fleet 02 permission filtering.
- Fleet 03 event/health deltas.
- Fleet 04 query and health-state definitions.
