import { useMemo, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { ConfirmDialog } from '../common/ModalDialog.jsx';
import { buildFleetSavedViewPayload } from './fleetQuery.js';

export function SavedViewsBar({ data, state, onApply, onRefresh, t }) {
  const views = data?.views || [];
  const [selectedUuid, setSelectedUuid] = useState('');
  const [showForm, setShowForm] = useState(false);
  const [name, setName] = useState('');
  const [shared, setShared] = useState(false);
  const [saving, setSaving] = useState(false);
  const [deleteCandidate, setDeleteCandidate] = useState(null);
  const selected = useMemo(
    () => views.find((view) => view.uuid === selectedUuid) || null,
    [selectedUuid, views]
  );

  const selectView = (uuid) => {
    setSelectedUuid(uuid);
    const view = views.find((candidate) => candidate.uuid === uuid);
    if (view && !onApply(view)) {
      showStatusMessage(t('fleet.views.unsupported'), 'warning', 8000);
    }
  };

  const save = async (event) => {
    event.preventDefault();
    if (!name.trim()) return;
    setSaving(true);
    try {
      const created = await fetchJSON('/api/fleet/views', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(buildFleetSavedViewPayload(name, shared, state)),
      });
      setName('');
      setShared(false);
      setShowForm(false);
      setSelectedUuid(created.uuid);
      await onRefresh();
      showStatusMessage(t('fleet.views.saved'), 'success');
    } catch (error) {
      showStatusMessage(t('fleet.views.saveError', { message: error.message }), 'error', 8000);
    } finally {
      setSaving(false);
    }
  };

  const remove = async (view) => {
    if (!view?.owned) return;
    try {
      await fetchJSON(`/api/fleet/views/${encodeURIComponent(view.uuid)}`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ revision: view.revision }),
      });
      setSelectedUuid('');
      await onRefresh();
      showStatusMessage(t('fleet.views.deleted'), 'success');
    } catch (error) {
      showStatusMessage(t('fleet.views.deleteError', { message: error.message }), 'error', 8000);
    }
  };

  return (
    <section className="mb-4 rounded-lg border border-border bg-card p-3 shadow-sm" aria-label={t('fleet.views.title')}>
      <div className="flex flex-col gap-2 sm:flex-row sm:items-center">
        <label className="text-sm font-semibold" htmlFor="fleet-saved-view">{t('fleet.views.title')}</label>
        <select
          id="fleet-saved-view"
          className="min-w-0 flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm sm:max-w-sm"
          value={selectedUuid}
          onChange={(event) => selectView(event.currentTarget.value)}
        >
          <option value="">{views.length ? t('fleet.views.choose') : t('fleet.views.none')}</option>
          {views.map((view) => (
            <option key={view.uuid} value={view.uuid}>
              {view.name}{view.is_shared ? ` · ${t('fleet.views.shared')}` : ''}
            </option>
          ))}
        </select>
        <div className="flex flex-wrap gap-2">
          <button type="button" className="btn-secondary" onClick={() => setShowForm((value) => !value)}>
            {t('fleet.views.saveCurrent')}
          </button>
          {selected?.owned && (
            <button type="button" className="btn-secondary text-[hsl(var(--danger))]" onClick={() => setDeleteCandidate(selected)}>
              {t('common.delete')}
            </button>
          )}
        </div>
      </div>
      {showForm && (
        <form className="mt-3 flex flex-col gap-3 border-t border-border pt-3 sm:flex-row sm:items-end" onSubmit={save}>
          <label className="min-w-0 flex-1 text-sm">
            <span className="mb-1 block font-medium">{t('fleet.views.name')}</span>
            <input
              className="w-full rounded-md border border-input bg-background px-3 py-2"
              value={name}
              maxLength="127"
              required
              onInput={(event) => setName(event.currentTarget.value)}
            />
          </label>
          {data?.can_share && (
            <label className="touch-target flex min-h-11 cursor-pointer items-center gap-2 text-sm">
              <input type="checkbox" className="h-4 w-4 rounded" checked={shared} onChange={(event) => setShared(event.currentTarget.checked)} />
              {t('fleet.views.share')}
            </label>
          )}
          <div className="flex gap-2">
            <button type="submit" className="btn-primary" disabled={saving || !name.trim()}>
              {saving ? t('common.saving') : t('common.saveChanges')}
            </button>
            <button type="button" className="btn-secondary" onClick={() => setShowForm(false)}>{t('common.cancel')}</button>
          </div>
        </form>
      )}
      <ConfirmDialog
        isOpen={Boolean(deleteCandidate)}
        onClose={() => setDeleteCandidate(null)}
        onConfirm={() => remove(deleteCandidate)}
        title={t('fleet.views.title')}
        message={deleteCandidate ? t('fleet.views.deleteConfirm', { name: deleteCandidate.name }) : ''}
        confirmLabel={t('common.delete')}
        cancelLabel={t('common.cancel')}
        variant="danger"
      />
    </section>
  );
}
