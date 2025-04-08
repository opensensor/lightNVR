/**
 * LogsView Component
 * Displays and manages system logs
 */

import { html } from '../../../html-helper.js';

/**
 * Format log level with appropriate styling
 * 
 * @param {string} level Log level
 * @returns {JSX.Element} Formatted log level badge
 */
function formatLogLevel(level) {
  // Handle null or undefined level
  if (level === null || level === undefined) {
    return html`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">UNKNOWN</span>`;
  }
  
  // Convert to lowercase string for case-insensitive comparison
  const levelLower = String(level).toLowerCase().trim();
  
  // Match against known log levels
  if (levelLower === 'error' || levelLower === 'err') {
    return html`<span class="px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200">ERROR</span>`;
  } else if (levelLower === 'warning' || levelLower === 'warn') {
    return html`<span class="px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200">WARN</span>`;
  } else if (levelLower === 'info') {
    return html`<span class="px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">INFO</span>`;
  } else if (levelLower === 'debug' || levelLower === 'dbg') {
    return html`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">DEBUG</span>`;
  } else {
    // For any other value, display it as is (uppercase)
    const levelText = String(level).toUpperCase();
    return html`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">${levelText}</span>`;
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
  return html`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
      <div class="flex justify-between items-center mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold">System Logs</h3>
        <div class="flex space-x-2">
          <select 
            id="log-level" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value=${logLevel}
            onChange=${e => {
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
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value=${logCount}
            onChange=${e => setLogCount(parseInt(e.target.value, 10))}
          >
            <option value="50">50 lines</option>
            <option value="100">100 lines</option>
            <option value="200">200 lines</option>
            <option value="500">500 lines</option>
          </select>
          <button 
            id="refresh-logs-btn" 
            class="px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${loadLogs}
          >
            Refresh
          </button>
          <button 
            id="clear-logs-btn" 
            class="px-3 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${clearLogs}
          >
            Clear Logs
          </button>
        </div>
      </div>
      <div class="logs-container bg-gray-100 dark:bg-gray-900 rounded p-4 overflow-auto max-h-96 font-mono text-sm">
        ${logs.length === 0 ? html`
          <div class="text-gray-500 dark:text-gray-400">No logs found</div>
        ` : logs.map((log, index) => html`
          <div key=${index} class="log-entry mb-1 last:mb-0">
            <span class="text-gray-500 dark:text-gray-400">${log.timestamp}</span>
            <span class="mx-2">${formatLogLevel(log.level)}</span>
            <span class=${`log-message ${log.level === 'error' ? 'text-red-600 dark:text-red-400' : ''}`}>${log.message}</span>
          </div>
        `)}
      </div>
    </div>
  `;
}
