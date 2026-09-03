import { useCallback, useEffect, useMemo, useState } from 'preact/hooks';
import { CollectionSelectorBuilder } from '../fleet/CollectionSelectorBuilder.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import {
  DEFAULT_SCHEDULE_WINDOW,
  EMPTY_ROUTE_DRAFT,
  HEALTH_EVENT_FAMILY,
  buildRoutePayload,
  healthSeveritiesFromCatalog,
  routeToDraft,
  shortEventType,
  validateRouteDraft,
} from './eventRouting.js';

const DETECTION_TYPE = 'io.lightnvr.detection.object.v1';
const fieldClasses = 'mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';
const DAYS = [
  ['events.schedule.sunday', 0],
  ['events.schedule.monday', 1],
  ['events.schedule.tuesday', 2],
  ['events.schedule.wednesday', 3],
  ['events.schedule.thursday', 4],
  ['events.schedule.friday', 5],
  ['events.schedule.saturday', 6],
];

function Field({ label, help, children }) {
  return (
    <label className="block text-sm font-medium">
      {label}
      {children}
      {help && <span className="mt-1 block text-xs font-normal text-muted-foreground">{help}</span>}
    </label>
  );
}

function ScheduleWindow({ window, index, onChange, onRemove, t }) {
  const toggleDay = (day) => {
    const days = window.days.includes(day)
      ? window.days.filter((value) => value !== day)
      : [...window.days, day];
    onChange({ ...window, days });
  };
  return (
    <article className="rounded-lg border border-border bg-muted/15 p-3">
      <div className="flex items-center justify-between gap-3">
        <h4 className="text-sm font-semibold">{t('events.schedule.windowNumber', { number: index + 1 })}</h4>
        <button type="button" className="text-sm text-[hsl(var(--danger))] hover:underline" onClick={onRemove}>{t('common.remove')}</button>
      </div>
      <div className="mt-3 flex flex-wrap gap-2">
        {DAYS.map(([key, day]) => {
          const checked = window.days.includes(day);
          return (
            <label key={day} className={`cursor-pointer rounded-full border px-2.5 py-1 text-xs ${checked ? 'border-[hsl(var(--primary))] bg-[hsl(var(--primary)/0.12)]' : 'border-border'}`}>
              <input type="checkbox" className="sr-only" checked={checked} onChange={() => toggleDay(day)} />
              {t(key)}
            </label>
          );
        })}
      </div>
      <div className="mt-3 grid grid-cols-2 gap-3">
        <Field label={t('events.schedule.start')}><input className={fieldClasses} type="time" value={window.start} onInput={(event) => onChange({ ...window, start: event.currentTarget.value })} /></Field>
        <Field label={t('events.schedule.end')}><input className={fieldClasses} type="time" value={window.end} onInput={(event) => onChange({ ...window, end: event.currentTarget.value })} /></Field>
      </div>
    </article>
  );
}

function PreviewResult({ preview, t }) {
  if (!preview) return null;
  const hasSystemEvents = (preview.event_types || []).some(
    (eventType) => eventType.subject_kind === 'system');
  return (
    <div className="rounded-lg border border-[hsl(var(--primary)/0.35)] bg-[hsl(var(--primary)/0.06)] p-4">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <h3 className="font-semibold">{t('events.route.previewResult')}</h3>
        <span className="rounded-full bg-[hsl(var(--primary)/0.14)] px-3 py-1 text-sm font-semibold">{t('events.route.cameraMatches', { count: preview.matched_camera_count || 0 })}</span>
      </div>
      {(preview.camera_sample || []).length > 0 && (
        <div className="mt-3 grid gap-2 sm:grid-cols-2">
          {preview.camera_sample.map((camera) => (
            <div key={camera.camera_uuid} className="rounded border border-border bg-card px-3 py-2 text-sm">
              <div className="font-medium">{camera.name}</div>
              <div className="truncate text-xs text-muted-foreground">{camera.location_path || camera.camera_uuid}</div>
            </div>
          ))}
        </div>
      )}
      <p className="mt-3 text-xs text-muted-foreground">{t('events.route.previewNoPublish')}</p>
      {hasSystemEvents && (
        <p className="mt-2 rounded-md border border-border bg-card p-3 text-xs font-medium">
          {t('events.route.systemPreviewHelp')}
        </p>
      )}
    </div>
  );
}

export function EventRouteEditor({ route, catalog, healthConditions, healthRegistryUnavailable, destinations, locations, tags, onPreview, onSave, onClose, t }) {
  const creating = !route;
  const [draft, setDraft] = useState(() => route ? routeToDraft(route) : { ...EMPTY_ROUTE_DRAFT, eventTypes: [], windows: [] });
  const [preview, setPreview] = useState(null);
  const [previewing, setPreviewing] = useState(false);
  const [saving, setSaving] = useState(false);
  const healthSeverities = useMemo(
    () => healthSeveritiesFromCatalog(catalog, draft.eventTypes),
    [catalog, draft.eventTypes]
  );
  const validationCode = useMemo(() => validateRouteDraft(draft, {
    catalog,
    healthConditions,
    healthSeverities,
  }), [catalog, draft, healthConditions, healthSeverities]);
  const detectionSelected = draft.eventTypes.includes(DETECTION_TYPE);
  const selectedDefinitions = draft.eventTypes.map(
    (type) => catalog.find((eventType) => eventType.type === type)).filter(Boolean);
  const healthSelected = selectedDefinitions.some(
    (eventType) => eventType.family === HEALTH_EVENT_FAMILY);
  const cameraOnly = selectedDefinitions.length === draft.eventTypes.length &&
    selectedDefinitions.every((eventType) => eventType.subject_kind === 'camera');

  useEffect(() => {
    const handleKey = (event) => {
      if (event.key === 'Escape' && !saving && !previewing) onClose();
    };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [onClose, previewing, saving]);

  const update = (changes) => {
    setDraft((current) => ({ ...current, ...changes }));
    setPreview(null);
  };
  const handleSelectorChange = useCallback((selector, selectorError) => {
    setDraft((current) => ({ ...current, selector, selectorError }));
    setPreview(null);
  }, []);
  const toggleEventType = (type) => {
    const eventTypes = draft.eventTypes.includes(type)
      ? draft.eventTypes.filter((value) => value !== type)
      : [...draft.eventTypes, type];
    update({
      eventTypes,
      scopeType: eventTypes.some((selectedType) =>
        catalog.find((eventType) => eventType.type === selectedType)?.subject_kind !== 'camera')
        ? 'all' : draft.scopeType,
      detectionFilterEnabled: type === DETECTION_TYPE && !eventTypes.includes(DETECTION_TYPE)
        ? false : draft.detectionFilterEnabled,
      healthFilterEnabled: eventTypes.some((selectedType) =>
        catalog.find((eventType) => eventType.type === selectedType)?.family === HEALTH_EVENT_FAMILY)
        ? draft.healthFilterEnabled : false,
    });
  };
  const toggleHealthCondition = (code) => update({
    healthConditionCodes: draft.healthConditionCodes.includes(code)
      ? draft.healthConditionCodes.filter((value) => value !== code)
      : [...draft.healthConditionCodes, code],
  });
  const toggleHealthSeverity = (severity) => update({
    healthSeverities: draft.healthSeverities.includes(severity)
      ? draft.healthSeverities.filter((value) => value !== severity)
      : [...draft.healthSeverities, severity],
  });
  const updateWindow = (index, nextWindow) => update({ windows: draft.windows.map((window, windowIndex) => windowIndex === index ? nextWindow : window) });
  const removeWindow = (index) => update({ windows: draft.windows.filter((_, windowIndex) => windowIndex !== index) });
  const setScheduled = (enabled) => update({
    scheduleEnabled: enabled,
    windows: enabled && draft.windows.length === 0 ? [{ ...DEFAULT_SCHEDULE_WINDOW, days: [...DEFAULT_SCHEDULE_WINDOW.days] }] : draft.windows,
  });

  const runPreview = async () => {
    if (validationCode) {
      showStatusMessage(t(`events.route.validation.${validationCode}`), 'error', 7000);
      return;
    }
    setPreviewing(true);
    try {
      setPreview(await onPreview(buildRoutePayload(draft, true)));
    } catch (_error) {
      // The workspace surfaces request errors and leaves the draft intact.
    } finally {
      setPreviewing(false);
    }
  };
  const submit = async (event) => {
    event.preventDefault();
    if (validationCode) {
      showStatusMessage(t(`events.route.validation.${validationCode}`), 'error', 7000);
      return;
    }
    setSaving(true);
    try {
      await onSave(buildRoutePayload(draft, creating));
    } catch (_error) {
      // The workspace surfaces request errors and keeps this editor open.
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/55 p-3" role="dialog" aria-modal="true" aria-labelledby="event-route-editor-title">
      <form className="flex max-h-[94vh] w-full max-w-6xl flex-col overflow-hidden rounded-xl border border-border bg-card text-card-foreground shadow-2xl" onSubmit={submit}>
        <div className="flex items-start justify-between gap-4 border-b border-border p-5">
          <div>
            <h2 id="event-route-editor-title" className="text-xl font-bold">{t(creating ? 'events.route.addTitle' : 'events.route.editTitle')}</h2>
            <p className="mt-1 text-sm text-muted-foreground">{t('events.route.editorDescription')}</p>
          </div>
          <button type="button" className="rounded px-2 text-2xl text-muted-foreground hover:bg-muted hover:text-foreground" onClick={onClose} aria-label={t('common.close')}>×</button>
        </div>

        <div className="space-y-6 overflow-y-auto p-5">
          <section>
            <div className="grid gap-4 md:grid-cols-2">
              <Field label={t('common.name')}><input className={fieldClasses} value={draft.name} maxLength="127" autoFocus onInput={(event) => update({ name: event.currentTarget.value })} /></Field>
              <Field label={t('events.route.destination')} help={t('events.route.destinationHelp')}>
                <select className={fieldClasses} value={draft.destination} onChange={(event) => update({ destination: event.currentTarget.value })}>
                  {destinations.map((destination) => <option key={destination.key} value={destination.key} disabled={destination.managed && destination.enabled === false}>{destination.name}{destination.enabled === false ? ` — ${t('common.disabled')}` : ''}</option>)}
                </select>
              </Field>
              <div className="md:col-span-2"><Field label={t('common.description')}><textarea className={`${fieldClasses} min-h-20`} value={draft.description} maxLength="511" onInput={(event) => update({ description: event.currentTarget.value })}></textarea></Field></div>
            </div>
            <label className="touch-target mt-4 inline-flex cursor-pointer items-center gap-3 rounded-md border border-border bg-muted/15 px-3 py-2 text-sm"><input type="checkbox" checked={draft.enabled} onChange={(event) => update({ enabled: event.currentTarget.checked })} /><span><span className="font-medium">{t('events.route.enabled')}</span> <span className="text-muted-foreground">— {t('events.route.enabledHelp')}</span></span></label>
          </section>

          <section className="border-t border-border pt-5">
            <h3 className="font-semibold">{t('events.route.eventTypes')}</h3>
            <p className="mt-1 text-sm text-muted-foreground">{t('events.route.eventTypesHelp')}</p>
            <div className="mt-3 grid gap-2 md:grid-cols-2 xl:grid-cols-3">
              {catalog.map((eventType) => {
                const selected = draft.eventTypes.includes(eventType.type);
                return (
                  <label key={eventType.type} className={`cursor-pointer rounded-lg border p-3 ${selected ? 'border-[hsl(var(--primary))] bg-[hsl(var(--primary)/0.08)]' : 'border-border bg-background'}`}>
                    <div className="flex items-start gap-3"><input className="mt-1" type="checkbox" checked={selected} onChange={() => toggleEventType(eventType.type)} /><span className="min-w-0"><span className="block font-medium">{shortEventType(eventType.type)}</span><span className="block text-xs text-muted-foreground">{eventType.description}</span><span className="mt-1 block font-mono text-[10px] text-muted-foreground break-all">{eventType.type}</span></span></div>
                  </label>
                );
              })}
            </div>
          </section>

          {detectionSelected && (
            <section className="border-t border-border pt-5">
              <label className="touch-target flex cursor-pointer items-start gap-3"><input className="mt-1" type="checkbox" checked={draft.detectionFilterEnabled} onChange={(event) => update({ detectionFilterEnabled: event.currentTarget.checked })} /><span><span className="block font-semibold">{t('events.route.detectionFilters')}</span><span className="block text-sm text-muted-foreground">{t('events.route.detectionFiltersHelp')}</span></span></label>
              {draft.detectionFilterEnabled && (
                <div className="mt-4 grid gap-4 md:grid-cols-3">
                  <Field label={t('events.route.labels')} help={t('events.route.listHelp')}><input className={fieldClasses} value={draft.labels} placeholder="person, vehicle" onInput={(event) => update({ labels: event.currentTarget.value })} /></Field>
                  <Field label={t('events.route.zones')} help={t('events.route.listHelp')}><input className={fieldClasses} value={draft.zones} placeholder="entrance, loading-dock" onInput={(event) => update({ zones: event.currentTarget.value })} /></Field>
                  <Field label={t('events.route.minConfidence')} help={t('events.route.confidenceHelp')}><input className={fieldClasses} type="number" min="0" max="1" step="0.01" value={draft.minConfidence} onInput={(event) => update({ minConfidence: event.currentTarget.value })} /></Field>
                </div>
              )}
            </section>
          )}

          {healthSelected && (
            <section className="border-t border-border pt-5">
              <label className="touch-target flex cursor-pointer items-start gap-3">
                <input className="mt-1" type="checkbox" checked={draft.healthFilterEnabled} onChange={(event) => update({ healthFilterEnabled: event.currentTarget.checked })} />
                <span>
                  <span className="block font-semibold">{t('events.route.healthFilters')}</span>
                  <span className="block text-sm text-muted-foreground">{t('events.route.healthFiltersHelp')}</span>
                </span>
              </label>
              {draft.healthFilterEnabled && (
                <div className="mt-4 space-y-4">
                  {healthRegistryUnavailable || healthConditions.length === 0 ? (
                    <p className="rounded-md border border-[hsl(var(--warning)/0.5)] bg-[hsl(var(--warning)/0.1)] p-3 text-sm">
                      {t('events.route.healthRegistryUnavailable')}
                    </p>
                  ) : (
                    <fieldset>
                      <legend className="text-sm font-semibold">{t('events.route.healthConditions')}</legend>
                      <div className="mt-2 grid gap-2 sm:grid-cols-2 xl:grid-cols-3">
                        {healthConditions.map((condition) => (
                          <label key={condition.code} className="flex cursor-pointer items-start gap-2 rounded-md border border-border p-2 text-sm">
                            <input className="mt-1" type="checkbox" checked={draft.healthConditionCodes.includes(condition.code)} onChange={() => toggleHealthCondition(condition.code)} />
                            <span><span className="block">{condition.code.replace(/[._]/g, ' ')}</span><code className="block text-[10px] text-muted-foreground">{condition.code}</code></span>
                          </label>
                        ))}
                      </div>
                    </fieldset>
                  )}
                  <fieldset>
                    <legend className="text-sm font-semibold">{t('events.route.healthSeverities')}</legend>
                    <div className="mt-2 flex flex-wrap gap-3">
                      {healthSeverities.map((severity) => (
                        <label key={severity} className="inline-flex cursor-pointer items-center gap-2 rounded-md border border-border px-3 py-2 text-sm">
                          <input type="checkbox" checked={draft.healthSeverities.includes(severity)} onChange={() => toggleHealthSeverity(severity)} />
                          {severity}
                        </label>
                      ))}
                    </div>
                  </fieldset>
                </div>
              )}
            </section>
          )}

          <section className="border-t border-border pt-5">
            <div className="flex flex-wrap items-start justify-between gap-3"><div><h3 className="font-semibold">{t('events.route.cameraScope')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('events.route.cameraScopeHelp')}</p></div><select className="rounded-md border border-input bg-background px-3 py-2 text-sm" value={draft.scopeType} onChange={(event) => update({ scopeType: event.currentTarget.value })}><option value="all">{t('events.route.allCameras')}</option><option value="selector" disabled={!cameraOnly}>{t('events.route.dynamicScope')}</option></select></div>
            {!cameraOnly && <p className="mt-3 rounded-md border border-border bg-muted/20 p-3 text-sm font-medium">{t('events.route.systemScopeHelp')}</p>}
            {draft.scopeType === 'selector' && <div className="mt-4"><CollectionSelectorBuilder key={route?.uuid || 'new-route'} initialSelector={draft.selector} locations={locations} tags={tags} allowCameraSelection idPrefix={`event-route-${route?.uuid || 'new'}`} onChange={handleSelectorChange} t={t} /></div>}
          </section>

          <section className="border-t border-border pt-5">
            <div className="flex flex-wrap items-center justify-between gap-3"><div><h3 className="font-semibold">{t('events.route.schedule')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('events.route.scheduleHelp')}</p></div><label className="touch-target inline-flex cursor-pointer items-center gap-2 text-sm"><input type="checkbox" checked={draft.scheduleEnabled} onChange={(event) => setScheduled(event.currentTarget.checked)} />{t('events.route.restrictSchedule')}</label></div>
            <div className="mt-4 max-w-sm"><Field label={t('events.schedule.timezone')} help={t('events.schedule.timezoneHelp')}><input className={fieldClasses} value={draft.timezone} placeholder="America/New_York" onInput={(event) => update({ timezone: event.currentTarget.value })} /></Field></div>
            {draft.scheduleEnabled && <div className="mt-4 space-y-3">{draft.windows.map((window, index) => <ScheduleWindow key={index} window={window} index={index} onChange={(nextWindow) => updateWindow(index, nextWindow)} onRemove={() => removeWindow(index)} t={t} />)}<button type="button" className="btn-secondary" onClick={() => update({ windows: [...draft.windows, { ...DEFAULT_SCHEDULE_WINDOW, days: [...DEFAULT_SCHEDULE_WINDOW.days] }] })}>{t('events.schedule.addWindow')}</button></div>}
          </section>

          <section className="border-t border-border pt-5">
            <h3 className="font-semibold">{t('events.route.suppression')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('events.route.suppressionHelp')}</p>
            <div className="mt-4 grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
              <Field label={t('events.route.debounce')} help={t('events.route.debounceHelp')}><input className={fieldClasses} type="number" min="0" max="86400" value={draft.debounceSeconds} onInput={(event) => update({ debounceSeconds: event.currentTarget.value })} /></Field>
              <Field label={t('events.route.cooldown')} help={t('events.route.cooldownHelp')}><input className={fieldClasses} type="number" min="0" max="604800" value={draft.cooldownSeconds} onInput={(event) => update({ cooldownSeconds: event.currentTarget.value })} /></Field>
              <Field label={t('events.route.grouping')} help={t('events.route.groupingHelp')}><input className={fieldClasses} type="number" min="0" max="3600" value={draft.groupingWindowSeconds} onInput={(event) => update({ groupingWindowSeconds: event.currentTarget.value })} /></Field>
              <Field label={t('events.route.rateLimit')} help={t('events.route.rateLimitHelp')}><input className={fieldClasses} type="number" min="0" max="60000" value={draft.maxEventsPerMinute} onInput={(event) => update({ maxEventsPerMinute: event.currentTarget.value })} /></Field>
            </div>
          </section>

          <PreviewResult preview={preview} t={t} />
        </div>

        <div className="flex flex-wrap items-center justify-end gap-3 border-t border-border bg-muted/20 p-4">
          {validationCode && <span className="mr-auto text-xs text-[hsl(var(--danger))]">{t(`events.route.validation.${validationCode}`)}</span>}
          <button type="button" className="btn-secondary" onClick={runPreview} disabled={saving || previewing || Boolean(validationCode)}>{previewing ? t('events.route.previewing') : t('events.route.preview')}</button>
          <button type="button" className="btn-secondary" onClick={onClose} disabled={saving || previewing}>{t('common.cancel')}</button>
          <button type="submit" className="btn-primary" disabled={saving || previewing || Boolean(validationCode)}>{saving ? t('common.saving') : t(creating ? 'events.route.create' : 'common.saveChanges')}</button>
        </div>
      </form>
    </div>
  );
}
