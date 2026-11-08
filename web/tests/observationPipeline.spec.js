const path = require('path');
const { buildSync } = require('esbuild');

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
  formatLag,
  readObservationPipeline,
  totalDropped,
  totalFailures,
} = loadJsx('js/components/preact/system/ObservationPipeline.tsx');

describe('Observation pipeline diagnostics helpers (INT_01 L10)', () => {
  test('ignores a missing or malformed observation_pipeline member', () => {
    expect(readObservationPipeline(undefined)).toBeNull();
    expect(readObservationPipeline({})).toBeNull();
    expect(readObservationPipeline({ observation_pipeline: { accepted: 3 } })).toBeNull();
    const pipeline = { active: false, accepted: 0 };
    expect(readObservationPipeline({ observation_pipeline: pipeline })).toBe(pipeline);
  });

  test('sums drops by reason and evidence failures, treating junk as zero', () => {
    const pipeline = {
      active: true,
      dropped: { queue_full: 3, budget_exhausted: 2, disabled: 1, untracked: 4 },
      evidence: { failures: { encode_failed: 1, write_failed: 2, db_error: -5, interrupted: 'x' } },
    };
    expect(totalDropped(pipeline)).toBe(10);
    expect(totalFailures(pipeline)).toBe(3);
    expect(totalDropped({ active: true })).toBe(0);
    expect(totalFailures({ active: true })).toBe(0);
  });

  test('formats relay lag as whole seconds', () => {
    expect(formatLag(undefined)).toBe('0');
    expect(formatLag(0.4)).toBe('0');
    expect(formatLag(59.6)).toBe('60');
    expect(formatLag(-3)).toBe('0');
  });
});
