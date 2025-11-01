/**
 * LogsPoller Component
 * Handles polling for logs via HTTP API
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { log_level_meets_minimum } from './SystemUtils.js';
import { fetchJSON } from '../../../fetch-utils.js';

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
  const [isPolling, setIsPolling] = useState(false);
  const pollingIntervalRef = useRef(null);
  const lastTimestampRef = useRef(null);

  // Try to load the last timestamp from localStorage on initial render
  useEffect(() => {
    const savedTimestamp = localStorage.getItem('lastLogTimestamp');
    if (savedTimestamp) {
      console.log('Loaded last log timestamp from localStorage:', savedTimestamp);
      lastTimestampRef.current = savedTimestamp;
    }
  }, []);

  // Function to fetch logs via HTTP API
  const fetchLogs = async () => {
    // Only fetch if we're on the system page
    if (!document.getElementById('system-page')) {
      console.log('Not on system page, skipping log fetch');
      return;
    }

    console.log('Fetching logs via HTTP API with level: debug (to get all logs, will filter on frontend)');

    try {
      // Fetch logs from the API
      const response = await fetchJSON('/api/system/logs?level=debug', {
        timeout: 10000,
        retries: 1
      });

      if (response && response.logs && Array.isArray(response.logs)) {
        // Clean and normalize logs
        const cleanedLogs = response.logs.map(log => {
          const normalizedLog = {
            timestamp: log.timestamp || 'Unknown',
            level: log.level || 'info',
            message: log.message || ''
          };

          // Convert level to lowercase for consistency
          if (normalizedLog.level) {
            normalizedLog.level = normalizedLog.level.toLowerCase();
          }

          // Normalize 'warn' to 'warning'
          if (normalizedLog.level === 'warn') {
            normalizedLog.level = 'warning';
          }

          return normalizedLog;
        });

        // Sort logs by timestamp (newest first)
        cleanedLogs.sort((a, b) => {
          return new Date(b.timestamp) - new Date(a.timestamp);
        });

        // Update last timestamp for future reference
        if (cleanedLogs.length > 0 && cleanedLogs[0].timestamp) {
          lastTimestampRef.current = cleanedLogs[0].timestamp;
          localStorage.setItem('lastLogTimestamp', cleanedLogs[0].timestamp);
          console.log('Updated and saved last log timestamp:', cleanedLogs[0].timestamp);
        }

        // Call the callback with the logs
        console.log(`Received ${cleanedLogs.length} logs via HTTP API`);
        onLogsReceived(cleanedLogs);
      } else {
        console.log('No logs received from API');
      }
    } catch (error) {
      console.error('Error fetching logs:', error);
      // Don't throw - just log the error and continue polling
    }
  };

  // Listen for manual refresh events
  useEffect(() => {
    const handleRefreshEvent = () => {
      console.log('Received refresh-logs event, triggering fetch');
      fetchLogs();
    };

    window.addEventListener('refresh-logs', handleRefreshEvent);

    return () => {
      window.removeEventListener('refresh-logs', handleRefreshEvent);
    };
  }, []);

  // Start/stop polling when isPolling or pollingInterval changes
  useEffect(() => {
    // Clear any existing interval
    if (pollingIntervalRef.current) {
      console.log('Clearing existing polling interval');
      clearInterval(pollingIntervalRef.current);
      pollingIntervalRef.current = null;
    }

    // Start polling if enabled
    if (isPolling) {
      console.log(`Starting log polling with interval: ${pollingInterval}ms`);

      // Fetch logs immediately
      fetchLogs();

      // Set up polling interval
      pollingIntervalRef.current = setInterval(() => {
        console.log('Polling interval triggered, fetching logs...');
        fetchLogs();
      }, pollingInterval);
    }

    // Clean up on unmount or when dependencies change
    return () => {
      if (pollingIntervalRef.current) {
        console.log('Cleaning up polling interval');
        clearInterval(pollingIntervalRef.current);
        pollingIntervalRef.current = null;
      }
    };
  }, [isPolling, pollingInterval]);

  // Start polling when component mounts and update when log level or count changes
  useEffect(() => {
    console.log(`LogsPoller: Setting up polling with log level ${logLevel}, count ${logCount}`);
    setIsPolling(false); // Stop any existing polling

    // Small delay to ensure any previous polling is cleaned up
    const timeoutId = setTimeout(() => {
      setIsPolling(true); // Start polling with new parameters
    }, 100);

    // Clean up on unmount
    return () => {
      console.log('LogsPoller: Cleaning up on unmount');
      clearTimeout(timeoutId);
      setIsPolling(false);
    };
  }, [logLevel, logCount]);

  // This component doesn't render anything visible
  return null;
}
