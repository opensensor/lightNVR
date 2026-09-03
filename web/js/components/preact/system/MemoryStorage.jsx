/** Memory and storage facts sourced from the completed health snapshot. */
import { useI18n } from '../../../i18n.js';

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

export function MemoryStorage({ systemInfo, formatBytes }) {
  const { t } = useI18n();
  const unknown = t('common.unknown');
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
        <CapacityRow label={t('system.lightNvrStorage')} section={systemInfo.disk} formatBytes={formatBytes} unknown={unknown} />
        <CapacityRow label={t('system.systemStorage')} section={systemInfo.systemDisk} formatBytes={formatBytes} unknown={unknown} />
      </div>
    </div>
  );
}
