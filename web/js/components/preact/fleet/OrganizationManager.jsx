import { useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { ConfirmDialog } from '../common/ModalDialog.jsx';
import { buildLocationRows, locationParentOptions } from './fleetOrganization.js';

const fieldClasses = 'w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';
const locationTypes = ['site', 'building', 'floor', 'area', 'zone'];

function LocationEditor({ editor, locations, onSaved, onCancel, t }) {
  const item = editor.item;
  const [form, setForm] = useState(() => ({
    name: item?.name || '',
    type: item?.type || 'area',
    parent_uuid: item?.parent_uuid || editor.parentUuid || '',
    sort_order: item?.sort_order || 0,
  }));
  const [saving, setSaving] = useState(false);
  const parentOptions = useMemo(() => locationParentOptions(locations, item?.uuid), [locations, item?.uuid]);

  const submit = async (event) => {
    event.preventDefault();
    setSaving(true);
    try {
      await fetchJSON(item ? `/api/locations/${item.uuid}` : '/api/locations', {
        method: item ? 'PUT' : 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: form.name.trim(),
          type: form.type,
          parent_uuid: form.parent_uuid || null,
          sort_order: Number(form.sort_order) || 0,
        }),
        timeout: 15000,
        retries: 1,
      });
      showStatusMessage(t(item ? 'fleet.organization.locationUpdated' : 'fleet.organization.locationCreated'), 'success');
      await onSaved();
    } catch (error) {
      showStatusMessage(t('fleet.organization.saveError', { message: error.message }), 'error', 8000);
    } finally {
      setSaving(false);
    }
  };

  return (
    <form className="mb-4 rounded-lg border border-border bg-muted/30 p-4" onSubmit={submit}>
      <h3 className="mb-3 font-semibold">{t(item ? 'fleet.organization.editLocation' : 'fleet.organization.addLocation')}</h3>
      <div className="grid gap-3 sm:grid-cols-2">
        <label className="text-sm font-medium">
          {t('common.name')}
          <input className={`${fieldClasses} mt-1`} value={form.name} maxLength={127} required autoFocus onInput={(event) => setForm({ ...form, name: event.currentTarget.value })} />
        </label>
        <label className="text-sm font-medium">
          {t('fleet.organization.type')}
          <select className={`${fieldClasses} mt-1`} value={form.type} onChange={(event) => setForm({ ...form, type: event.currentTarget.value })}>
            {locationTypes.map((type) => <option key={type} value={type}>{t(`fleet.locationType.${type}`)}</option>)}
          </select>
        </label>
        <label className="text-sm font-medium">
          {t('fleet.organization.parent')}
          <select className={`${fieldClasses} mt-1`} value={form.parent_uuid} onChange={(event) => setForm({ ...form, parent_uuid: event.currentTarget.value })}>
            <option value="">{t('fleet.organization.topLevel')}</option>
            {parentOptions.filter((location) => !location.is_system).map((location) => (
              <option key={location.uuid} value={location.uuid}>{location.path}</option>
            ))}
          </select>
        </label>
        <label className="text-sm font-medium">
          {t('fleet.organization.sortOrder')}
          <input className={`${fieldClasses} mt-1`} type="number" value={form.sort_order} onInput={(event) => setForm({ ...form, sort_order: event.currentTarget.value })} />
        </label>
      </div>
      <div className="mt-4 flex justify-end gap-2">
        <button type="button" className="btn-secondary" onClick={onCancel} disabled={saving}>{t('common.cancel')}</button>
        <button type="submit" className="btn-primary" disabled={saving || !form.name.trim()}>{saving ? t('common.saving') : t('common.saveChanges')}</button>
      </div>
    </form>
  );
}

function LocationsTab({ locations, loading, onRefresh, t }) {
  const [editor, setEditor] = useState(null);
  const [deleteCandidate, setDeleteCandidate] = useState(null);
  const rows = useMemo(() => buildLocationRows(locations), [locations]);

  const deleteLocation = async (location) => {
    try {
      await fetchJSON(`/api/locations/${location.uuid}`, { method: 'DELETE', timeout: 15000, retries: 1 });
      showStatusMessage(t('fleet.organization.locationDeleted'), 'success');
      await onRefresh();
    } catch (error) {
      showStatusMessage(t('fleet.organization.deleteError', { message: error.message }), 'error', 8000);
    }
  };

  return (
    <div>
      <div className="mb-4 flex items-center justify-between gap-3">
        <p className="text-sm text-muted-foreground">{t('fleet.organization.locationsDescription')}</p>
        {!editor && <button type="button" className="btn-primary whitespace-nowrap" onClick={() => setEditor({ parentUuid: '' })}>{t('fleet.organization.addRoot')}</button>}
      </div>
      {editor && <LocationEditor key={`${editor.item?.uuid || 'new'}-${editor.parentUuid || ''}`} editor={editor} locations={locations} t={t} onCancel={() => setEditor(null)} onSaved={async () => { setEditor(null); await onRefresh(); }} />}
      <div className="overflow-x-auto rounded-lg border border-border">
        <table className="min-w-full divide-y divide-border text-sm">
          <thead className="bg-muted/70 text-xs uppercase tracking-wide text-muted-foreground">
            <tr>
              <th className="px-3 py-2 text-left">{t('fleet.column.location')}</th>
              <th className="px-3 py-2 text-left">{t('fleet.organization.type')}</th>
              <th className="px-3 py-2 text-right">{t('fleet.organization.contents')}</th>
              <th className="px-3 py-2 text-right">{t('common.actions')}</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-border">
            {rows.map((location) => {
              const occupied = location.camera_count > 0 || location.child_count > 0;
              return (
                <tr key={location.uuid} className="hover:bg-muted/30">
                  <td className="px-3 py-2">
                    <div className="flex items-center" style={{ paddingLeft: `${Math.min(location.depth, 8) * 1.25}rem` }}>
                      {location.depth > 0 && <span className="mr-2 text-muted-foreground" aria-hidden="true">└</span>}
                      <span className="font-medium">{location.name}</span>
                      {location.is_system && <span className="ml-2 rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">{t('fleet.organization.system')}</span>}
                    </div>
                  </td>
                  <td className="px-3 py-2 text-muted-foreground">{t(`fleet.locationType.${location.type}`)}</td>
                  <td className="whitespace-nowrap px-3 py-2 text-right text-muted-foreground">{t('fleet.organization.contentCounts', { cameras: location.camera_count, children: location.child_count })}</td>
                  <td className="whitespace-nowrap px-3 py-2 text-right">
                    {!location.is_system && <>
                      <button type="button" className="px-2 py-1 text-[hsl(var(--primary))] hover:underline" onClick={() => setEditor({ parentUuid: location.uuid })}>{t('fleet.organization.addChild')}</button>
                      <button type="button" className="px-2 py-1 text-[hsl(var(--primary))] hover:underline" onClick={() => setEditor({ item: location })}>{t('common.edit')}</button>
                      <button type="button" className="px-2 py-1 text-[hsl(var(--danger))] hover:underline disabled:cursor-not-allowed disabled:opacity-40" disabled={occupied} title={occupied ? t('fleet.organization.locationNotEmpty') : ''} onClick={() => setDeleteCandidate(location)}>{t('common.delete')}</button>
                    </>}
                  </td>
                </tr>
              );
            })}
            {!loading && rows.length === 0 && <tr><td colSpan="4" className="px-4 py-10 text-center text-muted-foreground">{t('fleet.organization.noLocations')}</td></tr>}
          </tbody>
        </table>
      </div>
      <ConfirmDialog
        isOpen={Boolean(deleteCandidate)}
        onClose={() => setDeleteCandidate(null)}
        onConfirm={() => deleteLocation(deleteCandidate)}
        title={t('fleet.organization.locations')}
        message={deleteCandidate ? t('fleet.organization.deleteLocationConfirm', { name: deleteCandidate.name }) : ''}
        confirmLabel={t('common.delete')}
        cancelLabel={t('common.cancel')}
        variant="danger"
      />
    </div>
  );
}

function TagEditor({ editor, tags, onSaved, onCancel, t }) {
  const item = editor.item;
  const merging = editor.mode === 'merge';
  const [form, setForm] = useState(() => ({
    label: item?.label || '',
    color: item?.color || '',
    description: item?.description || '',
    target_uuid: '',
  }));
  const [saving, setSaving] = useState(false);

  const submit = async (event) => {
    event.preventDefault();
    setSaving(true);
    try {
      if (merging) {
        await fetchJSON(`/api/camera-tags/${item.uuid}/merge`, {
          method: 'POST', headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ target_uuid: form.target_uuid }), timeout: 15000, retries: 1,
        });
      } else {
        await fetchJSON(item ? `/api/camera-tags/${item.uuid}` : '/api/camera-tags', {
          method: item ? 'PUT' : 'POST', headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ label: form.label.trim(), color: form.color.trim(), description: form.description.trim() }),
          timeout: 15000, retries: 1,
        });
      }
      showStatusMessage(t(merging ? 'fleet.organization.tagsMerged' : item ? 'fleet.organization.tagUpdated' : 'fleet.organization.tagCreated'), 'success');
      await onSaved();
    } catch (error) {
      showStatusMessage(t('fleet.organization.saveError', { message: error.message }), 'error', 8000);
    } finally {
      setSaving(false);
    }
  };

  return (
    <form className="mb-4 rounded-lg border border-border bg-muted/30 p-4" onSubmit={submit}>
      <h3 className="mb-3 font-semibold">{t(merging ? 'fleet.organization.mergeTagNamed' : item ? 'fleet.organization.editTag' : 'fleet.organization.addTag', { name: item?.label })}</h3>
      {merging ? (
        <label className="block text-sm font-medium">
          {t('fleet.organization.mergeInto')}
          <select className={`${fieldClasses} mt-1`} value={form.target_uuid} required autoFocus onChange={(event) => setForm({ ...form, target_uuid: event.currentTarget.value })}>
            <option value="">{t('fleet.organization.chooseTag')}</option>
            {tags.filter((tag) => tag.uuid !== item.uuid).map((tag) => <option key={tag.uuid} value={tag.uuid}>{tag.label} ({tag.camera_count})</option>)}
          </select>
          <span className="mt-2 block text-xs text-muted-foreground">{t('fleet.organization.mergeWarning')}</span>
        </label>
      ) : (
        <div className="grid gap-3 sm:grid-cols-2">
          <label className="text-sm font-medium">
            {t('fleet.organization.tagLabel')}
            <input className={`${fieldClasses} mt-1`} value={form.label} maxLength={127} required autoFocus onInput={(event) => setForm({ ...form, label: event.currentTarget.value })} />
          </label>
          <label className="text-sm font-medium">
            {t('fleet.organization.color')}
            <div className="mt-1 flex items-center gap-2">
              <span className="h-8 w-8 flex-none rounded-full border border-border" style={{ backgroundColor: /^#[0-9a-f]{6}$/i.test(form.color) ? form.color : 'transparent' }}></span>
              <input className={fieldClasses} value={form.color} pattern="^#[0-9A-Fa-f]{6}$" placeholder="#2563eb" onInput={(event) => setForm({ ...form, color: event.currentTarget.value })} />
            </div>
          </label>
          <label className="text-sm font-medium sm:col-span-2">
            {t('fleet.organization.tagDescription')}
            <textarea className={`${fieldClasses} mt-1`} rows="2" value={form.description} maxLength={255} onInput={(event) => setForm({ ...form, description: event.currentTarget.value })}></textarea>
          </label>
        </div>
      )}
      <div className="mt-4 flex justify-end gap-2">
        <button type="button" className="btn-secondary" onClick={onCancel} disabled={saving}>{t('common.cancel')}</button>
        <button type="submit" className="btn-primary" disabled={saving || (merging ? !form.target_uuid : !form.label.trim())}>{saving ? t('common.saving') : t(merging ? 'fleet.organization.merge' : 'common.saveChanges')}</button>
      </div>
    </form>
  );
}

function TagsTab({ tags, loading, onRefresh, t }) {
  const [editor, setEditor] = useState(null);
  const [deleteCandidate, setDeleteCandidate] = useState(null);
  const sortedTags = useMemo(() => [...tags].sort((left, right) => left.label.localeCompare(right.label)), [tags]);
  const deleteTag = async (tag) => {
    try {
      await fetchJSON(`/api/camera-tags/${tag.uuid}`, { method: 'DELETE', timeout: 15000, retries: 1 });
      showStatusMessage(t('fleet.organization.tagDeleted'), 'success');
      await onRefresh();
    } catch (error) {
      showStatusMessage(t('fleet.organization.deleteError', { message: error.message }), 'error', 8000);
    }
  };

  return (
    <div>
      <div className="mb-4 flex items-center justify-between gap-3">
        <p className="text-sm text-muted-foreground">{t('fleet.organization.tagsDescription')}</p>
        {!editor && <button type="button" className="btn-primary whitespace-nowrap" onClick={() => setEditor({})}>{t('fleet.organization.addTag')}</button>}
      </div>
      {editor && <TagEditor key={`${editor.mode || 'edit'}-${editor.item?.uuid || 'new'}`} editor={editor} tags={tags} t={t} onCancel={() => setEditor(null)} onSaved={async () => { setEditor(null); await onRefresh(); }} />}
      <div className="overflow-x-auto rounded-lg border border-border">
        <table className="min-w-full divide-y divide-border text-sm">
          <thead className="bg-muted/70 text-xs uppercase tracking-wide text-muted-foreground">
            <tr>
              <th className="px-3 py-2 text-left">{t('fleet.organization.tagLabel')}</th>
              <th className="px-3 py-2 text-left">{t('fleet.organization.tagDescription')}</th>
              <th className="px-3 py-2 text-right">{t('fleet.organization.cameras')}</th>
              <th className="px-3 py-2 text-right">{t('common.actions')}</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-border">
            {sortedTags.map((tag) => (
              <tr key={tag.uuid} className="hover:bg-muted/30">
                <td className="px-3 py-2"><span className="mr-2 inline-block h-3 w-3 rounded-full border border-border align-middle" style={{ backgroundColor: tag.color || 'transparent' }}></span><span className="font-medium">{tag.label}</span></td>
                <td className="max-w-sm truncate px-3 py-2 text-muted-foreground">{tag.description || '—'}</td>
                <td className="px-3 py-2 text-right tabular-nums">{tag.camera_count}</td>
                <td className="whitespace-nowrap px-3 py-2 text-right">
                  <button type="button" className="px-2 py-1 text-[hsl(var(--primary))] hover:underline" onClick={() => setEditor({ item: tag })}>{t('common.edit')}</button>
                  {tags.length > 1 && <button type="button" className="px-2 py-1 text-[hsl(var(--primary))] hover:underline" onClick={() => setEditor({ item: tag, mode: 'merge' })}>{t('fleet.organization.merge')}</button>}
                  <button type="button" className="px-2 py-1 text-[hsl(var(--danger))] hover:underline" onClick={() => setDeleteCandidate(tag)}>{t('common.delete')}</button>
                </td>
              </tr>
            ))}
            {!loading && sortedTags.length === 0 && <tr><td colSpan="4" className="px-4 py-10 text-center text-muted-foreground">{t('fleet.organization.noTags')}</td></tr>}
          </tbody>
        </table>
      </div>
      <ConfirmDialog
        isOpen={Boolean(deleteCandidate)}
        onClose={() => setDeleteCandidate(null)}
        onConfirm={() => deleteTag(deleteCandidate)}
        title={t('fleet.organization.tags')}
        message={deleteCandidate ? t('fleet.organization.deleteTagConfirm', { name: deleteCandidate.label, count: deleteCandidate.camera_count }) : ''}
        confirmLabel={t('common.delete')}
        cancelLabel={t('common.cancel')}
        variant="danger"
      />
    </div>
  );
}

export function OrganizationManager({ locations, tags, loading, onRefresh, onClose, t }) {
  const [tab, setTab] = useState('locations');
  useEffect(() => {
    const handleKey = (event) => { if (event.key === 'Escape') onClose(); };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [onClose]);

  return (
    <div className="fixed inset-0 z-40 flex items-center justify-center bg-black/55 p-3" onMouseDown={(event) => { if (event.target === event.currentTarget) onClose(); }}>
      <div role="dialog" aria-modal="true" aria-labelledby="organization-title" className="flex max-h-[92vh] w-full max-w-5xl flex-col overflow-hidden rounded-xl border border-border bg-card text-card-foreground shadow-2xl">
        <header className="flex items-start justify-between border-b border-border p-4">
          <div>
            <h2 id="organization-title" className="text-xl font-bold">{t('fleet.organization.title')}</h2>
            <p className="mt-1 text-sm text-muted-foreground">{t('fleet.organization.description')}</p>
          </div>
          <button type="button" className="rounded p-2 text-2xl leading-none hover:bg-muted" onClick={onClose} aria-label={t('common.close')}>×</button>
        </header>
        <div className="border-b border-border px-4 pt-3" role="tablist">
          {['locations', 'tags'].map((value) => <button key={value} type="button" role="tab" aria-selected={tab === value} className={`mr-2 rounded-t-lg px-4 py-2 text-sm font-medium ${tab === value ? 'border border-b-0 border-border bg-background -mb-px' : 'text-muted-foreground'}`} onClick={() => setTab(value)}>{t(`fleet.organization.${value}`)}</button>)}
        </div>
        <div className="overflow-y-auto p-4">
          {tab === 'locations'
            ? <LocationsTab locations={locations} loading={loading} onRefresh={onRefresh} t={t} />
            : <TagsTab tags={tags} loading={loading} onRefresh={onRefresh} t={t} />}
        </div>
      </div>
    </div>
  );
}
