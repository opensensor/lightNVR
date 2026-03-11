/**
 * StorageHealth Component
 * Displays storage controller health: disk pressure, last cleanup stats,
 * and a manual cleanup trigger button.
 */

import { useState } from 'preact/hooks';
import { useQuery, useMutation, fetchJSON } from '../../../query-client.js';
import { nowMilliseconds } from '../../../utils/date-utils.js';
import { useI18n } from '../../../i18n.js';

/**
 * Map pressure level to a badge color class
 */
function pressureBadge(level, t) {
  switch (level) {
    case 'NORMAL':    return { bg: 'hsl(142 70% 45% / 0.15)', text: 'hsl(142 70% 35%)', label: t('system.pressure.normal') };
    case 'WARNING':   return { bg: 'hsl(38 92% 50% / 0.15)',  text: 'hsl(38 80% 40%)',  label: t('system.pressure.warning') };
    case 'CRITICAL':  return { bg: 'hsl(0 84% 60% / 0.15)',   text: 'hsl(0 84% 40%)',   label: t('system.pressure.critical') };
    case 'EMERGENCY': return { bg: 'hsl(0 84% 40% / 0.25)',   text: 'hsl(0 84% 30%)',   label: t('system.pressure.emergency') };
    default:          return { bg: 'hsl(var(--muted))',        text: 'hsl(var(--muted-foreground))', label: level || t('common.unknown') };
  }
}

function formatTimeAgo(epochSeconds, t) {
  if (!epochSeconds) return t('common.never');
  const diff = Math.floor(nowMilliseconds() / 1000) - epochSeconds;
  if (diff < 0) return t('system.justNow');
  if (diff < 60) return t('system.secondsAgo', { count: diff });
  if (diff < 3600) return t('system.minutesAgo', { count: Math.floor(diff / 60) });
  if (diff < 86400) return t('system.hoursAgo', { count: Math.floor(diff / 3600) });
  return t('system.daysAgo', { count: Math.floor(diff / 86400) });
}

/**
 * StorageHealth component
 * @param {Object} props Component props
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element}
 */
export function StorageHealth({ formatBytes }) {
  const { t } = useI18n();
  const [cleanupPending, setCleanupPending] = useState(false);
  const [cleanupTriggeredAt, setCleanupTriggeredAt] = useState(null);

  const { data: health, isLoading, error, refetch } = useQuery(
    ['storageHealth'],
    '/api/storage/health',
    { timeout: 10000, retries: 1 },
    { refetchInterval: 60000 }  // refresh every 60s (matches heartbeat)
  );

  const cleanupMutation = useMutation({
    mutationFn: async (aggressive) => {
      return await fetchJSON('/api/storage/cleanup', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ aggressive: !!aggressive }),
        timeout: 15000
      });
    },
    onMutate: () => {
      setCleanupPending(true);
      setCleanupTriggeredAt(nowMilliseconds());
    },
    onSettled: () => {
      setCleanupPending(false);
      // Refresh health data after cleanup — give the backend time to finish
      setTimeout(() => refetch(), 3000);
    }
  });

  if (isLoading) {
    return (
      <div className="bg-card text-card-foreground rounded-lg shadow p-4 h-full">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.storageHealth')}</h3>
        <div className="text-muted-foreground text-center py-4">{t('common.loading')}</div>
      </div>
    );
  }

  if (error || !health) {
    return (
      <div className="bg-card text-card-foreground rounded-lg shadow p-4 h-full">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.storageHealth')}</h3>
        <div className="text-muted-foreground text-center py-4">
          {t('system.storageHealthUnavailable')}
        </div>
      </div>
    );
  }

  const badge = pressureBadge(health.pressure_level, t);
  const freePct = health.free_space_pct != null ? health.free_space_pct.toFixed(1) : '?';
  const usedPct = health.free_space_pct != null ? (100 - health.free_space_pct).toFixed(1) : '?';
  const totalSpace = (health.used_space_bytes || 0) + (health.free_space_bytes || 0);
  const isElevated = health.pressure_level !== 'NORMAL';

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4 h-full">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.storageHealth')}</h3>
      <div className="space-y-3">
        {/* Pressure level badge */}
        <div className="flex justify-between items-center">
          <span className="font-medium">{t('system.diskPressure')}:</span>
          <span className="inline-block px-3 py-1 text-xs font-semibold rounded-full"
                style={{ backgroundColor: badge.bg, color: badge.text }}>
            {badge.label}
          </span>
        </div>

        {/* Free space bar */}
        <div>
          <div className="flex justify-between text-sm mb-1">
            <span>{t('system.bytesUsed', { value: formatBytes(health.used_space_bytes || 0) })}</span>
            <span>{t('system.bytesFreeSummary', { free: formatBytes(health.free_space_bytes || 0), total: formatBytes(totalSpace), percent: freePct })}</span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div className="h-2.5 rounded-full"
                 style={{
                   width: `${usedPct}%`,
                   backgroundColor: health.pressure_level === 'NORMAL' ? 'hsl(var(--primary))'
                     : health.pressure_level === 'WARNING' ? 'hsl(38 92% 50%)'
                     : 'hsl(0 84% 60%)'
                 }}></div>
          </div>
        </div>

        {/* Last cleanup stats */}
        <div className="flex justify-between text-sm">
          <span className="text-muted-foreground">{t('system.lastCleanup')}:</span>
          <span>{formatTimeAgo(health.last_cleanup_time, t)}</span>
        </div>
        {health.last_cleanup_deleted > 0 && (
          <div className="flex justify-between text-sm">
            <span className="text-muted-foreground">{t('system.deletedFreed')}:</span>
            <span>{t('system.deletedFreedSummary', { files: health.last_cleanup_deleted, bytes: formatBytes(health.last_cleanup_freed || 0) })}</span>
          </div>
        )}
        <div className="flex justify-between text-sm">
          <span className="text-muted-foreground">{t('system.lastHeartbeat')}:</span>
          <span>{formatTimeAgo(health.last_check_time, t)}</span>
        </div>

        {/* Cleanup trigger buttons */}
        {isElevated && (
          <div className="text-xs p-2 rounded"
               style={{ backgroundColor: badge.bg, color: badge.text }}>
            {t('system.diskPressureWarningPrefix', { level: badge.label })}{' '}
            <a href="settings.html" style={{ color: badge.text, textDecoration: 'underline' }}>{t('system.adjustRetentionSettings')}</a>.
          </div>
        )}
        <div className="pt-2 flex gap-2 flex-wrap">
          <button
            className="btn-primary text-xs px-3 py-1 rounded disabled:opacity-50"
            title={t('system.runCleanupTitle')}
            onClick={() => cleanupMutation.mutate(false)}
            disabled={cleanupPending}>
            {cleanupPending ? t('system.running') : t('system.runCleanup')}
          </button>
          <button
            className="btn-warning text-xs px-3 py-1 rounded disabled:opacity-50"
            title={t('system.forceFreeSpaceTitle')}
            onClick={() => cleanupMutation.mutate(true)}
            disabled={cleanupPending}>
            {t('system.forceFreeSpace')}
          </button>
        </div>
        <div className="text-xs text-muted-foreground">
          <em>{t('system.runCleanup')}</em> {t('system.runCleanupRespectsRetention')}&nbsp;
          <em>{t('system.forceFreeSpace')}</em> {t('system.forceFreeSpaceDeletesOldest')}
        </div>
        {cleanupTriggeredAt && (
          <div className="text-xs text-muted-foreground">
            {t('system.cleanupTriggered', { ago: formatTimeAgo(Math.floor(cleanupTriggeredAt / 1000), t) })}
          </div>
        )}
      </div>
    </div>
  );
}

