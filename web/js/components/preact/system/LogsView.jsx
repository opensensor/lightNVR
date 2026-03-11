/**
 * LogsView Component
 * Displays and manages system logs
 */

import { useI18n } from '../../../i18n.js';

/**
 * Format log level with appropriate styling
 *
 * @param {string} level Log level
 * @returns {JSX.Element} Formatted log level badge
 */
function formatLogLevel(level, t) {
  // Handle null or undefined level
  if (level === null || level === undefined) {
    return (
      <span className="badge-muted">
        {t('common.unknown').toUpperCase()}
      </span>
    );
  }

  // Convert to lowercase string for case-insensitive comparison
  const levelLower = String(level).toLowerCase().trim();

  // Match against known log levels
  if (levelLower === 'error' || levelLower === 'err') {
    return (
      <span className="badge-danger">
        {t('system.logLevel.error').toUpperCase()}
      </span>
    );
  } else if (levelLower === 'warning' || levelLower === 'warn') {
    return (
      <span className="badge-warning">
        {t('system.logLevel.warningShort').toUpperCase()}
      </span>
    );
  } else if (levelLower === 'info') {
    return (
      <span className="badge-info">
        {t('system.logLevel.info').toUpperCase()}
      </span>
    );
  } else if (levelLower === 'debug' || levelLower === 'dbg') {
    return (
      <span className="badge-muted">
        {t('system.logLevel.debug').toUpperCase()}
      </span>
    );
  } else {
    // For any other value, display it as is (uppercase)
    const levelText = String(level).toUpperCase();
    return (
      <span className="badge-muted">
        {levelText}
      </span>
    );
  }
}

/**
 * LogsView component
 * @param {Object} props Component props
 * @param {Array} props.logs Array of log entries
 * @param {string} props.logLevel Current log level filter
 * @param {number} props.logCount Number of logs to display
 * @param {number} props.pollingInterval Polling interval in milliseconds
 * @param {Function} props.setLogLevel Function to set log level
 * @param {Function} props.setLogCount Function to set log count
 * @param {Function} props.setPollingInterval Function to set polling interval
 * @param {Function} props.loadLogs Function to load logs
 * @param {Function} props.clearLogs Function to clear logs
 * @returns {JSX.Element} LogsView component
 */
export function LogsView({ logs, logLevel, logCount, pollingInterval, setLogLevel, setLogCount, setPollingInterval, loadLogs, clearLogs }) {
  const { t } = useI18n();

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4 mb-4">
      <div className="flex justify-between items-center mb-4 pb-2 border-b border-border">
        <h3 className="text-lg font-semibold">{t('system.systemLogs')}</h3>
        <div className="flex space-x-2">
          <select
            id="log-level"
            className="px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
            value={logLevel}
            onChange={e => {
              const newLevel = e.target.value;
              console.log(`LogsView: Log level changed from ${logLevel} to ${newLevel}`);
              setLogLevel(newLevel);
            }}
          >
            <option value="error">{t('system.logLevel.error')}</option>
            <option value="warning">{t('system.logLevel.warning')}</option>
            <option value="info">{t('system.logLevel.info')}</option>
            <option value="debug">{t('system.logLevel.debug')}</option>
          </select>
          <select
            id="log-count"
            className="px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
            value={logCount}
            onChange={e => setLogCount(parseInt(e.target.value, 10))}
          >
            <option value="50">{t('system.logLines', { count: 50 })}</option>
            <option value="100">{t('system.logLines', { count: 100 })}</option>
            <option value="200">{t('system.logLines', { count: 200 })}</option>
            <option value="500">{t('system.logLines', { count: 500 })}</option>
          </select>
          <select
            id="polling-interval"
            className="px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
            value={pollingInterval}
            onChange={e => {
              const newInterval = parseInt(e.target.value, 10);
              console.log(`LogsView: Polling interval changed to ${newInterval}ms`);
              setPollingInterval(newInterval);
            }}
          >
            <option value="1000">{t('system.secondsShort', { count: 1 })}</option>
            <option value="3000">{t('system.secondsShort', { count: 3 })}</option>
            <option value="5000">{t('system.secondsShort', { count: 5 })}</option>
            <option value="10000">{t('system.secondsShort', { count: 10 })}</option>
            <option value="30000">{t('system.secondsShort', { count: 30 })}</option>
          </select>
          <button
            id="refresh-logs-btn"
            className="btn-primary focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick={loadLogs}
          >
            {t('common.refresh')}
          </button>
          <button
            id="clear-logs-btn"
            className="btn-danger focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick={clearLogs}
          >
            {t('system.clearLogs')}
          </button>
        </div>
      </div>
      <div className="logs-container bg-muted/30 rounded p-4 overflow-auto max-h-96 font-mono text-sm">
        {logs.length === 0 ? (
          <div className="text-muted-foreground">{t('system.noLogsFound')}</div>
        ) : (
          logs.map((log, index) => (
            <div key={index} className="log-entry mb-1 last:mb-0">
              <span className="text-muted-foreground">{log.timestamp}</span>
              <span className="mx-2">{formatLogLevel(log.level, t)}</span>
              <span className={`log-message ${log.level === 'error' ? 'text-red-600 dark:text-red-400' : ''}`}>
                {log.message}
              </span>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
