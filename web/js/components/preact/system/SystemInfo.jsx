/**
 * SystemInfo Component
 * Displays basic system information like version, uptime, and CPU details
 */

import { useI18n } from '../../../i18n.js';

/**
 * SystemInfo component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatUptime Function to format uptime
 * @returns {JSX.Element} SystemInfo component
 */
export function SystemInfo({ systemInfo, formatUptime }) {
  const { t } = useI18n();

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.systemInformation')}</h3>
      <div className="space-y-2">
        <div className="flex justify-between">
          <span className="font-medium">{t('system.version')}:</span>
          <span>{systemInfo.version || t('common.unknown')}{systemInfo.git_commit ? `-${systemInfo.git_commit}` : ''}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">{t('system.uptime')}:</span>
          <span>{systemInfo.uptime ? formatUptime(systemInfo.uptime) : t('common.unknown')}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">{t('system.cpuModel')}:</span>
          <span>{systemInfo.cpu?.model || t('common.unknown')}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">{t('system.cpuCores')}:</span>
          <span>{systemInfo.cpu?.cores || t('common.unknown')}</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="font-medium">{t('system.cpuUsage')}:</span>
          <div className="w-32 bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div
              className="h-2.5 rounded-full"
              style={{ backgroundColor: 'hsl(var(--primary))', width: `${systemInfo.cpu?.usage || 0}%` }}
            ></div>
          </div>
          <span>{systemInfo.cpu?.usage ? `${systemInfo.cpu.usage.toFixed(1)}%` : t('common.unknown')}</span>
        </div>
      </div>
    </div>
  );
}
