import { useCallback, useEffect, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { CollectionSelectorBuilder } from './CollectionSelectorBuilder.jsx';
import { StaticCollectionMembers } from './StaticCollectionMembers.jsx';

const fieldClasses = 'w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';
const ALL_SELECTOR = { version: 1, expression: { op: 'all' } };

function CollectionEditor({ spec, locations, tags, onSaved, onCancel, t }) {
  const collection = spec.collection;
  const [form, setForm] = useState(() => ({
    name: collection?.name || '',
    description: collection?.description || '',
    type: collection?.type || 'static',
    shared: collection?.shared ?? true,
  }));
  const [members, setMembers] = useState(spec.members || []);
  const [selector, setSelector] = useState(collection?.selector || ALL_SELECTOR);
  const [selectorError, setSelectorError] = useState('');
  const [saving, setSaving] = useState(false);
  const handleSelectorChange = useCallback((value, error) => {
    setSelector(value);
    setSelectorError(error);
  }, []);

  const submit = async (event) => {
    event.preventDefault();
    if (form.type === 'smart' && (!selector || selectorError)) return;
    setSaving(true);
    try {
      const payload = {
        name: form.name.trim(),
        description: form.description.trim(),
        type: form.type,
        shared: form.shared,
      };
      if (form.type === 'smart') payload.selector = selector;
      const saved = await fetchJSON(collection ? `/api/camera-collections/${collection.uuid}` : '/api/camera-collections', {
        method: collection ? 'PUT' : 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
        timeout: 20000,
        retries: 1,
      });
      if (form.type === 'static') {
        await fetchJSON(`/api/camera-collections/${saved.uuid}/members`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ camera_uuids: members }),
          timeout: 30000,
          retries: 1,
        });
      }
      showStatusMessage(t(collection ? 'collections.updated' : 'collections.created'), 'success');
      await onSaved();
    } catch (error) {
      showStatusMessage(t('collections.saveError', { message: error.message }), 'error', 8000);
    } finally {
      setSaving(false);
    }
  };

  return (
    <form onSubmit={submit} className="space-y-4">
      <div className="flex items-center justify-between gap-3">
        <div><h3 className="text-lg font-semibold">{t(collection ? 'collections.edit' : 'collections.create')}</h3><p className="text-xs text-muted-foreground">{t('collections.editorDescription')}</p></div>
        <button type="button" className="btn-secondary" onClick={onCancel} disabled={saving}>{t('collections.backToList')}</button>
      </div>
      <div className="grid gap-3 sm:grid-cols-2">
        <label className="text-sm font-medium">
          {t('common.name')}
          <input className={`${fieldClasses} mt-1`} value={form.name} maxLength={127} required autoFocus onInput={(event) => setForm({ ...form, name: event.currentTarget.value })} />
        </label>
        <label className="text-sm font-medium">
          {t('collections.type')}
          <select className={`${fieldClasses} mt-1`} value={form.type} onChange={(event) => setForm({ ...form, type: event.currentTarget.value })}>
            <option value="static">{t('collections.type.static')}</option>
            <option value="smart">{t('collections.type.smart')}</option>
          </select>
        </label>
        <label className="text-sm font-medium sm:col-span-2">
          {t('collections.description')}
          <textarea className={`${fieldClasses} mt-1`} rows="2" maxLength={511} value={form.description} onInput={(event) => setForm({ ...form, description: event.currentTarget.value })}></textarea>
        </label>
        <label className="flex items-start gap-2 text-sm sm:col-span-2">
          <input type="checkbox" className="mt-1 h-4 w-4" checked={form.shared} onChange={(event) => setForm({ ...form, shared: event.currentTarget.checked })} />
          <span><span className="block font-medium">{t('collections.shared')}</span><span className="block text-xs text-muted-foreground">{t('collections.sharedHelp')}</span></span>
        </label>
      </div>

      {collection && collection.type !== form.type && <p className="rounded-md bg-[hsl(var(--warning)/0.13)] p-3 text-xs">{t('collections.typeChangeWarning')}</p>}
      {form.type === 'static'
        ? <StaticCollectionMembers selectedUuids={members} onChange={setMembers} t={t} />
        : <CollectionSelectorBuilder key={collection?.uuid || 'new-smart'} initialSelector={selector || ALL_SELECTOR} locations={locations} tags={tags} onChange={handleSelectorChange} t={t} />}
      {form.type === 'smart' && selectorError && <p className="text-sm text-[hsl(var(--danger))]">{t('collections.selectorError', { message: selectorError })}</p>}

      <div className="flex justify-end gap-2 border-t border-border pt-4">
        <button type="button" className="btn-secondary" onClick={onCancel} disabled={saving}>{t('common.cancel')}</button>
        <button type="submit" className="btn-primary" disabled={saving || !form.name.trim() || (form.type === 'smart' && (!selector || selectorError))}>{saving ? t('common.saving') : t('common.saveChanges')}</button>
      </div>
    </form>
  );
}

function CollectionPreview({ data, t }) {
  return (
    <div className="mt-3 rounded-md border border-border bg-muted/20 p-3">
      <div className="mb-2 text-sm font-semibold">{t('collections.preview.matches', { count: data.matched_count })}</div>
      <div className="grid gap-2 sm:grid-cols-2">
        {(data.sample || []).map((camera) => <div key={camera.camera_uuid} className="min-w-0 rounded border border-border bg-card px-2 py-1.5"><div className="truncate text-sm font-medium">{camera.name}</div><div className="truncate text-xs text-muted-foreground">{camera.location_path || t('fleet.unassigned')}</div></div>)}
      </div>
      {data.matched_count === 0 && <p className="text-sm text-muted-foreground">{t('collections.preview.empty')}</p>}
      {data.matched_count > (data.sample || []).length && <p className="mt-2 text-xs text-muted-foreground">{t('collections.preview.sample', { count: data.sample.length, total: data.matched_count })}</p>}
    </div>
  );
}

function CollectionList({ collections, loading, isAdmin, previews, previewingUuid, onPreview, onEdit, onDelete, onCreate, t }) {
  return (
    <div>
      <div className="mb-4 flex items-center justify-between gap-3">
        <p className="text-sm text-muted-foreground">{t('collections.listDescription')}</p>
        {isAdmin && <button type="button" className="btn-primary whitespace-nowrap" onClick={onCreate}>{t('collections.create')}</button>}
      </div>
      <div className="space-y-3">
        {collections.map((collection) => (
          <article key={collection.uuid} className="rounded-lg border border-border p-4">
            <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
              <div className="min-w-0">
                <div className="flex flex-wrap items-center gap-2">
                  <h3 className="font-semibold">{collection.name}</h3>
                  <span className="rounded-full bg-muted px-2 py-0.5 text-xs">{t(`collections.type.${collection.type}`)}</span>
                  <span className="rounded-full bg-muted px-2 py-0.5 text-xs">{t(collection.shared ? 'collections.visibility.shared' : 'collections.visibility.private')}</span>
                  {collection.selector_redacted && <span className="rounded-full bg-muted px-2 py-0.5 text-xs">{t('collections.selectorRedacted')}</span>}
                </div>
                {collection.description && <p className="mt-1 text-sm text-muted-foreground">{collection.description}</p>}
                <p className="mt-2 text-sm"><span className="font-semibold tabular-nums">{collection.effective_count}</span> {t('collections.cameras')}</p>
              </div>
              <div className="flex flex-wrap gap-2">
                <button type="button" className="btn-secondary" disabled={previewingUuid === collection.uuid} onClick={() => onPreview(collection)}>{previewingUuid === collection.uuid ? t('collections.preview.loading') : previews[collection.uuid] ? t('collections.preview.hide') : t('collections.preview.action')}</button>
                {isAdmin && <button type="button" className="btn-secondary" onClick={() => onEdit(collection)}>{t('common.edit')}</button>}
                {isAdmin && <button type="button" className="btn-danger" onClick={() => onDelete(collection)}>{t('common.delete')}</button>}
              </div>
            </div>
            {previews[collection.uuid] && <CollectionPreview data={previews[collection.uuid]} t={t} />}
          </article>
        ))}
        {!loading && collections.length === 0 && <div className="rounded-lg border border-dashed border-border px-6 py-12 text-center"><h3 className="font-semibold">{t('collections.emptyTitle')}</h3><p className="mt-1 text-sm text-muted-foreground">{t(isAdmin ? 'collections.emptyAdmin' : 'collections.emptyViewer')}</p></div>}
        {loading && collections.length === 0 && <p className="py-10 text-center text-sm text-muted-foreground">{t('common.loading')}</p>}
      </div>
    </div>
  );
}

export function CollectionManager({ collections, loading, isAdmin, locations, tags, onRefresh, onClose, t }) {
  const [editorSpec, setEditorSpec] = useState(null);
  const [editorLoading, setEditorLoading] = useState(false);
  const [previews, setPreviews] = useState({});
  const [previewingUuid, setPreviewingUuid] = useState('');

  useEffect(() => {
    const handleKey = (event) => { if (event.key === 'Escape' && !editorLoading) onClose(); };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [onClose, editorLoading]);

  const previewCollection = async (collection) => {
    if (previews[collection.uuid]) {
      setPreviews((current) => { const next = { ...current }; delete next[collection.uuid]; return next; });
      return;
    }
    setPreviewingUuid(collection.uuid);
    try {
      const result = await fetchJSON(`/api/camera-collections/${collection.uuid}/preview`, {
        method: 'POST', headers: { 'Content-Type': 'application/json' }, body: '{}', timeout: 20000, retries: 1,
      });
      setPreviews((current) => ({ ...current, [collection.uuid]: result }));
    } catch (error) {
      showStatusMessage(t('collections.preview.error', { message: error.message }), 'error', 8000);
    } finally {
      setPreviewingUuid('');
    }
  };

  const editCollection = async (collection) => {
    setEditorLoading(true);
    try {
      let members = [];
      if (collection.type === 'static') {
        const response = await fetchJSON(`/api/camera-collections/${collection.uuid}/members`, { timeout: 20000, retries: 1 });
        members = response.camera_uuids || [];
      }
      setEditorSpec({ collection, members });
    } catch (error) {
      showStatusMessage(t('collections.loadError', { message: error.message }), 'error', 8000);
    } finally {
      setEditorLoading(false);
    }
  };

  const deleteCollection = async (collection) => {
    if (!window.confirm(t('collections.deleteConfirm', { name: collection.name }))) return;
    try {
      await fetchJSON(`/api/camera-collections/${collection.uuid}`, { method: 'DELETE', timeout: 15000, retries: 1 });
      showStatusMessage(t('collections.deleted'), 'success');
      await onRefresh();
    } catch (error) {
      showStatusMessage(t('collections.deleteError', { message: error.message }), 'error', 8000);
    }
  };

  return (
    <div className="fixed inset-0 z-40 flex items-center justify-center bg-black/55 p-3" onMouseDown={(event) => { if (event.target === event.currentTarget && !editorLoading) onClose(); }}>
      <div role="dialog" aria-modal="true" aria-labelledby="collections-title" className="flex max-h-[94vh] w-full max-w-5xl flex-col overflow-hidden rounded-xl border border-border bg-card text-card-foreground shadow-2xl">
        <header className="flex items-start justify-between border-b border-border p-4">
          <div><h2 id="collections-title" className="text-xl font-bold">{t('collections.title')}</h2><p className="mt-1 text-sm text-muted-foreground">{t('collections.subtitle')}</p></div>
          <button type="button" className="rounded p-2 text-2xl leading-none hover:bg-muted" onClick={onClose} aria-label={t('common.close')}>×</button>
        </header>
        <div className="overflow-y-auto p-4">
          {editorSpec
            ? <CollectionEditor spec={editorSpec} locations={locations} tags={tags} t={t} onCancel={() => setEditorSpec(null)} onSaved={async () => { setEditorSpec(null); await onRefresh(); }} />
            : <CollectionList collections={collections} loading={loading || editorLoading} isAdmin={isAdmin} previews={previews} previewingUuid={previewingUuid} onPreview={previewCollection} onEdit={editCollection} onDelete={deleteCollection} onCreate={() => setEditorSpec({ collection: null, members: [] })} t={t} />}
        </div>
      </div>
    </div>
  );
}
