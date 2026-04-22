# PRD — Recordings & Timeline UX

**Status**: Draft
**Created**: 2026-04-22
**Owner**: TBD
**Driving signal**: [#331 (CDx4f3kCAf3Y)](https://github.com/opensensor/lightNVR/issues/331), the Recordings/Timeline items from [#399 (AndyIsHereBoi)](https://github.com/opensensor/lightNVR/issues/399).
**Scope**: `web/js/components/preact/RecordingsView.jsx`, `web/js/components/preact/timeline/*` (TimelinePage, TimelineRuler, TimelineCursor, TimelineControls, TimelinePlayer).

---

## 1. Problem

The Recordings list and the Timeline player are the two surfaces operators touch when something *did* happen. Both have rough edges that compound during incident review:

1. **Scrubbing the timeline kills playback.** If you're playing a clip and drag the playhead to look earlier, playback pauses (#399). Operators expect "scrub-and-keep-playing" because every other video player works that way (YouTube, VLC, native iOS).
2. **Stale recordings linger on the timeline ruler.** Background API tells the page about new recordings but never about deletions, so the ruler keeps showing colored bars for clips that are gone (#331). Hard refresh is the only fix today.
3. **No refresh button on Recordings.** Operators waiting for a clip to finalize have no way to manually re-poll without a full page reload (#399).
4. **No async feedback on filters and bulk actions.** Apply a filter, batch-delete, batch-download — UI doesn't show pending state, and a slow query feels broken (#399, general).
5. **Mobile timeline is unusable for fine work.** Cannot pinch-zoom the ruler; cannot fling-scroll; the cursor is hard to grab with a thumb.

## 2. Goals & non-goals

**Goals**
- Scrub continuity: playing state survives ruler interaction.
- Live consistency: the timeline ruler reflects the current set of recordings without a full reload.
- Manual refresh affordances on Recordings.
- Touch-grade gestures on the timeline (pinch-zoom, fling-scroll, larger touch targets).
- Universal async feedback for every filter / batch action.

**Non-goals**
- Cross-stream timeline (multi-camera unified ruler) — separate epic.
- Server-side persistence of UI filter state — a future "saved views" PRD.
- Editing/trimming recordings inside lightNVR.

## 3. Users & top tasks

| Persona | Task | Today's friction |
|---|---|---|
| Homeowner reviewing motion clips on phone | Open Timeline, scrub to the alert time, watch | Scrub pauses; mobile cursor is a 6 px bar |
| Power user auditing yesterday's events | Filter by stream + zone, bulk-download survivors | Filter UI gives no feedback; deletion artifacts persist on the ruler |
| Anyone after a manual delete | See the ruler reflect the deletion immediately | Need to F5; #331 |

## 4. Requirements

### 4.1 Scrub-and-play continuity (#399)

- Track `wasPlaying` state when the user grabs the cursor (pointerdown/touchstart on the cursor or a click anywhere on the ruler).
- During the drag, render an inline preview frame (cheap: nearest keyframe from the manifest) attached to the cursor, but do not pause/resume the player on every move.
- On pointerup/touchend: if `wasPlaying`, seek and immediately call `play()`. If paused going in, stay paused.
- Keyboard parity: `Space` toggles play/pause; `←` / `→` step ±5 s; `Shift+←` / `Shift+→` step ±60 s. None of those should change the play/pause state.

**Acceptance**: Manual test from #399 — playing a clip, drag the cursor backward 10 s, release, playback continues from the new position with no perceptible re-buffer pause beyond what the network requires.

### 4.2 Timeline ruler stays in sync (#331)

- The TimelinePage already re-fetches the day's recording list on a periodic timer. Today it only adds new entries. Change to a **set-diff**:
  - `existing = state.recordings`
  - `incoming = api response`
  - For each recording in `existing` not in `incoming`: remove from the ruler.
  - For each in `incoming` not in `existing`: add to the ruler.
  - Diff key: `id`, fall back to `(stream_name, start_ts, end_ts)` if id missing.
- Removal animates briefly (100 ms fade) so the operator notices instead of "huh, did I miss something?"
- Soft-deleted recordings (if/when that's a thing) get a struck-through appearance instead of vanishing.

**Acceptance**: With the Timeline open, run `DELETE /api/recordings/<id>` from a terminal; within the next polling tick (≤ 10 s) the bar disappears from the ruler with the fade animation. Closes #331.

### 4.3 Manual refresh on Recordings (#399)

- Header gets a refresh icon button. Tap or click triggers an immediate re-query with the current filters; uses the `<AsyncButton>` primitive from PRD 01 so the icon spins while pending.
- Mobile: same button is also exposed via pull-to-refresh on the list.
- The query itself doesn't change; only the trigger does.

### 4.4 Universal async feedback

Every list-mutating action goes through the `<AsyncButton>` / `useAsyncAction` primitive (defined in PRD 01):

- "Apply Filters" button — pending state while query runs.
- Batch-Delete confirmation modal — second-step button locks during the request; results render inline ("Deleted 12 of 14; 2 failed: see log").
- Batch-Download — progress bar wired to existing batch-download progress endpoint (`test_batch_delete_progress` already exists for the delete side).

### 4.5 Mobile timeline gestures

- **Pinch-zoom on the ruler** changes the time scale (1 hr → 5 min → 1 min). Cursor stays anchored at its current time.
- **Two-finger pan** (or one-finger fling on the ruler track) horizontally scrolls the visible window.
- **Cursor handle**: 36×36 invisible hit area centered on the visible 4 px line so thumbs can grab it.
- **Snap-to-recording** when the cursor releases within 500 ms of an edge of a recording bar.

## 5. Phasing

| Phase | Scope | Estimate |
|---|---|---|
| P0 — Scrub continuity | TimelineCursor + TimelinePlayer; pure-frontend | 2 days |
| P1 — Set-diff sync | TimelinePage data layer; closes #331 | 1–2 days |
| P2 — Refresh + async feedback | Wire the AsyncButton primitive; assumes PRD 01 P0 has shipped | 1 day |
| P3 — Mobile gestures | Pinch/fling/snap on the ruler | 3–4 days |

## 6. Acceptance criteria

- Closes #331: deleted recordings disappear from ruler within one polling tick.
- Closes the relevant items in #399: scrub continuity, refresh button, async feedback.
- Mobile time-to-cursor (median) < 1 s on a 375 px viewport.
- 0 reported "ghost" recordings on the ruler in beta after P1.

## 7. Risks

| Risk | Mitigation |
|---|---|
| Diffing runs every poll on a busy day → CPU spike | Cap diff candidates to the visible time window; index by id |
| Pinch-zoom collides with browser pinch-zoom on iOS | Use `touch-action: none` on the ruler track; reserve `pinch-zoom` only when both fingers land on the track |
| Scrub-keep-playing causes excessive seeks while dragging | Throttle network seeks to commit only on pointerup; show local preview-frame during drag |

## 8. Related issues

- [#331 — Timeline doesn't auto-remove deleted recordings](https://github.com/opensensor/lightNVR/issues/331)
- [#399 — UI and styling issues](https://github.com/opensensor/lightNVR/issues/399) (Recordings refresh, scrub continuity, async feedback)
