import { useCallback, useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { enhancedFetch } from '../../../fetch-utils.js';
import { useI18n } from '../../../i18n.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import {
  EMPTY_AUDIT_FILTERS,
  auditDateRangeIsValid,
  auditOutcomeTone,
  auditPageBounds,
  buildAuditQuery,
} from './auditHistory.js';

const fieldClasses = 'w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';
const EMPTY_PAGE = { page: 1, page_size: 50, count: 0, total: 0, events: [] };

function formatTimestamp(timestamp) {
  if (!timestamp) return '—';
  const date = new Date(Number(timestamp) * 1000);
  return Number.isNaN(date.getTime()) ? '—' : date.toLocaleString();
}

function outcomeClass(outcome) {
  const tone = auditOutcomeTone(outcome);
  return tone === 'success'
    ? 'badge-success'
    : tone === 'danger'
      ? 'badge-danger'
      : tone === 'warning'
        ? 'badge-warning'
        : 'badge-info';
}

function EventCard({ event, expanded, onToggle, onFilterAction, t }) {
  const target = [event.target_type, event.target_uuid].filter(Boolean).join(' · ') || t('audit.noTarget');
  const actor = event.principal_username || t('audit.unauthenticated');
  return (
    <article className="rounded-lg border border-border bg-background p-4">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <div className="flex flex-wrap items-center gap-2">
            <button type="button" className="font-mono text-left text-sm font-semibold break-all underline decoration-dotted underline-offset-2" onClick={onFilterAction} title={t('audit.filterToAction')}>{event.action}</button>
            <span className={`rounded-full px-2 py-0.5 text-xs font-semibold ${outcomeClass(event.outcome)}`}>{event.outcome}</span>
          </div>
          <time className="mt-1 block text-xs text-muted-foreground" dateTime={new Date(Number(event.occurred_at) * 1000).toISOString()}>{formatTimestamp(event.occurred_at)}</time>
        </div>
        <button type="button" className="btn-secondary" aria-expanded={expanded} onClick={onToggle}>
          {expanded ? t('audit.hideDetails') : t('audit.showDetails')}
        </button>
      </div>

      <dl className="mt-3 grid gap-3 text-sm sm:grid-cols-2 xl:grid-cols-4">
        <div><dt className="text-xs font-medium uppercase tracking-wide text-muted-foreground">{t('audit.actor')}</dt><dd className="mt-0.5 break-all">{actor}</dd><dd className="text-xs text-muted-foreground">{event.auth_method || '—'}</dd></div>
        <div><dt className="text-xs font-medium uppercase tracking-wide text-muted-foreground">{t('audit.source')}</dt><dd className="mt-0.5 font-mono text-xs break-all">{event.remote_address || '—'}</dd></div>
        <div><dt className="text-xs font-medium uppercase tracking-wide text-muted-foreground">{t('audit.target')}</dt><dd className="mt-0.5 font-mono text-xs break-all">{target}</dd></div>
        <div><dt className="text-xs font-medium uppercase tracking-wide text-muted-foreground">{t('audit.requestId')}</dt><dd className="mt-0.5 font-mono text-xs break-all">{event.request_id}</dd></div>
      </dl>

      {expanded && (
        <div className="mt-4 border-t border-border pt-3">
          {event.api_token_uuid && <p className="mb-2 text-xs"><span className="font-medium">{t('audit.apiToken')}:</span> <span className="font-mono break-all">{event.api_token_uuid}</span></p>}
          <pre className="max-h-64 overflow-auto whitespace-pre-wrap break-words rounded-md bg-muted p-3 text-xs text-foreground">{JSON.stringify(event.details || {}, null, 2)}</pre>
        </div>
      )}
    </article>
  );
}

export function AuditHistoryModal({ users = [], onClose, getAuthHeaders }) {
  const { t } = useI18n();
  const [draftFilters, setDraftFilters] = useState({ ...EMPTY_AUDIT_FILTERS });
  const [filters, setFilters] = useState({ ...EMPTY_AUDIT_FILTERS });
  const [pageNumber, setPageNumber] = useState(1);
  const [pageSize, setPageSize] = useState(50);
  const [page, setPage] = useState(EMPTY_PAGE);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [expandedUuid, setExpandedUuid] = useState('');
  const [retentionDays, setRetentionDays] = useState(365);
  const [retentionDraft, setRetentionDraft] = useState('365');
  const [savingRetention, setSavingRetention] = useState(false);
  const [exporting, setExporting] = useState(false);
  const requestSequence = useRef(0);

  const queryString = useMemo(
    () => buildAuditQuery(filters, pageNumber, pageSize),
    [filters, pageNumber, pageSize]
  );

  const loadEvents = useCallback(async () => {
    const sequence = ++requestSequence.current;
    setLoading(true);
    setError('');
    try {
      const response = await fetchJSON(`/api/audit/events?${queryString}`, {
        headers: getAuthHeaders(),
        cache: 'no-store',
        timeout: 20000,
        retries: 0,
      });
      if (sequence === requestSequence.current) {
        setPage({ ...EMPTY_PAGE, ...response, events: response?.events || [] });
      }
    } catch (requestError) {
      if (sequence === requestSequence.current) setError(requestError.message);
    } finally {
      if (sequence === requestSequence.current) setLoading(false);
    }
  }, [getAuthHeaders, queryString]);

  useEffect(() => { loadEvents(); }, [loadEvents]);
  useEffect(() => {
    fetchJSON('/api/audit/settings', {
      headers: getAuthHeaders(), cache: 'no-store', timeout: 15000, retries: 0,
    }).then((settings) => {
      const days = Number(settings?.retention_days) || 365;
      setRetentionDays(days);
      setRetentionDraft(String(days));
    }).catch((requestError) => setError(requestError.message));
  }, [getAuthHeaders]);
  useEffect(() => {
    const handleKey = (event) => { if (event.key === 'Escape' && !savingRetention && !exporting) onClose(); };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [onClose, savingRetention, exporting]);

  const applyFilters = (event) => {
    event.preventDefault();
    if (!auditDateRangeIsValid(draftFilters)) {
      showStatusMessage(t('audit.invalidDateRange'), 'error');
      return;
    }
    setFilters({ ...draftFilters });
    setPageNumber(1);
    setExpandedUuid('');
  };
  const clearFilters = () => {
    const cleared = { ...EMPTY_AUDIT_FILTERS };
    setDraftFilters(cleared);
    setFilters(cleared);
    setPageNumber(1);
    setExpandedUuid('');
  };
  const updateDraft = (name, value) => setDraftFilters((current) => ({ ...current, [name]: value }));
  const filterToAction = (action) => {
    const next = { ...filters, action };
    setDraftFilters(next);
    setFilters(next);
    setPageNumber(1);
    setExpandedUuid('');
  };

  const saveRetention = async () => {
    const days = Number(retentionDraft);
    if (!Number.isInteger(days) || days < 1 || days > 3650) {
      showStatusMessage(t('audit.retentionInvalid'), 'error');
      return;
    }
    setSavingRetention(true);
    try {
      const response = await fetchJSON('/api/audit/settings', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify({ retention_days: days }),
        timeout: 20000,
        retries: 0,
      });
      setRetentionDays(response.retention_days);
      setRetentionDraft(String(response.retention_days));
      showStatusMessage(t('audit.retentionSaved', { count: response.pruned_events || 0 }), 'success');
      await loadEvents();
    } catch (requestError) {
      showStatusMessage(t('audit.retentionError', { message: requestError.message }), 'error', 8000);
    } finally {
      setSavingRetention(false);
    }
  };

  const exportPage = async () => {
    setExporting(true);
    try {
      const response = await enhancedFetch(`/api/audit/events/export?${queryString}`, {
        headers: getAuthHeaders(), cache: 'no-store', timeout: 30000, retries: 0,
      });
      const blob = await response.blob();
      const url = URL.createObjectURL(blob);
      const anchor = document.createElement('a');
      anchor.href = url;
      anchor.download = `lightnvr-audit-${new Date().toISOString().slice(0, 10)}.csv`;
      document.body.appendChild(anchor);
      anchor.click();
      anchor.remove();
      URL.revokeObjectURL(url);
      showStatusMessage(t('audit.exported'), 'success');
    } catch (requestError) {
      showStatusMessage(t('audit.exportError', { message: requestError.message }), 'error', 8000);
    } finally {
      setExporting(false);
    }
  };

  const bounds = auditPageBounds(page);
  const hasNext = bounds.end < bounds.total;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-2 sm:p-3" onClick={() => !savingRetention && !exporting && onClose()}>
      <section role="dialog" aria-modal="true" aria-labelledby="audit-history-title" className="flex max-h-[96vh] w-full max-w-7xl flex-col overflow-hidden rounded-lg bg-card text-card-foreground shadow-xl" onClick={(event) => event.stopPropagation()}>
        <header className="flex items-start justify-between gap-4 border-b border-border p-4 sm:p-5">
          <div><h2 id="audit-history-title" className="text-xl font-bold">{t('audit.title')}</h2><p className="mt-1 text-sm text-muted-foreground">{t('audit.description')}</p></div>
          <button type="button" className="btn-secondary" onClick={onClose} disabled={savingRetention || exporting}>{t('common.close')}</button>
        </header>

        <div className="min-h-0 flex-1 overflow-y-auto p-4 sm:p-5">
          <section className="mb-4 rounded-lg border border-border bg-muted/40 p-4" aria-labelledby="audit-retention-title">
            <div className="flex flex-wrap items-end justify-between gap-3">
              <div><h3 id="audit-retention-title" className="font-semibold">{t('audit.retention')}</h3><p className="mt-1 text-xs text-muted-foreground">{t('audit.retentionDescription', { days: retentionDays })}</p></div>
              <div className="flex items-end gap-2">
                <label className="text-xs font-medium">{t('audit.retentionDays')}<input className={`${fieldClasses} mt-1 w-28`} type="number" min="1" max="3650" value={retentionDraft} onInput={(event) => setRetentionDraft(event.currentTarget.value)} /></label>
                <button type="button" className="btn-secondary" onClick={saveRetention} disabled={savingRetention || retentionDraft === String(retentionDays)}>{savingRetention ? t('common.saving') : t('common.saveChanges')}</button>
              </div>
            </div>
          </section>

          <form className="mb-4 rounded-lg border border-border p-4" onSubmit={applyFilters}>
            <div className="mb-3 flex flex-wrap items-center justify-between gap-2"><h3 className="font-semibold">{t('audit.filters')}</h3><button type="button" className="text-sm underline" onClick={clearFilters}>{t('audit.clearFilters')}</button></div>
            <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
              <label className="text-xs font-medium">{t('audit.actor')}<select className={`${fieldClasses} mt-1`} value={draftFilters.principalUserId} onChange={(event) => updateDraft('principalUserId', event.currentTarget.value)}><option value="">{t('common.all')}</option>{users.map((user) => <option key={user.id} value={user.id}>{user.username}</option>)}</select></label>
              <label className="text-xs font-medium">{t('audit.outcome')}<select className={`${fieldClasses} mt-1`} value={draftFilters.outcome} onChange={(event) => updateDraft('outcome', event.currentTarget.value)}><option value="">{t('common.all')}</option>{['allowed', 'denied', 'success', 'failure', 'error'].map((outcome) => <option key={outcome} value={outcome}>{outcome}</option>)}</select></label>
              <label className="text-xs font-medium sm:col-span-2">{t('audit.action')}<input className={`${fieldClasses} mt-1`} value={draftFilters.action} placeholder="recordings.export" onInput={(event) => updateDraft('action', event.currentTarget.value)} /></label>
              <label className="text-xs font-medium">{t('audit.since')}<input className={`${fieldClasses} mt-1`} type="datetime-local" value={draftFilters.since} onInput={(event) => updateDraft('since', event.currentTarget.value)} /></label>
              <label className="text-xs font-medium">{t('audit.until')}<input className={`${fieldClasses} mt-1`} type="datetime-local" value={draftFilters.until} onInput={(event) => updateDraft('until', event.currentTarget.value)} /></label>
              <label className="text-xs font-medium">{t('audit.requestId')}<input className={`${fieldClasses} mt-1 font-mono`} value={draftFilters.requestId} onInput={(event) => updateDraft('requestId', event.currentTarget.value)} /></label>
              <label className="text-xs font-medium">{t('audit.targetUuid')}<input className={`${fieldClasses} mt-1 font-mono`} value={draftFilters.targetUuid} onInput={(event) => updateDraft('targetUuid', event.currentTarget.value)} /></label>
            </div>
            <div className="mt-3 flex justify-end"><button type="submit" className="btn-primary">{t('audit.applyFilters')}</button></div>
          </form>

          <div className="mb-3 flex flex-wrap items-center justify-between gap-3">
            <p className="text-sm text-muted-foreground">{loading ? t('common.loading') : t('audit.resultRange', bounds)}</p>
            <div className="flex flex-wrap items-end gap-2">
              <label className="text-xs font-medium">{t('audit.pageSize')}<select className={`${fieldClasses} mt-1 w-24`} value={pageSize} onChange={(event) => { setPageSize(Number(event.currentTarget.value)); setPageNumber(1); }}>{[25, 50, 100].map((size) => <option key={size} value={size}>{size}</option>)}</select></label>
              <button type="button" className="btn-secondary" onClick={loadEvents} disabled={loading}>{t('common.refresh')}</button>
              <button type="button" className="btn-secondary" onClick={exportPage} disabled={exporting || page.count === 0}>{exporting ? t('audit.exporting') : t('audit.exportPage')}</button>
            </div>
          </div>

          {error && <div className="mb-3 rounded-md badge-danger p-3">{error} <button type="button" className="ml-2 underline" onClick={loadEvents}>{t('common.retry')}</button></div>}
          {!loading && !error && page.events.length === 0 && <div className="rounded-lg border border-dashed border-border p-10 text-center"><h3 className="font-semibold">{t('audit.empty')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('audit.emptyDescription')}</p></div>}
          <div className={`space-y-3 ${loading ? 'opacity-60' : ''}`} aria-live="polite" aria-busy={loading}>
            {page.events.map((event) => <EventCard key={event.uuid} event={event} expanded={expandedUuid === event.uuid} onToggle={() => setExpandedUuid((current) => current === event.uuid ? '' : event.uuid)} onFilterAction={() => filterToAction(event.action)} t={t} />)}
          </div>

          <nav className="mt-4 flex items-center justify-between border-t border-border pt-4" aria-label={t('audit.pagination')}>
            <button type="button" className="btn-secondary" disabled={loading || pageNumber <= 1} onClick={() => setPageNumber((current) => Math.max(1, current - 1))}>{t('common.previous')}</button>
            <span className="text-sm">{t('audit.page', { page: pageNumber })}</span>
            <button type="button" className="btn-secondary" disabled={loading || !hasNext} onClick={() => setPageNumber((current) => current + 1)}>{t('common.next')}</button>
          </nav>
        </div>
      </section>
    </div>
  );
}
