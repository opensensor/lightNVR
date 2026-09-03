import { useState } from 'preact/hooks';
import { useQuery } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import { formatBytes } from './SystemUtils.js';

const HISTORY_PAGE_SIZE = 20;

export function buildIncidentHistoryUrl(cursor = '') {
  const suffix = cursor ? `&cursor=${encodeURIComponent(cursor)}` : '';
  return `/api/system/health/incidents?limit=${HISTORY_PAGE_SIZE}&include_closed=true${suffix}`;
}

function formatHistoryValue(observation, unknown) {
  if (!observation || observation.capability !== 'available' || !Number.isFinite(observation.value)) return unknown;
  if (observation.unit === 'ratio') return `${(observation.value * 100).toFixed(1)}%`;
  if (observation.unit === 'bytes') return formatBytes(observation.value);
  if (observation.unit === 'celsius') return `${observation.value.toFixed(1)} °C`;
  return observation.value.toLocaleString();
}

function historyRemediationKey(condition = '') {
  const prefix = condition.split('.')[0];
  const known = new Set(['memory', 'cpu', 'io', 'filesystem', 'thermal', 'network', 'clock', 'process', 'storage', 'hardware', 'health', 'system', 'event']);
  return `system.health.remediation.${known.has(prefix) ? prefix : 'default'}`;
}

function label(value = '') {
  return value.replaceAll('.', ' ').replaceAll('_', ' ');
}

function severityClass(severity) {
  if (severity === 'critical' || severity === 'error') return 'badge-danger';
  if (severity === 'warning') return 'badge-warning';
  if (severity === 'none') return 'badge-success';
  return 'bg-muted text-muted-foreground';
}

function dateTime(value, unknown) {
  return Number.isFinite(value) && value > 0 ? new Date(value).toLocaleString() : unknown;
}

export function HealthIncidentHistory() {
  const { t } = useI18n();
  const [cursor, setCursor] = useState('');
  const [previous, setPrevious] = useState([]);
  const query = useQuery(
    ['systemHealthIncidents', cursor],
    buildIncidentHistoryUrl(cursor),
    { timeout: 15000, retries: 1 },
    { staleTime: 30000, refetchOnWindowFocus: false }
  );
  const incidents = query.data?.incidents || [];
  const nextCursor = query.data?.next_cursor || '';
  const goNext = () => {
    if (!nextCursor) return;
    setPrevious((items) => [...items, cursor].slice(-20));
    setCursor(nextCursor);
  };
  const goPrevious = () => {
    if (!previous.length) return;
    const items = previous.slice(0, -1);
    setCursor(previous[previous.length - 1]);
    setPrevious(items);
  };

  return (
    <section className="rounded-lg border border-border bg-card p-4" aria-labelledby="incident-history-heading" data-testid="health-incident-history">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <div><h3 id="incident-history-heading" className="font-semibold">{t('system.health.incidentHistory')}</h3><p className="mt-1 text-xs text-muted-foreground">{t('system.health.incidentHistoryHelp')}</p></div>
        <button type="button" className="btn-secondary" onClick={() => query.refetch()} disabled={query.isFetching}>{query.isFetching ? t('system.health.refreshing') : t('common.refresh')}</button>
      </div>
      {query.isLoading && !query.data && <p className="py-6 text-sm text-muted-foreground" aria-live="polite">{t('system.health.loadingHistory')}</p>}
      {query.error && !query.data && <div className="py-6" role="alert"><p className="text-sm text-destructive">{t('system.health.historyError')}</p><button type="button" className="btn-secondary mt-3" onClick={() => query.refetch()}>{t('common.retry')}</button></div>}
      {!query.isLoading && !query.error && incidents.length === 0 && <p className="py-6 text-sm text-muted-foreground">{t('system.health.emptyHistory')}</p>}
      {incidents.length > 0 && <div className="mt-4 space-y-3">{incidents.map((incident) => (
        <article key={incident.incident_id} className="rounded-md border border-border bg-background p-3" data-testid="health-history-item">
          <div className="flex flex-wrap items-start justify-between gap-2"><div><h4 className="font-semibold capitalize">{label(incident.condition)}</h4><p className="text-xs text-muted-foreground">{incident.subject} · {incident.scope}</p></div><span className={`rounded-full px-2 py-0.5 text-xs font-semibold ${severityClass(incident.severity)}`}>{incident.severity || t('common.unknown')} · {incident.state || t('common.unknown')}</span></div>
          <dl className="mt-3 grid gap-2 text-sm sm:grid-cols-2 lg:grid-cols-4"><div><dt className="text-muted-foreground">{t('system.health.firstSeen')}</dt><dd>{dateTime(incident.first_observed_at_ms, t('common.unknown'))}</dd></div><div><dt className="text-muted-foreground">{t('system.health.lastSeen')}</dt><dd>{dateTime(incident.last_observed_at_ms, t('common.unknown'))}</dd></div><div><dt className="text-muted-foreground">{t('system.health.currentValue')}</dt><dd>{formatHistoryValue(incident.observation, t('common.unknown'))}</dd></div><div><dt className="text-muted-foreground">{t('system.health.deliveryState')}</dt><dd>{incident.reconciliation || t('common.unknown')}</dd></div></dl>
          <p className="mt-3 text-sm"><strong>{t('system.health.remediation')}:</strong> {t(historyRemediationKey(incident.condition))}</p>
        </article>
      ))}</div>}
      <nav className="mt-4 flex items-center justify-between gap-3" aria-label={t('system.health.historyPagination')}>
        <button type="button" className="btn-secondary" onClick={goPrevious} disabled={!previous.length || query.isFetching}>{t('common.previous')}</button>
        <span className="text-xs text-muted-foreground">{t('system.health.pageCount', { count: incidents.length })}</span>
        <button type="button" className="btn-secondary" onClick={goNext} disabled={!nextCursor || query.isFetching}>{t('common.next')}</button>
      </nav>
    </section>
  );
}
