/**
 * LogsView Component
 * Displays and manages system logs
 */

/**
 * Format log level with appropriate styling
 *
 * @param {string} level Log level
 * @returns {JSX.Element} Formatted log level badge
 */
function formatLogLevel(level) {
  // Handle null or undefined level
  if (level === null || level === undefined) {
    return (
      <span className="badge-muted">
        UNKNOWN
      </span>
    );
  }

  // Convert to lowercase string for case-insensitive comparison
  const levelLower = String(level).toLowerCase().trim();

  // Match against known log levels
  if (levelLower === 'error' || levelLower === 'err') {
    return (
      <span className="badge-danger">
        ERROR
      </span>
    );
  } else if (levelLower === 'warning' || levelLower === 'warn') {
    return (
      <span className="badge-warning">
        WARN
      </span>
    );
  } else if (levelLower === 'info') {
    return (
      <span className="badge-info">
        INFO
      </span>
    );
  } else if (levelLower === 'debug' || levelLower === 'dbg') {
    return (
      <span className="badge-muted">
        DEBUG
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
 * @param {Function} props.setLogLevel Function to set log level
 * @param {Function} props.setLogCount Function to set log count
 * @param {Function} props.loadLogs Function to load logs
 * @param {Function} props.clearLogs Function to clear logs
 * @returns {JSX.Element} LogsView component
 */
export function LogsView({ logs, logLevel, logCount, setLogLevel, setLogCount, loadLogs, clearLogs }) {
  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
      <div className="flex justify-between items-center mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-semibold">System Logs</h3>
        <div className="flex space-x-2">
          <select
            id="log-level"
            className="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value={logLevel}
            onChange={e => {
              const newLevel = e.target.value;
              console.log(`LogsView: Log level changed from ${logLevel} to ${newLevel}`);
              setLogLevel(newLevel);
            }}
          >
            <option value="error">Error</option>
            <option value="warning">Warning</option>
            <option value="info">Info</option>
            <option value="debug">Debug</option>
          </select>
          <select
            id="log-count"
            className="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value={logCount}
            onChange={e => setLogCount(parseInt(e.target.value, 10))}
          >
            <option value="50">50 lines</option>
            <option value="100">100 lines</option>
            <option value="200">200 lines</option>
            <option value="500">500 lines</option>
          </select>
          <button
            id="refresh-logs-btn"
            className="px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick={loadLogs}
          >
            Refresh
          </button>
          <button
            id="clear-logs-btn"
            className="px-3 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick={clearLogs}
          >
            Clear Logs
          </button>
        </div>
      </div>
      <div className="logs-container bg-gray-100 dark:bg-gray-900 rounded p-4 overflow-auto max-h-96 font-mono text-sm">
        {logs.length === 0 ? (
          <div className="text-gray-500 dark:text-gray-400">No logs found</div>
        ) : (
          logs.map((log, index) => (
            <div key={index} className="log-entry mb-1 last:mb-0">
              <span className="text-gray-500 dark:text-gray-400">{log.timestamp}</span>
              <span className="mx-2">{formatLogLevel(log.level)}</span>
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
