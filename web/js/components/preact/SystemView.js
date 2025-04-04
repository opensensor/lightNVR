/**
 * LightNVR Web Interface SystemView Component
 * Preact component for the system page
 */

import { h } from '../../preact.min.js';
import { html } from '../../html-helper.js';
import { useState, useEffect, useRef } from '../../preact.hooks.module.js';
import { showStatusMessage } from './UI.js';
import { ContentLoader } from './LoadingIndicator.js';
import { fetchJSON, enhancedFetch, createRequestController } from '../../fetch-utils.js';

// Import system components
import { SystemControls } from './system/SystemControls.js';
import { SystemInfo } from './system/SystemInfo.js';
import { MemoryStorage } from './system/MemoryStorage.js';
import { StreamStorage } from './system/StreamStorage.js';
import { NetworkInfo } from './system/NetworkInfo.js';
import { StreamsInfo } from './system/StreamsInfo.js';
import { LogsView } from './system/LogsView.js';
import { LogsPoller } from './system/LogsPoller.js';

// Import utility functions
import { formatBytes, formatUptime, log_level_meets_minimum } from './system/SystemUtils.js';

/**
 * SystemView component
 * @returns {JSX.Element} SystemView component
 */
export function SystemView() {
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
  
  // Wrap setLogLevel to add logging
  const handleSetLogLevel = (newLevel) => {
    console.log(`SystemView: Setting log level from ${logLevel} to ${newLevel}`);
    console.log('Current stack trace:', new Error().stack);
    
    // Update both the state and the ref
    setLogLevel(newLevel);
    logLevelRef.current = newLevel;
    
    console.log(`SystemView: logLevelRef is now: ${logLevelRef.current}`);
    
    // Verify the state was updated
    setTimeout(() => {
      console.log(`SystemView: After setState, logLevel is now: ${logLevel}`);
      console.log(`SystemView: After setState, logLevelRef is now: ${logLevelRef.current}`);
    }, 0);
  };
  const [logCount, setLogCount] = useState(100);
  const [isRestarting, setIsRestarting] = useState(false);
  const [isShuttingDown, setIsShuttingDown] = useState(false);
  
  // State for loading and data status
  const [isLoading, setIsLoading] = useState(true);
  const [hasData, setHasData] = useState(false);

  // Handler for when logs are received from the LogsPoller
  const handleLogsReceived = (newLogs) => {
    console.log('SystemView received new logs:', newLogs.length);
    
    // Filter logs based on the current log level
    const currentLogLevel = logLevelRef.current;
    console.log(`Filtering ${newLogs.length} logs based on log level: ${currentLogLevel}`);
    
    const filteredLogs = newLogs.filter(log => {
      // Ensure debug logs from WebSocket are included when debug is selected
      const result = log_level_meets_minimum(log.level, currentLogLevel);
      
      // Add debug logging to help diagnose filtering issues
      if (log.level === 'debug' && currentLogLevel === 'debug' && !result) {
        console.warn('Debug log filtered out despite debug level selected:', log);
      }
      
      return result;
    });
    
    console.log(`After filtering: ${filteredLogs.length} logs match the current log level`);
    
    // Update the logs state with the filtered logs
    setLogs(filteredLogs);
    
    // Force a re-render to ensure the UI updates
    setTimeout(() => {
      const logsContainer = document.querySelector('.logs-container');
      if (logsContainer) {
        // Scroll to the bottom of the logs container
        logsContainer.scrollTop = logsContainer.scrollHeight;
      }
    }, 100);
  };

  // Request controller for cancelling requests on unmount
  const requestControllerRef = useRef(null);

  // Load system info and logs on mount
  useEffect(() => {
    // Create a new request controller
    requestControllerRef.current = createRequestController();
    
    loadSystemInfo();
    loadLogs();
    
    // No automatic polling intervals for system info - user will manually refresh when needed
    // This prevents unnecessary network traffic and processing
    console.log('System page loaded - no automatic polling for system info');
    
    // Clean up any existing WebSocket subscriptions and cancel pending requests on unmount
    return () => {
      if (window.wsClient && typeof window.wsClient.unsubscribe === 'function') {
        console.log('Cleaning up any WebSocket subscriptions on unmount');
        window.wsClient.unsubscribe('system/logs');
      }
      
      if (requestControllerRef.current) {
        requestControllerRef.current.abort();
      }
    };
  }, []);
  
  // Load logs when log level or count changes
  useEffect(() => {
    console.log(`SystemView: Log level changed to ${logLevel} or count changed to ${logCount}`);
    
    // Only load logs via HTTP API once on initial load
    // WebSocket polling will handle updates after that
    if (logs.length === 0) {
      console.log('Initial logs load via HTTP API');
      loadLogs();
    } else {
      console.log('Filtering existing logs based on new log level');
      // Filter existing logs based on the new log level from the ref
      const currentLogLevel = logLevelRef.current;
      console.log(`Filtering existing logs using logLevelRef.current: ${currentLogLevel}`);
      
      setLogs(prevLogs => {
        // Special case for debug level - include all logs
        return prevLogs.filter(log => {
          return log_level_meets_minimum(log.level, currentLogLevel);
        });
      });
    }
  }, [logLevel, logCount]);
  
  // Load system info from API
  const loadSystemInfo = async () => {
    try {
      setIsLoading(true);
      
      const data = await fetchJSON('/api/system/info', {
        signal: requestControllerRef.current?.signal,
        timeout: 15000, // 15 second timeout
        retries: 2,     // Retry twice
        retryDelay: 1000 // 1 second between retries
      });
      
      setSystemInfo(data);
      setHasData(true);
    } catch (error) {
      // Only show error if the request wasn't cancelled
      if (error.message !== 'Request was cancelled') {
        console.error('Error loading system info:', error);
        // Don't show error message for this, just log it
        setHasData(false);
      }
    } finally {
      setIsLoading(false);
    }
  };
  
  // Load logs from API
  const loadLogs = async () => {
    try {
      // Always use debug level to get all logs, then filter on frontend
      const currentLogLevel = logLevelRef.current;
      console.log(`Loading logs from API with level: debug (to get all logs, will filter on frontend)`);
      
      const data = await fetchJSON(`/api/system/logs?level=debug&count=${logCount}`, {
        signal: requestControllerRef.current?.signal,
        timeout: 20000, // 20 second timeout for potentially large log data
        retries: 1,     // Retry once
        retryDelay: 1000 // 1 second between retries
      });
      
      // Check if we have logs
      if (data.logs && Array.isArray(data.logs)) {
        // Check if logs are already structured objects or raw strings
        if (data.logs.length > 0 && typeof data.logs[0] === 'object' && data.logs[0].level) {
        // Logs are already structured objects, filter them based on the current log level
        let filteredLogs = data.logs.filter(log => {
            return log_level_meets_minimum(log.level, currentLogLevel);
          });
        
        console.log(`Filtered ${data.logs.length} logs to ${filteredLogs.length} based on log level ${currentLogLevel}`);
        setLogs(filteredLogs);
        } else {
          // Logs are raw strings, parse them into structured objects
          const parsedLogs = data.logs.map(logLine => {
            // Parse log line (format: [TIMESTAMP] [LEVEL] MESSAGE)
            let timestamp = 'Unknown';
            let level = 'debug';
            let message = logLine;
            
            // Try to extract timestamp and level using regex
            const logRegex = /\[(.*?)\]\s*\[(.*?)\]\s*(.*)/;
            const match = logLine.match(logRegex);
            
            if (match && match.length >= 4) {
              timestamp = match[1];
              level = match[2].toLowerCase();
              message = match[3];
              
              // Normalize log level
              if (level === 'warn') {
                level = 'warning';
              }
            }
            
            return {
              timestamp,
              level,
              message
            };
          });
          
          // Filter the parsed logs based on the current log level
          let filteredLogs = parsedLogs.filter(log => {
              return log_level_meets_minimum(log.level, currentLogLevel);
            });
          
          console.log(`Filtered ${parsedLogs.length} parsed logs to ${filteredLogs.length} based on log level ${currentLogLevel}`);
          setLogs(filteredLogs);
        }
      } else {
        setLogs([]);
      }
    } catch (error) {
      console.error('Error loading logs:', error);
      showStatusMessage('Error loading logs: ' + error.message);
    }
  };
  
  // Clear logs
  const clearLogs = async () => {
    if (!confirm('Are you sure you want to clear all logs?')) {
      return;
    }
    
    try {
      showStatusMessage('Clearing logs...');
      
      await enhancedFetch('/api/system/logs/clear', {
        method: 'POST',
        signal: requestControllerRef.current?.signal,
        timeout: 15000, // 15 second timeout
        retries: 1,     // Retry once
        retryDelay: 1000 // 1 second between retries
      });
      
      showStatusMessage('Logs cleared successfully');
      loadLogs(); // Reload logs after clearing
    } catch (error) {
      console.error('Error clearing logs:', error);
      showStatusMessage('Error clearing logs: ' + error.message);
    }
  };
  
  // Restart system
  const restartSystem = async () => {
    if (!confirm('Are you sure you want to restart the system?')) {
      return;
    }
    
    try {
      setIsRestarting(true);
      showStatusMessage('Restarting system...');
      
      await enhancedFetch('/api/system/restart', {
        method: 'POST',
        signal: requestControllerRef.current?.signal,
        timeout: 30000, // 30 second timeout for system restart
        retries: 0      // No retries for system restart
      });
      
      showStatusMessage('System is restarting. Please wait...');
      
      // Wait for system to restart
      setTimeout(() => {
        window.location.reload();
      }, 10000);
    } catch (error) {
      console.error('Error restarting system:', error);
      showStatusMessage('Error restarting system: ' + error.message);
      setIsRestarting(false);
    }
  };
  
  // Shutdown system
  const shutdownSystem = async () => {
    if (!confirm('Are you sure you want to shut down the system?')) {
      return;
    }
    
    try {
      setIsShuttingDown(true);
      showStatusMessage('Shutting down system...');
      
      await enhancedFetch('/api/system/shutdown', {
        method: 'POST',
        signal: requestControllerRef.current?.signal,
        timeout: 30000, // 30 second timeout for system shutdown
        retries: 0      // No retries for system shutdown
      });
      
      showStatusMessage('System is shutting down. You will need to manually restart it.');
    } catch (error) {
      console.error('Error shutting down system:', error);
      showStatusMessage('Error shutting down system: ' + error.message);
      setIsShuttingDown(false);
    }
  };
  
  // Check if WebSocket client is initialized
  useEffect(() => {
    if (!window.wsClient) {
      console.log('WebSocket client not available in SystemView, it should be initialized in preact-app.js');
    } else {
      console.log('WebSocket client is available in SystemView');
    }
  }, []);

  return html`
    <section id="system-page" class="page">
      <${SystemControls} 
        restartSystem=${restartSystem} 
        shutdownSystem=${shutdownSystem} 
        isRestarting=${isRestarting} 
        isShuttingDown=${isShuttingDown} 
      />
      
      <${ContentLoader}
        isLoading=${isLoading}
        hasData=${hasData}
        loadingMessage="Loading system information..."
        emptyMessage="System information not available. Please try again later."
      >
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <${SystemInfo} systemInfo=${systemInfo} formatUptime=${formatUptime} />
          <${MemoryStorage} systemInfo=${systemInfo} formatBytes=${formatBytes} />
        </div>
        
        <div class="grid grid-cols-1 gap-4 mb-4">
          <${StreamStorage} systemInfo=${systemInfo} formatBytes=${formatBytes} />
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <${NetworkInfo} systemInfo=${systemInfo} />
          <${StreamsInfo} systemInfo=${systemInfo} formatBytes=${formatBytes} />
        </div>
        
        <${LogsView} 
          logs=${logs} 
          logLevel=${logLevel} 
          logCount=${logCount} 
          setLogLevel=${handleSetLogLevel} 
          setLogCount=${setLogCount} 
          loadLogs=${loadLogs} 
          clearLogs=${clearLogs} 
        />
        
        <${LogsPoller}
          logLevel=${logLevel}
          logCount=${logCount}
          onLogsReceived=${handleLogsReceived}
        />
      <//>
    </section>
  `;
}

/**
 * Load SystemView component
 */
export function loadSystemView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;
  
  // Render the SystemView component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${SystemView} />`, mainContent);
    
    // Refresh system info immediately after rendering
    setTimeout(() => {
      const event = new CustomEvent('refresh-system-info');
      window.dispatchEvent(event);
    }, 100);
  });
}

// Add a global event listener for refreshing system info
window.addEventListener('load', () => {
  window.addEventListener('refresh-system-info', async () => {
    try {
      const response = await enhancedFetch('/api/system/info', {
        timeout: 15000, // 15 second timeout
        retries: 1,     // Retry once
        retryDelay: 1000 // 1 second between retries
      });
      
      if (response.ok) {
        console.log('System info refreshed');
      }
    } catch (error) {
      console.error('Error refreshing system info:', error);
    }
  });
});
