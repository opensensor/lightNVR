/**
 * LightNVR Web Interface System Management
 * Contains functionality for system operations (info, logs, restart, shutdown, backup)
 */

// WebSocket client instance
let wsClient = null;
// Flag to track if we're subscribed to system logs
let subscribedToSystemLogs = false;
// Track the timestamp of the last log received
let lastLogTimestamp = null;

/**
 * Set up event handlers for system page
 */
function setupSystemHandlers() {
    // Initialize WebSocket client if not already initialized
    if (!wsClient && typeof WebSocketClient === 'function') {
        wsClient = new WebSocketClient();
        console.log('WebSocket client initialized for system page');
    }
    
    // Set up refresh button
    const refreshBtn = document.getElementById('refresh-system-btn');
    if (refreshBtn) {
        refreshBtn.addEventListener('click', loadSystemInfo);
    }
    
    // Set up restart button
    const restartBtn = document.getElementById('restart-btn');
    if (restartBtn) {
        restartBtn.addEventListener('click', restartService);
    }
    
    // Set up shutdown button
    const shutdownBtn = document.getElementById('shutdown-btn');
    if (shutdownBtn) {
        shutdownBtn.addEventListener('click', shutdownService);
    }
    
    // Clear logs button is now handled by Alpine.js in system.html
    
    // Set up backup config button
    const backupConfigBtn = document.getElementById('backup-config-btn');
    if (backupConfigBtn) {
        backupConfigBtn.addEventListener('click', backupConfig);
    }
    
    // Set up auto-refresh timer (every 30 seconds)
    setInterval(loadSystemInfo, 30000);
}

/**
 * Load system information
 */
function loadSystemInfo() {
    const systemContainer = document.querySelector('.system-container');
    if (!systemContainer) return;

    showLoading(systemContainer);

    // Fetch system information from API
    fetch('/api/system')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load system information');
            }
            return response.json();
        })
        .then(data => {
            // Update system information
            document.getElementById('system-version').textContent = data.version || '';

            // Format uptime
            const uptime = data.uptime || 0;
            const days = Math.floor(uptime / 86400);
            const hours = Math.floor((uptime % 86400) / 3600);
            const minutes = Math.floor((uptime % 3600) / 60);
            document.getElementById('system-uptime').textContent = `${days}d ${hours}h ${minutes}m`;

            document.getElementById('system-cpu').textContent = `${data.cpu_usage || 0}%`;
            document.getElementById('system-memory').textContent = `${data.memory_usage || 0} MB / ${data.memory_total || 0} MB`;
            document.getElementById('system-storage').textContent = `${data.storage_usage || 0} GB / ${data.storage_total || 0} GB`;

            // Update stream statistics
            document.getElementById('system-active-streams').textContent = `${data.active_streams || 0} / ${data.max_streams || 0}`;
            document.getElementById('system-recording-streams').textContent = `${data.recording_streams || 0}`;
            document.getElementById('system-received').textContent = `${data.data_received || 0} MB`;
            document.getElementById('system-recorded').textContent = `${data.data_recorded || 0} MB`;

            // Check daemon mode status using the daemon_mode property from system info
            checkDaemonMode(data.daemon_mode);

            // Load logs
            loadSystemLogs();
        })
        .catch(error => {
            console.error('Error loading system information:', error);
            // Use placeholder data
            document.getElementById('system-version').textContent = '';
            document.getElementById('system-uptime').textContent = '0d 0h 0m';
            document.getElementById('system-cpu').textContent = '0%';
            document.getElementById('system-memory').textContent = '0 MB / 0 MB';
            document.getElementById('system-storage').textContent = '0 GB / 0 GB';
            document.getElementById('system-active-streams').textContent = '0 / 0';
            document.getElementById('system-recording-streams').textContent = '0';
            document.getElementById('system-received').textContent = '0 MB';
            document.getElementById('system-recorded').textContent = '0 MB';
        })
        .finally(() => {
            hideLoading(systemContainer);
        });
}

/**
 * Load system logs
 * Uses WebSocket if available, falls back to HTTP
 */
function loadSystemLogs() {
    const logsContainer = document.getElementById('system-logs');
    if (!logsContainer) return;

    // If WebSocket client is available and connected, use it
    if (wsClient && wsClient.isConnected()) {
        // Subscribe to system logs if not already subscribed
        if (!subscribedToSystemLogs) {
            console.log('Subscribing to system logs via WebSocket');
            
            // Register handler for system logs updates
            wsClient.on('update', 'system/logs', (payload) => {
                console.log('Received system logs update via WebSocket:', payload);
                
                if (payload && payload.logs && Array.isArray(payload.logs)) {
                    const logsContainer = document.getElementById('system-logs');
                    if (logsContainer) {
                        // Clear existing logs
                        logsContainer.innerHTML = '';
                        
                        // Create log entries
                        payload.logs.forEach(log => {
                            // Create log entry element
                            const logEntry = document.createElement('div');
                            logEntry.className = 'log-entry';
                            
                            // Add timestamp
                            const timestamp = document.createElement('span');
                            timestamp.className = 'text-gray-500 dark:text-gray-400';
                            timestamp.textContent = log.timestamp || 'Unknown';
                            logEntry.appendChild(timestamp);
                            
                            // Add level
                            const level = document.createElement('span');
                            level.className = 'mx-2';
                            
                            // Format level based on value
                            let levelClass = '';
                            let levelText = (log.level || 'info').toUpperCase();
                            
                            switch (log.level) {
                                case 'error':
                                    levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
                                    break;
                                case 'warning':
                                    levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
                                    break;
                                case 'info':
                                    levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200';
                                    break;
                                case 'debug':
                                    levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
                                    break;
                                default:
                                    levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
                            }
                            
                            level.className += ' ' + levelClass;
                            level.textContent = levelText;
                            logEntry.appendChild(level);
                            
                            // Add message
                            const message = document.createElement('span');
                            message.className = log.level === 'error' ? 'text-red-600 dark:text-red-400' : '';
                            message.textContent = log.message || '';
                            logEntry.appendChild(message);
                            
                            // Add log entry to container
                            logsContainer.appendChild(logEntry);
                        });
                        
                        // Scroll to bottom of logs container
                        logsContainer.scrollTop = logsContainer.scrollHeight;
                        
                        // Update the last log timestamp if we have logs
                        if (payload.logs.length > 0) {
                            lastLogTimestamp = payload.logs[payload.logs.length - 1].timestamp;
                            console.log('Updated last log timestamp:', lastLogTimestamp);
                        }
                    }
                }
            });
            
            // Subscribe to system logs topic with last timestamp if available
            wsClient.subscribe('system/logs', lastLogTimestamp ? { since: lastLogTimestamp } : null);
            subscribedToSystemLogs = true;
        }
    } else {
        console.log('WebSocket not available or not connected, using HTTP fallback for system logs');
        
        // Fetch logs from API (HTTP fallback)
        fetch('/api/system/logs')
            .then(response => {
                if (!response.ok) {
                    throw new Error('Failed to load system logs');
                }
                return response.json();
            })
            .then(data => {
                if (data.logs && Array.isArray(data.logs)) {
                    // Clear existing logs
                    logsContainer.innerHTML = '';
                    
                    // Create log entries
                    data.logs.forEach(log => {
                        // Create log entry element
                        const logEntry = document.createElement('div');
                        logEntry.className = 'log-entry';
                        
                        // Add timestamp
                        const timestamp = document.createElement('span');
                        timestamp.className = 'text-gray-500 dark:text-gray-400';
                        timestamp.textContent = log.timestamp || 'Unknown';
                        logEntry.appendChild(timestamp);
                        
                        // Add level
                        const level = document.createElement('span');
                        level.className = 'mx-2';
                        
                        // Format level based on value
                        let levelClass = '';
                        let levelText = (log.level || 'info').toUpperCase();
                        
                        switch (log.level) {
                            case 'error':
                                levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
                                break;
                            case 'warning':
                                levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
                                break;
                            case 'info':
                                levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200';
                                break;
                            case 'debug':
                                levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
                                break;
                            default:
                                levelClass = 'px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
                        }
                        
                        level.className += ' ' + levelClass;
                        level.textContent = levelText;
                        logEntry.appendChild(level);
                        
                        // Add message
                        const message = document.createElement('span');
                        message.className = log.level === 'error' ? 'text-red-600 dark:text-red-400' : '';
                        message.textContent = log.message || '';
                        logEntry.appendChild(message);
                        
                        // Add log entry to container
                        logsContainer.appendChild(logEntry);
                    });
                    
                    // Scroll to bottom of logs container
                    logsContainer.scrollTop = logsContainer.scrollHeight;
                    
                    // Update the last log timestamp if we have logs
                    if (data.logs.length > 0) {
                        lastLogTimestamp = data.logs[data.logs.length - 1].timestamp;
                        console.log('Updated last log timestamp:', lastLogTimestamp);
                    }
                } else {
                    logsContainer.innerHTML = '<div class="text-gray-500 dark:text-gray-400">No logs available</div>';
                }
            })
            .catch(error => {
                console.error('Error loading system logs:', error);
                logsContainer.textContent = 'Error loading logs';
                
                // If WebSocket is available but not connected, try to connect
                if (wsClient && !wsClient.isConnected()) {
                    console.log('Attempting to reconnect WebSocket');
                    wsClient.connect();
                }
            });
    }
}

/**
 * Restart service
 */
function restartService() {
    if (!confirm('Are you sure you want to restart the LightNVR service?')) {
        return;
    }

    const systemContainer = document.querySelector('.system-container');
    if (systemContainer) {
        showLoading(systemContainer);
    }

    // Send restart request to API
    fetch('/api/system/restart', {
        method: 'POST'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to restart service');
            }
            return response.json();
        })
        .then(data => {
            hideLoading(systemContainer);

            // Update status
            document.getElementById('status-indicator').className = 'status-warning';
            document.getElementById('status-text').textContent = 'System restarting...';

            // Show message
            showStatusMessage('Service restart initiated. The system will be unavailable for a few moments.', 5000);

            // After a delay, check system status
            setTimeout(checkSystemStatus, 5000);
        })
        .catch(error => {
            console.error('Error restarting service:', error);
            showStatusMessage('Error restarting service: ' + error.message, 5000);

            if (systemContainer) {
                hideLoading(systemContainer);
            }
        });
}

/**
 * Shutdown service
 */
function shutdownService() {
    if (!confirm('Are you sure you want to shutdown the LightNVR service?')) {
        return;
    }

    const systemContainer = document.querySelector('.system-container');
    if (systemContainer) {
        showLoading(systemContainer);
    }

    // Send shutdown request to API
    fetch('/api/system/shutdown', {
        method: 'POST'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to shutdown service');
            }
            return response.json();
        })
        .then(data => {
            hideLoading(systemContainer);

            // Update status
            document.getElementById('status-indicator').className = 'status-danger';
            document.getElementById('status-text').textContent = 'System shutting down...';

            // Show message
            showStatusMessage('Service shutdown initiated. The system will become unavailable shortly.', 5000);
        })
        .catch(error => {
            console.error('Error shutting down service:', error);
            showStatusMessage('Error shutting down service: ' + error.message, 5000);

            if (systemContainer) {
                hideLoading(systemContainer);
            }
        });
}

// The clearLogs function has been removed as it's now handled by the Alpine.js component in system.html

/**
 * Backup configuration
 */
function backupConfig() {
    const systemContainer = document.querySelector('.system-container');
    if (systemContainer) {
        showLoading(systemContainer);
    }

    // Send backup request to API
    fetch('/api/system/backup', {
        method: 'POST'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to create backup');
            }
            return response.json();
        })
        .then(data => {
            if (data.backupUrl) {
                // Create a link and trigger download
                const link = document.createElement('a');
                link.href = data.backupUrl;
                link.download = data.filename || 'lightnvr-backup.json';
                document.body.appendChild(link);
                link.click();
                document.body.removeChild(link);

                // Show message
                showStatusMessage('Configuration backup created and downloaded successfully', 5000);
            } else {
                showStatusMessage('Configuration backup created successfully', 5000);
            }
        })
        .catch(error => {
            console.error('Error creating backup:', error);
            showStatusMessage('Error creating backup: ' + error.message, 5000);
        })
        .finally(() => {
            if (systemContainer) {
                hideLoading(systemContainer);
            }
        });
}

/**
 * Check system status after restart
 */
function checkSystemStatus() {
    fetch('/api/system/status')
        .then(response => {
            if (!response.ok) {
                // If server not responding yet, try again after delay
                setTimeout(checkSystemStatus, 2000);
                return;
            }
            return response.json();
        })
        .then(data => {
            if (data && data.status === 'ok') {
                // Service is back online
                document.getElementById('status-indicator').className = 'status-ok';
                document.getElementById('status-text').textContent = 'System running normally';

                // Reload system information
                loadSystemInfo();
            } else {
                // Not ready yet, try again
                setTimeout(checkSystemStatus, 2000);
            }
        })
        .catch(error => {
            // Error or server not responding, try again
            console.log('Waiting for system to restart...');
            setTimeout(checkSystemStatus, 2000);
        });
}

/**
 * Check if system is running in daemon mode and update UI accordingly
 * Uses the daemon_mode property from system info instead of making a separate request
 */
function checkDaemonMode(daemonMode) {
    const restartBtn = document.getElementById('restart-btn');
    if (!restartBtn) return;
    
    if (!daemonMode) {
        // Not running in daemon mode, disable restart button
        restartBtn.disabled = true;
        restartBtn.title = 'Restart is only available when running in daemon mode';
        restartBtn.classList.add('btn-disabled');
    } else {
        // Running in daemon mode, enable restart button
        restartBtn.disabled = false;
        restartBtn.title = 'Restart the LightNVR service';
        restartBtn.classList.remove('btn-disabled');
    }
}
