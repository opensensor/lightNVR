// @ts-nocheck
/** Memory and storage facts sourced from the completed health snapshot. */
import { useI18n } from '../../../i18n.ts';
import { useQuery } from '../../../query-client.ts';
import { meterPercent, summarizeStorageTiers } from './storageTiers.ts';

export function metricKnown(section, value) {
  return Number.isFinite(value) && (!section?.capability || section.capability === 'available');
}

export function boundedPercent(used, total) {
  if (!Number.isFinite(used) || !Number.isFinite(total) || total <= 0) return null;
  return Math.max(0, Math.min(100, used / total * 100));
}

function Meter({ value, label, unknown }) {
  const known = Number.isFinite(value);
  return (
    <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700" role="progressbar" aria-label={label} aria-valuemin="0" aria-valuemax="100" aria-valuenow={known ? value : undefined} aria-valuetext={known ? `${value.toFixed(1)}%` : unknown}>
      <div className="h-2.5 rounded-full" style={{ backgroundColor: known ? 'hsl(var(--primary))' : 'hsl(var(--muted))', width: known ? `${value}%` : '100%' }}></div>
    </div>
  );
}

function CapacityRow({ label, section, formatBytes, unknown }) {
  const totalKnown = metricKnown(section, section?.total);
  const usedKnown = metricKnown(section, section?.used);
  const percent = totalKnown && usedKnown ? boundedPercent(section.used, section.total) : null;
  return (
    <div>
      <div className="flex flex-wrap justify-between gap-2 mb-1">
        <span className="font-medium">{label}:</span>
        <span>{totalKnown && usedKnown ? `${formatBytes(section.used)} / ${formatBytes(section.total)}` : unknown}</span>
      </div>
      <Meter value={percent} label={label} unknown={unknown} />
      {!totalKnown || !usedKnown ? <p className="mt-1 text-xs text-muted-foreground">{section?.capability || unknown}</p> : null}
    </div>
  );
}

const CLASS_BADGES = {
  hot: 'bg-red-500/15 text-red-600 dark:text-red-300',
  warm: 'bg-amber-500/15 text-amber-700 dark:text-amber-300',
  cold: 'bg-sky-500/15 text-sky-700 dark:text-sky-300',
};
const PRESSURE_BADGES = {
  high: 'bg-amber-500/15 text-amber-700 dark:text-amber-300',
  reserve: 'bg-red-500/15 text-red-600 dark:text-red-300',
  unavailable: 'bg-muted text-muted-foreground',
};

/**
 * Per-target recording storage. Shown instead of the single legacy bar when
 * recordings span several targets (e.g. a hot disk plus a cold bucket), since
 * that bar divides every recording byte by the local disk's capacity.
 */
export function RecordingStorageTiers({ summary, formatBytes, unknown }) {
  const { t } = useI18n();
  const classTotals = ['hot', 'warm', 'cold']
    .filter((storageClass) => summary.rows.some((row) => row.storageClass === storageClass))
    .map((storageClass) => `${t(`system.storageTiers.class.${storageClass}`)} ${formatBytes(summary.byClass[storageClass])}`);
  return (
    <div className="space-y-3" data-testid="storage-tiers">
      <div className="flex flex-wrap justify-between gap-2">
        <span className="font-medium">{t('system.storageTiers.title')}:</span>
        <span>{t('system.storageTiers.summary', {
          total: formatBytes(summary.storedBytes), count: summary.rows.length,
        })}</span>
      </div>
      {classTotals.length > 1 && (
        <p className="text-xs text-muted-foreground -mt-2">{classTotals.join(' · ')}</p>
      )}
      {summary.rows.map((row) => {
        const percent = meterPercent(row.meter);
        const pressureBadge = row.pressure !== 'normal' && PRESSURE_BADGES[row.pressure];
        return (
          <div key={row.uuid} data-testid="storage-tier-row" className="ps-3 border-s-2 border-border">
            <div className="flex flex-wrap justify-between gap-2 mb-1">
              <span className="flex flex-wrap items-center gap-1.5">
                <span className="font-medium">{row.name}</span>
                <span className={`inline-block px-1.5 py-0.5 text-xs rounded ${CLASS_BADGES[row.storageClass]}`}>
                  {t(`system.storageTiers.class.${row.storageClass}`)}
                </span>
                <span className="inline-block px-1.5 py-0.5 text-xs rounded bg-muted text-muted-foreground">
                  {t(`system.storageTiers.kind.${row.kind}`)}
                </span>
                {pressureBadge && (
                  <span className={`inline-block px-1.5 py-0.5 text-xs rounded ${pressureBadge}`}>
                    {t(`system.storageTiers.pressure.${row.pressure}`)}
                  </span>
                )}
              </span>
              <span>{row.meter
                ? `${formatBytes(row.meter.used)} / ${formatBytes(row.meter.total)}`
                : formatBytes(row.storedBytes)}</span>
            </div>
            {row.meter && (
              <Meter value={percent} label={row.name} unknown={unknown} />
            )}
            <p className="mt-1 text-xs text-muted-foreground">
              {[
                t('system.storageTiers.recordings', {
                  bytes: formatBytes(row.recordingBytes), count: row.recordingCount,
                }),
                row.replicaBytes > 0
                  ? t('system.storageTiers.copies', { bytes: formatBytes(row.replicaBytes) }) : null,
                row.meter?.kind === 'filesystem' ? t('system.storageTiers.diskUsage') : null,
                row.meter?.kind === 'budget' ? t('system.storageTiers.budgetUsage') : null,
                row.kind === 'bucket' && !row.meter ? t('system.storageTiers.noBudget') : null,
                row.duplicateFilesystem ? t('system.storageTiers.sharedFilesystem') : null,
              ].filter(Boolean).join(' · ')}
            </p>
          </div>
        );
      })}
    </div>
  );
}

export function MemoryStorage({ systemInfo, formatBytes }) {
  const { t } = useI18n();
  const unknown = t('common.unknown');
  // Same query key as Settings → Storage. Accounts without storage-configure
  // permission get a 403 and simply keep the single legacy bar.
  const targetsQuery = useQuery(['storage-targets'], '/api/storage-targets',
    { cache: 'no-store', timeout: 15000, retries: 0 }, { staleTime: 30000, retry: false });
  const tierSummary = targetsQuery.data ? summarizeStorageTiers(targetsQuery.data.targets) : null;
  const processSections = [
    { name: 'LightNVR', section: systemInfo.memory, badge: 'bg-primary/10 text-primary' },
    { name: 'go2rtc', section: systemInfo.go2rtcMemory, badge: 'badge-success' },
    { name: 'detector', section: systemInfo.detectorMemory, badge: 'badge-warning' },
  ];
  const processValues = processSections.map(({ section }) => metricKnown(section, section?.used) ? section.used : null);
  const totalSystemMemory = metricKnown(systemInfo.systemMemory, systemInfo.systemMemory?.total)
    ? systemInfo.systemMemory.total : null;
  const allProcessesKnown = processValues.every(Number.isFinite);
  const combinedMemoryUsed = allProcessesKnown ? processValues.reduce((sum, value) => sum + value, 0) : null;
  const combinedMemoryPercent = boundedPercent(combinedMemoryUsed, totalSystemMemory);

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.memoryAndStorage')}</h3>
      <div className="space-y-4">
        <div>
          <div className="flex flex-wrap justify-between gap-2 mb-1">
            <span className="font-medium">{t('system.processMemory')}:</span>
            <div className="flex flex-wrap justify-end gap-1">
              {processSections.map(({ name, section, badge }) => (
                <span key={name} className={`inline-block px-2 py-0.5 text-xs rounded ${badge}`}>
                  {name}: {metricKnown(section, section?.used) ? formatBytes(section.used) : unknown}
                </span>
              ))}
            </div>
          </div>
          <div className="flex flex-wrap justify-between gap-2 text-xs text-muted-foreground mb-1">
            <span>{allProcessesKnown && Number.isFinite(totalSystemMemory)
              ? t('system.combinedMemory', { used: formatBytes(combinedMemoryUsed), total: formatBytes(totalSystemMemory) })
              : t('system.health.valueUnavailable')}</span>
            <span>{Number.isFinite(combinedMemoryPercent)
              ? t('system.percentOfTotalMemory', { percent: combinedMemoryPercent.toFixed(1) })
              : unknown}</span>
          </div>
          <Meter value={combinedMemoryPercent} label={t('system.processMemory')} unknown={unknown} />
        </div>
        <CapacityRow label={t('system.systemMemory')} section={systemInfo.systemMemory} formatBytes={formatBytes} unknown={unknown} />
        {tierSummary?.tiered
          ? <RecordingStorageTiers summary={tierSummary} formatBytes={formatBytes} unknown={unknown} />
          : <CapacityRow label={t('system.lightNvrStorage')} section={systemInfo.disk} formatBytes={formatBytes} unknown={unknown} />}
        <CapacityRow label={t('system.systemStorage')} section={systemInfo.systemDisk} formatBytes={formatBytes} unknown={unknown} />
      </div>
    </div>
  );
}
