# Mobile footage workflow and browser baseline — 2026-11-07

Base: `f7d391e7c67aadadd9c8e227e0c3b0e15b5376e7` (merged #853).

Table, Grid, Timeline, Investigation and Plates share the Grid-style five-link
navigation. Links carry camera/time context and retain the originating recording
list's filter/sort/page state in a local return URL. Timeline and Plates accept
one camera; Investigation can retain multiple. Returning retains the full list
scope. Explicit Table/Grid URL choices survive separate phone/desktop defaults.

With thumbnails disabled, Grid now renders compact metadata cards and makes no
thumbnail requests. Sorting, error retry and filters remain reachable. Protection
buttons report pending state. Batch deletion uses the shared accessible dialog
shell, with initial Cancel focus, containment, Escape and focus restoration.

## Validation

- 967 unit tests in 83 suites, including navigation URL and existing timeline /
  gesture regressions.
- 24 focused component-browser cases covering keyboard/live navigation,
  interrupted playback notices and stale media responses.
- 15 real-server browser contract cases: existing mobile viewing and evidence
  workflows plus seven new footage-navigation cases. The strengthened seven-case
  rerun also passed, waiting for loaded camera selections before forward links.
- Widths 360, 390, 768 and 1440; thumbnail-free playback decodes actual synthetic
  MP4, no page overflow, explicit view URLs and phone preference isolation.
  Existing thumbnail-enabled playback includes portrait/landscape rotation.
- Error/retry, selection/cancel/confirmation focus and scoped viewers are covered.
  Screenshots inspected for phone Grid/Investigation and desktop Timeline.
- Web/contract/test TypeScript checks, frontend lint and production build passed.

The owned fixture uses synthetic cameras, media and credentials. No production
camera or personal footage was accessed. Successful test screenshots remain in
`test-results/contracts/recordingsNavigation*` in the worktree.

## Remaining acceptance

M1's implementation and automated workflow checks are ready for review. M2's
shared navigation is implemented; the full populated/busy-day timeline audit,
touch scrubbing across gaps and live-to-investigation workflow qualification
remain open. No existing gesture implementation was replaced.

M3 targets iPhone/Android and NVR hardware from 2024 onward, per the product
decision. Actual device models, OS/browser versions, networks, installed mode,
keyboard/safe-area behavior, background/resume and audio recovery still require
physical-device access. Chromium viewport/CPU emulation does not close them.

F13's browser harness provides a loading baseline; it does not isolate validator
CPU or measure first live-camera frame. Named target capacity, extended storage /
upgrade qualification and F15 production acceptance remain open. No validator
split is justified solely by these workstation observations.

See [browser harness instructions](../../tests/performance/browser-startup.md).


## Serial production-browser observation

Chromium `151.0.7922.34`, Node `v24.22.1` on
AMD Ryzen 9 7950X 16-Core Processor (32 logical CPUs,
131,701,190,656 bytes RAM), Linux `7.0.0-38-generic`.
Three samples per row; phone uses touch and 4× CPU emulation at 390×844,
desktop 1440×900. Cold/warm HTTP caches use no request interception.

| Profile | Page | Cache | Controls median ms | Script median ms | Heap median MiB | Resource transfer median KiB |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| desktop | index.html | cold | 382 | 37 | 6.0 | 747 |
| desktop | index.html | warm | 408 | 16 | 9.3 | 748 |
| desktop | recordings.html | cold | 430 | 42 | 6.5 | 539 |
| desktop | recordings.html | warm | 467 | 29 | 10.8 | 515 |
| desktop | settings.html | cold | 315 | 38 | 6.2 | 486 |
| desktop | settings.html | warm | 335 | 24 | 10.5 | 484 |
| desktop | api-docs.html | cold | 1504 | 192 | 109.9 | 1021 |
| desktop | api-docs.html | warm | 337 | 29 | 134.9 | 1021 |
| phone-emulated | index.html | cold | 489 | 151 | 6.2 | 759 |
| phone-emulated | index.html | warm | 423 | 62 | 9.5 | 753 |
| phone-emulated | recordings.html | cold | 490 | 154 | 6.0 | 539 |
| phone-emulated | recordings.html | warm | 472 | 107 | 11.4 | 515 |
| phone-emulated | settings.html | cold | 416 | 140 | 6.5 | 496 |
| phone-emulated | settings.html | warm | 465 | 117 | 11.2 | 498 |
| phone-emulated | api-docs.html | cold | 5102 | 792 | 113.0 | 1021 |
| phone-emulated | api-docs.html | warm | 4093 | 172 | 84.7 | 1021 |

- desktop, cold: decoded recording frame median 610 ms from navigation.
- desktop, warm: decoded recording frame median 640 ms from navigation.
- phone-emulated, cold: decoded recording frame median 797 ms from navigation.
- phone-emulated, warm: decoded recording frame median 740 ms from navigation.

Frame timing includes the automated Play action and uses a 96×64 H.264 MP4;
it is not camera-to-screen latency. Script timing covers the current document,
including recording playback where applicable. Counters reset on navigation;
subtracting a prior document's counters is invalid. Transfer sizes exclude the
document; cache revalidation can still transfer substantial bytes. Controls
are measured when the primary selector is visible, not when every background
action finishes. Heap is observed current usage, not peak or retained memory.
Three samples per cell are a baseline, not a tail-latency or phone budget.

The native fixture uses the recording-stop build merged in
[#854](https://github.com/opensensor/lightNVR/pull/854), binary SHA-256
`db5a8b5699c26a041470b53382b2d592a8705bdf1ad1fe549230ebcd9da7cf44`. Every built web asset is hashed in the receipt.
Raw receipt: `/tmp/lightnvr-mobile-experience/build/browser-startup-counters/results.json`
(SHA-256 `f480d465e39182f7861960135705dbf4298fe61e960e4996490d6d36466da6a8`).

Earlier exploratory receipts are retained but superseded: the initial metric
calculation subtracted counters across documents, producing invalid negative
warm-load CPU values. Only the corrected serial run above is used here.
