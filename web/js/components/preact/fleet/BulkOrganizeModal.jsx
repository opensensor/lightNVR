import { useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { applyBulkOrganization, buildLocationRows } from './fleetOrganization.js';

export function BulkOrganizeModal({ cameras, locations, tags, onComplete, onClose, t }) {
  const [locationUuid, setLocationUuid] = useState('');
  const [tagOperation, setTagOperation] = useState('none');
  const [tagUuids, setTagUuids] = useState([]);
  const [progress, setProgress] = useState({ completed: 0, total: cameras.length });
  const [working, setWorking] = useState(false);
  const [failures, setFailures] = useState([]);
  const locationRows = useMemo(() => buildLocationRows(locations), [locations]);
  const sortedTags = useMemo(() => [...tags].sort((left, right) => left.label.localeCompare(right.label)), [tags]);
  const hasTagChange = tagOperation === 'replace' || (tagOperation !== 'none' && tagUuids.length > 0);
  const hasChanges = Boolean(locationUuid) || hasTagChange;

  useEffect(() => {
    const handleKey = (event) => { if (event.key === 'Escape' && !working) onClose(); };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [onClose, working]);

  const toggleTag = (uuid) => {
    setTagUuids((current) => current.includes(uuid) ? current.filter((item) => item !== uuid) : [...current, uuid]);
  };

  const submit = async (event) => {
    event.preventDefault();
    if (!hasChanges) return;
    setWorking(true);
    setFailures([]);
    setProgress({ completed: 0, total: cameras.length });
    try {
      const result = await applyBulkOrganization(
        cameras,
        { locationUuid, tagOperation, tagUuids },
        fetchJSON,
        (completed, total) => setProgress({ completed, total })
      );
      await onComplete(result);
      if (result.failed.length === 0) {
        showStatusMessage(t('fleet.bulk.success', { count: result.succeeded.length }), 'success');
        onClose();
      } else {
        setFailures(result.failed);
        showStatusMessage(t('fleet.bulk.partial', { succeeded: result.succeeded.length, failed: result.failed.length }), 'warning', 8000);
      }
    } catch (error) {
      showStatusMessage(t('fleet.bulk.error', { message: error.message }), 'error', 8000);
    } finally {
      setWorking(false);
    }
  };

  return (
    <div className="fixed inset-0 z-40 flex items-center justify-center bg-black/55 p-3" onMouseDown={(event) => { if (!working && event.target === event.currentTarget) onClose(); }}>
      <div role="dialog" aria-modal="true" aria-labelledby="bulk-organize-title" className="max-h-[92vh] w-full max-w-2xl overflow-y-auto rounded-xl border border-border bg-card text-card-foreground shadow-2xl">
        <header className="flex items-start justify-between border-b border-border p-4">
          <div>
            <h2 id="bulk-organize-title" className="text-xl font-bold">{t('fleet.bulk.title')}</h2>
            <p className="mt-1 text-sm text-muted-foreground">{t('fleet.bulk.description', { count: cameras.length })}</p>
          </div>
          <button type="button" className="rounded p-2 text-2xl leading-none hover:bg-muted disabled:opacity-40" onClick={onClose} disabled={working} aria-label={t('common.close')}>×</button>
        </header>

        <form onSubmit={submit} className="space-y-5 p-4">
          <fieldset>
            <legend className="font-semibold">{t('fleet.bulk.location')}</legend>
            <p className="mb-2 text-xs text-muted-foreground">{t('fleet.bulk.locationHelp')}</p>
            <select className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value={locationUuid} onChange={(event) => setLocationUuid(event.currentTarget.value)} disabled={working}>
              <option value="">{t('fleet.bulk.keepLocations')}</option>
              {locationRows.map((location) => <option key={location.uuid} value={location.uuid}>{location.path}</option>)}
            </select>
          </fieldset>

          <fieldset>
            <legend className="font-semibold">{t('fleet.bulk.tags')}</legend>
            <p className="mb-2 text-xs text-muted-foreground">{t('fleet.bulk.tagsHelp')}</p>
            <div className="mb-3 grid grid-cols-2 gap-2 sm:grid-cols-4">
              {['none', 'add', 'remove', 'replace'].map((operation) => (
                <label key={operation} className={`cursor-pointer rounded-md border p-2 text-center text-sm ${tagOperation === operation ? 'border-[hsl(var(--primary))] bg-[hsl(var(--primary)/0.1)]' : 'border-border'}`}>
                  <input className="sr-only" type="radio" name="tag-operation" value={operation} checked={tagOperation === operation} onChange={() => setTagOperation(operation)} disabled={working} />
                  {t(`fleet.bulk.tagOperation.${operation}`)}
                </label>
              ))}
            </div>
            {tagOperation !== 'none' && (
              <div className="max-h-52 overflow-y-auto rounded-md border border-border p-2">
                {sortedTags.map((tag) => (
                  <label key={tag.uuid} className="flex cursor-pointer items-center justify-between gap-3 rounded px-2 py-1.5 hover:bg-muted/50">
                    <span className="flex min-w-0 items-center gap-2">
                      <input type="checkbox" className="h-4 w-4" checked={tagUuids.includes(tag.uuid)} onChange={() => toggleTag(tag.uuid)} disabled={working} />
                      <span className="h-3 w-3 flex-none rounded-full border border-border" style={{ backgroundColor: tag.color || 'transparent' }}></span>
                      <span className="truncate text-sm">{tag.label}</span>
                    </span>
                    <span className="text-xs tabular-nums text-muted-foreground">{tag.camera_count}</span>
                  </label>
                ))}
                {sortedTags.length === 0 && <p className="p-3 text-center text-sm text-muted-foreground">{t('fleet.organization.noTags')}</p>}
              </div>
            )}
            {tagOperation === 'replace' && <p className="mt-2 text-xs text-[hsl(var(--warning-foreground))]">{t('fleet.bulk.replaceWarning')}</p>}
          </fieldset>

          {working && (
            <div>
              <div className="mb-1 flex justify-between text-xs text-muted-foreground"><span>{t('fleet.bulk.working')}</span><span>{progress.completed}/{progress.total}</span></div>
              <div className="h-2 overflow-hidden rounded-full bg-muted"><div className="h-full bg-[hsl(var(--primary))] transition-all" style={{ width: `${progress.total ? (progress.completed / progress.total) * 100 : 0}%` }}></div></div>
            </div>
          )}

          {failures.length > 0 && (
            <div className="rounded-md border border-[hsl(var(--danger)/0.45)] bg-[hsl(var(--danger)/0.08)] p-3">
              <h3 className="font-semibold">{t('fleet.bulk.failures')}</h3>
              <ul className="mt-2 max-h-32 list-disc overflow-y-auto pl-5 text-sm">
                {failures.map(({ camera, error }) => <li key={camera.camera_uuid}>{camera.name}: {error}</li>)}
              </ul>
            </div>
          )}

          <div className="flex justify-end gap-2 border-t border-border pt-4">
            <button type="button" className="btn-secondary" onClick={onClose} disabled={working}>{t('common.cancel')}</button>
            <button type="submit" className="btn-primary" disabled={working || !hasChanges}>{working ? t('fleet.bulk.working') : t('fleet.bulk.apply')}</button>
          </div>
        </form>
      </div>
    </div>
  );
}
