# PRD — Live View Ergonomics

**Status**: Draft
**Created**: 2026-04-22
**Owner**: TBD
**Driving signal**: [#326 (CDx4f3kCAf3Y)](https://github.com/opensensor/lightNVR/issues/326), [#397 (AndyIsHereBoi)](https://github.com/opensensor/lightNVR/issues/397), and the Live View / mobile items from [#399](https://github.com/opensensor/lightNVR/issues/399).
**Scope**: `web/js/components/preact/LiveView.jsx`, `HLSVideoCell.jsx`, `MSEVideoCell.jsx`, `GridPicker.jsx`, `FullscreenManager.jsx`, plus a new per-stream "Playback profile" model.

---

## 1. Problem

Live View is the page operators use most, often from a phone. Today it has three distinct ergonomic gaps:

1. **No control over which playback transport is offered.** A single global "WebRTC enabled" checkbox is the only knob (#397). Operators who find HLS too jittery and WebRTC too flaky want to force MSE; some want WebRTC for one camera and HLS for another. Right now they can't.
2. **Grid placement isn't durable.** Reordering streams in WebRTC view doesn't stick when the user switches to HLS/MSE, and vice versa (#326). Different state stores per renderer mean the user reorders the same cameras three times.
3. **Mobile gestures don't exist.** No swipe-to-cycle, no pinch-to-zoom on a tile, no long-press menu. Fullscreen requires precision-tapping a small icon. Pulling to refresh does nothing.

Layered on these: the recording-state badge on each tile is ambiguous (#399 — "Enabled (Detection)" hides the constant-recording case), and there's no visual cue when a tile is unhealthy vs just buffering.

## 2. Goals & non-goals

**Goals**
- Per-stream playback transport preference + per-instance fallback chain.
- Single source of truth for grid placement, shared across renderers.
- First-class mobile gestures with discoverable affordances.
- Clear, glanceable health/recording badges.

**Non-goals**
- New playback engine. We're managing existing HLS/MSE/WebRTC, not adding LL-HLS or DASH.
- Audio mixing across tiles. Continue to mute by default; audio-on is per-tile and exclusive (already today).
- PTZ redesign — that's its own epic.

## 3. Users & top tasks

| Persona | Task | Today's friction |
|---|---|---|
| Phone user reacting to a doorbell ping | Open the app → see front-door tile in 2 taps → expand to full screen | Front door isn't first; fullscreen tap target is small |
| Power user with 8 cameras on a wall display | Arrange tiles, expect arrangement to persist regardless of renderer | Arrangement diverges across HLS/MSE/WebRTC views (#326) |
| Operator on flaky LTE | Force MSE for the wired indoor cams, accept HLS fallback for cellular outdoor cams | Single global toggle; no per-stream choice (#397) |
| Anyone | Tell at a glance "is this tile recording? is the stream healthy?" | Badges are text-only and ambiguous |

## 4. Requirements

### 4.1 Per-stream playback transport preference

Add a new field to `stream_config_t` (and `db_streams`) and a UI control on the Stream Edit modal:

- **`playback_transport`** enum: `auto` (default), `webrtc_only`, `mse_only`, `hls_only`, `webrtc_then_mse`, `mse_then_hls`.
- **`auto`** behavior matches today's: try WebRTC if globally enabled, else MSE, else HLS.
- The Live View tile reads the per-stream value and only attempts the listed transports in order, surfacing a "fallback used" badge when it had to drop down.

UI:
- Stream edit modal grows a "Playback transport" select, default Auto.
- Settings → Streams Defaults gets the same control as a global default.
- The existing global "WebRTC enabled" checkbox becomes a 3-checkbox group (WebRTC / MSE / HLS) per #397, gating which transports the system advertises at all. Per-stream overrides cannot select a globally-disabled transport — UI greys those out with a tooltip.

**Acceptance**: Disable WebRTC globally; per-stream "WebRTC only" overrides surface a clear "WebRTC disabled at server" warning instead of silently degrading.

### 4.2 Unified grid placement model

Today the placement state lives in three independent places (one per renderer). Consolidate:

- Single canonical `grid_layout` per user (or per-device if logged out demo mode), keyed by stream UUID, storing `(slot_index, layout_size)`.
- `LiveView.jsx` reads/writes this single store regardless of active renderer.
- Migrating: on first load after this lands, merge the three legacy stores using last-write-wins per stream.
- Layout-size changes (e.g. 4 → 9 cells) preserve relative order; new slots fill from the top.
- Touch reorder: drag-handle becomes long-press anywhere on the tile (mobile) / corner grip (desktop).

**Acceptance**: User reorders in HLS view, switches to MSE view, sees same arrangement. Resizing browser shrinks/grows the grid without losing user-defined slot assignments where slots remain.

### 4.3 Mobile gestures

| Gesture | Action |
|---|---|
| Tap on tile | Reveal tile chrome (badges, fullscreen, audio toggle) for 4 s |
| Double-tap on tile | Toggle fullscreen |
| Long-press on tile | Open context menu (Mute/Unmute, Snapshot, Open Recordings for this stream, Reorder mode) |
| Pinch-zoom inside fullscreen tile | Digital zoom on the video element (record offset for snapshots) |
| Pull-down at top of grid | Refresh streams list + reload manifests |
| Swipe left/right on a fullscreen tile | Cycle to next/previous stream in current grid order |

Discoverability: first time each gesture would have helped (e.g. user taps fullscreen icon manually 3×), show a one-time tip toast: "Tip: double-tap the tile to fullscreen."

### 4.4 Recording & health badges

Replace the current text-only badge with a 2-glyph layout in the tile's top-right:

- **Recording glyph**: filled red dot (constant), red dot with motion arc (detection), red dot with clock (schedule), or stacked combinations. Tooltip / aria-label spells it out: "Constant + Detection".
- **Health glyph**: green ✓ (streaming, no recent stalls), yellow • (buffering / recovering), red ⚠ (no stream / consecutive errors). Drives off existing `stream_state` from the Health API.

Both glyphs live in a 28×28 pill on desktop, full-size 36×36 on mobile.

**Acceptance**: Fixes #399 specific item ("recording indicator only says 'Enabled (Detection)' even if constant + detection are enabled"). All combinations of the three recording modes render distinctly.

### 4.5 Fullscreen polish

- Native picture-in-picture button on tiles where the browser supports it (Chrome/Edge desktop, Safari mobile iOS 14+).
- "Always-fullscreen on tap" preference (off by default) for kiosk-style wall displays.
- Fullscreen exit gesture: swipe-down on mobile, Esc on desktop. Today's pinch-out collision risk is mitigated by reserving pinch for digital zoom only.

## 5. Data model changes

```
ALTER TABLE streams
  ADD COLUMN playback_transport TEXT NOT NULL DEFAULT 'auto'
    CHECK (playback_transport IN
      ('auto','webrtc_only','mse_only','hls_only',
       'webrtc_then_mse','mse_then_hls'));
```

`system_settings` adds:
- `transport_webrtc_offered` (bool, default true)
- `transport_mse_offered` (bool, default true)
- `transport_hls_offered` (bool, default true)
- `live_view_grid_layout_v2` (JSON, single source of truth)

Migration: run-once on boot (similar to T14 from the go2rtc work) — merges legacy `grid_layout_webrtc/mse/hls` keys into the new field, last-write-wins.

## 6. Phasing

| Phase | Scope | Estimate |
|---|---|---|
| P0 — Glyph badges + fullscreen polish | Pure-frontend; no schema work | 2 days |
| P1 — Per-stream transport (#397) | Schema + migration + UI + LiveView routing | 3–5 days |
| P2 — Unified grid model (#326) | Migration + LiveView refactor | 3 days |
| P3 — Mobile gestures | Add gesture library, instrumentation for the tip toasts | 4–5 days |

## 7. Acceptance criteria

- A user can set `front_door` to `mse_only` and `garage` to `webrtc_then_mse` independently and Live View honors both.
- Grid arrangement persists across renderer changes (issue #326 closed).
- All recording-mode combinations render with distinct, accessible badges.
- Mobile reachability: every interactive element on Live View is reachable with one thumb on a 6.7" phone in portrait.

## 8. Risks

| Risk | Mitigation |
|---|---|
| Per-stream transport explodes the test matrix | Start with the 3 single-transport modes; chained fallback in P2 |
| Gesture library bloats the bundle | Hand-roll the small set we need (tap, dbltap, longpress, swipe) — no library |
| Grid migration loses placements when slot count differs | Stable-sort by legacy slot then by name, fill empties top-to-bottom; show a one-time "We restored your layout" toast with an Undo |
| iOS Safari fullscreen quirks | Use the existing `FullscreenManager.jsx` with a Safari-specific pseudo-fullscreen path |

## 9. Related issues

- [#326 — Live View grid placement not remembered](https://github.com/opensensor/lightNVR/issues/326)
- [#397 — Per-stream-type checkboxes for dashboard](https://github.com/opensensor/lightNVR/issues/397)
- [#399 — UI and styling issues](https://github.com/opensensor/lightNVR/issues/399) (specifically: logo→home, recording badge clarity, mobile reachability)
