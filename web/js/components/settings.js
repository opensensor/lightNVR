/**
 * LightNVR Web Interface Settings Management
 * Contains functionality for managing system settings
 */

/**
 * Load settings with improved error handling and progress indication
 */
function loadSettings() {
    const settingsContainer = document.querySelector('.settings-container');
    if (!settingsContainer) return;

    // Create and show progress indicator
    const progressContainer = createProgressIndicator(settingsContainer, 'Loading settings');
    updateProgress(progressContainer, 10, 'Connecting to server...');

    // Set a timeout to detect slow server responses
    const timeoutId = setTimeout(() => {
        updateProgress(progressContainer, 30, 'Server is taking longer than expected...');
    }, 2000);

    // Fetch settings from the server with improved error handling
    fetch('/api/settings', {
        method: 'GET',
        headers: {
            'Cache-Control': 'no-cache',
            'Pragma': 'no-cache'
        }
    })
        .then(response => {
            updateProgress(progressContainer, 50, 'Received server response...');
            
            if (!response.ok) {
                return response.text().then(text => {
                    console.error('Server returned error:', text);
                    throw new Error(`Server error: ${response.status} ${response.statusText}`);
                });
            }
            
            return response.text().then(text => {
                updateProgress(progressContainer, 70, 'Processing data...');
                try {
                    return JSON.parse(text);
                } catch (e) {
                    console.error('Invalid JSON received:', text);
                    throw new Error('Invalid response format');
                }
            });
        })
        .then(settings => {
            updateProgress(progressContainer, 80, 'Applying settings...');
            console.log('Received settings:', settings); // Debug log

            if (!settings || typeof settings !== 'object') {
                console.error('Unexpected settings format:', settings);
                throw new Error('Unexpected settings format');
            }

            // Update form fields with settings values - with extensive error handling
            try {
                const logLevel = document.getElementById('setting-log-level');
                if (logLevel) logLevel.value = typeof settings.log_level === 'number' ? settings.log_level : 2;

                const storagePath = document.getElementById('setting-storage-path');
                if (storagePath) storagePath.value = typeof settings.storage_path === 'string' ?
                    settings.storage_path : '/var/lib/lightnvr/recordings';

                const maxStorage = document.getElementById('setting-max-storage');
                if (maxStorage) maxStorage.value = typeof settings.max_storage === 'number' ?
                    settings.max_storage : 0;

                const retention = document.getElementById('setting-retention');
                if (retention) retention.value = typeof settings.retention === 'number' ?
                    settings.retention : 30;

                const autoDelete = document.getElementById('setting-auto-delete');
                if (autoDelete) autoDelete.checked = settings.auto_delete === true ||
                    settings.auto_delete === 'true';

                const webPort = document.getElementById('setting-web-port');
                if (webPort) webPort.value = typeof settings.web_port === 'number' ?
                    settings.web_port : 8080;

                const authEnabled = document.getElementById('setting-auth-enabled');
                if (authEnabled) authEnabled.checked = settings.auth_enabled === true ||
                    settings.auth_enabled === 'true';

                const username = document.getElementById('setting-username');
                if (username) username.value = typeof settings.username === 'string' ?
                    settings.username : 'admin';

                const password = document.getElementById('setting-password');
                if (password) password.value = ''; // Always blank for security

                const bufferSize = document.getElementById('setting-buffer-size');
                if (bufferSize) bufferSize.value = typeof settings.buffer_size === 'number' ?
                    settings.buffer_size : 1024;

                const useSwap = document.getElementById('setting-use-swap');
                if (useSwap) useSwap.checked = settings.use_swap === true ||
                    settings.use_swap === 'true';

                const swapSize = document.getElementById('setting-swap-size');
                if (swapSize) swapSize.value = typeof settings.swap_size === 'number' ?
                    settings.swap_size : 128;

                updateProgress(progressContainer, 100, 'Settings loaded successfully');
                console.log('Settings loaded successfully');
            } catch (error) {
                console.error('Error processing settings:', error);
                throw new Error('Failed to process settings: ' + error.message);
            }
        })
        .catch(error => {
            console.error('Error loading settings:', error);
            updateProgress(progressContainer, 100, 'Error: ' + error.message, true);
            showStatusMessage('Error loading settings: ' + error.message, 'error');
            
            // Add retry button
            const retryButton = document.createElement('button');
            retryButton.textContent = 'Retry';
            retryButton.className = 'btn-primary';
            retryButton.style.marginTop = '10px';
            retryButton.addEventListener('click', () => {
                removeProgressIndicator(progressContainer);
                loadSettings();
            });
            progressContainer.appendChild(retryButton);
        })
        .finally(() => {
            clearTimeout(timeoutId);
            
            // Remove progress indicator after a delay
            setTimeout(() => {
                removeProgressIndicator(progressContainer);
            }, 1000);
        });
}

/**
 * Save settings with improved timeout handling and error recovery
 */
function saveSettings() {
    const settingsContainer = document.querySelector('.settings-container');
    if (!settingsContainer) return;

    // Show loading indicator with timeout information
    const loadingIndicator = document.createElement('div');
    loadingIndicator.className = 'loading-indicator';
    loadingIndicator.innerHTML = '<div class="spinner"></div><p>Saving settings...</p><div class="timeout-counter">30</div>';
    loadingIndicator.style.position = 'absolute';
    loadingIndicator.style.top = '0';
    loadingIndicator.style.left = '0';
    loadingIndicator.style.width = '100%';
    loadingIndicator.style.height = '100%';
    loadingIndicator.style.backgroundColor = 'rgba(0, 0, 0, 0.5)';
    loadingIndicator.style.display = 'flex';
    loadingIndicator.style.flexDirection = 'column';
    loadingIndicator.style.justifyContent = 'center';
    loadingIndicator.style.alignItems = 'center';
    loadingIndicator.style.zIndex = '1000';
    loadingIndicator.style.color = 'white';
    
    // Add spinner style
    const spinner = loadingIndicator.querySelector('.spinner');
    spinner.style.width = '40px';
    spinner.style.height = '40px';
    spinner.style.border = '4px solid rgba(255, 255, 255, 0.3)';
    spinner.style.borderRadius = '50%';
    spinner.style.borderTopColor = 'white';
    spinner.style.animation = 'spin 1s ease-in-out infinite';
    spinner.style.marginBottom = '10px';
    
    // Style the timeout counter
    const timeoutCounter = loadingIndicator.querySelector('.timeout-counter');
    timeoutCounter.style.marginTop = '10px';
    timeoutCounter.style.fontSize = '14px';
    timeoutCounter.style.color = '#ccc';
    
    settingsContainer.style.position = 'relative';
    settingsContainer.appendChild(loadingIndicator);
    
    // Set up timeout counter
    let timeLeft = 30;
    const countdownInterval = setInterval(() => {
        timeLeft--;
        timeoutCounter.textContent = timeLeft;
        
        if (timeLeft <= 10) {
            timeoutCounter.style.color = '#ff6b6b';
        }
        
        if (timeLeft <= 0) {
            clearInterval(countdownInterval);
        }
    }, 1000);
    
    // Set up request timeout
    const timeoutId = setTimeout(() => {
        clearInterval(countdownInterval);
        
        // Show timeout error
        const statusEl = document.getElementById('settings-status');
        if (statusEl) {
            statusEl.textContent = 'Error: Request timed out. The server may still be processing your request.';
            statusEl.className = 'status-message error';
            statusEl.style.display = 'block';
            
            // Add retry button
            const retryButton = document.createElement('button');
            retryButton.textContent = 'Retry';
            retryButton.className = 'btn-primary';
            retryButton.style.marginTop = '10px';
            retryButton.addEventListener('click', () => {
                statusEl.style.display = 'none';
                saveSettings();
            });
            statusEl.appendChild(retryButton);
        } else {
            showStatusMessage('Error: Request timed out', 'error');
        }
        
        // Remove loading indicator
        if (settingsContainer.contains(loadingIndicator)) {
            settingsContainer.removeChild(loadingIndicator);
        }
    }, 30000); // 30 second timeout

    try {
        // Collect all settings from the form with validation
        const settings = {
            log_level: parseInt(document.getElementById('setting-log-level')?.value || '2', 10),
            storage_path: document.getElementById('setting-storage-path')?.value || '/var/lib/lightnvr/recordings',
            max_storage: parseInt(document.getElementById('setting-max-storage')?.value || '0', 10),
            retention: parseInt(document.getElementById('setting-retention')?.value || '30', 10),
            auto_delete: document.getElementById('setting-auto-delete')?.checked || true,
            web_port: parseInt(document.getElementById('setting-web-port')?.value || '8080', 10),
            auth_enabled: document.getElementById('setting-auth-enabled')?.checked || true,
            username: document.getElementById('setting-username')?.value || 'admin',
            password: document.getElementById('setting-password')?.value || '********',
            buffer_size: parseInt(document.getElementById('setting-buffer-size')?.value || '1024', 10),
            use_swap: document.getElementById('setting-use-swap')?.checked || true,
            swap_size: parseInt(document.getElementById('setting-swap-size')?.value || '128', 10)
        };

        // Basic validation
        if (settings.web_port < 1 || settings.web_port > 65535) {
            throw new Error('Web port must be between 1 and 65535');
        }

        if (settings.buffer_size < 128) {
            throw new Error('Buffer size must be at least 128 KB');
        }

        if (settings.swap_size < 32) {
            throw new Error('Swap size must be at least 32 MB');
        }

        // Send settings to the server with improved error handling and timeout
        fetch('/api/settings', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(settings)
        })
        .then(response => {
            if (!response.ok) {
                return response.text().then(text => {
                    try {
                        const data = JSON.parse(text);
                        throw new Error(data.error || 'Failed to save settings');
                    } catch (e) {
                        throw new Error(`Server error: ${response.status} ${response.statusText}`);
                    }
                });
            }
            return { success: true };
        })
        .then(() => {
            console.log('Settings saved successfully');
            
            // Clear timeout and interval
            clearTimeout(timeoutId);
            clearInterval(countdownInterval);
            
            // Show success message
            const statusEl = document.getElementById('settings-status');
            if (statusEl) {
                statusEl.textContent = 'Settings saved successfully';
                statusEl.className = 'status-message success';
                statusEl.style.display = 'block';
                
                // Hide after 3 seconds
                setTimeout(() => {
                    statusEl.style.display = 'none';
                }, 3000);
            } else {
                // Fallback to the global status message
                showStatusMessage('Settings saved successfully');
            }
            
            // Remove loading indicator
            if (settingsContainer.contains(loadingIndicator)) {
                settingsContainer.removeChild(loadingIndicator);
            }
        })
        .catch(error => {
            console.error('Error saving settings:', error);
            
            // Clear timeout and interval
            clearTimeout(timeoutId);
            clearInterval(countdownInterval);
            
            // Show error message
            const statusEl = document.getElementById('settings-status');
            if (statusEl) {
                statusEl.textContent = 'Error saving settings: ' + error.message;
                statusEl.className = 'status-message error';
                statusEl.style.display = 'block';
                
                // Add retry button
                const retryButton = document.createElement('button');
                retryButton.textContent = 'Retry';
                retryButton.className = 'btn-primary';
                retryButton.style.marginTop = '10px';
                retryButton.addEventListener('click', () => {
                    statusEl.style.display = 'none';
                    saveSettings();
                });
                statusEl.appendChild(retryButton);
            } else {
                // Fallback to the global status message
                showStatusMessage('Error saving settings: ' + error.message, 'error');
            }
            
            // Remove loading indicator
            if (settingsContainer.contains(loadingIndicator)) {
                settingsContainer.removeChild(loadingIndicator);
            }
        });
    } catch (error) {
        console.error('Error preparing settings:', error);
        
        // Clear timeout and interval
        clearTimeout(timeoutId);
        clearInterval(countdownInterval);
        
        // Show error message
        const statusEl = document.getElementById('settings-status');
        if (statusEl) {
            statusEl.textContent = 'Error: ' + error.message;
            statusEl.className = 'status-message error';
            statusEl.style.display = 'block';
            
            // Add retry button
            const retryButton = document.createElement('button');
            retryButton.textContent = 'Retry';
            retryButton.className = 'btn-primary';
            retryButton.style.marginTop = '10px';
            retryButton.addEventListener('click', () => {
                statusEl.style.display = 'none';
                saveSettings();
            });
            statusEl.appendChild(retryButton);
        } else {
            showStatusMessage('Error: ' + error.message, 'error');
        }
        
        // Remove loading indicator
        if (settingsContainer.contains(loadingIndicator)) {
            settingsContainer.removeChild(loadingIndicator);
        }
    }
}


/**
 * Setup settings page event handlers
 */
function setupSettingsHandlers() {
    // Save button click handler
    const saveButton = document.getElementById('save-settings-btn');
    if (saveButton) {
        saveButton.addEventListener('click', saveSettings);
    }
}
