/**
 * StreamsInfo Component
 * Displays information about streams and recordings
 */

import { useI18n } from '../../../i18n.js';

/**
 * StreamsInfo component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element} StreamsInfo component
 */
export function StreamsInfo({ systemInfo, formatBytes }) {
  const { t } = useI18n();

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('system.streamsAndRecordings')}</h3>
      <div className="space-y-2">
        <div className="flex justify-between">
          <span className="font-medium">{t('system.activeStreams')}:</span>
          <span>{systemInfo.streams?.active || 0} / {systemInfo.streams?.total || 0}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">{t('nav.recordings')}:</span>
          <span>{systemInfo.recordings?.count || 0}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">{t('system.recordingsSize')}:</span>
          <span>{systemInfo.recordings?.size ? formatBytes(systemInfo.recordings.size) : '0'}</span>
        </div>
      </div>
    </div>
  );
}
