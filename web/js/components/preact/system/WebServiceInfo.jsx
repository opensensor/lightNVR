/**
 * WebServiceInfo Component
 * Displays web server activity (request counts, health) and process thread info.
 */

import { useQuery } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';

/**
 * Small status badge — green for healthy, red otherwise.
 */
function StatusBadge({ healthy, t }) {
  const bg   = healthy ? 'hsl(142 70% 45% / 0.15)' : 'hsl(0 84% 60% / 0.15)';
  const text = healthy ? 'hsl(142 70% 35%)'         : 'hsl(0 84% 40%)';
  const label = healthy ? t('system.healthy') : t('system.unhealthy');
  return (
    <span className="inline-block px-3 py-1 text-xs font-semibold rounded-full"
          style={{ backgroundColor: bg, color: text }}>
      {label}
    </span>
  );
}

/**
 * WebServiceInfo component
 * @param {Object} props
 * @param {Object} props.systemInfo  System information object (for thread counts)
 * @returns {JSX.Element}
 */
export function WebServiceInfo({ systemInfo }) {
  const { t } = useI18n();
  const { data: health, isLoading, error } = useQuery(
    ['webHealth'],
    '/api/health',
    { timeout: 5000, retries: 1 },
    { refetchInterval: 30000 }
  );

  const threads      = systemInfo?.threads      ?? '—';
  const poolSize     = systemInfo?.webThreadPoolSize ?? '—';
  const isHealthy    = !error && health?.healthy !== false;
  const totalReqs    = health?.totalRequests  ?? 0;
  const failedReqs   = health?.failedRequests ?? 0;
  const successRate  = totalReqs > 0
    ? ((1 - failedReqs / totalReqs) * 100).toFixed(1)
    : null;

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.webService')}</h3>

      {isLoading ? (
        <div className="text-muted-foreground text-sm">{t('common.loading')}</div>
      ) : (
        <div className="space-y-2">
          {/* Health status */}
          <div className="flex justify-between items-center">
            <span className="font-medium">{t('users.status')}:</span>
            <StatusBadge healthy={isHealthy} t={t} />
          </div>

          {/* Request counters */}
          <div className="flex justify-between">
            <span className="font-medium">{t('system.totalRequests')}:</span>
            <span>{totalReqs.toLocaleString()}</span>
          </div>
          <div className="flex justify-between">
            <span className="font-medium">{t('system.failedRequests')}:</span>
            <span>{failedReqs.toLocaleString()}</span>
          </div>
          {successRate !== null && (
            <div className="flex justify-between items-center">
              <span className="font-medium">{t('system.successRate')}:</span>
              <div className="flex items-center gap-2">
                <div className="w-20 bg-gray-200 rounded-full h-2 dark:bg-gray-700">
                  <div className="h-2 rounded-full"
                       style={{
                         width: `${successRate}%`,
                         backgroundColor: successRate >= 99
                           ? 'hsl(142 70% 45%)'
                           : successRate >= 95
                             ? 'hsl(38 92% 50%)'
                             : 'hsl(0 84% 60%)'
                       }} />
                </div>
                <span className="text-sm">{successRate}%</span>
              </div>
            </div>
          )}

          {/* Divider */}
          <div className="border-t border-border pt-2 mt-2 space-y-2">
            <div className="flex justify-between">
              <span className="font-medium">{t('system.processThreads')}:</span>
              <span>{threads}</span>
            </div>
            <div className="flex justify-between">
              <span className="font-medium">{t('system.httpThreadPool')}:</span>
              <span>{poolSize}</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

