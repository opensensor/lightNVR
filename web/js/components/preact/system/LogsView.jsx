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
 * @param {number} props.pollingInterval Polling interval in milliseconds
 * @param {Function} props.setLogLevel Function to set log level
 * @param {Function} props.setLogCount Function to set log count
 * @param {Function} props.setPollingInterval Function to set polling interval
 * @param {Function} props.loadLogs Function to load logs
 * @param {Function} props.clearLogs Function to clear logs
 * @returns {JSX.Element} LogsView component
 */
export function LogsView({ logs, logLevel, logCount, pollingInterval, setLogLevel, setLogCount, setPollingInterval, loadLogs, clearLogs }) {
  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4 mb-4">
      <div className="flex justify-between items-center mb-4 pb-2 border-b border-border">
        <h3 className="text-lg font-semibold">System Logs</h3>
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
            <option value="error">Error</option>
            <option value="warning">Warning</option>
            <option value="info">Info</option>
            <option value="debug">Debug</option>
          </select>
          <select
            id="log-count"
            className="px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
            value={logCount}
            onChange={e => setLogCount(parseInt(e.target.value, 10))}
          >
            <option value="50">50 lines</option>
            <option value="100">100 lines</option>
            <option value="200">200 lines</option>
            <option value="500">500 lines</option>
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
            <option value="1000">1 sec</option>
            <option value="3000">3 sec</option>
            <option value="5000">5 sec</option>
            <option value="10000">10 sec</option>
            <option value="30000">30 sec</option>
          </select>
          <button
            id="refresh-logs-btn"
            className="btn-primary focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick={loadLogs}
          >
            Refresh
          </button>
          <button
            id="clear-logs-btn"
            className="btn-danger focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick={clearLogs}
          >
            Clear Logs
          </button>
        </div>
      </div>
      <div className="logs-container bg-muted/30 rounded p-4 overflow-auto max-h-96 font-mono text-sm">
        {logs.length === 0 ? (
          <div className="text-muted-foreground">No logs found</div>
        ) : (
          logs.map((log, index) => (
            <div key={index} className="log-entry mb-1 last:mb-0">
              <span className="text-muted-foreground">{log.timestamp}</span>
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
