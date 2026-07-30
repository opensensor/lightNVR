/**
 * StreamStorage Component
 * Displays storage usage per stream with slivers in a progress bar
 */

import { useI18n } from '../../../i18n.js';

/**
 * StreamStorage component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element} StreamStorage component
 */
export function StreamStorage({ systemInfo, formatBytes }) {
  const { t } = useI18n();
  // Check if stream storage information is available
  if (!systemInfo.streamStorage || !Array.isArray(systemInfo.streamStorage) || systemInfo.streamStorage.length === 0) {
    return (
      <div className="bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Stream Storage</h3>
        <div className="text-muted-foreground text-center py-4">
          {t('system.noStreamStorageInformationAvailable')}
        </div>
      </div>
    );
  }

  // Calculate total storage used by all streams
  const totalStreamStorage = systemInfo.streamStorage.reduce((total, stream) => total + stream.size, 0);

  // Calculate the percentage of total disk space used by all streams
  const totalDiskSpace = systemInfo.disk?.total || 0;
  const totalStreamStoragePercent = totalDiskSpace ?
    (totalStreamStorage / totalDiskSpace * 100).toFixed(1) : 0;

  // Resolve tri-state retention: -1 inherits global, 0 is unlimited.
  const globalRetentionDays = systemInfo.global_retention_days ?? 30;

  // Calculate the percentage of each stream relative to the total stream storage
  const streamStorageData = systemInfo.streamStorage.map(stream => ({
    name: stream.name,
    size: stream.size,
    count: stream.count,
    slicePercent: totalStreamStorage ? (stream.size / totalStreamStorage * 100).toFixed(1) : 0,
    retentionDays: stream.retention_days ?? -1,
    detectionRetentionDays: stream.detection_retention_days ?? -1,
    maxStorageMb: stream.max_storage_mb || 0,
    effectiveRetentionDays: stream.retention_days < 0 ? globalRetentionDays : stream.retention_days,
  }));

  // Sort streams by size (largest first)
  streamStorageData.sort((a, b) => b.size - a.size);

  // Generate theme-aware colors for each stream
  const getStreamColor = (index) => {
    const hues = [217, 142, 38, 0, 265, 350, 215, 180]; // primary, success, warning, danger, purple, rose, slate, teal
    const hue = hues[index % hues.length];
    return {
      bg: `hsl(${hue} 70% 50% / 0.2)`,
      text: `hsl(${hue} 70% 40%)`
    };
  };

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.streamStorage')}</h3>

      <div className="space-y-4">
        <div>
          <div className="flex justify-between mb-1">
            <span className="font-medium">{t('system.storagePerStream')}:</span>
            <div className="flex flex-wrap justify-end gap-1">
              {streamStorageData.map((stream, index) => {
                const color = getStreamColor(index);
                return (
                  <a
                    key={stream.name}
                    href={`recordings.html?stream=${encodeURIComponent(stream.name)}`}
                    className="inline-block px-2 py-0.5 text-xs rounded hover:opacity-80 transition-opacity cursor-pointer"
                    style={{ backgroundColor: color.bg, color: color.text }}
                    title={t('system.viewRecordingsForStream', { stream: stream.name })}
                  >
                    {stream.name}: {formatBytes(stream.size)}
                  </a>
                );
              })}
            </div>
          </div>

          <div className="flex justify-between text-xs text-muted-foreground mb-1">
            <span>{t('system.combinedStorage', { used: formatBytes(totalStreamStorage), total: formatBytes(totalDiskSpace) })}</span>
            <span>{t('system.percentOfTotalStorage', { percent: totalStreamStoragePercent })}</span>
          </div>

          <div className="w-full bg-muted rounded-full h-2.5 overflow-hidden">
            <div className="flex h-full" style={{ width: `${totalStreamStoragePercent}%` }}>
              {streamStorageData.map((stream, index) => {
                const color = getStreamColor(index);
                return (
                  <a
                    key={stream.name}
                    href={`recordings.html?stream=${encodeURIComponent(stream.name)}`}
                    className="h-2.5 block hover:opacity-70 transition-opacity cursor-pointer"
                    style={{ width: `${stream.slicePercent}%`, backgroundColor: color.text }}
                    title={t('system.viewRecordingsForStream', { stream: stream.name })}
                  ></a>
                );
              })}
            </div>
          </div>

          <div className="mt-4">
            <h4 className="font-medium mb-2">{t('system.streamDetails')}</h4>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2">
              {streamStorageData.map((stream, index) => {
                const color = getStreamColor(index);
                const retentionLabel = stream.retentionDays < 0
                  ? globalRetentionDays === 0
                    ? 'Unlimited (global)'
                    : t('system.retentionGlobalDays', { days: globalRetentionDays })
                  : stream.retentionDays === 0 ? 'Unlimited' : `${stream.retentionDays}d`;
                const quotaLabel = stream.maxStorageMb > 0
                  ? t('system.quotaMbLimit', { mb: stream.maxStorageMb })
                  : t('system.noQuota');
                return (
                  <a
                    key={stream.name}
                    href={`recordings.html?stream=${encodeURIComponent(stream.name)}`}
                    className="flex items-center p-2 rounded bg-muted hover:bg-muted/70 transition-colors cursor-pointer"
                  >
                    <div className="w-3 h-3 rounded-full mr-2 flex-shrink-0" style={{ backgroundColor: color.text }}></div>
                    <div className="min-w-0">
                      <div className="font-medium">{stream.name}</div>
                      <div className="text-xs text-muted-foreground">
                        {t('system.streamStorageRecordingsSummary', { size: formatBytes(stream.size), percent: stream.slicePercent, count: stream.count })}
                      </div>
                      <div className="text-xs text-muted-foreground">
                        {t('system.retentionAndQuota', { retention: retentionLabel, quota: quotaLabel })}
                      </div>
                    </div>
                  </a>
                );
              })}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
