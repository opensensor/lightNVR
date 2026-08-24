# PRD — Investigation Workspace & Search

**Status**: Draft
**Created**: 2026-08-23
**Owner**: TBD
**Priority**: P0 — daily operator workflow
**Driving reference**: [exacqVision parity assessment](COMPETITIVE_01_ExacqVision_Parity.md)
**Scope**: Synchronized multi-camera review, scalable event/metadata search,
metadata-backed region search, thumbnail drill-down, and handoff to evidence cases
and export.

---

## 1. Problem

lightNVR can list and filter recordings, show thumbnails and detection labels,
play individual recordings, and render one stream on a day timeline. Those are
useful retrieval primitives, but an operator investigating an incident still has
to move between cameras and mentally align timestamps.

A typical question such as “show the loading dock, north hallway, and lobby from
30 seconds before this person detection through five minutes after it” currently
requires repeated filtering, separate playback, and manual time matching. Gaps,
clock uncertainty, and late-loading cameras make that reconstruction unreliable.

The current APIs also have investigation-scale limits:

- `GET /api/timeline/segments` accepts one mutable stream name and may return raw
  segments for a complete day.
- `GET /api/detection/results/{stream}` is optimized for live overlays and stores
  at most 20 results in its response structure, even when a historical range is
  requested.
- Recordings and detections retain stream names, but a historical investigation
  needs the immutable Fleet 01 camera identity that existed at capture time.
- The event delivery outbox is a retry queue, not an investigation index, and
  must not become one.

Without a dedicated investigation contract, adding a multi-camera page would
multiply single-camera requests, leak mutable identity into saved links, and
produce a UI that looks synchronized without being able to explain its gaps.

## 2. Goals

- Review multiple cameras against one shared UTC cursor and time scale.
- Keep each camera visibly aligned through footage gaps rather than silently
  jumping it to the next recording.
- Search recordings and persisted detections across cameras, collections,
  locations, tags, labels, zones, source, confidence, and time.
- Support a metadata-backed “search this area” workflow using stored bounding
  boxes and motion intervals before considering expensive video reprocessing.
- Move quickly between event results, thumbnails, and synchronized playback.
- Hand an exact camera/time selection to existing batch download and, when OPS 02
  lands, to a durable case, hold, and verifiable export.
- Enforce Fleet 02 scope before counts, facets, thumbnails, or media reveal the
  existence of a camera.
- Remain usable on embedded hardware through bounded queries, timeline
  aggregation, lazy media loading, and an explicit decoder budget.

## 3. Non-goals

- Evidence holds, chain of custody, checksums, or export manifests; OPS 02 owns
  those guarantees.
- A federated query across independent NVR databases; that belongs in the
  lightnvr.com control plane after the local contract is stable.
- Retrospectively running object detection across every recorded frame in v1.
- Face identification, license-plate OCR, person re-identification, or natural-
  language video search.
- Video editing, redaction, annotation drawing, or transcoded highlight reels.
- Claiming frame-accurate synchronization when camera clocks or source timestamps
  cannot support it.
- Replacing the lightweight Recordings list or single-camera timeline for simple
  tasks.

## 4. Product model

### 4.1 Investigation query

An investigation query contains:

- Authorized camera selector or fixed camera UUID list.
- UTC start and end timestamps.
- Optional event types, object labels, zone UUIDs, confidence range, recording
  tags, capture methods, source types, and protected state.
- Optional normalized image-space region for one camera.
- Stable sort and opaque result cursor.

The query is ephemeral by default. URL state may reproduce it, but every load
re-evaluates current authorization. Saving evidence is a separate bookmark/case
operation, not an accidental side effect of searching.

### 4.2 Investigation track

Each selected camera has one track with:

- Capture-time camera UUID, current display name, and location context.
- Recording intervals and source type (`local`, future `archive`, or future
  `camera_edge`).
- Detection/motion intervals and result markers.
- Explicit gaps, incomplete/open segments, and unavailable media.
- Camera clock offset/quality when ONVIF 01 provides it.

Tracks share a UTC axis. Local time and daylight-saving transitions are display
concerns only; they do not change ordering or cursor arithmetic.

### 4.3 Playback set

The workspace may show up to 16 selected tracks, but only a bounded visible set
actively decodes video. The default desktop decoder budget is four; mobile
defaults to one. An operator may change the budget when the browser and host can
support more, but the product never begins decoding every selected track merely
because it is present in the timeline.

One panel is the primary panel. It supplies audio, frame-step behavior, and the
reference state shown in the detail pane. At most one recording audio track plays
at a time.

### 4.4 Result, bookmark, and case

- A **result** is a search hit tied to camera UUID, time interval, source, and any
  available recording/detection identifiers.
- A **bookmark** is a durable review marker with title, note, cameras, interval,
  and representative result. It does not silently promise retention.
- A **case** is the OPS 02 object that applies holds, preserves provenance, and
  owns evidence exports.

Before OPS 02 P0 exists, the UI must say that a bookmark can outlive its media and
offer the existing explicit Protect action for currently matched recordings.
After OPS 02 P0, “Preserve as case” is the preferred durable workflow.

## 5. Requirements

### 5.1 Entry points and camera selection

- Open an investigation from:
  - A recording, thumbnail, detection, or timeline position.
  - Selected rows/cards in Recordings.
  - A camera collection, location subtree, or Fleet selector.
  - A Live View camera with a configurable before/after window.
- Use Fleet 01 camera UUIDs in workspace state. Names are labels, not identity.
- Default the time range around the initiating event, not to an arbitrary full
  day. The initial event window is 30 seconds before and five minutes after.
- Show selected camera count and estimated media/query cost before expanding a
  large selector into tracks.
- Limit interactive playback selection to 16 tracks in v1. Search may cover a
  larger authorized selector and add chosen results to the playback set.
- Preserve query, cursor, primary camera, selected tracks, visible time range, and
  filters in navigation-safe state. Shared links contain no credentials or media
  tokens.

### 5.2 Multi-camera timeline

- Render one virtualized row per selected camera on a shared horizontal UTC axis.
- Distinguish continuous, scheduled, detection, motion, and manual recording;
  detection/event markers must not color an entire enclosing continuous segment.
- Render gaps explicitly. Clicking or playing through a gap keeps that panel on
  the shared time and displays “No footage,” rather than advancing it alone.
- Provide two clearly named playback modes:
  - **Wall-clock**: time advances continuously; panels show gaps.
  - **Skip common gaps**: jump only when no selected camera has footage until the
    next earliest shared-data boundary.
- At wide zoom levels, return aggregate availability/event buckets. Fetch raw
  segment boundaries only for the visible window and adjacent preload margin.
- Abort superseded requests during pan/zoom and ignore stale responses.
- Overlay known recording gaps and clock-quality warnings from telemetry/ONVIF
  without implying that an absent metric proves continuous footage.
- Keep URL date/time parsing DST-safe by storing UTC instants and using elapsed
  timeline offsets, building on the existing timeline utilities.

### 5.3 Synchronized playback coordinator

- Maintain one authoritative UTC cursor and map it independently to each panel's
  recording segment and media offset.
- Play, pause, seek, and speed changes apply to every active panel.
- A coordinated seek waits for active panels up to a bounded barrier. Panels that
  miss the barrier are marked late and join when ready; one slow source must not
  freeze the entire workspace indefinitely.
- After a seek settles, active panels with usable timestamps remain within 500 ms
  of the shared cursor at 1x. If drift exceeds the threshold for three seconds,
  correct it and show a diagnostic counter; never hide chronic source-clock
  uncertainty.
- Inactive panels show a representative frame or explicit unloaded state and join
  at the current shared cursor when activated.
- Only the primary panel emits audio. Changing primary camera cross-fades or
  cleanly switches audio without starting multiple tracks.
- Frame-step affects the primary panel and pauses the set. Other panels seek to
  the primary panel's resulting UTC timestamp when it is known.
- Continue existing keyboard behavior and add camera-panel focus navigation with
  visible focus state and no collision with browser shortcuts.

### 5.4 Investigation search contract

- Add a dedicated, paginated server-side search API. Do not fan out through the
  live detection endpoint and do not query the Fleet 03 delivery outbox.
- The request accepts a fixed camera UUID list or Fleet selector, UTC interval,
  filters, limit, and opaque cursor. The default/max result limits are 100/500.
- Results contain:
  - Capture-time camera UUID and current authorized display context.
  - Start/end timestamps and source/event type.
  - Recording ID when media is locally addressable.
  - Detection ID, label, confidence, normalized bounding box, zone UUID, track ID,
    and producer source when available.
  - Thumbnail reference or status, never inline base64 media.
  - Media availability and known-gap state.
- Return authorized facet counts for camera, location, label, zone, source,
  capture method, and recording tag. Counts must be computed after scope filtering.
- Use stable cursor pagination under concurrent inserts. A repeated page request
  must not reorder already returned results.
- Explain incomplete search coverage when detection metadata has expired, an
  archive adapter cannot search a tier, or a camera's historical identity is
  unresolved.
- Search metadata storage has its own explicit retention setting and health
  metric; it must not silently inherit the transient event-delivery outbox TTL.

### 5.5 Capture-time identity and schema

- Persist camera UUID on newly created recordings and detections. Do not resolve a
  renamed stream at query time and assume it is the historical source.
- Backfill historical rows only where the mapping is unambiguous. Mark unresolved
  rows with their legacy stream name and expose that uncertainty in results.
- Preserve existing recording and detection IDs and APIs during migration.
- Index the actual query shapes for camera/time, time/type, label/time, zone/time,
  recording link, and stable cursor ordering. Validate plans with million-row
  detection fixtures rather than adding speculative indexes.
- Store boxes in normalized source-image coordinates and retain the dimensions or
  transform metadata needed to compare them to a later user-drawn region.
- Keep camera-native ONVIF metadata and local/API detections distinguishable by
  producer source while presenting normalized labels and zones to the UI.

### 5.6 Event and metadata search UI

- Combine a time-range control, camera/collection selector, and progressive
  filters for event type, label, zone, confidence, source, tag, and capture method.
- Show a result histogram above the timeline and a paginated result rail beside or
  below playback. Selecting a hit moves the shared cursor without discarding the
  query.
- Result cards show camera, local display time plus unambiguous offset/zone,
  labels, confidence, source, duration, availability, and thumbnail state.
- A “previous/next result” command works while video retains focus.
- Allow adding a result's camera or nearby Fleet 01 location cameras to the active
  playback set without rebuilding the query manually.
- Empty states distinguish no matches, no authorized cameras, expired metadata,
  unavailable archive, and query failure.

### 5.7 Metadata-backed region search

- From one primary camera, let the operator draw a rectangle or polygon in
  normalized image coordinates and choose overlap semantics:
  - Bounding-box center inside region.
  - Any intersection.
  - Minimum intersection percentage.
- Match persisted detection boxes and explicit motion regions within the selected
  time range. Reuse zone geometry utilities where possible without modifying the
  camera's live detection-zone configuration.
- Label results as **metadata search**. If the interval lacks spatial metadata,
  state that it was not searched; absence of a hit is not proof that nothing
  crossed the region.
- Do not decode historical video synchronously inside an HTTP request. A future
  offline scan must be a bounded, cancellable job with separate resource policy.
- Save a region as investigation query state or promote it to a configured
  detection zone through a separate authorized camera-configuration action.

### 5.8 Thumbnail drill-down

- Show lazily loaded thumbnails for result intervals and recording boundaries,
  reusing existing thumbnail generation/cache behavior.
- Support iterative time narrowing: select a thumbnail or bracket and regenerate
  samples for the narrower interval until the operator reaches the desired event.
- Preserve the active camera, query filters, and shared cursor while narrowing or
  backing out.
- Bound generation concurrency and queue length. Visible playback/recording work
  has priority over investigation thumbnails.
- A failed thumbnail shows retry and metadata-only navigation; it does not remove
  the result.

### 5.9 Bookmark, case, and export handoff

- Create a bookmark from the shared cursor, selected interval, result, or set of
  tracks. Store immutable camera UUIDs plus the original query summary.
- Make retention semantics explicit at save time. A bookmark alone is not a hold.
- “Protect current recordings” uses existing `evidence.protect`, previews the
  exact overlapping recordings, and reports partial failures.
- “Preserve as case” hands camera UUIDs, UTC intervals, selected results, notes,
  gaps, and available source identifiers to OPS 02 without client-side expansion
  of a dynamic selector.
- “Download selected footage” may use the existing authorized batch-download job
  for an immediate convenience export. Label it separately from a verified OPS 02
  evidence export.
- Export/case buttons remain visible but disabled with an actionable permission
  explanation when the user can replay but cannot export or protect.

### 5.10 Authorization, privacy, and audit

- Require `recordings.replay` for timeline availability, search results,
  thumbnails, and playback on each camera.
- Require `recordings.export` for download/export and `evidence.protect` for
  protection. OPS 02 introduces case-specific actions when implemented.
- Resolve selectors and facet counts after authorization. Explicitly requesting an
  unauthorized camera returns a scoped denial; broad selectors omit unauthorized
  members without revealing their count.
- Re-authorize media requests and bookmark/case/export mutations server-side;
  possession of workspace URL state never grants access.
- Do not put credentials, signed media references, private selector definitions,
  or notes in URLs, logs, analytics, or browser history.
- Audit bookmark/protection/case/export mutations and denials with correlation IDs.
  Do not flood durable audit storage with every cursor tick or ordinary play/pause.

### 5.11 Responsive and accessible operation

- Desktop uses timeline tracks plus a resizable playback/result workspace.
- Mobile defaults to one active video, a collapsible track list, and a bottom
  result sheet; the shared timeline remains horizontally navigable.
- All timeline markers, results, filters, and panels are keyboard reachable with
  accessible names and visible focus.
- Do not encode camera, source, gap, or event state by color alone.
- Maintain 44 px touch targets and respect reduced-motion preferences.
- Preserve operator context after a recoverable API/media failure and expose a
  retry at the failed scope instead of reloading the whole page.

### 5.12 Performance and observability

- A 16-camera, 24-hour initial view returns aggregated timeline data under 2 MiB;
  raw segments load only for the visible window.
- On the reference x86 test host, a 1,000,000-row detection fixture returns the
  first 100 authorized results for a selective camera/time query in under one
  second at p95. Record the query plan with the benchmark.
- A four-panel seek does not wait more than two seconds for its synchronization
  barrier; late panels remain identifiable and recover independently.
- Never decode or continuously fetch media for offscreen/inactive tracks.
- Limit concurrent thumbnail generation per client and globally; cancellation
  releases queued work.
- Export metrics for search latency/result count, timeline response size, media
  seek latency, sync drift/corrections, late panels, thumbnail queue depth/failure,
  and incomplete-coverage reasons without high-cardinality user/query labels.

## 6. API and data direction

Exact endpoint names can change during implementation, but the contracts should
separate these concerns:

| Contract | Direction |
| --- | --- |
| Investigation search | `POST /api/investigations/search` with selector/list, filters, UTC interval, stable cursor |
| Multi-track availability | `POST /api/investigations/timeline` with camera UUIDs, UTC window, and requested detail level/bucket size |
| Bookmark CRUD | `/api/investigation-bookmarks`; shared with OPS 02 as case-addressable review markers, without owning evidence holds |
| Playback | Reuse recording playback initially; add a time-addressed camera UUID contract only when archive/edge source resolution requires it |
| Thumbnail | Reuse recording thumbnails, then add result/time-addressed thumbnails without embedding media in search responses |

Both query endpoints return a coverage block that identifies requested range,
searched metadata range, unavailable source tiers, unresolved legacy identities,
and clock-quality summary.

## 7. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Capture-time camera identity; aggregated multi-track timeline API/UI; shared UTC cursor; bounded synchronized playback |
| P1 | Paginated investigation search, authorized facets, result rail/histogram, thumbnail navigation |
| P2 | Metadata-backed region search, iterative thumbnail drill-down, coverage explanations |
| P3 | Durable bookmarks and explicit protect/case/batch-download handoff; depends on OPS 02 for holds and verified export |
| P4 | Storage 01 archive and ONVIF 02 edge-source resolution; cloud federation remains separate |

P0 may ship behind an experimental flag while single-camera Timeline remains the
default. P1 is required before the workspace replaces any existing navigation.

## 8. Acceptance criteria

- Starting from one detection, an operator opens four related cameras at a window
  beginning 30 seconds before the event with no manual timestamp entry.
- Play, pause, 1x/2x speed, and seek operate on all active panels; after settling,
  panels with usable timestamps remain within 500 ms of the shared UTC cursor.
- If one camera has a two-minute outage, its panel shows the gap while the other
  cameras continue in wall-clock mode; no footage is silently skipped.
- Skip-common-gaps jumps only across an interval where every selected track lacks
  media.
- A 16-camera full-day view stays below the initial-response size budget and does
  not load 16 video decoders.
- A scoped user cannot infer unauthorized cameras through facet counts, timeline
  rows, thumbnails, media responses, or shared investigation URLs.
- Searching one million detection rows meets the p95 target and returns stable,
  nonduplicated pages while new detections are inserted.
- Drawing a region returns intersecting stored boxes and clearly reports intervals
  without spatial metadata as unsearched.
- Creating a bookmark explains that it is not a hold; protection and case actions
  show exact impact and require their own permissions.
- An existing single-camera timeline link and selected-recording link continue to
  work after P0 schema/API migrations.

## 9. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Multi-camera playback overwhelms embedded clients | Explicit decoder budget, inactive thumbnails, no automatic transcoding fan-out |
| UI appears synchronized while cameras have bad clocks | Shared UTC model, drift metrics, clock-quality warnings, honest 500 ms target |
| Full-day raw segment fan-out exhausts memory or response buffers | Bucketed overview, visible-window detail, request/response caps |
| Historical stream rename attributes footage to the wrong camera | Persist capture-time UUID; backfill only unambiguous rows; expose legacy uncertainty |
| Detection endpoint/outbox is reused as a search index | Dedicated paginated query path and explicit metadata retention/health |
| Region search is mistaken for exhaustive visual search | “Metadata search” labeling and coverage report; no synchronous frame scanning |
| Bookmark implies evidence retention | Explicit semantics; existing Protect preview; OPS 02 case hold for durable preservation |
| Search facets leak camera existence | Authorization before expansion, counts, and aggregation; scope-focused tests |
| Timeline refactor regresses mature single-camera behavior | Separate workspace route/state first; reuse tested UTC/DST utilities; compatibility tests |

## 10. Dependencies

- Fleet 01 camera UUIDs, selectors, locations, tags, and collections.
- Fleet 02 `recordings.replay`, `recordings.export`, and `evidence.protect`
  enforcement.
- UXD 03 single-camera timeline utilities and interaction behavior.
- OPS 02 for case holds, provenance, integrity, and verified exports.
- Storage 01 for searchable archive source resolution after local P0–P3.
- ONVIF 01 clock quality and ONVIF 02 metadata/edge sources when available.
- Fleet 03 event schemas may normalize result types, but its delivery outbox is
  not a dependency or query store.

## 11. Open questions

- Should the default desktop decoder budget remain four on every host, or be
  raised only after a browser capability probe and explicit operator choice?
- Does bookmark CRUD belong in core before OPS 02, or should P3 ship only with
  the first case schema to avoid two durable concepts?
- Which metadata retention default balances useful historical search against
  SQLite growth on small installations?
- Should time-addressed playback remain recording-ID based through P3, or should
  P0 introduce camera UUID + UTC source resolution before Storage 01 needs it?
- What reference hardware should gate the embedded response-time budget in
  addition to the x86 million-row search benchmark?
