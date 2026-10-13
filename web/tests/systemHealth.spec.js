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
  healthCoverageMessage,
  observationGroup,
  remediationKey,
} = loadJsx('js/components/preact/system/SystemHealth.jsx');
const { buildIncidentHistoryUrl } = loadJsx('js/components/preact/system/HealthIncidentHistory.jsx');
const { boundedPercent, metricKnown } = loadJsx('js/components/preact/system/MemoryStorage.jsx');

describe('System Health operator view helpers', () => {
  test('reports missing samples before claiming limited container coverage', () => {
    const health = {
      snapshot: { available: false },
      visibility: { effective_scope: 'unknown', host_hardware_visible: false },
    };
    expect(healthCoverageMessage(health).message).toBe('system.health.coverageUnknown');
    expect(healthCoverageMessage(health).title).toBe('system.health.coverage');
    expect(healthCoverageMessage(undefined).message).toBe('system.health.coverageUnknown');
    health.snapshot.available = true;
    expect(healthCoverageMessage(health).title).toBe('system.health.containerCoverage');
  });

  test('distinguishes runtime-disabled monitoring and a pending restart', () => {
    const monitoring = { configured_enabled: false, initialized: true, enabled: false, restart_required: false };
    expect(healthCoverageMessage({ monitoring }).message).toBe('system.health.monitoringDisabledHelp');
    monitoring.configured_enabled = true;
    monitoring.restart_required = true;
    expect(healthCoverageMessage({ monitoring }).message).toBe('system.health.monitoringRestartRequired');
    expect(healthCoverageMessage({ monitoring }).title).toBe('system.health.monitoringDisabled');
  });

  test('does not report disabled monitoring while initialization is pending', () => {
    expect(healthCoverageMessage({ monitoring: {
      configured_enabled: true, initialized: false, enabled: false,
    } }).message).toBe('system.health.coverageUnknown');
    expect(healthCoverageMessage({ monitoring: {
      configured_enabled: false, initialized: false, enabled: false,
    } }).message).toBe('system.health.monitoringDisabledHelp');
  });

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
