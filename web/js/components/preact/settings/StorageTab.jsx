/**
 * StorageTab — Storage path, HLS path, retention, auto-delete, thumbnails,
 * DB path + backup schedule + post-backup script.
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

import { useCallback, useState } from 'preact/hooks';
import { fetchJSON, useQuery } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { CollectionSelectorBuilder } from '../fleet/CollectionSelectorBuilder.jsx';

const ALL_SELECTOR = { version: 1, expression: { op: 'all' } };

const EMPTY_TARGET = {
  name: '',
  root_path: '',
  enabled: true,
  mount_required: true,
  storage_class: 'hot',
  reserve_gb: '0',
  high_watermark_pct: '90',
  low_watermark_pct: '80',
  migration_mbps: '0',
  archival_window_start: '00:00',
  archival_window_end: '00:00',
};

function minutesToTime(value) {
  const minutes = Math.max(0, Math.min(1439, Number(value) || 0));
  return `${String(Math.floor(minutes / 60)).padStart(2, '0')}:${String(minutes % 60).padStart(2, '0')}`;
}

function timeToMinutes(value) {
  const match = /^(\d{2}):(\d{2})$/.exec(value || '');
  if (!match) return null;
  const hours = Number(match[1]);
  const minutes = Number(match[2]);
  return hours <= 23 && minutes <= 59 ? hours * 60 + minutes : null;
}

function formatBytes(value) {
  const bytes = Number(value) || 0;
  if (bytes <= 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
  const exponent = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / (1024 ** exponent)).toFixed(exponent >= 3 ? 1 : 0)} ${units[exponent]}`;
}

function statusClass(status) {
  if (status === 'healthy') return 'badge-success';
  if (status === 'degraded') return 'badge-warning';
  if (status === 'unavailable') return 'badge-danger';
  return 'bg-muted text-muted-foreground';
}

function pressureClass(pressure) {
  if (pressure === 'reserve') return 'badge-danger';
  if (pressure === 'high') return 'badge-warning';
  if (pressure === 'normal') return 'badge-success';
  return 'bg-muted text-muted-foreground';
}

function migrationStateClass(state) {
  if (state === 'completed') return 'badge-success';
  if (state === 'failed') return 'badge-danger';
  if (state === 'retry_wait' || state === 'cleanup_pending') return 'badge-warning';
  if (state === 'cancelled') return 'bg-muted text-muted-foreground';
  return 'badge-info';
}

function targetToEditor(target) {
  return {
    uuid: target.uuid,
    revision: target.revision,
    is_default: target.is_default,
    recording_count: target.recording_count,
    replica_count: target.replica_count,
    name: target.name,
    root_path: target.root_path,
    enabled: target.enabled,
    mount_required: target.mount_required,
    storage_class: target.storage_class,
    reserve_gb: String(Math.round((Number(target.reserve_bytes) || 0) / (1024 ** 3) * 100) / 100),
    high_watermark_pct: String(target.high_watermark_pct),
    low_watermark_pct: String(target.low_watermark_pct),
    migration_mbps: String(Math.round(((Number(target.migration_bandwidth_bps) || 0) * 8 / 1000000) * 100) / 100),
    archival_window_start: minutesToTime(target.archival_window_start_minute),
    archival_window_end: minutesToTime(target.archival_window_end_minute),
  };
}

function StorageTargetEditor({ value, onChange, onCancel, onSave, busy, t }) {
  const editing = !!value.uuid;
  const rootLocked = value.is_default || Number(value.recording_count) > 0 || Number(value.replica_count) > 0;
  const set = (key, next) => onChange({ ...value, [key]: next });
  return (
    <div class="rounded-lg border border-primary/40 bg-primary/5 p-4 mb-4">
      <h4 class="font-semibold mb-1">{t(editing ? 'settings.storageTargets.edit' : 'settings.storageTargets.add')}</h4>
      <p class="text-sm text-muted-foreground mb-4">{t('settings.storageTargets.editorHelp')}</p>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <label class="text-sm font-medium">
          <span class="block mb-1">{t('settings.storageTargets.name')}</span>
          <input class="w-full p-2 border border-input rounded bg-background" value={value.name} onInput={(event) => set('name', event.currentTarget.value)} disabled={busy} />
        </label>
        <label class="text-sm font-medium">
          <span class="block mb-1">{t('settings.storageTargets.class')}</span>
          <select class="w-full p-2 border border-input rounded bg-background" value={value.storage_class} onChange={(event) => set('storage_class', event.currentTarget.value)} disabled={busy}>
            <option value="hot">{t('settings.storageTargets.hot')}</option>
            <option value="warm">{t('settings.storageTargets.warm')}</option>
            <option value="cold">{t('settings.storageTargets.cold')}</option>
          </select>
        </label>
        <label class="text-sm font-medium md:col-span-2">
          <span class="block mb-1">{t('settings.storageTargets.root')}</span>
          <input class="w-full p-2 border border-input rounded bg-background font-mono text-sm disabled:opacity-60" value={value.root_path} onInput={(event) => set('root_path', event.currentTarget.value)} disabled={busy || rootLocked} placeholder="/mnt/nvr-hot-01" />
          {rootLocked && <span class="block mt-1 text-xs text-muted-foreground">{t('settings.storageTargets.rootLocked')}</span>}
        </label>
        <label class="text-sm font-medium">
          <span class="block mb-1">{t('settings.storageTargets.reserveGb')}</span>
          <input type="number" min="0" step="1" class="w-full p-2 border border-input rounded bg-background" value={value.reserve_gb} onInput={(event) => set('reserve_gb', event.currentTarget.value)} disabled={busy} />
        </label>
        <div class="grid grid-cols-2 gap-3">
          <label class="text-sm font-medium">
            <span class="block mb-1">{t('settings.storageTargets.lowWatermark')}</span>
            <input type="number" min="0" max="98" step="0.5" class="w-full p-2 border border-input rounded bg-background" value={value.low_watermark_pct} onInput={(event) => set('low_watermark_pct', event.currentTarget.value)} disabled={busy} />
          </label>
          <label class="text-sm font-medium">
            <span class="block mb-1">{t('settings.storageTargets.highWatermark')}</span>
            <input type="number" min="1" max="99" step="0.5" class="w-full p-2 border border-input rounded bg-background" value={value.high_watermark_pct} onInput={(event) => set('high_watermark_pct', event.currentTarget.value)} disabled={busy} />
          </label>
        </div>
        <label class="text-sm font-medium">
          <span class="block mb-1">{t('settings.storageTargets.migrationBandwidth')}</span>
          <input type="number" min="0" step="0.5" class="w-full p-2 border border-input rounded bg-background" value={value.migration_mbps} onInput={(event) => set('migration_mbps', event.currentTarget.value)} disabled={busy} />
          <span class="block mt-1 text-xs font-normal text-muted-foreground">{t('settings.storageTargets.migrationBandwidthHelp')}</span>
        </label>
        <div class="grid grid-cols-2 gap-3">
          <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storageTargets.archiveStart')}</span><input type="time" class="w-full p-2 border border-input rounded bg-background" value={value.archival_window_start} onInput={(event) => set('archival_window_start', event.currentTarget.value)} disabled={busy} /></label>
          <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storageTargets.archiveEnd')}</span><input type="time" class="w-full p-2 border border-input rounded bg-background" value={value.archival_window_end} onInput={(event) => set('archival_window_end', event.currentTarget.value)} disabled={busy} /></label>
          <span class="col-span-2 text-xs text-muted-foreground">{t('settings.storageTargets.archiveWindowHelp')}</span>
        </div>
        <label class="touch-target flex cursor-pointer items-center gap-2 text-sm font-medium">
          <input type="checkbox" checked={value.enabled} onChange={(event) => set('enabled', event.currentTarget.checked)} disabled={busy || value.is_default} />
          {t('settings.storageTargets.enabled')}
        </label>
        <label class="touch-target flex cursor-pointer items-start gap-2 text-sm font-medium md:col-span-2">
          <input type="checkbox" class="mt-1" checked={value.mount_required} onChange={(event) => set('mount_required', event.currentTarget.checked)} disabled={busy} />
          <span><span class="block">{t('settings.storageTargets.mountRequired')}</span><span class="block text-xs font-normal text-muted-foreground">{t('settings.storageTargets.mountRequiredHelp')}</span></span>
        </label>
      </div>
      <div class="flex justify-end gap-2 mt-4">
        <button type="button" class="btn-secondary" onClick={onCancel} disabled={busy}>{t('common.cancel')}</button>
        <button type="button" class="btn-primary" onClick={onSave} disabled={busy || !value.name.trim() || !value.root_path.trim()}>{busy ? t('common.saving') : t('common.saveChanges')}</button>
      </div>
    </div>
  );
}

function StorageTargetsPanel({ canModifySettings, t }) {
  const { data, isLoading, isError, error, refetch } = useQuery(
    ['storage-targets'],
    '/api/storage-targets',
    { cache: 'no-store', timeout: 15000, retries: 1 },
    { staleTime: 10000 },
  );
  const [editor, setEditor] = useState(null);
  const [busyKey, setBusyKey] = useState('');
  const targets = data?.targets || [];

  const saveTarget = async () => {
    const reserveGb = Number(editor.reserve_gb);
    const low = Number(editor.low_watermark_pct);
    const high = Number(editor.high_watermark_pct);
    const migrationMbps = Number(editor.migration_mbps);
    const windowStart = timeToMinutes(editor.archival_window_start);
    const windowEnd = timeToMinutes(editor.archival_window_end);
    if (!Number.isFinite(reserveGb) || reserveGb < 0 || !Number.isFinite(low) || !Number.isFinite(high) || low < 0 || high >= 100 || low >= high || !Number.isFinite(migrationMbps) || migrationMbps < 0 || (migrationMbps > 0 && migrationMbps * 1000000 / 8 < 65536) || windowStart === null || windowEnd === null) {
      showStatusMessage(t('settings.storageTargets.invalidThresholds'), 'error');
      return;
    }
    setBusyKey(editor.uuid || 'new');
    try {
      const payload = {
        name: editor.name.trim(),
        root_path: editor.root_path.trim(),
        enabled: editor.enabled,
        mount_required: editor.mount_required,
        storage_class: editor.storage_class,
        reserve_bytes: Math.round(reserveGb * (1024 ** 3)),
        low_watermark_pct: low,
        high_watermark_pct: high,
        migration_bandwidth_bps: Math.round(migrationMbps * 1000000 / 8),
        archival_window_start_minute: windowStart,
        archival_window_end_minute: windowEnd,
      };
      if (editor.uuid) payload.revision = editor.revision;
      await fetchJSON(editor.uuid ? `/api/storage-targets/${encodeURIComponent(editor.uuid)}` : '/api/storage-targets', {
        method: editor.uuid ? 'PUT' : 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
        timeout: 25000,
        retries: 0,
      });
      await refetch();
      setEditor(null);
      showStatusMessage(t(editor.uuid ? 'settings.storageTargets.updated' : 'settings.storageTargets.created'), 'success');
    } catch (requestError) {
      showStatusMessage(requestError.message, 'error', 7000);
    } finally {
      setBusyKey('');
    }
  };

  const probeTarget = async (target) => {
    setBusyKey(target.uuid);
    try {
      await fetchJSON(`/api/storage-targets/${encodeURIComponent(target.uuid)}/probe`, { method: 'POST', timeout: 30000, retries: 0 });
      await refetch();
      showStatusMessage(t('settings.storageTargets.probePassed'), 'success');
    } catch (requestError) {
      await refetch();
      showStatusMessage(requestError.message, 'error', 7000);
    } finally {
      setBusyKey('');
    }
  };

  const deleteTarget = async (target) => {
    if (!window.confirm(t('settings.storageTargets.deleteConfirm', { name: target.name }))) return;
    setBusyKey(target.uuid);
    try {
      await fetchJSON(`/api/storage-targets/${encodeURIComponent(target.uuid)}?revision=${encodeURIComponent(target.revision)}`, { method: 'DELETE', timeout: 15000, retries: 0 });
      await refetch();
      showStatusMessage(t('settings.storageTargets.deleted'), 'success');
    } catch (requestError) {
      showStatusMessage(requestError.message, 'error', 7000);
    } finally {
      setBusyKey('');
    }
  };

  return (
    <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4" data-setting-label={t('settings.storageTargets.title')}>
      <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3 pb-3 border-b border-border">
        <div>
          <h3 class="text-lg font-semibold">{t('settings.storageTargets.title')}</h3>
          <p class="text-sm text-muted-foreground mt-1">{t('settings.storageTargets.description')}</p>
        </div>
        {canModifySettings && <button type="button" class="btn-primary shrink-0" onClick={() => setEditor({ ...EMPTY_TARGET })} disabled={!!editor}>{t('settings.storageTargets.add')}</button>}
      </div>
      <div class="rounded-md border border-blue-300/60 bg-blue-50/70 dark:bg-blue-950/20 px-3 py-2 text-sm mt-4">
        {t('settings.storageTargets.foundationNotice')}
      </div>
      {editor && <div class="mt-4"><StorageTargetEditor value={editor} onChange={setEditor} onCancel={() => setEditor(null)} onSave={saveTarget} busy={busyKey === (editor.uuid || 'new')} t={t} /></div>}
      {isLoading && <p class="py-8 text-center text-sm text-muted-foreground">{t('common.loading')}</p>}
      {isError && <div class="py-6 text-center"><p class="text-sm text-[hsl(var(--danger))]">{error?.message || t('settings.storageTargets.loadError')}</p><button type="button" class="btn-secondary mt-3" onClick={refetch}>{t('common.retry')}</button></div>}
      {!isLoading && !isError && <div class="divide-y divide-border mt-4">
        {targets.map((target) => {
          const health = target.health || {};
          const cannotDelete = target.is_default || Number(target.recording_count) > 0 || Number(target.replica_count) > 0;
          return (
            <article key={target.uuid} class="py-4 first:pt-0 last:pb-0">
              <div class="flex flex-col xl:flex-row xl:items-start xl:justify-between gap-4">
                <div class="min-w-0 flex-1">
                  <div class="flex flex-wrap items-center gap-2">
                    <h4 class="font-semibold">{target.name}</h4>
                    {target.is_default && <span class="badge-info rounded-full px-2 py-0.5 text-xs font-semibold">{t('settings.storageTargets.default')}</span>}
                    <span class={`rounded-full px-2 py-0.5 text-xs font-semibold ${statusClass(health.status)}`}>{health.status || 'unknown'}</span>
                    <span class={`rounded-full px-2 py-0.5 text-xs font-semibold ${pressureClass(health.pressure)}`}>{t(`settings.storageTargets.pressure.${health.pressure || 'unavailable'}`)}</span>
                    <span class="rounded-full bg-muted px-2 py-0.5 text-xs">{target.storage_class}</span>
                  </div>
                  <p class="font-mono text-xs break-all mt-1 text-muted-foreground">{target.root_path}</p>
                  {target.mount_required && <p class="text-xs mt-1 text-muted-foreground">{t('settings.storageTargets.mountGuard')}: <span class="font-mono">{target.mount_guard_path || t('settings.storageTargets.mountPending')}</span></p>}
                  <dl class="grid grid-cols-2 lg:grid-cols-4 gap-3 mt-3 text-sm">
                    <div><dt class="text-xs uppercase tracking-wide text-muted-foreground">{t('settings.storageTargets.capacity')}</dt><dd>{formatBytes(health.capacity_bytes)}</dd></div>
                    <div><dt class="text-xs uppercase tracking-wide text-muted-foreground">{t('settings.storageTargets.available')}</dt><dd>{formatBytes(health.available_bytes)} ({Math.max(0, 100 - (Number(health.used_pct) || 0)).toFixed(1)}%)</dd></div>
                    <div><dt class="text-xs uppercase tracking-wide text-muted-foreground">{t('settings.storageTargets.recordings')}</dt><dd>{Number(target.recording_count || 0).toLocaleString()} · {formatBytes(target.recording_bytes)}</dd><dd class="text-xs text-muted-foreground">{t('settings.storageTargets.replicas', { count: Number(target.replica_count || 0).toLocaleString(), bytes: formatBytes(target.replica_bytes) })}</dd></div>
                    <div><dt class="text-xs uppercase tracking-wide text-muted-foreground">{t('settings.storageTargets.reserve')}</dt><dd>{formatBytes(target.reserve_bytes)}</dd></div>
                  </dl>
                  {health.duplicate_filesystem && <p class="text-xs text-amber-700 dark:text-amber-300 mt-2">{t('settings.storageTargets.duplicateFilesystem')}</p>}
                  {(health.pressure === 'high' || health.pressure === 'reserve') && <p class="text-xs text-amber-700 dark:text-amber-300 mt-2">{t('settings.storageTargets.cleanupActive', { target: formatBytes(health.cleanup_target_bytes) })}</p>}
                  {health.last_error && <p class="text-xs text-[hsl(var(--danger))] mt-2">{health.last_error}</p>}
                </div>
                {canModifySettings && <div class="flex flex-wrap gap-2 xl:justify-end">
                  <button type="button" class="btn-secondary" onClick={() => probeTarget(target)} disabled={!!busyKey}>{busyKey === target.uuid ? t('settings.storageTargets.probing') : t('settings.storageTargets.test')}</button>
                  <button type="button" class="btn-secondary" onClick={() => setEditor(targetToEditor(target))} disabled={!!editor || !!busyKey}>{t('common.edit')}</button>
                  <button type="button" class="rounded-md border border-[hsl(var(--danger)/0.55)] px-3 py-2 text-sm text-[hsl(var(--danger))] disabled:opacity-50" onClick={() => deleteTarget(target)} disabled={cannotDelete || !!busyKey} title={cannotDelete ? t('settings.storageTargets.deleteLocked') : ''}>{t('common.delete')}</button>
                </div>}
              </div>
            </article>
          );
        })}
      </div>}
    </div>
  );
}

function StoragePoolsPanel({ canModifySettings, t }) {
  const poolsQuery = useQuery(['storage-pools'], '/api/storage-pools', { cache: 'no-store', timeout: 15000, retries: 1 }, { staleTime: 5000 });
  const targetsQuery = useQuery(['storage-targets'], '/api/storage-targets', { cache: 'no-store', timeout: 15000, retries: 1 }, { staleTime: 10000 });
  const [editor, setEditor] = useState(null);
  const [busy, setBusy] = useState(false);
  const pools = poolsQuery.data?.pools || [];
  const targets = targetsQuery.data?.targets || [];
  const targetNames = new Map(targets.map((target) => [target.uuid, target.name]));
  const openNew = () => setEditor({ name: '', strategy: 'most_free', enabled: true, members: [] });
  const openEdit = (pool) => setEditor({ ...pool, members: (pool.members || []).map((member) => member.target_uuid) });
  const toggleMember = (uuid) => setEditor((current) => ({ ...current, members: current.members.includes(uuid) ? current.members.filter((item) => item !== uuid) : [...current.members, uuid] }));
  const save = async () => {
    setBusy(true);
    try {
      const payload = { name: editor.name.trim(), strategy: editor.strategy, enabled: editor.enabled, members: editor.members.map((target_uuid, position) => ({ target_uuid, position, weight: 1 })) };
      if (editor.uuid) payload.revision = editor.revision;
      await fetchJSON(editor.uuid ? `/api/storage-pools/${encodeURIComponent(editor.uuid)}` : '/api/storage-pools', { method: editor.uuid ? 'PUT' : 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload), timeout: 20000, retries: 0 });
      await poolsQuery.refetch();
      setEditor(null);
      showStatusMessage(t(editor.uuid ? 'settings.storagePools.updated' : 'settings.storagePools.created'), 'success');
    } catch (requestError) { showStatusMessage(requestError.message, 'error', 7000); } finally { setBusy(false); }
  };
  const remove = async (pool) => {
    if (!window.confirm(t('settings.storagePools.deleteConfirm', { name: pool.name }))) return;
    setBusy(true);
    try {
      await fetchJSON(`/api/storage-pools/${encodeURIComponent(pool.uuid)}?revision=${encodeURIComponent(pool.revision)}`, { method: 'DELETE', timeout: 15000, retries: 0 });
      await poolsQuery.refetch();
      showStatusMessage(t('settings.storagePools.deleted'), 'success');
    } catch (requestError) { showStatusMessage(requestError.message, 'error', 7000); } finally { setBusy(false); }
  };
  const loading = poolsQuery.isLoading || targetsQuery.isLoading;
  const error = poolsQuery.error || targetsQuery.error;
  return <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4" data-setting-label={t('settings.storagePools.title')}>
    <div class="flex items-start justify-between gap-3 pb-3 border-b border-border"><div><h3 class="text-lg font-semibold">{t('settings.storagePools.title')}</h3><p class="text-sm text-muted-foreground mt-1">{t('settings.storagePools.description')}</p></div>{canModifySettings && <button type="button" class="btn-primary" onClick={openNew} disabled={!!editor || targets.length === 0}>{t('settings.storagePools.add')}</button>}</div>
    {editor && <div class="rounded-lg border border-primary/40 bg-primary/5 p-4 mt-4 space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4"><label class="text-sm font-medium"><span class="block mb-1">{t('common.name')}</span><input class="w-full p-2 border border-input rounded bg-background" value={editor.name} onInput={(event) => setEditor({ ...editor, name: event.currentTarget.value })} disabled={busy} /></label><label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePools.strategy')}</span><select class="w-full p-2 border border-input rounded bg-background" value={editor.strategy} onChange={(event) => setEditor({ ...editor, strategy: event.currentTarget.value })} disabled={busy}><option value="most_free">{t('settings.storagePools.mostFree')}</option><option value="round_robin">{t('settings.storagePools.roundRobin')}</option><option value="priority">{t('settings.storagePools.priority')}</option></select></label></div>
      <fieldset><legend class="text-sm font-semibold mb-2">{t('settings.storagePools.members')}</legend><div class="grid grid-cols-1 md:grid-cols-2 gap-2">{targets.map((target) => <label key={target.uuid} class="touch-target flex items-center gap-2 text-sm"><input type="checkbox" checked={editor.members.includes(target.uuid)} onChange={() => toggleMember(target.uuid)} disabled={busy} />{target.name}</label>)}</div></fieldset>
      <label class="touch-target flex items-center gap-2 text-sm font-medium"><input type="checkbox" checked={editor.enabled} onChange={(event) => setEditor({ ...editor, enabled: event.currentTarget.checked })} disabled={busy} />{t('settings.storagePools.enabled')}</label>
      <div class="flex justify-end gap-2"><button type="button" class="btn-secondary" onClick={() => setEditor(null)} disabled={busy}>{t('common.cancel')}</button><button type="button" class="btn-primary" onClick={save} disabled={busy || !editor.name.trim() || editor.members.length === 0}>{busy ? t('common.saving') : t('common.saveChanges')}</button></div>
    </div>}
    {loading && <p class="py-8 text-center text-sm text-muted-foreground">{t('common.loading')}</p>}
    {error && <p class="py-6 text-sm text-[hsl(var(--danger))]">{error.message}</p>}
    {!loading && !error && <div class="divide-y divide-border">{pools.map((pool) => <article key={pool.uuid} class="py-4 first:pt-4 last:pb-0"><div class="flex justify-between gap-3"><div><div class="flex items-center gap-2"><h4 class="font-semibold">{pool.name}</h4><span class={`rounded-full px-2 py-0.5 text-xs ${pool.enabled ? 'badge-success' : 'bg-muted text-muted-foreground'}`}>{t(pool.enabled ? 'settings.storagePools.active' : 'settings.storagePools.inactive')}</span></div><p class="text-xs text-muted-foreground mt-1">{t(`settings.storagePools.strategyValue.${pool.strategy}`)} · {(pool.members || []).map((member) => targetNames.get(member.target_uuid) || member.target_uuid).join(' → ')}</p></div>{canModifySettings && <div class="flex gap-2"><button type="button" class="btn-secondary" onClick={() => openEdit(pool)} disabled={busy || !!editor}>{t('common.edit')}</button><button type="button" class="btn-danger" onClick={() => remove(pool)} disabled={busy}>{t('common.delete')}</button></div>}</div></article>)}</div>}
    {!loading && !error && pools.length === 0 && <p class="py-8 text-center text-sm text-muted-foreground">{t('settings.storagePools.empty')}</p>}
  </div>;
}

function StorageMigrationJobsPanel({ t }) {
  const { data, isLoading, isError, error, refetch } = useQuery(
    ['storage-migrations'],
    '/api/storage-migrations',
    { cache: 'no-store', timeout: 15000, retries: 1 },
    { staleTime: 2000, refetchInterval: 3000 },
  );
  const jobs = data?.jobs || [];
  const [busyKey, setBusyKey] = useState('');
  const actOnJob = async (job, action) => {
    setBusyKey(job.uuid);
    try {
      await fetchJSON(`/api/storage-migrations/${encodeURIComponent(job.uuid)}/${action}`, { method: 'POST', timeout: 15000, retries: 0 });
      await refetch();
      showStatusMessage(t(action === 'cancel' ? 'settings.storageMigrations.cancelled' : 'settings.storageMigrations.retried'), 'success');
    } catch (requestError) {
      showStatusMessage(requestError.message, 'error', 7000);
      await refetch();
    } finally { setBusyKey(''); }
  };
  return (
    <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4" data-setting-label={t('settings.storageMigrations.title')}>
      <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3 pb-3 border-b border-border">
        <div>
          <h3 class="text-lg font-semibold">{t('settings.storageMigrations.title')}</h3>
          <p class="text-sm text-muted-foreground mt-1">{t('settings.storageMigrations.description')}</p>
        </div>
        <button type="button" class="btn-secondary shrink-0" onClick={refetch}>{t('common.refresh')}</button>
      </div>
      {isLoading && <p class="py-8 text-center text-sm text-muted-foreground">{t('common.loading')}</p>}
      {isError && <div class="py-6 text-center"><p class="text-sm text-[hsl(var(--danger))]">{error?.message || t('settings.storageMigrations.loadError')}</p><button type="button" class="btn-secondary mt-3" onClick={refetch}>{t('common.retry')}</button></div>}
      {!isLoading && !isError && jobs.length === 0 && <p class="py-8 text-center text-sm text-muted-foreground">{t('settings.storageMigrations.empty')}</p>}
      {!isLoading && !isError && jobs.length > 0 && <div class="divide-y divide-border">
        {jobs.map((job) => {
          const progress = Math.max(0, Math.min(100, Math.round((Number(job.progress) || 0) * 100)));
          return <article key={job.uuid} class="py-4 first:pt-4 last:pb-0">
            <div class="flex flex-col lg:flex-row lg:items-start lg:justify-between gap-3">
              <div class="min-w-0 flex-1">
                <div class="flex flex-wrap items-center gap-2">
                  <span class="font-semibold">{t('settings.storageMigrations.recording', { id: job.recording_id })}</span>
                  <span class="rounded-full bg-muted px-2 py-0.5 text-xs">{t(`settings.storageMigrations.operation.${job.operation || 'move'}`)}</span>
                  <span class={`rounded-full px-2 py-0.5 text-xs font-semibold ${migrationStateClass(job.state)}`}>{t(`settings.storageMigrations.state.${job.state}`)}</span>
                  <span class="text-xs text-muted-foreground">{t('settings.storageMigrations.attempt', { current: job.attempt_count, max: job.max_attempts })}</span>
                </div>
                <p class="mt-1 truncate font-mono text-xs text-muted-foreground" title={`${job.source_target_uuid} → ${job.destination_target_uuid}`}>{job.source_target_uuid} → {job.destination_target_uuid}</p>
                {job.last_error && <p class="mt-1 text-xs text-[hsl(var(--danger))]">{job.last_error}</p>}
              </div>
              <div class="w-full lg:w-56">
                <div class="mb-1 flex justify-between text-xs text-muted-foreground"><span>{formatBytes(job.bytes_copied)} / {formatBytes(job.bytes_total)}</span><span>{progress}%</span></div>
                <div class="h-2 overflow-hidden rounded-full bg-muted"><div class="h-full bg-[hsl(var(--primary))] transition-[width]" style={{ width: `${progress}%` }} /></div>
                <div class="flex justify-end gap-2 mt-2">
                  {['queued', 'copying', 'verifying', 'committing', 'retry_wait'].includes(job.state) && <button type="button" class="btn-secondary" disabled={!!busyKey || job.cancel_requested} onClick={() => actOnJob(job, 'cancel')}>{job.cancel_requested ? t('settings.storageMigrations.cancelling') : t('common.cancel')}</button>}
                  {['failed', 'cancelled'].includes(job.state) && <button type="button" class="btn-secondary" disabled={!!busyKey} onClick={() => actOnJob(job, 'retry')}>{t('common.retry')}</button>}
                </div>
              </div>
            </div>
          </article>;
        })}
      </div>}
    </div>
  );
}

function StoragePolicyEditor({ value, targets, pools, onChange, onSelectorChange, onCancel, onPreview, onSave, busy, previewing, saving, locations, tags, t }) {
  const set = (key, next) => onChange({ ...value, [key]: next });
  const fallbackTargets = targets.filter((target) => target.uuid !== value.primary_target_uuid);
  const invalidFallback = value.fallback_mode === 'target' && !value.fallback_target_uuid;
  return (
    <div class="rounded-lg border border-primary/40 bg-primary/5 p-4 mt-4 space-y-4">
      <div><h4 class="font-semibold">{t(value.uuid ? 'settings.storagePolicies.edit' : 'settings.storagePolicies.add')}</h4><p class="text-sm text-muted-foreground mt-1">{t('settings.storagePolicies.editorHelp')}</p></div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <label class="text-sm font-medium"><span class="block mb-1">{t('common.name')}</span><input class="w-full p-2 border border-input rounded bg-background" maxLength="127" value={value.name} onInput={(event) => set('name', event.currentTarget.value)} disabled={busy} /></label>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.priority')}</span><input type="number" min="-1000000" max="1000000" step="1" class="w-full p-2 border border-input rounded bg-background" value={value.priority} onInput={(event) => set('priority', event.currentTarget.value)} disabled={busy} /></label>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.primary')}</span><select class="w-full p-2 border border-input rounded bg-background" value={value.primary_target_uuid} onChange={(event) => { const primary = event.currentTarget.value; onChange({ ...value, primary_target_uuid: primary, fallback_target_uuid: value.fallback_target_uuid === primary ? '' : value.fallback_target_uuid }); }} disabled={busy}>{targets.map((target) => <option key={target.uuid} value={target.uuid}>{target.name}{target.enabled ? '' : ` — ${t('settings.storagePolicies.disabledTarget')}`}</option>)}</select></label>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.primaryPool')}</span><select class="w-full p-2 border border-input rounded bg-background" value={value.primary_pool_uuid || ''} onChange={(event) => set('primary_pool_uuid', event.currentTarget.value)} disabled={busy}><option value="">{t('settings.storagePolicies.exactTarget')}</option>{pools.map((pool) => <option key={pool.uuid} value={pool.uuid}>{pool.name}</option>)}</select></label>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.fallback')}</span><select class="w-full p-2 border border-input rounded bg-background" value={value.fallback_mode} onChange={(event) => { const mode = event.currentTarget.value; onChange({ ...value, fallback_mode: mode, fallback_target_uuid: mode === 'target' ? value.fallback_target_uuid : '' }); }} disabled={busy}><option value="default">{t('settings.storagePolicies.fallbackDefault')}</option><option value="target">{t('settings.storagePolicies.fallbackTarget')}</option><option value="pause">{t('settings.storagePolicies.fallbackPause')}</option><option value="fail">{t('settings.storagePolicies.fallbackFail')}</option></select></label>
        {value.fallback_mode === 'target' && <label class="text-sm font-medium md:col-start-2"><span class="block mb-1">{t('settings.storagePolicies.namedFallback')}</span><select class="w-full p-2 border border-input rounded bg-background" value={value.fallback_target_uuid} onChange={(event) => set('fallback_target_uuid', event.currentTarget.value)} disabled={busy}><option value="">{t('settings.storagePolicies.chooseTarget')}</option>{fallbackTargets.map((target) => <option key={target.uuid} value={target.uuid}>{target.name}{target.enabled ? '' : ` — ${t('settings.storagePolicies.disabledTarget')}`}</option>)}</select></label>}
        <div class="md:col-span-2 grid grid-cols-1 sm:grid-cols-3 gap-3"><label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.minimumDays')}</span><input type="number" min="0" max="36500" class="w-full p-2 border border-input rounded bg-background" value={value.minimum_retention_days} onInput={(event) => set('minimum_retention_days', event.currentTarget.value)} disabled={busy} /></label><label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.desiredDays')}</span><input type="number" min="0" max="36500" class="w-full p-2 border border-input rounded bg-background" value={value.desired_retention_days} onInput={(event) => set('desired_retention_days', event.currentTarget.value)} disabled={busy} /></label><label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.maximumDays')}</span><input type="number" min="0" max="36500" class="w-full p-2 border border-input rounded bg-background" value={value.maximum_retention_days} onInput={(event) => set('maximum_retention_days', event.currentTarget.value)} disabled={busy} /></label></div>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.copyCount')}</span><input type="number" min="1" max="8" class="w-full p-2 border border-input rounded bg-background" value={value.required_copy_count} onInput={(event) => set('required_copy_count', event.currentTarget.value)} disabled={busy} /></label>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.replicationPool')}</span><select class="w-full p-2 border border-input rounded bg-background" value={value.replication_pool_uuid || ''} onChange={(event) => set('replication_pool_uuid', event.currentTarget.value)} disabled={busy || Number(value.required_copy_count) <= 1}><option value="">{t('settings.storagePolicies.choosePool')}</option>{pools.map((pool) => <option key={pool.uuid} value={pool.uuid}>{pool.name}</option>)}</select></label>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.migrateAfter')}</span><input type="number" min="0" max="36500" class="w-full p-2 border border-input rounded bg-background" value={value.migration_after_days} onInput={(event) => set('migration_after_days', event.currentTarget.value)} disabled={busy} /></label>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.migrationTarget')}</span><select class="w-full p-2 border border-input rounded bg-background" value={value.migration_target_uuid || ''} onChange={(event) => set('migration_target_uuid', event.currentTarget.value)} disabled={busy || Number(value.migration_after_days) <= 0}><option value="">{t('settings.storagePolicies.chooseTarget')}</option>{targets.filter((target) => target.uuid !== value.primary_target_uuid).map((target) => <option key={target.uuid} value={target.uuid}>{target.name}</option>)}</select></label>
        <label class="text-sm font-medium"><span class="block mb-1">{t('settings.storagePolicies.pressurePriority')}</span><input type="number" min="-1000000" max="1000000" class="w-full p-2 border border-input rounded bg-background" value={value.pressure_priority} onInput={(event) => set('pressure_priority', event.currentTarget.value)} disabled={busy} /></label>
        <label class="touch-target flex cursor-pointer items-center gap-2 text-sm font-medium md:col-span-2"><input type="checkbox" checked={value.enabled} onChange={(event) => set('enabled', event.currentTarget.checked)} disabled={busy} />{t('settings.storagePolicies.enabled')}</label>
      </div>
      <div><h5 class="text-sm font-semibold mb-2">{t('settings.storagePolicies.selector')}</h5><CollectionSelectorBuilder key={value.uuid || 'new-storage-policy'} initialSelector={value.selector || ALL_SELECTOR} locations={locations} tags={tags} allowCameraSelection idPrefix={`storage-policy-${value.uuid || 'new'}`} onChange={onSelectorChange} t={t} /></div>
      {value.selector_error && <p class="text-sm text-[hsl(var(--danger))]">{value.selector_error}</p>}
      {value.preview && <div class={`rounded-md border px-3 py-3 text-sm ${value.preview.conflict_policy_count > 0 ? 'border-amber-400/70 bg-amber-50/70 dark:bg-amber-950/20' : 'border-emerald-400/60 bg-emerald-50/70 dark:bg-emerald-950/20'}`}>
        <p class="font-semibold">{t('settings.storagePolicies.previewSummary', { matched: value.preview.matched_camera_count, effective: value.preview.effective_camera_count })}</p>
        {value.preview.shadowed_camera_count > 0 && <p class="mt-1">{t('settings.storagePolicies.previewShadowed', { count: value.preview.shadowed_camera_count })}</p>}
        {value.preview.conflicts?.length > 0 && <ul class="list-disc pl-5 mt-2 space-y-1">{value.preview.conflicts.map((conflict) => <li key={conflict.policy_uuid}>{t(conflict.draft_precedes ? 'settings.storagePolicies.previewWins' : 'settings.storagePolicies.previewLoses', { policy: conflict.policy_name, count: conflict.overlap_camera_count, priority: conflict.priority })}</li>)}</ul>}
        {value.preview.matched_camera_count === 0 && <p class="mt-1 text-amber-800 dark:text-amber-200">{t('settings.storagePolicies.previewNoMatches')}</p>}
      </div>}
      {!value.preview && <p class="text-xs text-muted-foreground">{t('settings.storagePolicies.previewRequired')}</p>}
      <div class="flex flex-wrap justify-end gap-2"><button type="button" class="btn-secondary" onClick={onCancel} disabled={busy}>{t('common.cancel')}</button><button type="button" class="btn-secondary" onClick={onPreview} disabled={busy || !value.name.trim() || !value.primary_target_uuid || invalidFallback || !!value.selector_error || !value.selector}>{previewing ? t('settings.storagePolicies.previewing') : t('settings.storagePolicies.preview')}</button><button type="button" class="btn-primary" onClick={onSave} disabled={busy || !value.preview || !value.name.trim() || !value.primary_target_uuid || invalidFallback || !!value.selector_error || !value.selector}>{saving ? t('common.saving') : t('common.saveChanges')}</button></div>
    </div>
  );
}

function StoragePoliciesPanel({ canModifySettings, t }) {
  const policiesQuery = useQuery(['storage-policies'], '/api/storage-policies', { cache: 'no-store', timeout: 15000, retries: 1 }, { staleTime: 5000 });
  const targetsQuery = useQuery(['storage-targets'], '/api/storage-targets', { cache: 'no-store', timeout: 15000, retries: 1 }, { staleTime: 10000 });
  const poolsQuery = useQuery(['storage-pools'], '/api/storage-pools', { cache: 'no-store', timeout: 15000, retries: 1 }, { staleTime: 5000 });
  const locationsQuery = useQuery(['fleet-locations'], '/api/locations', {}, { staleTime: 60000 });
  const tagsQuery = useQuery(['fleet-tags'], '/api/camera-tags', {}, { staleTime: 60000 });
  const [editor, setEditor] = useState(null);
  const [busyKey, setBusyKey] = useState('');
  const policies = policiesQuery.data?.policies || [];
  const targets = targetsQuery.data?.targets || [];
  const pools = poolsQuery.data?.pools || [];
  const targetNames = new Map(targets.map((target) => [target.uuid, target.name]));
  const poolNames = new Map(pools.map((pool) => [pool.uuid, pool.name]));
  const onSelectorChange = useCallback((selector, error) => {
    setEditor((current) => current ? { ...current, selector, selector_error: error, preview: null, preview_key: '', preview_request_key: '' } : current);
  }, []);
  const updateEditor = useCallback((next) => setEditor({ ...next, preview: null, preview_key: '', preview_request_key: '' }), []);

  const openNew = () => setEditor({ name: '', enabled: true, priority: '100', selector: ALL_SELECTOR, selector_error: '', primary_target_uuid: targets.find((target) => target.is_default)?.uuid || targets[0]?.uuid || '', primary_pool_uuid: '', fallback_mode: 'default', fallback_target_uuid: '', minimum_retention_days: '0', desired_retention_days: '0', maximum_retention_days: '0', required_copy_count: '1', replication_pool_uuid: '', migration_after_days: '0', migration_target_uuid: '', pressure_priority: '100', preview: null });
  const openEdit = (policy) => setEditor({ ...policy, priority: String(policy.priority), primary_pool_uuid: policy.primary_pool_uuid || '', fallback_target_uuid: policy.fallback_target_uuid || '', minimum_retention_days: String(policy.minimum_retention_days || 0), desired_retention_days: String(policy.desired_retention_days || 0), maximum_retention_days: String(policy.maximum_retention_days || 0), required_copy_count: String(policy.required_copy_count || 1), replication_pool_uuid: policy.replication_pool_uuid || '', migration_after_days: String(policy.migration_after_days || 0), migration_target_uuid: policy.migration_target_uuid || '', pressure_priority: String(policy.pressure_priority ?? 100), selector_error: '', preview: null });
  const policyPayload = () => {
    const priority = Number(editor.priority);
    if (!Number.isInteger(priority) || priority < -1000000 || priority > 1000000) {
      showStatusMessage(t('settings.storagePolicies.invalidPriority'), 'error');
      return null;
    }
    const minimum = Number(editor.minimum_retention_days); const desired = Number(editor.desired_retention_days); const maximum = Number(editor.maximum_retention_days); const copies = Number(editor.required_copy_count); const migrationAfter = Number(editor.migration_after_days); const pressurePriority = Number(editor.pressure_priority);
    if (![minimum, desired, maximum, copies, migrationAfter, pressurePriority].every(Number.isInteger) || minimum < 0 || (desired > 0 && desired < minimum) || (maximum > 0 && ((desired > 0 && desired > maximum) || minimum > maximum)) || copies < 1 || copies > 8 || (copies > 1 && !editor.replication_pool_uuid) || migrationAfter < 0 || (migrationAfter > 0 && !editor.migration_target_uuid)) {
      showStatusMessage(t('settings.storagePolicies.invalidLifecycle'), 'error');
      return null;
    }
    const payload = { name: editor.name.trim(), enabled: editor.enabled, priority, selector: editor.selector, primary_target_uuid: editor.primary_target_uuid, primary_pool_uuid: editor.primary_pool_uuid || null, fallback_mode: editor.fallback_mode, fallback_target_uuid: editor.fallback_mode === 'target' ? editor.fallback_target_uuid : null, minimum_retention_days: minimum, desired_retention_days: desired, maximum_retention_days: maximum, required_copy_count: copies, replication_pool_uuid: copies > 1 ? editor.replication_pool_uuid : null, migration_after_days: migrationAfter, migration_target_uuid: migrationAfter > 0 ? editor.migration_target_uuid : null, pressure_priority: pressurePriority };
    if (editor.uuid) { payload.uuid = editor.uuid; payload.revision = editor.revision; }
    return payload;
  };
  const previewPolicy = async () => {
    const payload = policyPayload();
    if (!payload) return;
    const requestKey = JSON.stringify(payload);
    setEditor((current) => current ? { ...current, preview: null, preview_key: '', preview_request_key: requestKey } : current);
    setBusyKey('preview');
    try {
      const preview = await fetchJSON('/api/storage-policies/preview', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload), timeout: 20000, retries: 0 });
      setEditor((current) => current?.preview_request_key === requestKey ? { ...current, preview, preview_key: requestKey, preview_request_key: '' } : current);
    } catch (requestError) {
      setEditor((current) => current?.preview_request_key === requestKey ? { ...current, preview_request_key: '' } : current);
      showStatusMessage(requestError.message, 'error', 7000);
    } finally { setBusyKey(''); }
  };
  const savePolicy = async () => {
    const payload = policyPayload();
    if (!payload || !editor.preview || editor.preview_key !== JSON.stringify(payload)) return;
    setBusyKey(editor.uuid || 'new');
    try {
      delete payload.uuid;
      await fetchJSON(editor.uuid ? `/api/storage-policies/${encodeURIComponent(editor.uuid)}` : '/api/storage-policies', { method: editor.uuid ? 'PUT' : 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload), timeout: 20000, retries: 0 });
      await policiesQuery.refetch();
      setEditor(null);
      showStatusMessage(t(editor.uuid ? 'settings.storagePolicies.updated' : 'settings.storagePolicies.created'), 'success');
    } catch (requestError) {
      showStatusMessage(requestError.message, 'error', 7000);
      if (requestError.status === 409) await policiesQuery.refetch();
    } finally { setBusyKey(''); }
  };
  const deletePolicy = async (policy) => {
    if (!window.confirm(t('settings.storagePolicies.deleteConfirm', { name: policy.name }))) return;
    setBusyKey(policy.uuid);
    try {
      await fetchJSON(`/api/storage-policies/${encodeURIComponent(policy.uuid)}?revision=${encodeURIComponent(policy.revision)}`, { method: 'DELETE', timeout: 15000, retries: 0 });
      await policiesQuery.refetch();
      showStatusMessage(t('settings.storagePolicies.deleted'), 'success');
    } catch (requestError) { showStatusMessage(requestError.message, 'error', 7000); } finally { setBusyKey(''); }
  };
  const loading = policiesQuery.isLoading || targetsQuery.isLoading || poolsQuery.isLoading || locationsQuery.isLoading || tagsQuery.isLoading;
  const error = policiesQuery.error || targetsQuery.error || poolsQuery.error || locationsQuery.error || tagsQuery.error;

  return (
    <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4" data-setting-label={t('settings.storagePolicies.title')}>
      <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3 pb-3 border-b border-border"><div><h3 class="text-lg font-semibold">{t('settings.storagePolicies.title')}</h3><p class="text-sm text-muted-foreground mt-1">{t('settings.storagePolicies.description')}</p></div>{canModifySettings && <button type="button" class="btn-primary shrink-0" onClick={openNew} disabled={!!editor || targets.length === 0}>{t('settings.storagePolicies.add')}</button>}</div>
      {editor && <StoragePolicyEditor value={editor} targets={targets} pools={pools} locations={locationsQuery.data?.locations || []} tags={tagsQuery.data?.tags || []} onChange={updateEditor} onSelectorChange={onSelectorChange} onCancel={() => setEditor(null)} onPreview={previewPolicy} onSave={savePolicy} busy={!!busyKey} previewing={busyKey === 'preview'} saving={busyKey === (editor.uuid || 'new')} t={t} />}
      {loading && <p class="py-8 text-center text-sm text-muted-foreground">{t('common.loading')}</p>}
      {error && <div class="py-6 text-center"><p class="text-sm text-[hsl(var(--danger))]">{error.message}</p><button type="button" class="btn-secondary mt-3" onClick={() => Promise.all([policiesQuery.refetch(), targetsQuery.refetch(), poolsQuery.refetch(), locationsQuery.refetch(), tagsQuery.refetch()])}>{t('common.retry')}</button></div>}
      {!loading && !error && <div class="divide-y divide-border mt-4">{policies.map((policy) => <article key={policy.uuid} class="py-4 first:pt-0 last:pb-0"><div class="flex flex-col lg:flex-row lg:items-start lg:justify-between gap-3"><div><div class="flex flex-wrap items-center gap-2"><h4 class="font-semibold">{policy.name}</h4><span class="rounded-full bg-muted px-2 py-0.5 text-xs">{t('settings.storagePolicies.priorityValue', { priority: policy.priority })}</span><span class={`rounded-full px-2 py-0.5 text-xs font-semibold ${policy.enabled ? 'badge-success' : 'bg-muted text-muted-foreground'}`}>{t(policy.enabled ? 'settings.storagePolicies.active' : 'settings.storagePolicies.inactive')}</span></div><p class="text-sm mt-2">{t('settings.storagePolicies.routesTo', { target: policy.primary_pool_uuid ? poolNames.get(policy.primary_pool_uuid) || policy.primary_pool_uuid : targetNames.get(policy.primary_target_uuid) || policy.primary_target_uuid })}</p><p class="text-xs text-muted-foreground mt-1">{t('settings.storagePolicies.lifecycleSummary', { minimum: policy.minimum_retention_days, desired: policy.desired_retention_days, maximum: policy.maximum_retention_days, copies: policy.required_copy_count })}</p><p class="text-xs text-muted-foreground mt-1">{policy.fallback_mode === 'target' ? t('settings.storagePolicies.fallsBackTo', { target: targetNames.get(policy.fallback_target_uuid) || policy.fallback_target_uuid }) : t(`settings.storagePolicies.mode.${policy.fallback_mode}`)}</p></div>{canModifySettings && <div class="flex gap-2"><button type="button" class="btn-secondary" onClick={() => openEdit(policy)} disabled={!!editor || !!busyKey}>{t('common.edit')}</button><button type="button" class="btn-danger" onClick={() => deletePolicy(policy)} disabled={!!busyKey}>{t('common.delete')}</button></div>}</div></article>)}</div>}
      {!loading && !error && policies.length === 0 && <p class="py-8 text-center text-sm text-muted-foreground">{t('settings.storagePolicies.empty')}</p>}
    </div>
  );
}

function StorageCompliancePanel({ t }) {
  const { data, isLoading, isError, error, refetch } = useQuery(
    ['storage-compliance'], '/api/storage-compliance',
    { cache: 'no-store', timeout: 20000, retries: 1 },
    { staleTime: 30000, refetchInterval: 60000 },
  );
  const formatDays = (value) => value == null ? t('settings.storageCompliance.unknown') : t('settings.storageCompliance.days', { days: Number(value).toFixed(1) });
  return <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4" data-setting-label={t('settings.storageCompliance.title')}>
    <div class="flex items-start justify-between gap-3 pb-3 border-b border-border"><div><h3 class="text-lg font-semibold">{t('settings.storageCompliance.title')}</h3><p class="text-sm text-muted-foreground mt-1">{t('settings.storageCompliance.description')}</p></div><button type="button" class="btn-secondary" onClick={refetch}>{t('common.refresh')}</button></div>
    {isLoading && <p class="py-8 text-center text-sm text-muted-foreground">{t('common.loading')}</p>}
    {isError && <p class="py-6 text-sm text-[hsl(var(--danger))]">{error?.message || t('settings.storageCompliance.loadError')}</p>}
    {!isLoading && !isError && <>
      <p class="text-xs text-muted-foreground mt-3">{data?.forecast_note}</p>
      <h4 class="font-semibold mt-4">{t('settings.storageCompliance.targetForecasts')}</h4>
      <div class="overflow-x-auto mt-2"><table class="w-full text-sm"><thead><tr class="text-left text-xs uppercase text-muted-foreground"><th class="py-2 pr-3">{t('settings.storageCompliance.target')}</th><th class="py-2 pr-3">{t('settings.storageCompliance.dailyRate')}</th><th class="py-2 pr-3">{t('settings.storageCompliance.toHigh')}</th><th class="py-2 pr-3">{t('settings.storageCompliance.expected')}</th><th class="py-2">{t('settings.storageCompliance.confidence')}</th></tr></thead><tbody>{(data?.targets || []).map((target) => <tr key={target.target_uuid} class="border-t border-border"><td class="py-2 pr-3 font-medium">{target.target_name}</td><td class="py-2 pr-3">{formatBytes(target.observed_daily_bytes)}/{t('settings.storageCompliance.day')}</td><td class="py-2 pr-3">{formatDays(target.days_to_high_watermark)}</td><td class="py-2 pr-3">{formatDays(target.expected_retention_days)}</td><td class="py-2">{target.confidence}</td></tr>)}</tbody></table></div>
      <h4 class="font-semibold mt-5">{t('settings.storageCompliance.policyCompliance')}</h4>
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-3 mt-2">{(data?.policies || []).map((policy) => <article key={policy.policy_uuid} class={`rounded-md border p-3 ${policy.active_violation_count ? 'border-amber-400/70 bg-amber-50/50 dark:bg-amber-950/20' : 'border-border'}`}><div class="flex justify-between gap-3"><h5 class="font-semibold">{policy.policy_name}</h5><span class={`rounded-full px-2 py-0.5 text-xs ${policy.active_violation_count ? 'badge-warning' : 'badge-success'}`}>{policy.active_violation_count ? t('settings.storageCompliance.violationCount', { count: policy.active_violation_count }) : t('settings.storageCompliance.compliant')}</span></div><p class="text-xs text-muted-foreground mt-2">{t('settings.storageCompliance.policyForecast', { achieved: formatDays(policy.achieved_retention_days), expected: formatDays(policy.expected_retention_days), desired: policy.desired_retention_days })}</p>{policy.copy_deficit_recordings > 0 && <p class="text-xs text-[hsl(var(--danger))] mt-1">{t('settings.storageCompliance.copyDeficits', { count: policy.copy_deficit_recordings })}</p>}</article>)}</div>
      {(data?.active_violations || []).length > 0 && <><h4 class="font-semibold mt-5">{t('settings.storageCompliance.activeViolations')}</h4><div class="divide-y divide-border mt-2">{data.active_violations.map((violation) => <div key={violation.uuid} class="py-2 text-sm"><span class="font-medium">{violation.policy_name}</span> · <span class="text-amber-700 dark:text-amber-300">{t(`settings.storageCompliance.type.${violation.type}`)}</span><p class="text-xs text-muted-foreground mt-0.5">{violation.details}</p></div>)}</div></>}
    </>}
  </div>;
}

export function StorageTab({ settings, handleInputChange, canModifySettings, t }) {
  return (
    <div class="space-y-6">
      {canModifySettings && <StorageTargetsPanel canModifySettings={canModifySettings} t={t} />}
      {canModifySettings && <StoragePoolsPanel canModifySettings={canModifySettings} t={t} />}
      {canModifySettings && <StorageMigrationJobsPanel t={t} />}
      {canModifySettings && <StoragePoliciesPanel canModifySettings={canModifySettings} t={t} />}
      {canModifySettings && <StorageCompliancePanel t={t} />}
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.storage')}</h3>
        <div data-setting-label={t('settings.storagePath')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-storage-path" class="font-medium">{t('settings.storagePath')}</label>
          <input
            type="text"
            id="setting-storage-path"
            name="storagePath"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.storagePath}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          />
        </div>
        <div data-setting-label={t('settings.hlsStoragePath')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-storage-path-hls" class="font-medium">{t('settings.hlsStoragePath')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-storage-path-hls"
              name="storagePathHls"
              class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.storagePathHls}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.hlsStoragePathHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.recordingDirectoryFormat')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mp4-directory-format" class="font-medium">{t('settings.recordingDirectoryFormat')}</label>
          <div class="col-span-2">
            <select
              id="setting-mp4-directory-format"
              name="mp4DirectoryFormat"
              class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mp4DirectoryFormat}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            >
              <option value="flat">{t('settings.recordingDirectoryFlat')}</option>
              <option value="year_month">{t('settings.recordingDirectoryMonth')}</option>
              <option value="year_month_day">{t('settings.recordingDirectoryDay')}</option>
            </select>
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.recordingDirectoryFormatHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.maxStorageGb')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-max-storage" class="font-medium">{t('settings.maxStorageGb')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-max-storage"
              name="maxStorage"
              min="0"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.maxStorage}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.zeroUnlimited')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.retentionDays')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-retention" class="font-medium">{t('settings.retentionDays')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-retention"
              name="retention"
              min="0"
              max="365"
              class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.retention}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.zeroUnlimited')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.autoDeleteOldest')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-auto-delete" class="font-medium">{t('settings.autoDeleteOldest')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-auto-delete"
              name="autoDelete"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.autoDelete}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
          </div>
        </div>
        <div data-setting-label={t('settings.minFreePct')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-min-free-pct" class="font-medium">{t('settings.minFreePct')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-min-free-pct"
              name="minFreePct"
              min="0"
              max="90"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.minFreePct}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.minFreePctHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.pressureThresholds')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
          <label class="font-medium">{t('settings.pressureThresholds')}</label>
          <div class="col-span-2">
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
              <label class="flex flex-col text-sm gap-1">
                <span>{t('settings.pressureWarningPct')}</span>
                <input
                  type="number" name="pressureWarningPct" min="0" max="100" step="0.5"
                  class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.pressureWarningPct}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
              </label>
              <label class="flex flex-col text-sm gap-1">
                <span>{t('settings.pressureCriticalPct')}</span>
                <input
                  type="number" name="pressureCriticalPct" min="0" max="100" step="0.5"
                  class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.pressureCriticalPct}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
              </label>
              <label class="flex flex-col text-sm gap-1">
                <span>{t('settings.pressureEmergencyPct')}</span>
                <input
                  type="number" name="pressureEmergencyPct" min="0" max="100" step="0.5"
                  class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.pressureEmergencyPct}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
              </label>
            </div>
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.pressureThresholdsHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.enableGridViewThumbnails')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-generate-thumbnails" class="font-medium">{t('settings.enableGridViewThumbnails')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-generate-thumbnails"
              name="generateThumbnails"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.generateThumbnails}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableGridViewThumbnailsHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.thumbnailsPerRecording')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-thumbnails-per-recording" class="font-medium">{t('settings.thumbnailsPerRecording')}</label>
          <div class="col-span-2">
            <select
              id="setting-thumbnails-per-recording"
              name="thumbnailsPerRecording"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.thumbnailsPerRecording}
              onChange={handleInputChange}
              disabled={!canModifySettings || !settings.generateThumbnails}
            >
              <option value={1}>{t('settings.thumbnailsPerRecordingOne')}</option>
              <option value={3}>{t('settings.thumbnailsPerRecordingThree')}</option>
            </select>
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.thumbnailsPerRecordingHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.databasePath')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-db-path" class="font-medium">{t('settings.databasePath')}</label>
          <input
            type="text"
            id="setting-db-path"
            name="dbPath"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.dbPath}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          />
        </div>
        <div data-setting-label={t('settings.databaseBackupIntervalMinutes')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-db-backup-interval" class="font-medium">{t('settings.databaseBackupIntervalMinutes')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-db-backup-interval"
              name="dbBackupIntervalMinutes"
              min="0"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.dbBackupIntervalMinutes}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.databaseBackupIntervalHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.databaseBackupRetentionCopies')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-db-backup-retention" class="font-medium">{t('settings.databaseBackupRetentionCopies')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-db-backup-retention"
              name="dbBackupRetentionCount"
              min="0"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.dbBackupRetentionCount}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.databaseBackupRetentionHelpBefore')} <code>.bak</code> {t('settings.databaseBackupRetentionHelpAfter')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.postBackupScript')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
          <label for="setting-db-post-backup-script" class="font-medium">{t('settings.postBackupScript')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-db-post-backup-script"
              name="dbPostBackupScript"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-2xl disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.dbPostBackupScript}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder="/usr/local/bin/lightnvr-post-backup"
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.postBackupScriptHelp')}</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default StorageTab;
