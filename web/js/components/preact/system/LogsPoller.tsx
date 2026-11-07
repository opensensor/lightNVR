import {getSystemLogs} from '../../../api/system.ts';
/**
 * LogsPoller Component
 * Handles polling for logs via HTTP API
 */

import { useEffect, useRef, useCallback } from 'preact/hooks';
import { log_level_meets_minimum } from './SystemUtils.ts';
import { toEpochMilliseconds } from '../../../utils/date-utils.ts';
import type { SystemLogEntry } from '../../../types/system.ts';

export interface LogsPollerProps {
  logLevel: string;
  /** Parsed with parseInt on every render, so a string also works. */
  logCount: number | string;
  pollingInterval?: number;
  onLogsReceived: (logs: SystemLogEntry[]) => void;
}

/**
 * LogsPoller component
 * @param {Object} props Component props
 * @param {string} props.logLevel Current log level filter
 * @param {number} props.logCount Number of logs to display
 * @param {number} props.pollingInterval Polling interval in milliseconds
 * @param {Function} props.onLogsReceived Callback function when logs are received
 * @returns {JSX.Element} LogsPoller component (invisible)
 */
export function LogsPoller({ logLevel, logCount, pollingInterval = 5000, onLogsReceived }: LogsPollerProps) {
  const lastTimestampRef = useRef<string | null>(null);

  // Keep refs to the latest prop values so fetchLogs never needs to be recreated.
  // Assigning directly in render (not in an effect) ensures they are always current
  // before the next fetch fires.
  const onLogsReceivedRef = useRef(onLogsReceived);
  const logLevelRef = useRef(logLevel);
  const logCountRef = useRef<number>(logCount as number);
  onLogsReceivedRef.current = onLogsReceived;
  logLevelRef.current = String(logLevel).toLowerCase();
  logCountRef.current = parseInt(logCount as string);

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

    const normalizedLevel = logLevelRef.current;
    const logCount = logCountRef.current;

    console.log(`Fetching logs via HTTP API with level=${normalizedLevel} (${normalizedLevel} and above); additional filtering will be applied on the frontend`);

    try {
      const response = await getSystemLogs({level:normalizedLevel,count:logCount},{timeout:10000});

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
        cleanedLogs.sort((a, b) => toEpochMilliseconds(b.timestamp) - toEpochMilliseconds(a.timestamp));

        // Update last timestamp for future reference
        if (cleanedLogs.length > 0 && cleanedLogs[0]!.timestamp) {
          lastTimestampRef.current = cleanedLogs[0]!.timestamp;
          localStorage.setItem('lastLogTimestamp', cleanedLogs[0]!.timestamp);
          console.log('Updated and saved last log timestamp:', cleanedLogs[0]!.timestamp);
        }

        // Filter using the latest logLevel from the ref (hierarchical: 'debug' = all)
        let filteredLogs = cleanedLogs;
        if (normalizedLevel) {
          filteredLogs = cleanedLogs.filter(log => log_level_meets_minimum(log.level, normalizedLevel));
        }

        console.log(`Received ${filteredLogs.length} logs via HTTP API after filtering`);
        if (filteredLogs.length > logCount) {
          // Shrink to requested size if we received more than expected
          filteredLogs = filteredLogs.slice(0, logCount);
        }
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
