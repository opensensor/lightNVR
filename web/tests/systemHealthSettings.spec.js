const path = require('path');
const { buildSync } = require('esbuild');

function loadHealthHelpers() {
  const output = buildSync({
    absWorkingDir: path.resolve(__dirname, '..'),
    entryPoints: ['js/components/preact/settings/HealthTab.jsx'],
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
  HEALTH_SETTINGS_DEFAULTS,
  conditionOverrideMode,
  healthSettingsFromResponse,
  healthSettingsToPayload,
  updateConditionOverride,
  validateHealthSettings,
} = loadHealthHelpers();

const condition = {
  code: 'host.memory.pressure',
  unit: 'ratio',
  warning: 0.8,
  critical: 0.95,
  recovery: 0.7,
  warning_for_seconds: 60,
  critical_for_seconds: 30,
  recovery_for_seconds: 120,
};

function validSettings() {
  return {
    ...HEALTH_SETTINGS_DEFAULTS,
    healthConditionOverrides: { version: 1, conditions: [] },
    healthEffectivePolicy: { conditions: [condition] },
  };
}

describe('system health settings helpers', () => {
  test('maps the bounded health API names in both directions', () => {
    const mapped = healthSettingsFromResponse({
      health_enabled: false,
      health_profile: 'conservative',
      health_fast_interval_seconds: 20,
      health_normal_interval_seconds: 120,
      health_slow_interval_seconds: 600,
      health_device_interval_seconds: 1800,
      health_write_probe_enabled: false,
      health_hardware_provider: 'smartctl',
      health_presence_interval_seconds: 90,
      health_incident_retention_days: 365,
      health_condition_overrides: { version: 1, conditions: [{ code: condition.code, profile: 'disabled' }] },
      health_effective_policy: { conditions: [condition] },
    });
    expect(mapped.healthHardwareProvider).toBe('smartctl');
    expect(mapped.healthPresenceIntervalSeconds).toBe('90');
    expect(mapped.healthIncidentRetentionDays).toBe('365');

    const payload = healthSettingsToPayload(mapped);
    expect(payload).toMatchObject({
      health_hardware_provider: 'smartctl',
      health_presence_interval_seconds: 90,
      health_incident_retention_days: 365,
      health_write_probe_enabled: false,
    });
    expect(payload).not.toHaveProperty('hardware_provider');
    expect(payload).not.toHaveProperty('presence_interval_seconds');
    expect(payload).not.toHaveProperty('incident_retention_days');
    expect(payload.health_effective_policy).toBeUndefined();
  });

  test('enforces numeric bounds and ordered cadences before save', () => {
    expect(validateHealthSettings(validSettings())).toEqual([]);
    expect(validateHealthSettings({
      ...validSettings(),
      healthFastIntervalSeconds: '61',
    })).toContain('healthFastIntervalSeconds');
    expect(validateHealthSettings({
      ...validSettings(),
      healthNormalIntervalSeconds: '600',
      healthSlowIntervalSeconds: '60',
    })).toContain('ordered');
    expect(validateHealthSettings({
      ...validSettings(),
      healthHardwareProvider: '/dev/sda',
    })).toContain('provider');
  });

  test('builds only registry-backed profile and structured custom overrides', () => {
    const empty = { version: 1, conditions: [] };
    const disabled = updateConditionOverride(empty, condition, 'disabled');
    expect(disabled).toEqual({
      version: 1,
      conditions: [{ code: condition.code, profile: 'disabled' }],
    });
    expect(conditionOverrideMode(disabled, condition.code)).toBe('disabled');

    const custom = updateConditionOverride(empty, condition, 'custom', { warning: 0.82 });
    expect(custom.conditions[0]).toMatchObject({
      code: condition.code,
      unit: 'ratio',
      warning: 0.82,
      critical: 0.95,
      recovery: 0.7,
    });
    expect(validateHealthSettings({
      ...validSettings(),
      healthConditionOverrides: custom,
    })).toEqual([]);
    expect(updateConditionOverride(custom, condition, 'default')).toEqual(empty);
  });

  test('rejects arbitrary condition codes and unsafe threshold or dwell values', () => {
    const arbitrary = {
      version: 1,
      conditions: [{ code: 'private.device.path', profile: 'disabled' }],
    };
    expect(validateHealthSettings({
      ...validSettings(), healthConditionOverrides: arbitrary,
    })).toContain('overrides');

    const unsafe = updateConditionOverride(
      { version: 1, conditions: [] }, condition, 'custom',
      { critical: 0.75, recovery_for_seconds: 0 }
    );
    const errors = validateHealthSettings({
      ...validSettings(), healthConditionOverrides: unsafe,
    });
    expect(errors).toContain('thresholdOrder');
    expect(errors).toContain('dwell');
  });
});
