# Mobile viewing experience

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

`codex/mobile-live-ux` builds on the API contract review stack, including #838.
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

## Next slices

1. **Find footage on a phone:** compact recording list when thumbnails are
   disabled, mobile sorting and batch actions, and camera/time filters that do
   not require navigating a desktop table. Keep explicit selection and deletion
   confirmation. Include empty, loading, error and interrupted-media states.
2. **Timeline and investigation:** thumb-friendly time navigation and scrubbing,
   clear camera/date context, a compact events list, and a predictable route back
   to live view. Carry camera scope and timestamps across these transitions.
3. **Real-device qualification:** iPhone Safari and Android Chrome, portrait and
   landscape, browser toolbar/keyboard changes, installed mode, rotation during
   playback, background/resume, slow network and connection recovery. Verify
   audio, fullscreen and safe areas on actual devices. Test long translations,
   RTL and enlarged text before calling the mobile viewing program complete.

Administration screens remain outside this pass. Browser viewport tests do not
establish real-device playback or performance qualification. Desktop regressions
remain a release gate for each mobile slice.
