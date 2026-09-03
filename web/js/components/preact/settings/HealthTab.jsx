const fieldClass = 'mt-1 w-full min-h-11 rounded border border-input bg-background p-2 text-foreground disabled:cursor-not-allowed disabled:opacity-60';

export const HEALTH_SETTINGS_DEFAULTS = Object.freeze({
  healthEnabled: true,
  healthProfile: 'balanced',
  healthFastIntervalSeconds: '10',
  healthNormalIntervalSeconds: '60',
  healthSlowIntervalSeconds: '300',
  healthDeviceIntervalSeconds: '900',
  healthWriteProbeEnabled: true,
  healthHardwareProvider: 'auto',
  healthPresenceIntervalSeconds: '60',
  healthIncidentRetentionDays: '90',
});

export const HEALTH_LIMITS = Object.freeze({
  healthFastIntervalSeconds: [5, 60],
  healthNormalIntervalSeconds: [15, 600],
  healthSlowIntervalSeconds: [60, 3600],
  healthDeviceIntervalSeconds: [300, 86400],
  healthPresenceIntervalSeconds: [15, 3600],
  healthIncidentRetentionDays: [7, 3650],
});

export function healthSettingsFromResponse(response = {}) {
  return {
    healthEnabled: response.health_enabled !== false,
    healthProfile: response.health_profile || HEALTH_SETTINGS_DEFAULTS.healthProfile,
    healthFastIntervalSeconds: String(response.health_fast_interval_seconds ?? HEALTH_SETTINGS_DEFAULTS.healthFastIntervalSeconds),
    healthNormalIntervalSeconds: String(response.health_normal_interval_seconds ?? HEALTH_SETTINGS_DEFAULTS.healthNormalIntervalSeconds),
    healthSlowIntervalSeconds: String(response.health_slow_interval_seconds ?? HEALTH_SETTINGS_DEFAULTS.healthSlowIntervalSeconds),
    healthDeviceIntervalSeconds: String(response.health_device_interval_seconds ?? HEALTH_SETTINGS_DEFAULTS.healthDeviceIntervalSeconds),
    healthWriteProbeEnabled: response.health_write_probe_enabled !== false,
    healthHardwareProvider: response.health_hardware_provider || HEALTH_SETTINGS_DEFAULTS.healthHardwareProvider,
    healthPresenceIntervalSeconds: String(response.health_presence_interval_seconds ?? HEALTH_SETTINGS_DEFAULTS.healthPresenceIntervalSeconds),
    healthIncidentRetentionDays: String(response.health_incident_retention_days ?? HEALTH_SETTINGS_DEFAULTS.healthIncidentRetentionDays),
    healthConditionOverrides: response.health_condition_overrides || { version: 1, conditions: [] },
    healthEffectivePolicy: response.health_effective_policy || { conditions: [] },
  };
}

export function healthSettingsToPayload(settings) {
  return {
    health_enabled: Boolean(settings.healthEnabled),
    health_profile: settings.healthProfile,
    health_fast_interval_seconds: parseInt(settings.healthFastIntervalSeconds, 10),
    health_normal_interval_seconds: parseInt(settings.healthNormalIntervalSeconds, 10),
    health_slow_interval_seconds: parseInt(settings.healthSlowIntervalSeconds, 10),
    health_device_interval_seconds: parseInt(settings.healthDeviceIntervalSeconds, 10),
    health_write_probe_enabled: Boolean(settings.healthWriteProbeEnabled),
    health_hardware_provider: settings.healthHardwareProvider,
    health_presence_interval_seconds: parseInt(settings.healthPresenceIntervalSeconds, 10),
    health_incident_retention_days: parseInt(settings.healthIncidentRetentionDays, 10),
    health_condition_overrides: settings.healthConditionOverrides,
  };
}

const DWELL_MAX_SECONDS = 604800;
const OVERRIDE_PROFILES = new Set(['balanced', 'conservative', 'disabled']);

function wholeNumber(value) {
  const parsed = Number(value);
  return Number.isInteger(parsed) ? parsed : null;
}

function effectiveConditions(settings) {
  return Array.isArray(settings?.healthEffectivePolicy?.conditions)
    ? settings.healthEffectivePolicy.conditions.filter((condition) => condition?.code)
    : [];
}

function overrideConditions(settings) {
  return Array.isArray(settings?.healthConditionOverrides?.conditions)
    ? settings.healthConditionOverrides.conditions
    : [];
}

export function validateHealthSettings(settings) {
  const errors = [];
  if (!['balanced', 'conservative', 'disabled'].includes(settings.healthProfile))
    errors.push('profile');
  if (!['auto', 'smartctl', 'disabled'].includes(settings.healthHardwareProvider))
    errors.push('provider');
  const values = {};
  Object.entries(HEALTH_LIMITS).forEach(([field, [minimum, maximum]]) => {
    const value = wholeNumber(settings[field]);
    values[field] = value;
    if (value === null || value < minimum || value > maximum) errors.push(field);
  });
  if (values.healthFastIntervalSeconds !== null &&
      values.healthNormalIntervalSeconds !== null &&
      values.healthSlowIntervalSeconds !== null &&
      values.healthDeviceIntervalSeconds !== null &&
      !(values.healthFastIntervalSeconds <= values.healthNormalIntervalSeconds &&
        values.healthNormalIntervalSeconds <= values.healthSlowIntervalSeconds &&
        values.healthSlowIntervalSeconds <= values.healthDeviceIntervalSeconds))
    errors.push('ordered');

  const registry = new Map(effectiveConditions(settings).map((condition) => [condition.code, condition]));
  const seen = new Set();
  const overrides = overrideConditions(settings);
  if (settings?.healthConditionOverrides?.version !== 1 || overrides.length > registry.size)
    errors.push('overrides');
  overrides.forEach((override) => {
    const base = registry.get(override?.code);
    if (!base || seen.has(override.code)) {
      errors.push('overrides');
      return;
    }
    seen.add(override.code);
    if (override.profile) {
      if (!OVERRIDE_PROFILES.has(override.profile) ||
          Object.keys(override).some((key) => !['code', 'profile'].includes(key)))
        errors.push('overrides');
      return;
    }
    if (base.warning === undefined || override.unit !== base.unit) {
      errors.push('overrides');
      return;
    }
    const thresholds = ['warning', 'critical', 'recovery'].map((key) => Number(override[key]));
    if (thresholds.some((value) => !Number.isFinite(value))) {
      errors.push('thresholds');
      return;
    }
    if (base.unit === 'ratio' && thresholds.some((value) => value < 0 || value > 1))
      errors.push('thresholds');
    const lowerIsWorse = Number(base.critical) < Number(base.warning);
    if (lowerIsWorse
      ? !(thresholds[1] < thresholds[0] && thresholds[0] < thresholds[2])
      : !(thresholds[2] < thresholds[0] && thresholds[0] < thresholds[1]))
      errors.push('thresholdOrder');
    const dwell = ['warning_for_seconds', 'critical_for_seconds', 'recovery_for_seconds']
      .map((key) => wholeNumber(override[key]));
    if (dwell.some((value) => value === null || value < 0 || value > DWELL_MAX_SECONDS) ||
        dwell[0] === 0 || dwell[2] === 0)
      errors.push('dwell');
  });
  return [...new Set(errors)];
}

export function conditionOverrideMode(overrides, code) {
  const override = (overrides?.conditions || []).find((item) => item.code === code);
  if (!override) return 'default';
  return override.profile || 'custom';
}

export function updateConditionOverride(overrides, condition, mode, changes = {}) {
  const current = Array.isArray(overrides?.conditions) ? overrides.conditions : [];
  const remaining = current.filter((item) => item.code !== condition.code);
  if (mode === 'default') return { version: 1, conditions: remaining };
  const next = mode === 'custom' ? {
    code: condition.code,
    unit: condition.unit,
    warning: condition.warning,
    critical: condition.critical,
    recovery: condition.recovery,
    warning_for_seconds: condition.warning_for_seconds,
    critical_for_seconds: condition.critical_for_seconds,
    recovery_for_seconds: condition.recovery_for_seconds,
    ...changes,
  } : { code: condition.code, profile: mode };
  return {
    version: 1,
    conditions: [...remaining, next].sort((left, right) => left.code.localeCompare(right.code)),
  };
}

function Setting({ label, help, children }) {
  return (
    <div class="settings-item" data-setting-label={`${label} ${help || ''}`}>
      <label class="block text-sm font-medium">
        {label}
        {children}
      </label>
      {help && <p class="mt-1 text-xs text-muted-foreground">{help}</p>}
    </div>
  );
}

function NumericSetting({ name, settings, onChange, disabled, label, help }) {
  const [minimum, maximum] = HEALTH_LIMITS[name];
  return (
    <Setting label={label} help={help}>
      <input class={fieldClass} type="number" name={name} min={minimum} max={maximum} step="1" value={settings[name]} disabled={disabled} onInput={onChange} />
    </Setting>
  );
}

function ConditionOverride({ condition, overrides, disabled, onChange, t }) {
  const mode = conditionOverrideMode(overrides, condition.code);
  const explicit = (overrides.conditions || []).find((item) => item.code === condition.code);
  const custom = mode === 'custom' ? explicit : null;
  const supportsThresholds = condition.warning !== undefined;
  const setMode = (nextMode) => onChange(updateConditionOverride(
    overrides, condition, nextMode));
  const setCustom = (field, value) => onChange(updateConditionOverride(
    overrides, condition, 'custom', { ...custom, [field]: Number(value) }));
  const thresholdMax = condition.unit === 'ratio' ? 1 : undefined;
  const thresholdMin = condition.unit === 'ratio' ? 0 : undefined;
  return (
    <details class="rounded-lg border border-border bg-background p-3" data-setting-label={`${condition.code} ${t('settings.health.conditionOverride')}`}>
      <summary class="cursor-pointer list-none">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div><div class="font-medium">{condition.code.replace(/[._]/g, ' ')}</div><code class="text-[10px] text-muted-foreground">{condition.code}</code></div>
          <span class="rounded-full bg-muted px-2 py-1 text-xs">{mode === 'default' ? t('settings.health.profileDefault') : mode}</span>
        </div>
      </summary>
      <div class="mt-4">
        <Setting label={t('settings.health.overrideMode')} help={t('settings.health.overrideModeHelp')}>
          <select class={fieldClass} value={mode} disabled={disabled} onChange={(event) => setMode(event.currentTarget.value)}>
            <option value="default">{t('settings.health.profileDefault')}</option>
            <option value="balanced">{t('settings.health.profile.balanced')}</option>
            <option value="conservative">{t('settings.health.profile.conservative')}</option>
            <option value="disabled">{t('common.disabled')}</option>
            {supportsThresholds && <option value="custom">{t('settings.health.custom')}</option>}
          </select>
        </Setting>
        {custom && (
          <div class="mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            {['warning', 'critical', 'recovery'].map((field) => (
              <Setting key={field} label={t(`settings.health.${field}`)} help={condition.unit}>
                <input class={fieldClass} type="number" min={thresholdMin} max={thresholdMax} step="any" value={custom[field]} disabled={disabled} onInput={(event) => setCustom(field, event.currentTarget.value)} />
              </Setting>
            ))}
            {['warning_for_seconds', 'critical_for_seconds', 'recovery_for_seconds'].map((field) => (
              <Setting key={field} label={t(`settings.health.${field}`)}>
                <input class={fieldClass} type="number" min="0" max={DWELL_MAX_SECONDS} step="1" value={custom[field]} disabled={disabled} onInput={(event) => setCustom(field, event.currentTarget.value)} />
              </Setting>
            ))}
          </div>
        )}
      </div>
    </details>
  );
}

export function HealthTab({ settings, setSettings, handleInputChange, canModifySettings, validationErrors = [], serverError, t }) {
  const disabled = !canModifySettings;
  const conditions = effectiveConditions(settings);
  const overrides = settings.healthConditionOverrides || { version: 1, conditions: [] };
  const reset = () => setSettings((current) => ({
    ...current,
    ...HEALTH_SETTINGS_DEFAULTS,
    healthConditionOverrides: { version: 1, conditions: [] },
  }));
  const setOverrides = (next) => setSettings((current) => ({
    ...current,
    healthConditionOverrides: next,
  }));
  return (
    <div class="space-y-6" data-testid="health-settings-tab">
      <section class="settings-group rounded-lg bg-card p-4 text-card-foreground shadow">
        <div class="flex flex-wrap items-start justify-between gap-3">
          <div><h3 class="text-lg font-semibold">{t('settings.health.title')}</h3><p class="mt-1 text-sm text-muted-foreground">{t('settings.health.description')}</p></div>
          <button type="button" class="btn-secondary" disabled={disabled} onClick={reset}>{t('settings.health.resetDefaults')}</button>
        </div>
        <p class="mt-4 rounded-md border border-border bg-muted/20 p-3 text-sm" data-testid="health-reload-guidance">{t('settings.health.reloadGuidance')}</p>
        {disabled && <p class="mt-3 text-sm font-medium">{t('settings.readOnly')}</p>}
        {serverError && <p role="alert" class="mt-3 rounded-md bg-[hsl(var(--danger)/0.12)] p-3 text-sm text-[hsl(var(--danger))]">{serverError}</p>}
        {validationErrors.length > 0 && <p role="alert" class="mt-3 rounded-md bg-[hsl(var(--danger)/0.12)] p-3 text-sm text-[hsl(var(--danger))]">{t(`settings.health.validation.${validationErrors[0]}`)}</p>}
        <div class="mt-5 grid gap-4 md:grid-cols-2">
          <label class="settings-item flex cursor-pointer items-start gap-3 rounded-md border border-border p-3" data-setting-label={t('settings.health.enabled')}><input type="checkbox" name="healthEnabled" checked={settings.healthEnabled} disabled={disabled} onChange={handleInputChange} /><span><span class="block font-medium">{t('settings.health.enabled')}</span><span class="block text-xs text-muted-foreground">{t('settings.health.enabledHelp')}</span></span></label>
          <label class="settings-item flex cursor-pointer items-start gap-3 rounded-md border border-border p-3" data-setting-label={t('settings.health.writeProbe')}><input type="checkbox" name="healthWriteProbeEnabled" checked={settings.healthWriteProbeEnabled} disabled={disabled} onChange={handleInputChange} /><span><span class="block font-medium">{t('settings.health.writeProbe')}</span><span class="block text-xs text-muted-foreground">{t('settings.health.writeProbeHelp')}</span></span></label>
          <Setting label={t('settings.health.profile')} help={t('settings.health.profileHelp')}><select class={fieldClass} name="healthProfile" value={settings.healthProfile} disabled={disabled} onChange={handleInputChange}><option value="balanced">{t('settings.health.profile.balanced')}</option><option value="conservative">{t('settings.health.profile.conservative')}</option><option value="disabled">{t('common.disabled')}</option></select></Setting>
          <Setting label={t('settings.health.hardwareProvider')} help={t(`settings.health.providerHelp.${settings.healthHardwareProvider}`)}><select class={fieldClass} name="healthHardwareProvider" value={settings.healthHardwareProvider} disabled={disabled} onChange={handleInputChange}><option value="auto">{t('settings.health.provider.auto')}</option><option value="smartctl">{t('settings.health.provider.smartctl')}</option><option value="disabled">{t('common.disabled')}</option></select></Setting>
        </div>
      </section>

      <section class="settings-group rounded-lg bg-card p-4 text-card-foreground shadow">
        <h3 class="text-lg font-semibold">{t('settings.health.cadences')}</h3>
        <p class="mt-1 text-sm text-muted-foreground">{t('settings.health.cadencesHelp')}</p>
        <div class="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
          <NumericSetting name="healthFastIntervalSeconds" settings={settings} onChange={handleInputChange} disabled={disabled} label={t('settings.health.fastInterval')} help="5–60 s" />
          <NumericSetting name="healthNormalIntervalSeconds" settings={settings} onChange={handleInputChange} disabled={disabled} label={t('settings.health.normalInterval')} help="15–600 s" />
          <NumericSetting name="healthSlowIntervalSeconds" settings={settings} onChange={handleInputChange} disabled={disabled} label={t('settings.health.slowInterval')} help="60–3600 s" />
          <NumericSetting name="healthDeviceIntervalSeconds" settings={settings} onChange={handleInputChange} disabled={disabled} label={t('settings.health.deviceInterval')} help="300–86400 s" />
          <NumericSetting name="healthPresenceIntervalSeconds" settings={settings} onChange={handleInputChange} disabled={disabled} label={t('settings.health.presenceInterval')} help="15–3600 s" />
          <NumericSetting name="healthIncidentRetentionDays" settings={settings} onChange={handleInputChange} disabled={disabled} label={t('settings.health.retention')} help="7–3650 days" />
        </div>
      </section>

      <section class="settings-group rounded-lg bg-card p-4 text-card-foreground shadow">
        <div class="flex flex-wrap items-start justify-between gap-3"><div><h3 class="text-lg font-semibold">{t('settings.health.conditions')}</h3><p class="mt-1 text-sm text-muted-foreground">{t('settings.health.conditionsHelp')}</p></div><button type="button" class="btn-secondary" disabled={disabled || !Array.isArray(overrides.conditions) || overrides.conditions.length === 0} onClick={() => setOverrides({ version: 1, conditions: [] })}>{t('settings.health.clearOverrides')}</button></div>
        {conditions.length === 0 ? <p class="mt-4 rounded-md border border-border p-3 text-sm text-muted-foreground">{t('settings.health.registryUnavailable')}</p> : <div class="mt-4 grid gap-3 xl:grid-cols-2">{conditions.map((condition) => <ConditionOverride key={condition.code} condition={condition} overrides={overrides} disabled={disabled} onChange={setOverrides} t={t} />)}</div>}
      </section>
    </div>
  );
}
