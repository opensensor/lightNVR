/**
 * StorageHealth Component
 * Displays storage controller health: disk pressure, last cleanup stats,
 * and a manual cleanup trigger button.
 */

import { useState } from 'preact/hooks';
import { useQuery, useMutation, fetchJSON } from '../../../query-client.js';

/**
 * Map pressure level to a badge color class
 */
function pressureBadge(level) {
  switch (level) {
    case 'NORMAL':    return { bg: 'hsl(142 70% 45% / 0.15)', text: 'hsl(142 70% 35%)', label: 'Normal' };
    case 'WARNING':   return { bg: 'hsl(38 92% 50% / 0.15)',  text: 'hsl(38 80% 40%)',  label: 'Warning' };
    case 'CRITICAL':  return { bg: 'hsl(0 84% 60% / 0.15)',   text: 'hsl(0 84% 40%)',   label: 'Critical' };
    case 'EMERGENCY': return { bg: 'hsl(0 84% 40% / 0.25)',   text: 'hsl(0 84% 30%)',   label: 'Emergency' };
    default:          return { bg: 'hsl(var(--muted))',        text: 'hsl(var(--muted-foreground))', label: level || 'Unknown' };
  }
}

function formatTimeAgo(epochSeconds) {
  if (!epochSeconds) return 'Never';
  const diff = Math.floor(Date.now() / 1000) - epochSeconds;
  if (diff < 0) return 'Just now';
  if (diff < 60) return `${diff}s ago`;
  if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
  return `${Math.floor(diff / 86400)}d ago`;
}

/**
 * StorageHealth component
 * @param {Object} props Component props
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element}
 */
export function StorageHealth({ formatBytes }) {
  const [cleanupPending, setCleanupPending] = useState(false);

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
    onMutate: () => setCleanupPending(true),
    onSettled: () => {
      setCleanupPending(false);
      // Refresh health data after cleanup
      setTimeout(() => refetch(), 2000);
    }
  });

  if (isLoading) {
    return (
      <div className="bg-card text-card-foreground rounded-lg shadow p-4 h-full">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Storage Health</h3>
        <div className="text-muted-foreground text-center py-4">Loading...</div>
      </div>
    );
  }

  if (error || !health) {
    return (
      <div className="bg-card text-card-foreground rounded-lg shadow p-4 h-full">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Storage Health</h3>
        <div className="text-muted-foreground text-center py-4">
          Storage health data unavailable
        </div>
      </div>
    );
  }

  const badge = pressureBadge(health.pressure_level);
  const freePct = health.free_space_pct != null ? health.free_space_pct.toFixed(1) : '?';
  const usedPct = health.free_space_pct != null ? (100 - health.free_space_pct).toFixed(1) : '?';

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4 h-full">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Storage Health</h3>
      <div className="space-y-3">
        {/* Pressure level badge */}
        <div className="flex justify-between items-center">
          <span className="font-medium">Disk Pressure:</span>
          <span className="inline-block px-3 py-1 text-xs font-semibold rounded-full"
                style={{ backgroundColor: badge.bg, color: badge.text }}>
            {badge.label}
          </span>
        </div>

        {/* Free space bar */}
        <div>
          <div className="flex justify-between text-sm mb-1">
            <span>{formatBytes(health.used_space_bytes || 0)} used</span>
            <span>{formatBytes(health.free_space_bytes || 0)} free ({freePct}%)</span>
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
          <span className="text-muted-foreground">Last Cleanup:</span>
          <span>{formatTimeAgo(health.last_cleanup_time)}</span>
        </div>
        {health.last_cleanup_deleted > 0 && (
          <div className="flex justify-between text-sm">
            <span className="text-muted-foreground">Deleted / Freed:</span>
            <span>{health.last_cleanup_deleted} files / {formatBytes(health.last_cleanup_freed || 0)}</span>
          </div>
        )}
        <div className="flex justify-between text-sm">
          <span className="text-muted-foreground">Last Heartbeat:</span>
          <span>{formatTimeAgo(health.last_check_time)}</span>
        </div>

        {/* Cleanup trigger button */}
        <div className="pt-2 flex gap-2">
          <button
            className="btn-primary text-xs px-3 py-1 rounded disabled:opacity-50"
            onClick={() => cleanupMutation.mutate(false)}
            disabled={cleanupPending}>
            {cleanupPending ? 'Running...' : 'Run Cleanup'}
          </button>
          {health.pressure_level !== 'NORMAL' && (
            <button
              className="btn-warning text-xs px-3 py-1 rounded disabled:opacity-50"
              onClick={() => cleanupMutation.mutate(true)}
              disabled={cleanupPending}>
              Aggressive Cleanup
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

