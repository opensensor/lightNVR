/**
 * LightNVR Web Interface SystemView Component
 * Preact component for the system page
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import { useQuery, useMutation, fetchJSON } from '../../query-client.js';
import { validateSession } from '../../utils/auth-utils.js';

// Import system components
import { SystemControls } from './system/SystemControls.jsx';
import { SystemInfo } from './system/SystemInfo.jsx';
import { MemoryStorage } from './system/MemoryStorage.jsx';
import { StreamStorage } from './system/StreamStorage.jsx';
import { StorageHealth } from './system/StorageHealth.jsx';
import { NetworkInfo } from './system/NetworkInfo.jsx';
import { StreamsInfo } from './system/StreamsInfo.jsx';
import { LogsView } from './system/LogsView.jsx';
import { LogsPoller } from './system/LogsPoller.jsx';

// Import utility functions
import { formatBytes, formatUptime, log_level_meets_minimum } from './system/SystemUtils.js';

/**
 * SystemView component
 * @returns {JSX.Element} SystemView component
 */
export function SystemView() {
  // Define all state variables first
  const [systemInfo, setSystemInfo] = useState({
    version: '',
    uptime: '',
    cpu: {
      model: '',
      cores: 0,
      usage: 0
    },
    memory: {
      total: 0,
      used: 0,
      free: 0
    },
    go2rtcMemory: {
      total: 0,
      used: 0,
      free: 0
    },
    detectorMemory: {
      total: 0,
      used: 0,
      free: 0
    },
    systemMemory: {
      total: 0,
      used: 0,
      free: 0
    },
    disk: {
      total: 0,
      used: 0,
      free: 0
    },
    systemDisk: {
      total: 0,
      used: 0,
      free: 0
    },
    network: {
      interfaces: []
    },
    streams: {
      active: 0,
      total: 0
    },
    recordings: {
      count: 0,
      size: 0
    }
  });
  const [logs, setLogs] = useState([]);
  const [logLevel, setLogLevel] = useState('debug');
  const logLevelRef = useRef('debug');
  const [logCount, setLogCount] = useState(100);
  const [pollingInterval, setPollingInterval] = useState(5000); // Default to 5 seconds
  const [isRestarting, setIsRestarting] = useState(false);
  const [hasData, setHasData] = useState(false);

  // User role state for permission-based UI
  const [userRole, setUserRole] = useState(null);

  // Fetch user role on mount
  useEffect(() => {
    const fetchUserRole = async () => {
      try {
        const result = await validateSession();
        if (result.valid && result.role) {
          setUserRole(result.role);
        } else {
          setUserRole('');
        }
      } catch (error) {
        console.error('Error fetching user role:', error);
        setUserRole('');
      }
    };
    fetchUserRole();
  }, []);

  // Role is still loading if null
  const roleLoading = userRole === null;
  // Only admin can restart/shutdown system; default to false while loading
  const canControlSystem = !roleLoading && userRole === 'admin';

  // Define all query hooks next
  const {
    data: systemInfoData,
    isLoading
  } = useQuery(
    ['systemInfo'],
    '/api/system/info',
    {
      timeout: 15000,
      retries: 2,
      retryDelay: 1000
    }
  );

  // Define all mutation hooks next
  const clearLogsMutation = useMutation({
    mutationKey: ['clearLogs'],
    mutationFn: async () => {
      return await fetchJSON('/api/system/logs/clear', {
        method: 'POST',
        timeout: 10000,
        retries: 1
      });
    },
    onSuccess: () => {
      showStatusMessage('Logs cleared successfully');
      setLogs([]);
    },
    onError: (error) => {
      console.error('Error clearing logs:', error);
      showStatusMessage(`Error clearing logs: ${error.message}`);
    }
  });

  // Then define all handler functions
  const handleSetLogLevel = (newLevel) => {
    console.log(`SystemView: Setting log level from ${logLevel} to ${newLevel}`);
    setLogLevel(newLevel);
    logLevelRef.current = newLevel;
  };

  const handleLogsReceived = (newLogs) => {
    console.log('SystemView received new logs:', newLogs.length);
    const currentLogLevel = logLevelRef.current;
    const filteredLogs = newLogs.filter(log => log_level_meets_minimum(log.level, currentLogLevel));
    setLogs(filteredLogs);
  };

  // Update hasData based on systemInfoData
  useEffect(() => {
    if (systemInfoData) {
      setHasData(true);
    }
  }, [systemInfoData]);

  // Restart system mutation
  const restartSystemMutation = useMutation({
    mutationFn: async () => {
      return await fetchJSON('/api/system/restart', {
        method: 'POST',
        timeout: 30000, // 30 second timeout for system restart
        retries: 0      // No retries for system restart
      });
    },
    onMutate: () => {
      setIsRestarting(true);
      showStatusMessage('Restarting system...');
    },
    onSuccess: () => {
      showStatusMessage('System is restarting. Please wait...');
      // Don't auto-reload here - let the RestartModal handle reconnection detection
    },
    onError: (error) => {
      console.error('Error restarting system:', error);
      // Only show error and reset state if it's not a network error
      // (network errors are expected during restart)
      if (error.message && !error.message.includes('fetch') && !error.message.includes('network')) {
        showStatusMessage(`Error restarting system: ${error.message}`);
        setIsRestarting(false);
      }
      // Otherwise, the restart was likely initiated successfully and the server is shutting down
    }
  });

  // Update systemInfo state when data is loaded
  useEffect(() => {
    if (systemInfoData) {
      setSystemInfo(systemInfoData);
    }
  }, [systemInfoData]);

  // Component cleanup on unmount
  useEffect(() => {
    return () => {
      console.log('SystemView component unmounting');
    };
  }, []);

  // Clear logs function
  const clearLogs = () => {
    if (!confirm('Are you sure you want to clear all logs?')) {
      return;
    }

    clearLogsMutation.mutate();
  };

  // Restart system function (called when user confirms in modal)
  const handleRestartConfirm = () => {
    restartSystemMutation.mutate();
  };

  // Component initialization
  useEffect(() => {
    console.log('SystemView component initialized');
  }, []);

  return (
    <section id="system-page" className="page">
      <SystemControls
        onRestartConfirm={handleRestartConfirm}
        isRestarting={isRestarting}
        canControlSystem={canControlSystem}
      />

      <ContentLoader
        isLoading={isLoading}
        hasData={hasData}
        loadingMessage="Loading system information..."
        emptyMessage="System information not available. Please try again later."
      >
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <SystemInfo systemInfo={systemInfo} formatUptime={formatUptime} />
          <MemoryStorage systemInfo={systemInfo} formatBytes={formatBytes} />
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
          <div className="md:col-span-2">
            <StreamStorage systemInfo={systemInfo} formatBytes={formatBytes} />
          </div>
          <div className="md:col-span-1">
            <StorageHealth formatBytes={formatBytes} />
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <NetworkInfo systemInfo={systemInfo} />
          <StreamsInfo systemInfo={systemInfo} formatBytes={formatBytes} />
        </div>

        <LogsView
          logs={logs}
          logLevel={logLevel}
          logCount={logCount}
          pollingInterval={pollingInterval}
          setLogLevel={handleSetLogLevel}
          setLogCount={setLogCount}
          setPollingInterval={setPollingInterval}
          loadLogs={() => {
            // Trigger a manual log refresh
            console.log('Manually triggering log refresh');
            const event = new CustomEvent('refresh-logs');
            window.dispatchEvent(event);
          }}
          clearLogs={clearLogs}
        />

        <LogsPoller
          logLevel={logLevel}
          logCount={logCount}
          pollingInterval={pollingInterval}
          onLogsReceived={handleLogsReceived}
        />
      </ContentLoader>
    </section>
  );
}
