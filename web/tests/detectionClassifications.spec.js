const fs = require('fs');
const path = require('path');
const { buildSync } = require('esbuild');

// INT_01 L15: the minimal classification display next to detections.
function loadJsx(relativePath) {
  const output = buildSync({
    absWorkingDir: path.resolve(__dirname, '..'),
    entryPoints: [relativePath],
    bundle: true,
    format: 'cjs',
    platform: 'node',
    alias: {
      react: 'preact/compat',
      'react-dom': 'preact/compat',
      'react/jsx-runtime': 'preact/jsx-runtime',
    },
    write: false,
    logLevel: 'silent',
  }).outputFiles[0].text;
  const loaded = { exports: {} };
  Function('module', 'exports', 'require', output)(loaded, loaded.exports, require);
  return loaded.exports;
}

const {
  classifiedDetections,
  detectionClassificationView,
  detectionOverlayLabel,
  renderDetectionClassifications,
} = loadJsx('js/components/preact/DetectionClassifications.tsx');

const english = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../public/locales/en.json'), 'utf8'));
// The production t() template rule, against the real English catalog.
const t = (key, params = {}) => {
  expect(english).toHaveProperty([key]);
  return english[key].replace(/\{([^}]+)\}/g, (_, name) => String(params[name.trim()] ?? ''));
};

// Flatten a Preact vnode tree to its text, and collect link targets.
function textOf(node) {
  if (node === null || node === undefined || typeof node === 'boolean') return '';
  if (typeof node === 'string' || typeof node === 'number') return String(node);
  if (Array.isArray(node)) return node.map(textOf).join('');
  return textOf(node.props?.children);
}
function linksOf(node, out = []) {
  if (!node || typeof node !== 'object') return out;
  if (Array.isArray(node)) { node.forEach((child) => linksOf(child, out)); return out; }
  if (node.type === 'a') out.push(node.props.href);
  linksOf(node.props?.children, out);
  return out;
}

const OBSERVATION_ID = '44444444-4444-4444-8444-444444444444';
const base = { label: 'bird', confidence: 0.82, x: 0.1, y: 0.2, width: 0.3, height: 0.4,
  timestamp: 1794000195, end_timestamp: 1794000195 };
const annotation = (overrides) => ({
  key: 'yawamf/species', writer_namespace: 'yawamf', classifier: 'yawamf',
  model_version: 'convnext-v2-2026.10', label: 'Black-capped Chickadee', taxonomy_id: null,
  confidence: 0.93, classified_at: '2026-11-08T14:03:15.002Z',
  input_asset_id: '55555555-5555-4555-8555-555555555555', input_revision: 2, kind: 'automated',
  annotation_revision: 1, created_at: '2026-11-08T14:03:15.100Z', updated_at: '2026-11-08T14:03:15.100Z',
  ...overrides,
});
const automated = { ...base, observation: { id: OBSERVATION_ID, owner_confirmed: false,
  classification: annotation() } };
const owner = { ...base, observation: { id: OBSERVATION_ID, owner_confirmed: true,
  classification: annotation({ label: 'Carolina Chickadee', confidence: 1, kind: 'owner_confirmed',
    model_version: null }) } };

describe('detection classification display (INT_01 L15)', () => {
  test('a detection without an observation has no classification view', () => {
    expect(detectionClassificationView(base)).toBeNull();
    expect(detectionClassificationView({ ...base, observation: null })).toBeNull();
    expect(detectionClassificationView({ ...base, observation: { owner_confirmed: false } })).toBeNull();
    expect(detectionOverlayLabel(base)).toBe('bird (82%)');
    expect(classifiedDetections([base])).toEqual([]);
  });

  test('keeps the detector label and confidence beside the classification', () => {
    expect(detectionClassificationView(automated)).toEqual({
      observationId: OBSERVATION_ID,
      observationUrl: `/api/observations/${OBSERVATION_ID}`,
      detectorLabel: 'bird',
      detectorConfidence: 82,
      classifiedLabel: 'Black-capped Chickadee',
      classifiedConfidence: 93,
      classifier: 'yawamf',
      ownerConfirmed: false,
    });
    expect(detectionOverlayLabel(automated)).toBe('bird (82%) · Black-capped Chickadee (93%)');

    const tree = renderDetectionClassifications([automated], t);
    const text = textOf(tree);
    expect(text).toContain('Detector: bird (82%)');
    expect(text).toContain('Black-capped Chickadee (93%)');
    expect(text).toContain('yawamf');
    expect(text).not.toContain('Owner confirmed');
    expect(linksOf(tree)).toEqual([`/api/observations/${OBSERVATION_ID}`]);
  });

  test('an owner decision shows the owner label with a badge, never the automated one', () => {
    const view = detectionClassificationView(owner);
    expect(view.ownerConfirmed).toBe(true);
    expect(view.classifiedLabel).toBe('Carolina Chickadee');
    const text = textOf(renderDetectionClassifications([owner], t));
    expect(text).toContain('Detector: bird (82%)');
    expect(text).toContain('Carolina Chickadee (100%)');
    expect(text).toContain('Owner confirmed');
    expect(text).not.toContain('Black-capped Chickadee');
  });

  test('a linked but unclassified observation still links back', () => {
    const linked = { ...base, observation: { id: OBSERVATION_ID, owner_confirmed: false, classification: null } };
    const view = detectionClassificationView(linked);
    expect(view.classifiedLabel).toBeNull();
    expect(detectionOverlayLabel(linked)).toBe('bird (82%)');
    const tree = renderDetectionClassifications([linked], t);
    expect(textOf(tree)).toContain('Not classified');
    expect(linksOf(tree)).toEqual([`/api/observations/${OBSERVATION_ID}`]);
  });

  test('lists each observation once and renders nothing without links', () => {
    const other = { ...automated, timestamp: base.timestamp + 1 };
    expect(classifiedDetections([automated, base, other, owner]).map((d) => d.observationId))
      .toEqual([OBSERVATION_ID]);
    expect(renderDetectionClassifications([base], t)).toBeNull();
    expect(renderDetectionClassifications([], t)).toBeNull();
  });
});
