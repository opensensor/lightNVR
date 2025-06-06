/**
 * LightNVR Web Interface SystemView Component
 * Preact component for the system page
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import { useQuery, useMutation, fetchJSON } from '../../query-client.js';
import { WebSocketClient } from '../../websocket-client.js';

// Import system components
import { SystemControls } from './system/SystemControls.jsx';
import { SystemInfo } from './system/SystemInfo.jsx';
import { MemoryStorage } from './system/MemoryStorage.jsx';
import { StreamStorage } from './system/StreamStorage.jsx';
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
  const [isRestarting, setIsRestarting] = useState(false);
  const [isShuttingDown, setIsShuttingDown] = useState(false);
  const [hasData, setHasData] = useState(false);

  // Define all query hooks next
  const {
    data: systemInfoData,
    isLoading,
    error: systemInfoError,
    refetch: refetchSystemInfo
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
      // Wait for system to restart
      setTimeout(() => {
        window.location.reload();
      }, 10000);
    },
    onError: (error) => {
      console.error('Error restarting system:', error);
      showStatusMessage(`Error restarting system: ${error.message}`);
      setIsRestarting(false);
    }
  });

  // Shutdown system mutation
  const shutdownSystemMutation = useMutation({
    mutationFn: async () => {
      return await fetchJSON('/api/system/shutdown', {
        method: 'POST',
        timeout: 30000, // 30 second timeout for system shutdown
        retries: 0      // No retries for system shutdown
      });
    },
    onMutate: () => {
      setIsShuttingDown(true);
      showStatusMessage('Shutting down system...');
    },
    onSuccess: () => {
      showStatusMessage('System is shutting down. You will need to manually restart it.');
    },
    onError: (error) => {
      console.error('Error shutting down system:', error);
      showStatusMessage(`Error shutting down system: ${error.message}`);
      setIsShuttingDown(false);
    }
  });

  // Update systemInfo state when data is loaded
  useEffect(() => {
    if (systemInfoData) {
      setSystemInfo(systemInfoData);
    }
  }, [systemInfoData]);

  // Clean up WebSocket subscriptions on unmount
  useEffect(() => {
    return () => {
      if (window.wsClient && typeof window.wsClient.unsubscribe === 'function') {
        console.log('Cleaning up any WebSocket subscriptions on unmount');
        window.wsClient.unsubscribe('system/logs');
      }
    };
  }, []);

  // Clear logs function
  const clearLogs = () => {
    if (!confirm('Are you sure you want to clear all logs?')) {
      return;
    }

    clearLogsMutation.mutate();
  };

  // Restart system function
  const restartSystem = () => {
    if (!confirm('Are you sure you want to restart the system?')) {
      return;
    }

    restartSystemMutation.mutate();
  };

  // Shutdown system function
  const shutdownSystem = () => {
    if (!confirm('Are you sure you want to shut down the system?')) {
      return;
    }

    shutdownSystemMutation.mutate();
  };

  // Initialize WebSocket client
  useEffect(() => {
    // Initialize WebSocket client at the parent level
    // This ensures a single WebSocket connection is shared across all components
    if (typeof WebSocketClient !== 'undefined') {
      // Create a global WebSocket client instance
      window.wsClient = new WebSocketClient();
      console.log('WebSocket client initialized at application level');

      // Add additional event listeners for debugging
      if (window.wsClient) {
        // Log initial connection state
        console.log('Initial WebSocket connection state:', {
          connected: window.wsClient.isConnected(),
          clientId: window.wsClient.getClientId()
        });

        // Add socket event listeners when socket is created
        const originalConnect = window.wsClient.connect;
        window.wsClient.connect = function() {
          const result = originalConnect.apply(this, arguments);

          // Add event listeners to the new socket
          if (this.socket) {
            const originalOnOpen = this.socket.onopen;
            this.socket.onopen = (event) => {
              console.log('WebSocket connection opened at application level');
              if (originalOnOpen) originalOnOpen.call(this, event);
            };

            const originalOnError = this.socket.onerror;
            this.socket.onerror = (error) => {
              console.error('WebSocket error at application level:', error);
              if (originalOnError) originalOnError.call(this, error);
            };

            const originalOnClose = this.socket.onclose;
            this.socket.onclose = (event) => {
              console.log(`WebSocket connection closed at application level: ${event.code} ${event.reason}`);
              if (originalOnClose) originalOnClose.call(this, event);
            };

            const originalOnMessage = this.socket.onmessage;
            this.socket.onmessage = (event) => {
              // Only log non-welcome messages at application level to reduce noise
              if (!event.data.includes('"type":"welcome"')) {
                console.log('WebSocket message received at application level');
              }
              if (originalOnMessage) originalOnMessage.call(this, event);
            };
          }

          return result;
        };

        // Override handleMessage to log when client ID is set
        const originalHandleMessage = window.wsClient.handleMessage;
        window.wsClient.handleMessage = function(data) {
          const clientIdBefore = this.clientId;
          originalHandleMessage.call(this, data);
          const clientIdAfter = this.clientId;

          // Log when client ID changes
          if (clientIdBefore !== clientIdAfter && clientIdAfter) {
            console.log(`WebSocket client ID changed at application level: ${clientIdAfter}`);
          }
        };
      }
    } else {
      console.log('WebSocketClient is not defined, cannot initialize WebSocket client');
    }
  }, []);

  return (
    <section id="system-page" className="page">
      <SystemControls
        restartSystem={restartSystem}
        shutdownSystem={shutdownSystem}
        isRestarting={isRestarting}
        isShuttingDown={isShuttingDown}
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

        <div className="grid grid-cols-1 gap-4 mb-4">
          <StreamStorage systemInfo={systemInfo} formatBytes={formatBytes} />
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <NetworkInfo systemInfo={systemInfo} />
          <StreamsInfo systemInfo={systemInfo} formatBytes={formatBytes} />
        </div>

        <LogsView
          logs={logs}
          logLevel={logLevel}
          logCount={logCount}
          setLogLevel={handleSetLogLevel}
          setLogCount={setLogCount}
          loadLogs={() => {
            // If using WebSocket, trigger a WebSocket fetch
            if (window.wsClient && window.wsClient.isConnected()) {
              console.log('Manually triggering WebSocket fetch for logs');
              // Find the LogsPoller component and trigger a fetch
              const event = new CustomEvent('refresh-logs-websocket');
              window.dispatchEvent(event);
            }
          }}
          clearLogs={clearLogs}
        />

        <LogsPoller
          logLevel={logLevel}
          logCount={logCount}
          onLogsReceived={handleLogsReceived}
        />
      </ContentLoader>
    </section>
  );
}
