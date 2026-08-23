import { useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { buildLocationRows } from './fleetOrganization.js';
import {
  COLLECTION_RULE_TYPES,
  buildCollectionSelector,
  collectionRuleToExpression,
  createCollectionRule,
  parseCollectionSelector,
  updateCollectionRuleType,
  validateCollectionSelectorJson,
} from './collectionSelector.js';

const fieldClasses = 'rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';
const HEALTH_OPTIONS = ['unknown', 'up', 'degraded', 'down', 'disabled'];
const RECORDING_OPTIONS = ['off', 'continuous', 'detection'];
const CAPABILITY_OPTIONS = ['onvif', 'ptz', 'backchannel'];

function toggleValue(values, value) {
  return values.includes(value) ? values.filter((item) => item !== value) : [...values, value];
}

function ValueChecks({ values, options, onChange, getLabel, idPrefix }) {
  return (
    <div className="flex max-h-36 flex-wrap gap-2 overflow-y-auto rounded-md border border-border bg-background p-2">
      {options.map((option) => {
        const value = typeof option === 'string' ? option : option.value;
        const label = typeof option === 'string' ? getLabel(option) : option.label;
        const checked = values.includes(value);
        return (
          <label key={value} htmlFor={`${idPrefix}-${value}`} className={`cursor-pointer rounded-full border px-2.5 py-1 text-xs ${checked ? 'border-[hsl(var(--primary))] bg-[hsl(var(--primary)/0.12)]' : 'border-border'}`}>
            <input id={`${idPrefix}-${value}`} type="checkbox" className="sr-only" checked={checked} onChange={() => onChange(toggleValue(values, value))} />
            {label}
          </label>
        );
      })}
    </div>
  );
}

function RuleValueEditor({ rule, index, locations, tags, onChange, t }) {
  const idPrefix = `collection-rule-${index}`;
  if (rule.type === 'location_subtree') {
    return (
      <select className={`${fieldClasses} min-w-56 flex-1`} value={rule.uuid} onChange={(event) => onChange({ ...rule, uuid: event.currentTarget.value })}>
        <option value="">{t('collections.builder.chooseLocation')}</option>
        {locations.map((location) => <option key={location.uuid} value={location.uuid}>{location.path}</option>)}
      </select>
    );
  }
  if (rule.type === 'enabled') {
    return (
      <select className={`${fieldClasses} min-w-40 flex-1`} value={String(rule.value)} onChange={(event) => onChange({ ...rule, value: event.currentTarget.value === 'true' })}>
        <option value="true">{t('common.enabled')}</option>
        <option value="false">{t('common.disabled')}</option>
      </select>
    );
  }
  if (rule.type.startsWith('tag_')) {
    return (
      <div className="min-w-56 flex-1">
        <ValueChecks values={rule.values || []} options={tags.map((tag) => ({ value: tag.uuid, label: tag.label }))} onChange={(values) => onChange({ ...rule, values })} getLabel={(value) => value} idPrefix={idPrefix} />
      </div>
    );
  }
  if (rule.type === 'health') {
    return <div className="min-w-56 flex-1"><ValueChecks values={rule.values || []} options={HEALTH_OPTIONS} onChange={(values) => onChange({ ...rule, values })} getLabel={(value) => t(`fleet.health.${value}`)} idPrefix={idPrefix} /></div>;
  }
  if (rule.type === 'recording_mode') {
    return <div className="min-w-56 flex-1"><ValueChecks values={rule.values || []} options={RECORDING_OPTIONS} onChange={(values) => onChange({ ...rule, values })} getLabel={(value) => t(`fleet.recording.${value}`)} idPrefix={idPrefix} /></div>;
  }
  if (rule.type.startsWith('capability_')) {
    return <div className="min-w-56 flex-1"><ValueChecks values={rule.values || []} options={CAPABILITY_OPTIONS} onChange={(values) => onChange({ ...rule, values })} getLabel={(value) => t(`collections.capability.${value}`)} idPrefix={idPrefix} /></div>;
  }
  return (
    <input
      className={`${fieldClasses} min-w-56 flex-1`}
      value={(rule.values || []).join(', ')}
      placeholder={t(rule.type === 'vendor' ? 'collections.builder.vendorPlaceholder' : 'collections.builder.modelPlaceholder')}
      onInput={(event) => onChange({ ...rule, values: event.currentTarget.value.split(',').map((value) => value.trimStart()) })}
    />
  );
}

function SelectorPreview({ preview, t }) {
  if (!preview) return null;
  return (
    <div className="mt-4 rounded-lg border border-border bg-muted/30 p-3">
      <div className="flex items-center justify-between gap-3">
        <h4 className="font-semibold">{t('collections.preview.title')}</h4>
        <span className="rounded-full bg-[hsl(var(--primary)/0.12)] px-2.5 py-1 text-sm font-semibold">{t('collections.preview.matches', { count: preview.total })}</span>
      </div>
      <div className="mt-2 max-h-48 divide-y divide-border overflow-y-auto rounded border border-border bg-card">
        {(preview.cameras || []).map((camera) => (
          <div key={camera.camera_uuid} className="flex items-start justify-between gap-3 px-3 py-2 text-sm">
            <div className="min-w-0"><div className="truncate font-medium">{camera.name}</div><div className="truncate text-xs text-muted-foreground">{camera.location?.path || t('fleet.unassigned')}</div></div>
            <span className="whitespace-nowrap text-xs text-muted-foreground">{camera.health ? t(`fleet.health.${camera.health}`) : ''}</span>
          </div>
        ))}
        {preview.total === 0 && <p className="p-4 text-center text-sm text-muted-foreground">{t('collections.preview.empty')}</p>}
      </div>
      {preview.total > (preview.cameras || []).length && <p className="mt-2 text-xs text-muted-foreground">{t('collections.preview.sample', { count: preview.cameras.length, total: preview.total })}</p>}
    </div>
  );
}

export function CollectionSelectorBuilder({ initialSelector, locations, tags, onChange, t }) {
  const initial = initialSelector || { version: 1, expression: { op: 'all' } };
  const parsedInitial = parseCollectionSelector(initial);
  const [match, setMatch] = useState(parsedInitial.match);
  const [rules, setRules] = useState(parsedInitial.rules);
  const [advanced, setAdvanced] = useState(!parsedInitial.supported);
  const [advancedJson, setAdvancedJson] = useState(() => JSON.stringify(initial, null, 2));
  const [preview, setPreview] = useState(null);
  const [previewing, setPreviewing] = useState(false);
  const [previewError, setPreviewError] = useState('');
  const locationRows = useMemo(() => buildLocationRows(locations), [locations]);
  const advancedValidation = useMemo(() => validateCollectionSelectorJson(advancedJson), [advancedJson]);
  const visualSelector = useMemo(() => buildCollectionSelector(rules, match), [rules, match]);
  const currentSelector = advanced ? advancedValidation.selector : visualSelector;
  const visualError = !advanced && rules.some((rule) => !collectionRuleToExpression(rule))
    ? t('collections.builder.incompleteRule')
    : '';
  const validationError = advanced ? advancedValidation.error : visualError;

  useEffect(() => {
    onChange(currentSelector, validationError);
  }, [currentSelector, validationError, onChange]);

  const updateRule = (index, nextRule) => setRules((current) => current.map((rule, ruleIndex) => ruleIndex === index ? nextRule : rule));
  const switchMode = () => {
    if (advanced) {
      const parsed = advancedValidation.selector && parseCollectionSelector(advancedValidation.selector);
      if (!parsed?.supported) {
        setPreviewError(t('collections.builder.cannotVisualize'));
        return;
      }
      setRules(parsed.rules);
      setMatch(parsed.match);
      setAdvanced(false);
    } else {
      setAdvancedJson(JSON.stringify(visualSelector, null, 2));
      setAdvanced(true);
    }
    setPreview(null);
    setPreviewError('');
  };

  const runPreview = async () => {
    if (!currentSelector) return;
    setPreviewing(true);
    setPreviewError('');
    try {
      const result = await fetchJSON('/api/fleet/selectors/preview', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ selector: currentSelector, page: 1, page_size: 20, facets: false, explain: true }),
        timeout: 20000,
        retries: 1,
      });
      setPreview(result);
    } catch (error) {
      setPreviewError(error.message);
      setPreview(null);
    } finally {
      setPreviewing(false);
    }
  };

  return (
    <div className="rounded-lg border border-border p-3">
      <div className="mb-3 flex flex-wrap items-center justify-between gap-2">
        <div>
          <h3 className="font-semibold">{t('collections.builder.title')}</h3>
          <p className="text-xs text-muted-foreground">{t('collections.builder.description')}</p>
        </div>
        <button type="button" className="btn-secondary" onClick={switchMode}>{t(advanced ? 'collections.builder.visualMode' : 'collections.builder.advancedMode')}</button>
      </div>

      {advanced ? (
        <div>
          <textarea className="min-h-56 w-full rounded-md border border-input bg-background p-3 font-mono text-xs" value={advancedJson} onInput={(event) => { setAdvancedJson(event.currentTarget.value); setPreview(null); }} spellCheck="false"></textarea>
          {advancedValidation.error && <p className="mt-1 text-xs text-[hsl(var(--danger))]">{advancedValidation.error}</p>}
        </div>
      ) : (
        <div className="space-y-3">
          <div className="flex items-center gap-2 text-sm">
            <span>{t('collections.builder.match')}</span>
            <select className={fieldClasses} value={match} onChange={(event) => { setMatch(event.currentTarget.value); setPreview(null); }}>
              <option value="all">{t('collections.builder.allRules')}</option>
              <option value="any">{t('collections.builder.anyRule')}</option>
            </select>
          </div>
          {rules.map((rule, index) => (
            <div key={index} className="flex flex-col gap-2 rounded-md border border-border bg-muted/20 p-2 sm:flex-row sm:items-start">
              <select className={`${fieldClasses} sm:w-48`} value={rule.type} onChange={(event) => { updateRule(index, updateCollectionRuleType(rule, event.currentTarget.value)); setPreview(null); }}>
                {COLLECTION_RULE_TYPES.map((type) => <option key={type} value={type}>{t(`collections.rule.${type}`)}</option>)}
              </select>
              <RuleValueEditor rule={rule} index={index} locations={locationRows} tags={tags} t={t} onChange={(nextRule) => { updateRule(index, nextRule); setPreview(null); }} />
              <button type="button" className="rounded px-2 py-2 text-sm text-[hsl(var(--danger))] hover:bg-muted" onClick={() => { setRules((current) => current.filter((_, ruleIndex) => ruleIndex !== index)); setPreview(null); }} aria-label={t('collections.builder.removeRule')}>×</button>
            </div>
          ))}
          <button type="button" className="btn-secondary" onClick={() => { setRules((current) => [...current, createCollectionRule()]); setPreview(null); }}>{t('collections.builder.addRule')}</button>
          {rules.length === 0 && <p className="rounded-md bg-[hsl(var(--warning)/0.12)] p-2 text-xs">{t('collections.builder.matchesAll')}</p>}
        </div>
      )}

      <div className="mt-3 flex items-center justify-end gap-3 border-t border-border pt-3">
        {previewError && <span className="mr-auto text-xs text-[hsl(var(--danger))]">{previewError}</span>}
        <button type="button" className="btn-secondary" onClick={runPreview} disabled={previewing || !currentSelector || Boolean(validationError)}>{previewing ? t('collections.preview.loading') : t('collections.preview.action')}</button>
      </div>
      <SelectorPreview preview={preview} t={t} />
    </div>
  );
}
