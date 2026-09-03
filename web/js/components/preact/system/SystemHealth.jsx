import { useMemo } from 'preact/hooks';
import { useQuery } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import { formatBytes } from './SystemUtils.js';
import { HealthIncidentHistory } from './HealthIncidentHistory.jsx';

export const HEALTH_SAMPLE_LIMIT = 120;

const GROUP_ORDER = ['resources', 'storage', 'hardware', 'network', 'clock', 'components'];

export function observationGroup(metric = '') {
  if (metric.startsWith('filesystem.') || metric.startsWith('storage.')) return 'storage';
  if (metric.startsWith('thermal.') || metric.startsWith('hardware.')) return 'hardware';
  if (metric.startsWith('network.')) return 'network';
  if (metric.startsWith('clock.') || metric.startsWith('system.uptime')) return 'clock';
  if (metric.startsWith('process.') || metric.startsWith('system.') || metric.startsWith('health.')) return 'components';
  return 'resources';
}

export function groupHealthObservations(observations = []) {
  const groups = Object.fromEntries(GROUP_ORDER.map((name) => [name, []]));
  for (const observation of observations.slice(0, 256)) {
    groups[observationGroup(observation?.metric)].push(observation);
  }
  return groups;
}

export function boundedRecentSamples(samples = []) {
  return Array.isArray(samples) ? samples.slice(-HEALTH_SAMPLE_LIMIT) : [];
}

export function formatHealthValue(observation, unknown = 'Unknown') {
  if (!observation || observation.capability !== 'available' ||
      observation.freshness === 'stale' || !Number.isFinite(observation.value)) return unknown;
  const value = observation.value;
  switch (observation.unit) {
    case 'ratio': return `${(value * 100).toFixed(1)}%`;
    case 'bytes': return formatBytes(value);
    case 'seconds': return `${value.toFixed(value < 10 ? 1 : 0)}s`;
    case 'celsius': return `${value.toFixed(1)} °C`;
    case 'hertz': return `${value.toLocaleString()} Hz`;
    case 'boolean': return value ? 'Yes' : 'No';
    case 'count': return value.toLocaleString();
    default: return Number.isInteger(value) ? value.toLocaleString() : value.toFixed(3);
  }
}

export function remediationKey(condition = '') {
  const prefix = condition.split('.')[0];
  const known = new Set(['memory', 'cpu', 'io', 'filesystem', 'thermal', 'network', 'clock', 'process', 'storage', 'hardware', 'health', 'system', 'event']);
  return `system.health.remediation.${known.has(prefix) ? prefix : 'default'}`;
}

function statusClass(status) {
  if (status === 'healthy' || status === 'available' || status === 'fresh') return 'badge-success';
  if (status === 'warning' || status === 'pending' || status === 'stale') return 'badge-warning';
  if (status === 'error' || status === 'critical') return 'badge-danger';
  return 'bg-muted text-muted-foreground';
}

function readableMetric(metric = '') {
  return metric.replaceAll('.', ' ').replaceAll('_', ' ');
}

function relativeDuration(timestampMs, unknown) {
  if (!Number.isFinite(timestampMs) || timestampMs <= 0) return unknown;
  const seconds = Math.max(0, Math.floor((Date.now() - timestampMs) / 1000));
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h`;
  return `${Math.floor(seconds / 86400)}d`;
}

function ObservationCard({ observation, t }) {
  const unavailable = observation?.capability !== 'available' || observation?.freshness === 'stale';
  return (
    <article className={`rounded-md border p-3 ${unavailable ? 'border-border bg-muted/30' : 'border-border bg-background'}`}>
      <div className="flex flex-wrap items-start justify-between gap-2">
        <div className="min-w-0">
          <h5 className="break-words text-sm font-semibold capitalize">{readableMetric(observation.metric)}</h5>
          <p className="mt-1 break-all text-xs text-muted-foreground">{observation.resource} · {observation.scope}</p>
        </div>
        <strong className="text-sm">{formatHealthValue(observation, t('common.unknown'))}</strong>
      </div>
      <div className="mt-2 flex flex-wrap gap-2" aria-label={t('system.health.observationState')}>
        <span className={`rounded-full px-2 py-0.5 text-xs font-semibold ${statusClass(observation.capability)}`}>
          {observation.capability || t('common.unknown')}
        </span>
        <span className={`rounded-full px-2 py-0.5 text-xs font-semibold ${statusClass(observation.freshness)}`}>
          {observation.freshness || t('common.unknown')}
        </span>
      </div>
    </article>
  );
}

function IncidentCard({ incident, t }) {
  const threshold = incident.thresholds;
  return (
    <article className="rounded-lg border border-border bg-card p-4" data-testid="active-health-incident">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <h4 className="font-semibold">{readableMetric(incident.condition)}</h4>
          <p className="mt-1 text-xs text-muted-foreground">{incident.subject} · {incident.scope}</p>
        </div>
        <span className={`rounded-full px-2 py-1 text-xs font-semibold ${statusClass(incident.severity)}`}>
          {incident.severity || t('common.unknown')} · {incident.state || t('common.unknown')}
        </span>
      </div>
      <dl className="mt-3 grid grid-cols-1 gap-2 text-sm sm:grid-cols-2">
        <div><dt className="text-muted-foreground">{t('system.health.duration')}</dt><dd>{relativeDuration(incident.first_observed_at_ms, t('common.unknown'))}</dd></div>
        <div><dt className="text-muted-foreground">{t('system.health.currentValue')}</dt><dd>{formatHealthValue(incident.observation, t('common.unknown'))}</dd></div>
        {threshold && <div className="sm:col-span-2"><dt className="text-muted-foreground">{t('system.health.thresholds')}</dt><dd>{t('system.health.thresholdSummary', { warning: threshold.warning_threshold, critical: threshold.critical_threshold, recovery: threshold.recovery_threshold, unit: threshold.unit, seconds: threshold.warning_for_seconds })}</dd></div>}
      </dl>
      <p className="mt-3 rounded-md bg-muted p-3 text-sm"><strong>{t('system.health.remediation')}:</strong> {t(remediationKey(incident.condition))}</p>
    </article>
  );
}

function CoverageBanner({ health, t }) {
  const visibility = health?.visibility || {};
  const coverage = health?.coverage || {};
  const hostUnavailable = visibility.host_hardware_visible === false;
  const complete = coverage.complete === true;
  const unknown = health?.snapshot?.available !== true;
  const message = hostUnavailable
    ? t('system.health.hostHardwareUnavailable')
    : unknown
      ? t('system.health.coverageUnknown')
      : complete
        ? t('system.health.coverageComplete')
        : t('system.health.coveragePartial', { count: coverage.incomplete_count ?? 0 });
  return (
    <div role="status" data-testid="health-coverage-banner" className={`rounded-lg border p-4 ${hostUnavailable || unknown || !complete ? 'border-amber-400/70 bg-amber-50 text-amber-950 dark:bg-amber-950/20 dark:text-amber-100' : 'border-border bg-card'}`}>
      <p className="font-semibold">{hostUnavailable ? t('system.health.containerCoverage') : t('system.health.coverage')}</p>
      <p className="mt-1 text-sm">{message}</p>
      <p className="mt-2 text-xs opacity-80">{t('system.health.effectiveScope')}: {visibility.effective_scope || t('common.unknown')}</p>
    </div>
  );
}

function ComponentHealth({ self, t }) {
  if (!self) return null;
  const collectors = Array.isArray(self.collectors) ? self.collectors.slice(0, 33) : [];
  const staleCollectors = collectors.filter((collector) => collector.stale || collector.failures > 0 || collector.timeouts > 0);
  const delivery = self.event_delivery || {};
  const persistence = self.incident_persistence || {};
  return (
    <section className="rounded-lg border border-border bg-card p-4" aria-labelledby="component-health-heading" data-testid="health-component-status">
      <h3 id="component-health-heading" className="font-semibold">{t('system.health.componentStatus')}</h3>
      <div className="mt-3 grid gap-3 sm:grid-cols-3">
        <article className="rounded-md bg-muted p-3"><p className="text-xs text-muted-foreground">{t('system.health.collectors')}</p><p className="mt-1 font-semibold">{t('system.health.collectorSummary', { total: collectors.length, attention: staleCollectors.length })}</p></article>
        <article className="rounded-md bg-muted p-3"><p className="text-xs text-muted-foreground">{t('system.health.eventDelivery')}</p><p className={`mt-1 inline-block rounded-full px-2 py-0.5 text-sm font-semibold ${statusClass(delivery.degraded ? 'error' : delivery.outbox?.available === false ? 'unknown' : 'healthy')}`}>{delivery.outbox?.available === false ? t('common.unknown') : delivery.degraded ? t('system.health.degraded') : t('system.healthy')}</p>{delivery.circular_report_path && <p className="mt-2 text-xs">{t('system.health.circularDelivery')}</p>}</article>
        <article className="rounded-md bg-muted p-3"><p className="text-xs text-muted-foreground">{t('system.health.incidentPersistence')}</p><p className="mt-1 font-semibold">{t('system.health.persistenceSummary', { pending: persistence.pending ?? 0, failures: persistence.failures ?? 0, retries: persistence.retries ?? 0 })}</p></article>
      </div>
      {staleCollectors.length > 0 && <ul className="mt-3 flex flex-wrap gap-2" aria-label={t('system.health.collectorsNeedingAttention')}>{staleCollectors.map((collector) => <li key={`${collector.collector}-${collector.scope}`} className="rounded-full badge-warning px-2 py-1 text-xs">{collector.collector} · {collector.stale ? t('system.health.stale') : t('system.health.collectionFailures', { count: collector.failures + collector.timeouts })}</li>)}</ul>}
    </section>
  );
}

export function SystemHealth() {
  const { t } = useI18n();
  const query = useQuery(
    ['systemHealth'],
    '/api/system/health',
    { timeout: 15000, retries: 1 },
    { staleTime: 30000, refetchOnWindowFocus: false }
  );
  const health = query.data;
  const groups = useMemo(() => groupHealthObservations(health?.observations || []), [health?.observations]);
  const samples = boundedRecentSamples(health?.recent_samples);

  if (query.isLoading && !health) return <div className="rounded-lg border border-border bg-card p-6 text-muted-foreground" aria-live="polite">{t('system.health.loading')}</div>;
  if (query.error && !health) return (
    <div className="rounded-lg border border-border bg-card p-6" role="alert" data-testid="system-health-error">
      <p className="font-semibold text-destructive">{t('system.health.loadError')}</p>
      <p className="mt-1 text-sm text-muted-foreground">{query.error.message}</p>
      <button type="button" className="btn-secondary mt-4" onClick={() => query.refetch()}>{t('common.retry')}</button>
    </div>
  );

  const incidents = health?.active_incidents || [];
  const overall = health?.overall_state || 'unknown';
  return (
    <div className="space-y-5" data-testid="system-health-view">
      <header className="flex flex-col gap-3 rounded-lg border border-border bg-card p-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <p className="text-xs font-semibold uppercase tracking-[0.14em] text-muted-foreground">{t('system.health.operatorView')}</p>
          <div className="mt-1 flex flex-wrap items-center gap-2">
            <h2 className="text-xl font-semibold">{t('system.health.title')}</h2>
            <span role="status" data-testid="health-overall-state" className={`rounded-full px-3 py-1 text-sm font-semibold ${statusClass(overall)}`}>{overall}</span>
          </div>
          <p className="mt-1 text-sm text-muted-foreground">{t('system.health.activeIncidentCount', { count: incidents.length })}</p>
        </div>
        <button type="button" className="btn-secondary self-start" onClick={() => query.refetch()} disabled={query.isFetching}>
          {query.isFetching ? t('system.health.refreshing') : t('common.refresh')}
        </button>
      </header>

      <CoverageBanner health={health} t={t} />
      <ComponentHealth self={health?.self_observability} t={t} />

      <section aria-labelledby="active-health-heading">
        <h3 id="active-health-heading" className="text-lg font-semibold">{t('system.health.activeIncidents')}</h3>
        {incidents.length ? <div className="mt-3 grid gap-3 lg:grid-cols-2">{incidents.map((incident) => <IncidentCard key={incident.incident_id || `${incident.condition}-${incident.subject}`} incident={incident} t={t} />)}</div> : <p className="mt-2 rounded-lg border border-border bg-card p-4 text-sm text-muted-foreground">{t('system.health.noActiveIncidents')}</p>}
      </section>

      <section aria-labelledby="facts-heading">
        <h3 id="facts-heading" className="text-lg font-semibold">{t('system.health.facts')}</h3>
        <div className="mt-3 space-y-4">
          {GROUP_ORDER.map((group) => (
            <section key={group} className="rounded-lg border border-border bg-card p-4">
              <h4 className="font-semibold">{t(`system.health.group.${group}`)}</h4>
              {groups[group].length ? <div className="mt-3 grid gap-3 sm:grid-cols-2 xl:grid-cols-3">{groups[group].map((observation, index) => <ObservationCard key={`${observation.metric}-${observation.resource}-${index}`} observation={observation} t={t} />)}</div> : <p className="mt-2 text-sm text-muted-foreground">{t('system.health.noFacts')}</p>}
            </section>
          ))}
        </div>
      </section>

      <section className="rounded-lg border border-border bg-card p-4" aria-labelledby="sample-history-heading">
        <h3 id="sample-history-heading" className="font-semibold">{t('system.health.recentSamples')}</h3>
        <p className="mt-1 text-xs text-muted-foreground">{t('system.health.recentSamplesHelp', { count: HEALTH_SAMPLE_LIMIT })}</p>
        {samples.length ? <ol className="mt-3 grid gap-2 sm:grid-cols-2 lg:grid-cols-3">{[...samples].reverse().map((sample) => <li key={sample.sequence} className="rounded-md bg-muted p-3 text-sm"><p>{t('system.health.sampleSequence', { sequence: sample.sequence })}</p><p className="mt-1 text-xs text-muted-foreground">{Number.isFinite(sample.completed_at_ms) ? new Date(sample.completed_at_ms).toLocaleString() : t('common.unknown')}</p><p className="mt-1 text-xs text-muted-foreground">{t('system.health.sampleCoverage', { observations: sample.observation_count ?? 0, dropped: sample.observations_dropped ?? 0 })}</p></li>)}</ol> : <p className="mt-2 text-sm text-muted-foreground">{t('system.health.noSamples')}</p>}
      </section>

      <HealthIncidentHistory />
    </div>
  );
}
