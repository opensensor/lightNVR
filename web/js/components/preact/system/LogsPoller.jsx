/**
 * LogsPoller Component
 * Handles polling for logs via HTTP API
 */

import { useEffect, useRef, useCallback } from 'preact/hooks';
import { fetchJSON } from '../../../fetch-utils.js';
import { log_level_meets_minimum } from './SystemUtils.js';

/**
 * LogsPoller component
 * @param {Object} props Component props
 * @param {string} props.logLevel Current log level filter
 * @param {number} props.logCount Number of logs to display
 * @param {number} props.pollingInterval Polling interval in milliseconds
 * @param {Function} props.onLogsReceived Callback function when logs are received
 * @returns {JSX.Element} LogsPoller component (invisible)
 */
export function LogsPoller({ logLevel, logCount, pollingInterval = 5000, onLogsReceived }) {
  const lastTimestampRef = useRef(null);

  // Keep refs to the latest prop values so fetchLogs never needs to be recreated.
  // Assigning directly in render (not in an effect) ensures they are always current
  // before the next fetch fires.
  const onLogsReceivedRef = useRef(onLogsReceived);
  const logLevelRef = useRef(logLevel);
  onLogsReceivedRef.current = onLogsReceived;
  logLevelRef.current = logLevel;

  // Load saved timestamp from localStorage on mount
  useEffect(() => {
    const savedTimestamp = localStorage.getItem('lastLogTimestamp');
    if (savedTimestamp) {
      console.log('Loaded last log timestamp from localStorage:', savedTimestamp);
      lastTimestampRef.current = savedTimestamp;
    }
  }, []);

  // Stable fetch function — created once, reads latest values via refs.
  // No dependency on logLevel or onLogsReceived, so it never recreates and
  // never causes the polling interval to restart unexpectedly.
  const fetchLogs = useCallback(async () => {
    if (!document.getElementById('system-page')) {
      console.log('Not on system page, skipping log fetch');
      return;
    }

    console.log('Fetching logs via HTTP API with level=debug (debug and above); additional filtering will be applied on the frontend');

    try {
      const response = await fetchJSON('/api/system/logs?level=debug', {
        timeout: 10000,
        retries: 1
      });

      if (response && response.logs && Array.isArray(response.logs)) {
        // Clean and normalize logs
        const cleanedLogs = response.logs.map(log => {
          const normalizedLog = {
            timestamp: log.timestamp || 'Unknown',
            level: String(log.level || 'info').toLowerCase(),
            message: log.message || ''
          };
          // Normalize 'warn' to 'warning'
          if (normalizedLog.level === 'warn') {
            normalizedLog.level = 'warning';
          }
          return normalizedLog;
        });

        // Sort logs by timestamp (newest first)
        cleanedLogs.sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp));

        // Update last timestamp for future reference
        if (cleanedLogs.length > 0 && cleanedLogs[0].timestamp) {
          lastTimestampRef.current = cleanedLogs[0].timestamp;
          localStorage.setItem('lastLogTimestamp', cleanedLogs[0].timestamp);
          console.log('Updated and saved last log timestamp:', cleanedLogs[0].timestamp);
        }

        // Filter using the latest logLevel from the ref (hierarchical: 'debug' = all)
        const currentLevel = logLevelRef.current;
        let filteredLogs = cleanedLogs;
        if (currentLevel) {
          const normalizedLevel = String(currentLevel).toLowerCase();
          filteredLogs = cleanedLogs.filter(log => log_level_meets_minimum(log.level, normalizedLevel));
        }

        console.log(`Received ${filteredLogs.length} logs via HTTP API after filtering`);
        onLogsReceivedRef.current(filteredLogs);
      } else {
        console.log('No logs received from API');
      }
    } catch (error) {
      console.error('Error fetching logs:', error);
    }
  }, []); // stable — refs keep it up-to-date without recreation

  // Listen for manual refresh events
  useEffect(() => {
    const handleRefreshEvent = () => {
      console.log('Received refresh-logs event, triggering fetch');
      fetchLogs();
    };
    window.addEventListener('refresh-logs', handleRefreshEvent);
    return () => window.removeEventListener('refresh-logs', handleRefreshEvent);
  }, [fetchLogs]);

  // Start polling and restart ONLY when pollingInterval changes.
  // logLevel / logCount changes do NOT restart the interval — the next scheduled
  // fetch will automatically use the latest values via refs.
  useEffect(() => {
    console.log(`LogsPoller: Starting polling with interval: ${pollingInterval}ms`);

    // Fetch immediately on mount or when interval time changes
    fetchLogs();

    const intervalId = setInterval(() => {
      console.log('Polling interval triggered, fetching logs...');
      fetchLogs();
    }, pollingInterval);

    return () => {
      console.log('LogsPoller: Cleaning up polling interval');
      clearInterval(intervalId);
    };
  }, [pollingInterval, fetchLogs]); // fetchLogs is stable; pollingInterval is the only real trigger

  // This component doesn't render anything visible
  return null;
}
