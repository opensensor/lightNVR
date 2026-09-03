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
  HEALTH_SAMPLE_LIMIT,
  boundedRecentSamples,
  formatHealthValue,
  groupHealthObservations,
  observationGroup,
  remediationKey,
} = loadJsx('js/components/preact/system/SystemHealth.jsx');
const { buildIncidentHistoryUrl } = loadJsx('js/components/preact/system/HealthIncidentHistory.jsx');
const { boundedPercent, metricKnown } = loadJsx('js/components/preact/system/MemoryStorage.jsx');

describe('System Health operator view helpers', () => {
  test('groups generic observations without creating unbounded sections', () => {
    expect(observationGroup('host.cpu.busy_ratio')).toBe('resources');
    expect(observationGroup('filesystem.available_bytes')).toBe('storage');
    expect(observationGroup('storage.device.life_used_ratio')).toBe('storage');
    expect(observationGroup('hardware.fan.rpm')).toBe('hardware');
    expect(observationGroup('network.carrier')).toBe('network');
    expect(observationGroup('clock.synchronized')).toBe('clock');
    expect(observationGroup('process.fd_ratio')).toBe('components');

    const groups = groupHealthObservations(Array.from({ length: 300 }, (_, index) => ({
      metric: 'network.rx_bytes', resource: `interface-${index}`,
    })));
    expect(groups.network).toHaveLength(256);
    expect(Object.keys(groups)).toEqual([
      'resources', 'storage', 'hardware', 'network', 'clock', 'components',
    ]);
  });

  test('never formats unavailable or stale values as healthy zeroes', () => {
    expect(formatHealthValue({ capability: 'unsupported', freshness: 'unknown', value: null }, 'Unknown')).toBe('Unknown');
    expect(formatHealthValue({ capability: 'available', freshness: 'stale', value: 0 }, 'Unknown')).toBe('Unknown');
    expect(formatHealthValue({ capability: 'available', freshness: 'fresh', value: 0, unit: 'ratio' }, 'Unknown')).toBe('0.0%');
    expect(formatHealthValue({ capability: 'available', freshness: 'fresh', value: 1073741824, unit: 'bytes' }, 'Unknown')).toBe('1 GB');
  });

  test('keeps only the bounded authoritative generation summaries', () => {
    const samples = Array.from({ length: HEALTH_SAMPLE_LIMIT + 8 }, (_, sequence) => ({ sequence }));
    const bounded = boundedRecentSamples(samples);
    expect(bounded).toHaveLength(HEALTH_SAMPLE_LIMIT);
    expect(bounded[0].sequence).toBe(8);
    expect(bounded.at(-1).sequence).toBe(HEALTH_SAMPLE_LIMIT + 7);
    expect(boundedRecentSamples(null)).toEqual([]);
  });

  test('maps remediation to bounded translation keys', () => {
    expect(remediationKey('filesystem.read_only')).toBe('system.health.remediation.filesystem');
    expect(remediationKey('event.delivery_degraded')).toBe('system.health.remediation.event');
    expect(remediationKey('vendor.private')).toBe('system.health.remediation.default');
  });

  test('builds encoded bounded incident-history pages', () => {
    expect(buildIncidentHistoryUrl()).toBe('/api/system/health/incidents?limit=20&include_closed=true');
    expect(buildIncidentHistoryUrl('v1:123:a/b')).toBe('/api/system/health/incidents?limit=20&include_closed=true&cursor=v1%3A123%3Aa%2Fb');
  });

  test('legacy resource cards distinguish valid zero from unknown', () => {
    expect(metricKnown({ capability: 'available' }, 0)).toBe(true);
    expect(metricKnown({ capability: 'stale' }, 0)).toBe(false);
    expect(metricKnown({ capability: 'available' }, null)).toBe(false);
    expect(boundedPercent(0, 100)).toBe(0);
    expect(boundedPercent(120, 100)).toBe(100);
    expect(boundedPercent(null, 100)).toBeNull();
  });
});
