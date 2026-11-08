/**
 * ObservationPipeline Component
 * INT_01 L10: observation evidence and publication diagnostics from
 * GET /api/system/info "observation_pipeline" (contract section 15).
 */

import { useI18n } from '../../../i18n.ts';
import type { FormatBytes, ObservationPipelineDiagnostics, SystemInfoData } from '../../../types/system.ts';

export interface ObservationPipelineProps {
  systemInfo: SystemInfoData;
  formatBytes: FormatBytes;
}

const count = (value: unknown): number =>
  typeof value === 'number' && Number.isFinite(value) && value > 0 ? value : 0;

/** Narrow the untyped system-info member; null when absent or malformed. */
export function readObservationPipeline(systemInfo: SystemInfoData | undefined): ObservationPipelineDiagnostics | null {
  const value = systemInfo?.observation_pipeline;
  if (!value || typeof value !== 'object' || typeof value.active !== 'boolean') return null;
  return value;
}

export function totalDropped(pipeline: ObservationPipelineDiagnostics): number {
  const dropped = pipeline.dropped ?? {};
  return count(dropped.queue_full) + count(dropped.budget_exhausted) + count(dropped.disabled) + count(dropped.untracked);
}

export function totalFailures(pipeline: ObservationPipelineDiagnostics): number {
  const failures = pipeline.evidence?.failures ?? {};
  return Object.values(failures).reduce<number>((sum, value) => sum + count(value), 0);
}

/** Whole seconds; the label carries the unit. */
export function formatLag(seconds: unknown): string {
  return String(Math.round(count(seconds)));
}

function Row({ label, value, warn = false }: { label: string; value: string; warn?: boolean }) {
  return (
    <div className="flex justify-between gap-4">
      <span className="font-medium">{label}:</span>
      <span className={warn ? 'text-amber-600 dark:text-amber-400 font-semibold' : undefined}>{value}</span>
    </div>
  );
}

export function ObservationPipeline({ systemInfo, formatBytes }: ObservationPipelineProps) {
  const { t } = useI18n();
  const pipeline = readObservationPipeline(systemInfo);
  if (!pipeline) return null;

  const dropped = pipeline.dropped ?? {};
  const evidence = pipeline.evidence ?? {};
  const publications = pipeline.publications ?? {};
  const history = pipeline.history ?? {};
  const cameras = Array.isArray(pipeline.cameras) ? pipeline.cameras : [];
  const retained = cameras.reduce((sum, camera) => sum + count(camera.retained_bytes), 0);
  const droppedTotal = totalDropped(pipeline);
  const failureTotal = totalFailures(pipeline);
  const exhausted = count(pipeline.budget_exhausted_cameras);

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4" data-testid="observation-pipeline">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.observations.title')}</h3>
      {!pipeline.active ? (
        <p className="text-muted-foreground">{t('system.observations.inactive')}</p>
      ) : (
        <div className="space-y-2">
          <Row label={t('system.observations.accepted')} value={String(count(pipeline.accepted))} />
          <Row
            label={t('system.observations.dropped')}
            value={`${droppedTotal} (${count(dropped.queue_full)} / ${count(dropped.budget_exhausted)} / ${count(dropped.disabled)} / ${count(dropped.untracked)})`}
            warn={droppedTotal > 0}
          />
          <Row
            label={t('system.observations.backlog')}
            value={`${count(evidence.backlog_items)} (${formatBytes(count(evidence.backlog_bytes))})`}
          />
          <Row label={t('system.observations.highWater')} value={String(count(evidence.high_water_items))} />
          <Row label={t('system.observations.failures')} value={String(failureTotal)} warn={failureTotal > 0} />
          <Row label={t('system.observations.retained')} value={formatBytes(retained)} />
          <Row label={t('system.observations.budgetExhausted')} value={String(exhausted)} warn={exhausted > 0} />
          <Row
            label={t('system.observations.oldestRecoverable')}
            value={typeof history.oldest_observed_at === 'string'
              ? `#${count(history.oldest_seq)} · ${new Date(history.oldest_observed_at).toLocaleString()}`
              : `#${count(history.oldest_seq)}`}
          />
          <Row
            label={t('system.observations.publications')}
            value={`${count(publications.pending)} / ${count(publications.failed)} / ${count(publications.expired)}`}
            warn={count(publications.failed) > 0}
          />
          <Row
            label={t('system.observations.relayLag')}
            value={formatLag(publications.relay_lag_seconds)}
            warn={count(publications.relay_lag_seconds) >= 60}
          />
        </div>
      )}
    </div>
  );
}
