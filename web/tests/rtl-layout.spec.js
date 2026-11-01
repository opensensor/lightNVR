import fs from 'node:fs';
import path from 'node:path';
import { getTextDirection } from '../js/i18n.js';

// RTL layout support (#657). The UI uses logical utilities (ms-/me-/ps-/pe-,
// inset-s-/inset-e-, text-start/end, rounded-s/e, border-s/e) so it mirrors
// under <html dir="rtl">. A few places are physical on purpose: centring tricks
// (left-1/2 + -translate-x-1/2), full-bleed canvases over video, and the
// timeline/PTZ/detection components, where left always means earlier in time
// or camera-left.

const webRoot = path.resolve(__dirname, '..');

const PHYSICAL_CLASS = /(?<=^|[\s"'`{(:])-?(?:m[lr]|p[lr]|left|right|rounded-(?:[lr]|[tb][lr])|border-[lr]|scroll-[mp][lr])-[a-zA-Z0-9./%[\]#-]+(?=$|[\s"'`})])|(?<=^|[\s"'`{(:])(?:text-(?:left|right)|float-(?:left|right)|border-[lr]|rounded-[lr])(?=$|[\s"'`})])/g;

// Files whose geometry maps to time or camera motion, so it must not mirror.
const PHYSICAL_FILES = new Set([
  'js/components/preact/PTZControls.jsx',
  'js/components/preact/DetectionOverlay.jsx',
  'js/components/preact/timeline/TimelineRuler.jsx',
  'js/components/preact/timeline/TimelineCursor.jsx',
  'js/components/preact/timeline/TimelineSegments.jsx',
]);

// Remaining intentional physical utilities: "file|class" -> occurrences.
const ALLOWED = new Map([
  ['js/components/preact/recordings/RecordingsGrid.jsx|left-1/2', 1],
  ['js/components/preact/ToastContainer.jsx|left-1/2', 1],
  ['js/components/preact/UI.jsx|left-0', 1],
  ['js/components/preact/timeline/TimelinePlayer.jsx|left-0', 1],
  // Inline hint inside the dir="ltr" timeline track.
  ['js/components/preact/timeline/TimelinePage.jsx|right-2', 1],
  ['css/components.css|left-1/2', 1],
  ['css/live.css|left-1/2', 1],
  ['css/base.css|left-0', 1],
]);

function listFiles(dir, exts) {
  const out = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (['locales', 'node_modules', '__tests__'].includes(entry.name)) continue;
      out.push(...listFiles(full, exts));
    } else if (exts.some((ext) => entry.name.endsWith(ext))) {
      out.push(full);
    }
  }
  return out;
}

function physicalUtilities() {
  const found = new Map();
  const files = [
    ...listFiles(path.join(webRoot, 'js'), ['.js', '.jsx']),
    ...listFiles(path.join(webRoot, 'css'), ['.css']),
  ];
  for (const file of files) {
    const rel = path.relative(webRoot, file).split(path.sep).join('/');
    if (PHYSICAL_FILES.has(rel)) continue;
    const lines = fs.readFileSync(file, 'utf8').split('\n');
    for (const line of lines) {
      const trimmed = line.trim();
      if (trimmed.startsWith('//') || trimmed.startsWith('*') || trimmed.startsWith('/*')) continue;
      // CSS: only @apply lines carry Tailwind classes.
      if (rel.endsWith('.css') && !trimmed.startsWith('@apply')) continue;
      for (const match of line.matchAll(PHYSICAL_CLASS)) {
        const key = `${rel}|${match[0]}`;
        found.set(key, (found.get(key) || 0) + 1);
      }
    }
  }
  return found;
}

describe('RTL layout support', () => {
  test('getTextDirection reports rtl for Arabic, Hebrew, Persian and Urdu', () => {
    for (const locale of ['ar', 'ar-EG', 'he', 'fa', 'ur']) {
      expect(getTextDirection(locale)).toBe('rtl');
    }
    for (const locale of ['en', 'pt-BR', 'fil', 'de', '', undefined]) {
      expect(getTextDirection(locale ?? '')).toBe('ltr');
    }
  });

  test('components use logical direction utilities except for allow-listed physical ones', () => {
    const unexpected = [...physicalUtilities()]
      .filter(([key, count]) => (ALLOWED.get(key) || 0) < count)
      .map(([key, count]) => `${key} (x${count})`);
    // Use ms-/me-/ps-/pe-/inset-s-/inset-e-/text-start/text-end/rounded-s/e/border-s/e.
    // If a new usage really is physical, add it to ALLOWED with a reason.
    expect(unexpected).toEqual([]);
  });
});
