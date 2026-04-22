# PRD — Mobile-First UX Foundation

**Status**: Draft
**Created**: 2026-04-22
**Owner**: TBD
**Driving signal**: [#399 (AndyIsHereBoi)](https://github.com/opensensor/lightNVR/issues/399) plus general usability friction across the web UI.
**Scope**: Cross-cutting UX work touching every page (Header, navigation, Settings, Streams, button feedback, theming).

---

## 1. Problem

LightNVR's web UI grew organically. Several frictions affect every page and disproportionately hurt the **mobile** experience that operators reach for during field checks (driveway gate, parked vehicle, doorbell event):

- **Async actions feel broken.** Buttons don't disable while the request is in flight, and there's no spinner. Users tap repeatedly, accidentally triggering multiple saves (#399).
- **Settings is a wall of text.** Single long scrolling form with no sections; users cannot find what they came for.
- **Mobile chrome doesn't respect the user.** Logo isn't a home link; transient toast notifications cover the bottom navigation; scrollbar is invisible against the dark theme.
- **Streams page wraps poorly.** Card grid forces horizontal scroll on desktop even when there's space (#399 screenshot).

Today's frame is "desktop first, mobile if it happens to fit." Mobile is where the high-frequency tasks happen — quickly checking a feed during a notification — so this PRD inverts the priority.

## 2. Goals & non-goals

**Goals**
- Eliminate accidental double-submits via consistent async UI primitives.
- Make Settings navigable in < 5 seconds for any setting.
- Make every page primary-usable on a 375px-wide viewport (iPhone SE / older Androids) without horizontal scroll.
- Establish design tokens (spacing, color, touch target) that future PRs can lean on.

**Non-goals**
- Visual rebrand. Keep current daisyUI/tailwind palette; tighten only.
- Full design system. We're cleaning up the load-bearing pieces, not shipping a Figma kit.
- Native apps. PWA enhancements are out of scope (covered by a future PRD if pursued).

## 3. Users & top tasks

| Persona | Primary task | Where today's UX hurts |
|---|---|---|
| Homeowner on phone, just got a doorbell ping | Open Live View, find the right camera, confirm | Logo doesn't go home from sub-pages; toast covers nav; grid placement isn't preserved |
| Pro installer at a customer site | Adjust a stream setting, save, verify | Settings is one giant form; double-saves from spammy taps; no save-confirmed feedback |
| Power user at desktop | Bulk-edit zones, manage retention, audit logs | Settings still slow to navigate; Streams card grid wastes horizontal space |

## 4. Design principles

1. **Mobile-first sizing.** Touch targets ≥ 44×44 px. All primary controls thumb-reachable on a one-handed grip.
2. **Always say what's happening.** Every async action shows pending state within 100 ms; success/failure resolves visibly.
3. **Idempotent UI.** A second tap on a pending action is a no-op, not a re-submit.
4. **Find before scroll.** Long forms get sectioned navigation + a search-by-label affordance.
5. **Respect the chrome.** Toasts, modals, and FABs never overlap primary navigation.

## 5. Requirements

### 5.1 Async-action primitive

A shared `<AsyncButton>` (or `useAsyncAction` hook) wrapping every save/delete/retry in the app:

- `onClick` returns a promise; while pending: disable the control AND swap the label/icon for a spinner.
- A second click during pending is silently dropped.
- On resolve: brief inline check icon for ~1.5 s, then return to idle.
- On reject: inline ⚠ + red border + tooltip with the error message.
- Optional `confirmText` prop turns it into a 2-step click (mobile-safe destructive action).

**Acceptance**: Save Settings, Apply Filter, Delete Recording, Restart go2rtc, all PTZ buttons all use the primitive. Manual test: rapid-tapping any of these never triggers more than one server call.

### 5.2 Settings page restructure

Convert the current ~2200-line single-form view into a tabbed layout matching the System page's existing tab style (operator already knows it):

- **Tabs** (mobile: horizontal scrollable; desktop: vertical sidebar): General, Storage, Streams Defaults, Detection, go2rtc, MQTT, Auth/Security, Appearance, Advanced.
- **In-tab section anchors** (e.g. inside go2rtc: "Process", "Networking", "Override").
- A single search box at the top filters labels live; matched fields scroll into view with a brief highlight.
- Save button is sticky on mobile (bottom of viewport above nav); always-visible-and-disabled-when-no-changes on desktop.

**Acceptance**: From a cold load, an operator finds the "Web Thread Pool Size" field in < 5 s using either tab nav or search. Settings page LCP doesn't regress (we render only the active tab's form fields).

### 5.3 Mobile chrome hygiene

- **Logo is a Link to `/` (live view) on every page**, including Settings, Recordings, Timeline, System.
- **Toast container** moves to top-center (mobile) / top-right (desktop). Never overlaps the bottom nav. Toasts respect `safe-area-inset-bottom` on iOS notch/gesture-bar phones.
- **Bottom nav** (mobile only) gets a backdrop blur + bg so transparent toasts can't visually fight with it.

### 5.4 Theme polish

- Custom-styled scrollbar tokens (`webkit-scrollbar` + `scrollbar-color` for Firefox) using muted-foreground at 60% opacity. Same in light and dark themes.
- Add a "Reduce motion" preference that honors `prefers-reduced-motion` AND lets the user override; disables modal/sheet animations and the toast slide-in.

### 5.5 Streams page card layout

- On viewports ≥ 640 px, cards lay out in a responsive grid (`grid-cols-[repeat(auto-fill,minmax(360px,1fr))]`) instead of a fixed table that overflows.
- Each card collapses denser fields (binding, source URL) behind a "Details" disclosure on mobile; primary metadata (name, status, recording mode, snapshot) stays visible.
- Recording-mode badge clarified: instead of "Enabled (Detection)" alone when constant + detection are both on, show "Constant + Detection" (#399 specific item). Pattern: comma-joined modes in display order [Constant, Detection, Schedule].

### 5.6 Touch targets & spacing audit

- All buttons and links meet the 44×44 hit-area floor (use Tailwind `min-h-11 min-w-11 flex items-center justify-center` or padding equivalents). Add an automated lint via a small script that crawls compiled CSS and flags interactive elements below threshold.
- Form fields get `min-h-11` and the focus ring uses `ring-2 ring-primary` so it's visible on both themes.

## 6. Phasing

| Phase | Scope | Estimate |
|---|---|---|
| P0 — primitives | `AsyncButton`/`useAsyncAction`, toast position, logo→home, scrollbar styling | 2–3 days |
| P1 — Settings | Tab restructure + search + sticky save | 3–5 days |
| P2 — Streams page | Grid layout + recording-mode badge + mobile collapse | 2 days |
| P3 — Audit | Touch-target lint + reduce-motion + safe-area pass | 1–2 days |

## 7. Acceptance criteria (measurable)

- 0 reported double-submits in beta after P0.
- Settings page time-to-first-interaction unchanged (±10%) after P1.
- 0 horizontal scrollbars on Streams, Settings, Recordings, Timeline at 375/414/768/1024/1440 px viewports.
- All interactive elements ≥ 44×44 px (lint passes).
- Lighthouse mobile score ≥ 90 on Live View, Recordings, Settings.

## 8. Open questions — DEFAULTS for v1 implementation

These were originally open; for the first execution pass we lock the
following defaults so tasks are concrete. Reopen in a follow-up PRD if
the assumptions don't hold.

- **Search-in-Settings indexing** → index by visible `<label>` text only
  in v1 (matches what the user sees; no i18n-key plumbing required).
  Layer i18n-key matching in a v2 if the operator base goes multi-lingual.
- **Bottom-nav redesign** → keep the current 4 tabs as-is. Nav restructure
  is out of scope; only the chrome (backdrop, safe-area) changes.
- **`<Sheet>` primitive** → out of scope for v1. Continue to use the
  existing centered modal. If T2's settings restructure gains a sheet
  pattern, that's a follow-up PRD.

## 9. Related issues

- [#399 — UI and styling issues](https://github.com/opensensor/lightNVR/issues/399) (driving signal)
- [#326 — Stream-to-grid placement not remembered](https://github.com/opensensor/lightNVR/issues/326) (covered in PRD 02)
- [#331 — Timeline doesn't auto-remove deleted recordings](https://github.com/opensensor/lightNVR/issues/331) (covered in PRD 03)
- [#397 — Per-stream-type checkboxes for dashboard](https://github.com/opensensor/lightNVR/issues/397) (covered in PRD 02)

---

## 10. Tasks

Each task is implementable in isolation by a single subagent.  Maps directly
to the requirements in §5 and the phasing in §6.  Listed in dependency
order; the orchestrator may run unblocked tasks in parallel.

### T1: Async-action primitive (`AsyncButton` / `useAsyncAction`)

- **depends_on**: []
- **location**: `web/js/components/preact/AsyncButton.jsx` (new),
  `web/js/hooks/useAsyncAction.js` (new), update at least 3 existing
  callsites (Save Settings, Apply Filter on Recordings, a Stream
  delete) to demonstrate adoption.
- **description**: Implement the shared primitive described in §5.1.
  Two surfaces: (a) `<AsyncButton onClick={asyncFn}>` that wraps
  `<button>`, disables itself while the returned promise is pending,
  swaps its label/icon for a spinner, and renders a brief check icon
  on success / error chip on failure; (b) `useAsyncAction(fn)` hook
  returning `{run, pending, error, lastSuccess}` for cases where the
  button styling can't be a wrapper (icon-only buttons, items inside
  composite controls).  A second click during pending is silently
  dropped (idempotent).  Optional `confirmText` prop turns it into a
  2-step click for destructive actions.
- **validation**:
  - Manual: rapid-tap each migrated callsite — only one server call
    per logical action.
  - Add a Jest test that mounts `<AsyncButton>` with a slow promise
    and asserts double-clicks fire the handler exactly once.
- **status**: Completed
- **log**:
  - Implemented `useAsyncAction(fn)` hook with idempotency guard
    (tracks in-flight promise in a ref; second call while pending
    returns the same promise), mount-safety, and synchronous-throw
    handling. Exposes `{run, pending, error, lastSuccess, reset}`.
  - Implemented `<AsyncButton>` wrapping the hook. Props: `onClick`,
    `children`/`idleLabel`, `confirmText` (two-step destructive click
    with 4 s window and click-outside cancel), `successLabel`,
    `successDurationMs`, `confirmWindowMs`, plus pass-through of
    standard button props. Renders inline 16×16 SVG spinner (Tailwind
    `animate-spin`), check icon on resolve (~1.5 s flash), error chip
    with message on reject, and a red border on the button while an
    error is present.
  - Migrated 3 callsites:
    1. `SettingsView.jsx` — bottom Save Settings button now uses
       `<AsyncButton>` and awaits `saveSettingsMutation.mutateAsync`.
    2. `recordings/FiltersSidebar.jsx` — Apply Filters uses
       `<AsyncButton>`; `applyFilters` in `RecordingsView.jsx` now
       returns the promise from `queryClient.invalidateQueries` so
       the pending state is visible until the refetch settles.
    3. `StreamDeleteModal.jsx` — the "Disable" button uses
       `<AsyncButton>` with `confirmText` (new
       `streams.disableStreamConfirm` locale key) to demonstrate the
       mobile-safe destructive pattern. The "Yes, delete permanently"
       button also uses `<AsyncButton>` so the DELETE call surfaces
       pending state. `deleteStream` / `disableStream` in
       `StreamsView.jsx` now return promises via `mutateAsync`.
  - Build passes: `npm --prefix web run build` → `vite v7.3.2 built
    in 9.84s`; all 3 migrated bundles (settings, recordings, streams)
    present in `dist/assets/`.
  - Jest test `web/tests/useAsyncAction.spec.js` covers: (1)
    double-rapid-click fires handler exactly once, (2) rejection
    captured into `error`, (3) `lastSuccess` timestamp recorded, (4)
    synchronous throw handled. All 4 pass. Component-level JSX Jest
    test skipped — `reason_not_testable:
    jest-not-configured-for-jsx-components` (`web/babel.config.cjs`
    has only `@babel/preset-env`, no JSX preset; no jsdom installed).
    The hook holds the load-bearing idempotency logic, so the unit
    test provides meaningful coverage of the double-submit guard.
- **files edited/created**:
  - created `web/js/components/preact/AsyncButton.jsx`
  - created `web/js/hooks/useAsyncAction.js`
  - created `web/tests/useAsyncAction.spec.js`
  - modified `web/js/components/preact/SettingsView.jsx`
  - modified `web/js/components/preact/RecordingsView.jsx`
  - modified `web/js/components/preact/recordings/FiltersSidebar.jsx`
  - modified `web/js/components/preact/StreamDeleteModal.jsx`
  - modified `web/js/components/preact/StreamsView.jsx`
  - modified `web/public/locales/en.json` (added
    `streams.disableStreamConfirm`)

### T3: Mobile chrome hygiene (logo→home, toast position, safe-area)

- **depends_on**: []
- **location**: `web/js/components/preact/Header.jsx`,
  `web/js/components/preact/ToastContainer.jsx`,
  `web/js/components/preact/UI.jsx` (or wherever the bottom nav lives),
  `web/css/app.css` (or tailwind layer for safe-area utilities).
- **description**: Per §5.3.
  - Logo in `Header.jsx` becomes a `<Link to="/">`-equivalent on every
    page (it currently is not a link from sub-pages).  Home == Live View.
  - `ToastContainer`: top-center on viewports < 640px, top-right on
    desktop.  Never overlaps the bottom nav on mobile.  Honor
    `safe-area-inset-top` on iOS so toasts don't slip under the notch.
  - Bottom nav: add `backdrop-blur` + `bg-background/90` so transparent
    overlays don't fight with it.  Reserve `safe-area-inset-bottom`
    padding so the nav sits above iOS gesture bar.
- **validation**:
  - Manual: hit each top-level page (Live, Recordings, Timeline,
    Settings, System) — clicking the logo lands on `/`.
  - Manual on a 375px viewport: trigger a toast (e.g. save Settings) —
    toast renders top-center, doesn't cover bottom nav, doesn't sit
    under the notch.
- **status**: Completed
- **log**:
  - Header.jsx: wrapped the `LightNVR` + version logo block in an `<a>`
    that routes to Live View via the existing `getLiveViewHref()`
    helper and calls `forceNavigation()` on click (matches the
    convention used by every other nav item in the same file). The
    logo is now a real link with `aria-label={t('nav.live')}` and a
    visible `focus:ring-2 ring-primary` ring on keyboard focus.
    Behaviour is identical from every top-level page (Live,
    Recordings, Streams, Settings, Users, System, Timeline).
  - ToastContainer.jsx: replaced the single absolute-centered container
    class with a responsive one — `top-4 left-1/2 -translate-x-1/2`
    (mobile, top-center) flipping to
    `sm:top-4 sm:right-4 sm:left-auto sm:translate-x-0` (desktop,
    top-right). Added `pt-[env(safe-area-inset-top)]` so iOS notch
    devices don't clip the first toast. Because toasts are now
    top-anchored on mobile, they can never overlap the bottom nav.
  - Footer.jsx is the only bottom-anchored chrome element in the
    current tree (searched for `bottom-0`, `fixed bottom`,
    `MobileBottomNav`, `BottomNav` — no dedicated bottom-nav
    component exists yet; the 4-tab mobile nav is a dropdown under
    the Header). Applied the §5.3 treatment there:
    `backdrop-blur supports-[backdrop-filter]:bg-[hsl(var(--card)/0.8)]`
    (opaque fallback on browsers without `backdrop-filter`) and
    `pb-[calc(0.75rem+env(safe-area-inset-bottom))]` so the footer
    clears the iOS home-indicator gesture bar without collapsing on
    non-iOS browsers.
  - Did NOT touch `web/css/app.css` (T4 owns theme / global CSS).
    Tailwind v4 resolves `backdrop-blur`, `supports-[backdrop-filter]`,
    arbitrary `bg-[hsl(var(--card)/0.8)]`, and
    `pb-[calc(...+env(safe-area-inset-bottom))]` without additional
    utility-layer work — verified by `grep`ing the compiled
    `web/dist/css/*.css` output.
  - `npm --prefix web run build` → 278 modules transformed, built in
    ~10s, 0 errors. (Also regenerated `web/js/version.js` via
    `scripts/extract_version.sh` first — it's a `.gitignore`d
    auto-generated file required by the build.)
- **files edited/created**:
  - `web/js/components/preact/Header.jsx` (logo → Live View link)
  - `web/js/components/preact/ToastContainer.jsx` (responsive
    positioning + safe-area-top)
  - `web/js/components/preact/Footer.jsx` (backdrop-blur +
    safe-area-bottom)
  - `docs/prd/UXD_01_MobileFirstFoundation.md` (this log)

### T4: Theme polish (scrollbar, reduce-motion)

- **depends_on**: []
- **location**: `web/css/app.css` (or a new
  `web/css/theme/scrollbar.css`), `web/js/utils/preferences.js` (or
  similar), `web/js/components/preact/SettingsView.jsx` for the
  "Reduce motion" toggle UI.
- **description**: Per §5.4.
  - Custom scrollbar: `::-webkit-scrollbar` (8px), thumb at
    `hsl(var(--muted-foreground) / 0.6)`, track transparent.  Firefox
    fallback via `scrollbar-color`.  Same in light + dark themes.
  - "Reduce motion" preference: a localStorage-backed setting
    surfaced in Settings → Appearance.  Honors
    `(prefers-reduced-motion: reduce)` as the default but lets the
    user override either way.  When active: disable modal animations,
    toast slide-in, page transitions; preserve essential motion
    (loading spinners stay).
- **validation**:
  - Manual: scrollbar visible on dark theme; remains visible on light.
  - Manual: toggle Reduce Motion off → on; modal opens animate then
    cease to animate; CSS shows `prefers-reduced-motion` override.
- **status**: Completed
- **log**:
  - Created standalone `web/css/theme/scrollbar.css` covering both the
    custom-scrollbar rules (10px webkit thumb at
    `hsl(var(--muted-foreground) / 0.6)` with `border-radius: 9999px`
    and a 2px transparent border for visual padding, transparent
    track; `scrollbar-color` + `scrollbar-width: thin` Firefox
    fallback) and the reduce-motion neutralizer rule keyed off
    `[data-reduce-motion="true"]`.  Wired into `web/css/main.css` via
    a single `@import './theme/scrollbar.css';` line — kept app-side
    CSS additions minimal to stay surgical ahead of T3/T2.
  - Added `web/js/utils/reduceMotion.js` exporting
    `getReduceMotionPref()`, `setReduceMotionPref(value)`,
    `applyReduceMotion()`, plus `isReduceMotionActive()` and
    `installReduceMotionMediaListener()`.  Storage key
    `lightnvr.reduceMotion` with values `'auto' | 'on' | 'off'`
    (default `'auto'`, which defers to
    `matchMedia('(prefers-reduced-motion: reduce)')`).  Safe against
    missing `localStorage` / `matchMedia` (SSR + test env).
  - Extended the existing inline theme-init script in
    `web/vite-plugin-theme-inject.js` to also resolve the
    reduce-motion preference and set
    `<html data-reduce-motion="true|false">` before first paint so
    animations don't flash pre-JS.  Re-applied from JS in `i18n.js`
    `initI18n()` (shared boot path imported by every page entry) and
    installed a media-query change listener there so the `'auto'`
    state tracks OS changes live.
  - Surfaced a three-way Auto/On/Off radiogroup toggle in the
    Appearance section of `SettingsView.jsx` (added to both the
    viewer-only render path and the admin render path, since the
    Appearance block is duplicated in both).  Additive only — the
    surrounding Appearance collapse/expand UI is left untouched for
    T2's future restructure.
  - Also broadened the `copy-css-files` Vite plugin in
    `web/vite.config.js` to recursively copy `web/css/**/*.css` so
    the new `theme/` subdirectory reaches `dist/css/theme/` at build
    time (the previous implementation walked only the top level).
  - Verified via `npm --prefix web run build` — build succeeds;
    `dist/css/theme/scrollbar.css` and the
    `lightnvr.reduceMotion` snippet in `dist/index.html` both ship.
- **files edited/created**:
  - Created `web/css/theme/scrollbar.css`
  - Created `web/js/utils/reduceMotion.js`
  - Edited `web/css/main.css` (single `@import` line)
  - Edited `web/js/i18n.js` (boot call in `initI18n`)
  - Edited `web/vite-plugin-theme-inject.js` (first-paint attribute)
  - Edited `web/vite.config.js` (recursive CSS copy)
  - Edited `web/js/components/preact/SettingsView.jsx` (Appearance toggle)

### T2: Settings page restructure (tabs + search + sticky save)

- **depends_on**: [T1]
- **location**: `web/js/components/preact/SettingsView.jsx` (split from
  ~2229 lines into a parent + per-tab subcomponents under
  `web/js/components/preact/settings/*.jsx`).
- **description**: Per §5.2.  Split the monolith into one component
  per tab in a new `settings/` subfolder: `GeneralTab.jsx`,
  `StorageTab.jsx`, `StreamsDefaultsTab.jsx`, `DetectionTab.jsx`,
  `Go2rtcTab.jsx`, `MqttTab.jsx`, `AuthTab.jsx`, `AppearanceTab.jsx`,
  `AdvancedTab.jsx`.  Parent `SettingsView` owns the shared
  `settings` state, the active-tab state (synced to the URL hash so
  reload preserves position), and the sticky Save button.  Save
  button uses T1's `<AsyncButton>`.  Search input at the top filters
  field labels live; matched fields scroll into view + briefly
  highlight (use `scrollIntoView({behavior: 'smooth', block: 'center'})`).
  In-tab section anchors via `<details>` or sticky subheaders.
- **validation**:
  - Manual: load Settings, find "Web Thread Pool Size" in <5s using
    either tabs or search.
  - Save button stays disabled until any field is dirty; rapid-tapping
    Save fires one request thanks to T1.
  - URL hash reflects active tab (e.g. `/settings#go2rtc`); reload
    restores it.
- **status**: Completed
- **log**:
  - Split the 2,328-line `SettingsView.jsx` into an 853-line parent and 9
    per-tab subcomponents under `web/js/components/preact/settings/`
    (~1,881 lines distributed across tab files, biggest is Go2rtcTab
    at 531 due to the T9 override editor).
    The parent retains `settings` state, baseline snapshot for dirty
    tracking, save wiring, the `saveSettingsMutation` / `handleSave`
    flow, shared `handleInputChange` / `handleThresholdChange` handlers,
    viewer-mode gating, restart notice, tab chrome, search input, and
    the sticky-save bar. Every tab is a pure-JSX component that takes
    `settings` + handlers as props; only `Go2rtcTab.jsx` holds local
    state (the T9 override validation, effective-config modal state,
    and preset-insertion helper) because no other tab needs them.
  - Tab navigation mirrors SystemView's horizontal-on-mobile /
    vertical-on-desktop pattern. On mobile (`< 640px`) the tab strip is
    `flex overflow-x-auto snap-x snap-mandatory gap-2`, active tab gets
    a primary-colored underline. On desktop (`≥ 640px`) the sidebar
    becomes vertical (`sm:flex-col sm:gap-1`) with active tab rendered
    as a primary-bg pill. Every tab button has `min-h-11` (T6 floor).
  - URL hash persistence: parent reads `location.hash` on mount
    (defaulting to `#general`), updates via `history.replaceState` on
    tab change (no history stacking), and subscribes to `hashchange`
    for browser back/forward. Valid hashes: `#general`, `#storage`,
    `#streams`, `#detection`, `#go2rtc`, `#mqtt`, `#auth`,
    `#appearance`, `#advanced`. Invalid / missing hash → `general`.
  - Live label search: a debounced (150 ms) search input walks
    `[data-setting-label]` attributes across ALL tab panels (kept
    mounted with `display:none` on inactive tabs specifically so the
    search scan doesn't miss anything). On non-empty query: matching
    wrappers get `ring-2 ring-primary bg-primary/5`, non-matches get
    `opacity-40`, and the tab that owns the first match is
    auto-selected before `scrollIntoView({behavior:'smooth',
    block:'center'})` fires. A `×` clear button appears while the
    query is non-empty; clearing returns all fields to idle styling.
    Labels are indexed by visible `<label>` text only (PRD §8 v1
    default — no i18n-key plumbing).
  - Sticky Save: `sticky bottom-0 z-30 backdrop-blur` bar with
    `paddingBottom: calc(0.75rem + env(safe-area-inset-bottom))` so
    it respects the iOS gesture bar. Preserves T1's `<AsyncButton>`
    wiring verbatim (calls `saveSettingsMutation.mutateAsync` via the
    existing `saveSettings()` helper). Save button is disabled until
    `settings` differs from the baseline-snapshot ref that's captured
    on load and re-captured after a successful POST.
  - Preserved verbatim: T4 Reduce Motion toggle (now in
    `AppearanceTab.jsx`, same three-way radiogroup), the T9/T14/T4b
    go2rtc override editor with quarantine banner + size indicator +
    validate-on-blur + effective-config modal + presets dropdown +
    supported-sections collapsible (moved to `Go2rtcTab.jsx` along
    with `overrideValidation`, `effectiveConfig`, `OVERRIDE_PRESETS`,
    `KNOWN_GO2RTC_SECTIONS`, `utf8ByteLength`, `GO2RTC_OVERRIDE_MAX_BYTES`).
  - Viewer short-circuit preserved: when `userRole === 'viewer'`, the
    parent renders only `<AppearanceTab>` + the admin-only notice, no
    tab chrome, no search, no save button.
  - Counts:  71 `onChange={handleInputChange}` callsites before → 71
    after (cross-checked with `grep -c`). Every `{settings.X}` state
    access present in the original tree is present in exactly one tab.
  - i18n: added 15 new keys to `en.json` (tab labels + search
    placeholder + "Unsaved changes" + "Authentication & Access" +
    "Streams Defaults" header + description) and the Portuguese
    equivalents to `pt-BR.json`. Other locales fall back to the
    English label via the existing `t(key) || key` path + the
    `labelFallback` field on each tab definition.
  - Build: `npm --prefix web run build` → `✓ built in 10.24s`, 0
    errors. Settings bundle at `dist/assets/settings-*.js`
    (~96 kB raw / ~17 kB gz — ~1 kB larger than pre-T2, accounting
    for the tab-chrome and search logic).
  - Tests: `npm --prefix web test -- tests/useAsyncAction.spec.js`
    → 4/4 pass. Full Jest suite: `i18n.spec.js` 4/5 (same
    pre-existing "handles complete locale loading failure
    gracefully" failure flagged by T1/T5), `navigation-utils.spec.js`,
    `timeline-utils.spec.js`, `recordingsAPI.spec.js` all pass.
- **files edited/created**:
  - created `web/js/components/preact/settings/GeneralTab.jsx`
  - created `web/js/components/preact/settings/StorageTab.jsx`
  - created `web/js/components/preact/settings/StreamsDefaultsTab.jsx`
  - created `web/js/components/preact/settings/DetectionTab.jsx`
  - created `web/js/components/preact/settings/Go2rtcTab.jsx`
  - created `web/js/components/preact/settings/MqttTab.jsx`
  - created `web/js/components/preact/settings/AuthTab.jsx`
  - created `web/js/components/preact/settings/AppearanceTab.jsx`
  - created `web/js/components/preact/settings/AdvancedTab.jsx`
  - rewrote `web/js/components/preact/SettingsView.jsx`
    (2,328 → 853 lines; kept shared state + handlers, added tab
    chrome / hash sync / live search / sticky save)
  - modified `web/public/locales/en.json` (15 new keys for tabs +
    search + sticky-save strip + Authentication/StreamsDefaults
    headers)
  - modified `web/public/locales/pt-BR.json` (same 15 keys,
    translated)
  - modified `docs/prd/UXD_01_MobileFirstFoundation.md` (this log)

### T5: Streams page card layout (responsive grid + badge)

- **depends_on**: [T1]
- **location**: `web/js/components/preact/StreamsView.jsx`,
  possibly a new `StreamCard.jsx` extracted component.
- **description**: Per §5.5.  Replace the current overflow-prone table
  with a responsive grid via
  `grid grid-cols-[repeat(auto-fill,minmax(360px,1fr))] gap-4`.  Each
  card shows snapshot (top), name + status badge (middle), recording
  + transport badges (bottom).  On viewports < 640px, collapse
  binding/source URL behind a `<details>` "Details" disclosure.
  Recording-mode badge: comma-joined modes from
  `[Constant, Detection, Schedule]` in display order — fixes the
  #399 specific item where "Enabled (Detection)" hid that constant
  recording was also active.  Action buttons (edit, delete, snapshot)
  use T1's `<AsyncButton>` for the network-bound ones.
- **validation**:
  - Manual at 375 / 768 / 1280 / 1920 viewports: zero horizontal
    scrollbar; cards reflow gracefully.
  - With Constant + Detection both enabled, the badge reads
    "Constant + Detection" not "Enabled (Detection)".
- **status**: Completed
- **log**:
  - Extracted a new `web/js/components/preact/StreamCard.jsx`
    component. Layout per PRD: snapshot (16:9 go2rtc frame.jpeg
    proxy with neutral placeholder on error) → title + status
    badge → resolution/FPS/codec secondary line → mode-badges row
    → details disclosure → action row. Cards are focused and
    testable in isolation; StreamsView keeps data-flow concerns.
  - Replaced the former `<table id="streams-table">` block in
    `StreamsView.jsx` with a responsive grid:
    `grid grid-cols-[repeat(auto-fill,minmax(360px,1fr))] gap-4`.
    The `auto-fill` + `minmax` combo is load-bearing — it's what
    eliminates the #399 "horizontal scroll even though there's
    space" complaint at every viewport from 375 px through 1920+.
  - Recording-mode badge fixes the #399 "Enabled (Detection) when
    Constant is also on" bug. New `collectRecordingModes(stream)`
    helper returns an ordered `[Constant, Detection, Schedule]`
    subset built from `record`, `detection_based_recording`, and
    `record_on_schedule`, then joins them with ` + ` for display
    (`"Constant + Detection"`, `"Detection"`, `"Constant + Schedule"`).
    Zero active modes renders a muted "Not recording" pill. Any
    active mode uses a tinted green pill. `aria-label` spells out
    the full set so screen readers aren't misled by the visual
    join. Schedule flag semantics: when `record_on_schedule` is
    true we surface "Schedule" instead of "Constant" (the stream
    only records within the scheduled window).
  - Binding / source URL collapses behind a `<details class="sm:hidden">`
    disclosure on < 640 px, with a parallel `hidden sm:block`
    copy for tablet/desktop. Per-card credential-reveal toggle
    (eye / eye-off icon) replaces the page-level `revealedUrls`
    Set that the old table owned.
  - Action row uses T1's `<AsyncButton>` for the network-bound
    enable and disable toggles. Disable path passes
    `confirmText={t('streams.disableStreamConfirm')}` so the
    destructive two-step click lives on the card itself — the
    old `window.confirm` path stays in place as a fallback via
    the non-card `handleToggleStreamEnabled` (bulk-action code
    still calls it). Delete still opens `StreamDeleteModal`,
    which already internally uses `<AsyncButton>` per T1.
  - Bulk-select toolbar + sort controls preserved above the grid.
    Sort chips replace the former sortable `<th>` headers.
    Select-all checkbox moved into the bulk-action toolbar.
  - Added 9 locale keys to `web/public/locales/en.json`:
    `streams.notRecording`, `streams.recordingModesList`,
    `streams.details`, `streams.binding`, `streams.subStreamUrl`,
    `streams.transportLabel`, `streams.snapshotFor`,
    `streams.snapshotUnavailable`, `streams.sortBy`. Other
    locales inherit English fallbacks via the existing
    `t(key) || key` path in `web/js/i18n.js`.
  - `npm --prefix web run build` → 282 modules transformed,
    built in 10.54 s, 0 errors. Streams bundle shipped at
    `dist/assets/streams-*.js` (~110 kB gz 24 kB).
  - `npm --prefix web test`:
    - `tests/useAsyncAction.spec.js` → 4/4 pass (the hook T5
      relies on for async-button semantics).
    - `tests/navigation-utils.spec.js`,
      `tests/timeline-utils.spec.js`,
      `tests/recordingsAPI.spec.js` → pass.
    - `tests/i18n.spec.js` → 4/5 pass; the one failure
      ("handles complete locale loading failure gracefully") is
      pre-existing and flagged by T1's agent report. Not
      introduced by T5.
    - `tests/e2e/*` specs all fail at WebDriver connect
      (`net::ERR_CONNECTION_REFUSED`): they need a running
      `http://localhost:8080`. Environment-dependent, not
      related to T5.
- **files edited/created**:
  - created `web/js/components/preact/StreamCard.jsx`
  - modified `web/js/components/preact/StreamsView.jsx`
    (grid replaces table; card-based render; removed
    `expandedStreams` / `revealedUrls` / `toggleUrlReveal`
    helpers now owned per-card; added `enableStreamFromCard` /
    `disableStreamFromCard` promise-returning helpers for
    AsyncButton)
  - modified `web/public/locales/en.json` (9 new keys)
  - modified `docs/prd/UXD_01_MobileFirstFoundation.md`
    (this log)

### T6: Touch-target & spacing audit

- **depends_on**: [T1, T2, T3, T5]
- **location**: New script `scripts/audit-touch-targets.mjs`,
  `web/js/components/preact/**/*.jsx` (auto-flag callsites below 44px),
  `web/css/app.css` (form-field min-height + focus-ring tokens).
- **description**: Per §5.6.  Two parts:
  1. **Lint script**: a small Node script that walks the compiled
     `dist/` CSS + JSX, finds elements with interactive roles
     (`<button>`, `<a>`, `<input>`, `[role="button"]`), and flags any
     whose computed min-height OR min-width on `< 640px` viewport is
     below 44px.  Output a CSV of `(file, line, selector, computed
     size)`.  Wire into `npm run audit` (new script).  Don't fail the
     build on findings; report-only.
  2. **Apply the floors**: bulk-edit obvious offenders by adding
     Tailwind `min-h-11 min-w-11 inline-flex items-center justify-center`
     to `<button>` callsites that lack any size class.  Form fields
     (`<input>`, `<select>`, `<textarea>`) get `min-h-11` via a
     global rule.  Focus ring switches to `ring-2 ring-primary
     ring-offset-2 ring-offset-background` so it's visible on both
     themes.
- **validation**:
  - `npm run audit` runs without crashing and produces a CSV.
  - Manual: tab through the Live, Recordings, Settings pages with
    keyboard — every focusable element shows a visible ring.
- **status**: Not Completed
- **log**:
- **files edited/created**:

## Parallel Execution Groups

| Wave | Tasks | Can Start When |
|------|-------|----------------|
| 1 | T1, T3, T4 | Immediately |
| 2 | T2, T5 | T1 complete |
| 3 | T6 | T2, T3, T5 complete |
