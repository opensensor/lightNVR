# Mobile viewing experience

**Status (2026-11-07)**: First slice merged in
[#840](https://github.com/opensensor/lightNVR/pull/840), reviewed on main
`c9bb7b0b`. M1–M3 below remain open. Hosted checks on the final main revision
were queued/pending at this checkpoint; local evidence below belongs to the
implementation revision.

Requested 2026-11-07 after reviewing the phone live-view screenshot. Prioritize
camera viewing, navigation and recorded playback; administration is a later
pass. Preserve the desktop experience and existing authorization behavior.

## Problems and acceptance

The original phone layout spends most of the first screen on nested padding,
transport tabs and unlabeled desktop controls. The collapsed navigator appears
as an almost empty bar. Recordings initially present a full filter sidebar and
a wide table; playback loses usable space when the phone rotates.

For the first implementation:

- At 360–768 CSS pixels, show the first camera within 260 pixels of the page
  top and use almost the full screen width. No page-level horizontal scroll.
- Keep favorites, cameras-per-page and fullscreen reachable. Put transport and
  secondary preferences behind a labeled, keyboard-accessible disclosure.
  Opening options must not replace an existing video element.
- Name the camera navigator and show the scoped camera count. Start it closed
  on phones, with its preference independent from desktop.
- Open navigation over the current page, with expanded/current-page semantics,
  Escape dismissal, focus restoration and outside-pointer dismissal.
- Start phone recordings in thumbnail view when available; preserve explicit
  URL choices, offer table view, and keep filters reachable even for empty
  results. Phone choices must not overwrite desktop preferences.
- Use inline playback with touch-sized controls and an accessible close action.
  Keep the video visible while controls scroll; landscape must fit media and
  controls inside the current viewport. Preserve actual playback, speed change
  and focus when returning to the recording list.

## Implemented first slice

Merged #840 builds on the merged API contract stack, including #838.
The live toolbar uses the same mounted controls and players at both sizes.
Mobile has a native cameras-per-page selector; desktop keeps its grid picker.
The recordings view uses separate mobile preferences, collapsed filters and
thumbnail cards. Playback uses dynamic viewport height, inline video, bounded
landscape columns, focus containment and focus restoration.

New real-server browser cases in `tests/contracts/mobileViewing.ui.spec.ts`
cover 360, 390 and 768 pixel layouts with touch emulation, a 1440 pixel desktop,
and 390×844 to 844×390 playback rotation using an actual generated MP4.
Screenshots use isolated synthetic cameras/media, not production camera access.
Validation passed: all 49 real-server browser contract cases (44 existing and
five new), 172 related unit/localization tests, and five existing component
browser cases. Web/contract type checks, lint, production build and documentation
links pass. The review URL is recorded in the PR description.

## Next slices and acceptance

The older [foundation](UXD_01_MobileFirstFoundation.md),
[live ergonomics](UXD_02_LiveViewErgonomics.md) and
[recordings/timeline](UXD_03_RecordingsAndTimeline.md) implementations remain the
baseline. Timeline pinch/pan/fling/snap and live gestures already exist. The next
slices address complete phone workflows and qualification, not a replacement
gesture implementation.

### M1: Find footage on a phone — implementation ready for review

The follow-up removes the forced Table fallback when thumbnails are disabled.
Grid uses compact metadata cards without thumbnail requests, while keeping
camera/time/duration, playback availability and protection state visible. Shared
sorting, reachable filters/retry and explicit selection work on phones. URL
choices and desktop preferences remain separate from phone preferences. Batch
delete confirmation now uses the shared accessible dialog shell.

Acceptance:

- At 360, 390 and 768 CSS pixels, both thumbnail settings support finding and
  playing footage without page overflow. At 1440 pixels desktop behavior remains
  covered. Test the thumbnail-disabled path explicitly; #840's recording case
  enables thumbnails and does not establish this behavior.
- Sorting and filtering preserve URL scope, timestamps and pagination intent.
  Empty/loading/error results keep filters and retry reachable. Interrupted media
  shows the existing explanation instead of an unexplained broken player.
- Selection is explicit, with selected count and clear/cancel affordances.
  Preserve deletion confirmation, protection and permission checks, request
  pending state, partial failures and batch download/delete completion feedback.
- Real-server browser cases cover both settings, success/error/retry, scoped
  viewers and an actual playable recording. Verify focus restoration and phone
  preference isolation as in the existing slice.

### M2: Timeline and investigation workflow — shared navigation implemented

Table, Grid, Timeline, Investigation and Plates now share the Grid-style
five-link navigation with a current-page indicator. Links retain camera/time
context and a validated local return URL carrying the original list filters,
sort and page. Timeline and Plates still operate on one camera; Investigation
can carry multiple cameras. The return URL retains the full list scope.

The broader workflow audit remains open. Audit the existing ruler and
gesture primitives in the complete flow: live camera → recordings → timeline or
investigation → playback → return. Improve camera/date context and the compact
event list where the audit demonstrates friction.

Acceptance: camera scope and time survive forward/back navigation; scrub/release
preserves playback intent; gaps and unavailable footage have clear states;
filters and controls stay reachable in portrait/landscape. Cover populated and
busy-day fixtures, keyboard/touch paths and scoped principals. Retain existing
pinch/pan/fling/snap regressions and desktop mouse/wheel behavior.

### M3: Real-device and accessibility qualification

Supported target generation is 2024 or newer, per the product decision. No
physical device or target NVR instance has been supplied for this pass. Viewport
and CPU emulation remain separate evidence. The #840 slice can be qualified
now; repeat affected paths after M1/M2. Record
device model, OS/browser versions, build/fork hashes, network and result for each
case. Test iPhone Safari and Android Chrome in portrait, landscape and installed
mode where supported: browser toolbar/keyboard changes, rotation during playback,
background/resume, slow network, disconnect/reconnect, audio listen/talk,
fullscreen and safe areas. Include long translations, RTL and enlarged text.

Acceptance requires actual media playback/recovery and accessible controls on the
named devices. A fixture or viewport pass cannot close a physical-device case.
Share these receipts with engineering F14; use F13 browser measurements for
performance budgets rather than declaring an unmeasured phone latency target.

Administration screens remain outside this pass. Browser viewport tests do not
establish real-device playback or performance qualification. Desktop regressions
remain a release gate for each mobile slice.

The [follow-up validation receipt](../validation/mobile-recordings-2026-11-07.md)
records desktop/phone browser coverage, measurements and remaining limits.
