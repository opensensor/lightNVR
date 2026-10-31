# T10 localization follow-up records

The strengthened lint rule exposes 21 existing attribute/interpolation literals
in ten files. No baseline increase or detector relaxation is permitted. Append
these task records, add T10.3 to T14/T15, and order overlapping T7.* source writers
after the relevant T10 child. English fallback is supported by the existing
locale loader and catalog policy; no untranslated copies are added to the other
locale catalogs.

### T10.1: Supply attribute translation keys and regression fixtures
- **depends_on**: [T10]
- **agent_type**: worker
- **ownership**: `web/public/locales/en.json`, `tests/quality/localized-attributes.test.mjs`, `quality/evidence/T10.1.md`.
- **description**: Add explicit English keys for newly exposed placeholder examples, plan labels/shortcuts, recording thumbnail/count and tag-removal labels. Reuse existing settings.tabsLabel, recordings.download/protect/unprotect keys. Test real i18n interpolation and English fallback for a partial locale; do not translate protocol examples or add fabricated translations to other catalogs.
- **acceptance_criteria**:
  - Required source keys exist and interpolate variable values, singular/plural counts, shortcut labels and multiline examples without raw key names or dropped parameters.
  - Existing English output is preserved except the intentional reuse of the more descriptive existing Protect/Unprotect recording labels.
  - No i18n literal allowance is increased; non-English partial-catalog fallback follows existing behavior.
- **validation**:
  - `node --test tests/quality/localized-attributes.test.mjs`
  - `npm --prefix web test -- --runInBand --testPathPatterns='i18n|locale-coverage'`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T10.2: Localize settings and stream editor attributes
- **depends_on**: [T10.1]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/SettingsView.jsx`, `web/js/components/preact/StreamConfigModal.jsx`, `web/js/components/preact/settings/DetectionTab.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/js/components/preact/settings/StorageTab.jsx`, `quality/evidence/T10.2.md`.
- **description**: Replace the nine newly exposed literals in five settings files with T10.1 keys. Remove redundant English fallback for the existing settings.tabsLabel key. Preserve dynamic bucket interpolation and full multiline URL/config examples, handlers, field values and disabled-state logic.
- **acceptance_criteria**:
  - All reported attribute literals in these five files pass the strengthened i18n rule with unchanged baseline counts.
  - Example URL schemes, credentials-as-examples, protocol options, whitespace/newlines and storage bucket interpolation remain accurate.
  - No strings are moved to helper constants merely to evade AST inspection.
- **validation**:
  - `node --test tests/quality/localized-attributes.test.mjs`
  - `(cd web && node_modules/.bin/eslint js/components/preact/SettingsView.jsx js/components/preact/StreamConfigModal.jsx js/components/preact/settings/DetectionTab.jsx js/components/preact/settings/Go2rtcTab.jsx js/components/preact/settings/StorageTab.jsx)`
  - `npm --prefix web test -- --runInBand --testPathPatterns='i18n|locale-coverage|go2rtcSettings'`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T10.3: Localize live and recording attributes and close lint integration
- **depends_on**: [T10.2]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/js/components/preact/live/PlanSketchControls.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/recordings/RecordingsTable.jsx`, `web/js/components/preact/recordings/TagsOverlay.jsx`, `quality/evidence/T10.3.md`.
- **description**: Replace remaining newly detected attribute literals with T10.1 keys and existing download/protection keys. Obtain i18n in TagChip at component top level, preserve callback and focus behavior, and preserve count/singular/plural and filename/size interpolation. Run final whole-tree lint and compatibility tests after serial source fixes.
- **acceptance_criteria**:
  - No new i18n errors remain; the 897 existing accessibility warnings stay explicit rather than being suppressed or rebaselined.
  - Labels retain the stream/plan/tag/count/size values and shortcut information; existing action handlers remain unchanged.
  - T10 compatibility tests and relevant localization/live/recording regressions pass; baseline JSON bytes remain unchanged.
- **validation**:
  - `node --test tests/quality/lint-compat.test.mjs tests/quality/localized-attributes.test.mjs`
  - `npm --prefix web run lint`
  - `npm --prefix web test -- --runInBand --testPathPatterns='i18n|locale-coverage|planSketch|buildingPlan|recording|tag'`
- **status**: Not Completed
- **log**:
- **files edited/created**:
