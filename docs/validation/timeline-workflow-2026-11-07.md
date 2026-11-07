# Populated Timeline and Investigation workflow — 2026-11-07

Base: `6b346f40d90cf526aaaa1385cdd8341196bfcfdb` (merged #857).
This follows the merged [footage/navigation slice](mobile-recordings-2026-11-07.md).

## Changes demonstrated by the audit

- The Timeline playhead has a 44×44 CSS-pixel touch target while retaining the
  exact visual timestamp and pointer grab offset.
- Dropping in a gap clamps to the nearest recording edge and explains the move.
  Gap previews no longer show a nearby frame labelled with the empty-gap time.
- Media-clock updates cannot overwrite the operator's preview clock during a
  drag. Paused seeks stay paused, including a seek to the end frame. Playing
  seeks resume; Play at the end advances explicitly instead of allowing the
  browser to replay the old clip. Natural end-of-file advance remains covered.
- Moving between recordings outside the visible range recenters at the existing
  zoom. Ordinary clock updates retain a deliberately panned view. Secondary
  mouse clicks on the track no longer seek.
- Investigation's shared and per-player range controls preserve prior playback
  intent. Pointer gestures pause for the gesture and restore that intent on
  release/cancellation; keyboard steps retain intent. Explicit event jumps retain
  their existing pause behavior.
- Phone Timeline prioritizes full-width video, one primary playback row and the
  scrubber. Camera/date uses a collapsible context row; speed, protection, tags,
  zoom and file actions appear on demand under Playback options. Fullscreen is
  available on the video. Landscape gives the video the wider column and places
  controls alongside it. Disclosures keep the mounted video, expose expanded
  state and support Escape/focus restoration. Routine load-success toasts no
  longer cover the viewing surface. Enlarged text and RTL remain covered.

## Evidence

`tests/contracts/fixtures/recordings/busy-day.ts` seeds an owned temporary server
with 300 playable 30-second recordings separated by 90-second gaps, one explicit
interrupted recording, and another principal's camera. Media is real generated
96×64 H.264 at 10 fps. The Timeline response contains 301 entries for camera A;
the viewer cannot select camera B. No production footage or camera is accessed.

Three new real-server cases in `timelineWorkflow.ui.spec.ts` passed against the
production frontend build. Coverage includes 390-pixel actual dispatched touch
input, 1440-pixel mouse input, paused/playing seek, held preview-clock stability,
gap feedback, explicit and natural clip advance, 44-pixel target size, reduced-
motion pan/pinch without media seeking, RTL at 125% root text size, scoped
Timeline-to-Investigation navigation, keyboard/player scrub, 844×390 rotation,
and interrupted-footage recovery through Previous recording. The strengthened
phone case also checks video/toolbar geometry at 360 and 768 pixels, fullscreen
entry/exit without media replacement, and a fully visible landscape player after
scrolling the viewing workspace into view.

After the initial enlarged-text overflow fix, the layout was reworked to give
video priority. The final combined run covers all 20 cases: three new workflow
cases plus 17 existing mobile-viewing, recordings-navigation and evidence cases.
Results are recorded after the final layout build, separately from exploratory
failures retained in earlier logs. The strengthened phone case then passed separately, including fullscreen
and all viewport assertions; the two other workflow cases also passed their
preceding rerun.

Also passed: 974 unit tests in 83 suites (including seven added gap-resolution
cases), 30 focused component-browser cases, web and contract TypeScript checks,
frontend lint, production build and documentation links. Existing gesture and
keyboard regressions remain included in the unit/component runs. Phone and landscape Timeline, enlarged-text RTL Timeline and landscape
Investigation screenshots were inspected.

Native server binary: the recording-stop build merged in #854, SHA-256
`db5a8b5699c26a041470b53382b2d592a8705bdf1ad1fe549230ebcd9da7cf44`.
Its native source matches this base; intervening merges changed frontend code.
Local logs are `/tmp/m2-video-unit.log`, `/tmp/m2-video-component.log` and
`/tmp/m2-video-final.log`, plus `/tmp/m2-video-final-workflow.log`. Screenshots and
Playwright results are under `/tmp/lightnvr-m2-workflows/test-results/contracts`.
Selected screenshots are retained at
`/home/matteius/opensensor/lightNVR/build/hardening-resume/previews`.

Reproduce after building the native server and frontend:

```sh
LIGHTNVR_CONTRACT_BIN=/absolute/path/to/lightnvr \
  node_modules/.bin/playwright test timelineWorkflow.ui.spec.ts \
  recordingsNavigation.ui.spec.ts mobileViewing.ui.spec.ts evidence.ui.spec.ts \
  --config=tests/contracts/playwright.config.ts --project=ui
```

## Remaining acceptance

This is a populated playback correctness audit, not a camera-capacity or
first-live-frame benchmark. M2's complete live-camera handoff and M3's named
2024+ iPhone/Android/NVR qualification remain open, including Safari, installed
mode, safe areas, background/resume, listen/talk and physical network recovery.
RTL direction and enlarged text were exercised; this run did not validate every
translation. F13 still needs populated authorization-metadata/validator cost
attribution and real live-frame measurement before setting target budgets.
