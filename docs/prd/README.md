# LightNVR PRDs

This folder collects Product Requirement Documents that scope larger pieces of work.  They are deliberately written as Markdown (rather than the historical `.docx` PRDs at the repo root) so they can be reviewed in PRs and linked from issues.

Each PRD is self-contained: problem, goals, requirements, phasing, acceptance, related issues.

## Active PRDs

### UX / Design

| PRD | Topic | Driving issues |
| --- | --- | --- |
| [UXD 01 — Mobile-First UX Foundation](UXD_01_MobileFirstFoundation.md) | Cross-cutting: async feedback, settings restructure, mobile chrome, theme polish, streams page layout | [#399](https://github.com/opensensor/lightNVR/issues/399) |
| [UXD 02 — Live View Ergonomics](UXD_02_LiveViewErgonomics.md) | Per-stream playback transport, unified grid placement, mobile gestures, badge clarity | [#326](https://github.com/opensensor/lightNVR/issues/326), [#397](https://github.com/opensensor/lightNVR/issues/397), [#399](https://github.com/opensensor/lightNVR/issues/399) |
| [UXD 03 — Recordings & Timeline UX](UXD_03_RecordingsAndTimeline.md) | Scrub continuity, ruler set-diff sync, refresh affordances, mobile timeline gestures | [#331](https://github.com/opensensor/lightNVR/issues/331), [#399](https://github.com/opensensor/lightNVR/issues/399) |

The three UXD PRDs share a primitive — an `<AsyncButton>` / `useAsyncAction` hook — defined in PRD 01 and consumed by 02 and 03.  Land 01 P0 first.
